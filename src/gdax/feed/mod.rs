use futures::prelude::*;
use serde_json;
use std::collections::{BTreeMap, VecDeque};
use tokio_core::reactor::Handle;
use websocket::{ClientBuilder, WebSocketError, OwnedMessage};

pub mod message;

use self::message::{Subscription, Channel, Product, IncomingMessage, OutgoingMessage};

const LIVE_URL: &'static str = "wss://ws-feed.gdax.com";
const SANDBOX_URL: &'static str = "wss://ws-feed-public.sandbox.gdax.com";

type FeedFuture = Box<Future<Item=Feed, Error=WebSocketError>>;

pub struct FeedBuilder {
  sandbox:       bool,
  subscriptions: Vec<Subscription>,
}

impl FeedBuilder {
  pub fn new() -> FeedBuilder {
    FeedBuilder {
      sandbox:       false,
      subscriptions: vec![],
    }
  }

  pub fn sandbox(mut self, sandbox: bool) -> FeedBuilder {
    self.sandbox = sandbox;
    self
  }

  pub fn _subscribe(mut self, channel: Channel, product: Product) -> FeedBuilder {
    self.subscriptions.push(Subscription {
      name:        channel,
      product_ids: vec![product],
    });
    self
  }

  pub fn subscribe_to_all(mut self) -> FeedBuilder {
    for channel in Channel::all() {
      self.subscriptions.push(Subscription {
        name:        channel,
        product_ids: Product::all()
      });
    }
    self
  }

  pub fn connect(self, handle: &Handle) -> Result<FeedFuture, WebSocketError> {
    let url = if self.sandbox { SANDBOX_URL } else { LIVE_URL };

    let unboxed = ClientBuilder::new(url)?
      .async_connect_secure(None, &handle)
      .map(move |(connection, headers)| {
        info!("Connection to {} established", url);
        trace!("received headers: {:?}", headers);

        let mut buffer = VecDeque::new();

        buffer.push_front(Feed::serialize(OutgoingMessage::Subscribe {
          channels: self.subscriptions
        }).unwrap());

        let (sink, stream) = connection.split();
        Feed {
          buffer,
          closed: false,
          sink:   Box::new(sink),
          stream: Box::new(stream),
        }
      });

    Ok(Box::new(unboxed))
  }
}

#[derive(Debug)]
pub enum FeedError {
  Deserialization(serde_json::Error),
  Serialization(serde_json::Error),
  WebSocket(WebSocketError),
}

impl From<WebSocketError> for FeedError {
  fn from(error: WebSocketError) -> FeedError {
    FeedError::WebSocket(error)
  }
}

pub struct Feed {
  buffer: VecDeque<OwnedMessage>,
  closed: bool,
  sink:   Box<Sink<SinkItem=OwnedMessage, SinkError=WebSocketError>>,
  stream: Box<Stream<Item=OwnedMessage, Error=WebSocketError>>,
}

impl Feed {
  fn try_empty_buffer(&mut self) -> Result<Async<()>, FeedError> {
    while let Some(item) = self.buffer.pop_front() {
      if let AsyncSink::NotReady(item) = self.sink.start_send(item)?  {
        self.buffer.push_front(item);
        self.sink.poll_complete()?;
        return Ok(Async::NotReady);
      }
    }
    Ok(Async::Ready(()))
  }

  fn serialize(message: OutgoingMessage) -> Result<OwnedMessage, FeedError> {
    Ok(OwnedMessage::Text(serde_json::to_string(&message).map_err(FeedError::Serialization)?))
  }

  fn deserialize(text: &str) -> Result<IncomingMessage, FeedError> {
    serde_json::from_str(&text).map_err(FeedError::Deserialization)
  }
}

impl Stream for Feed {
  type Item  = IncomingMessage;
  type Error = FeedError;

  fn poll(&mut self) -> Poll<Option<IncomingMessage>, FeedError> {
    loop {
      self.try_empty_buffer()?;

      match try_ready!(self.stream.poll()) {
        ref message if self.closed => {
          warn!("Received message after close: {:?}", message);
        }
        Some(OwnedMessage::Close(close_data)) => {
          self.closed = true;
          self.buffer.push_back(OwnedMessage::Close(close_data));
        }
        Some(OwnedMessage::Ping(ping_bytes)) => {
          self.buffer.push_back(OwnedMessage::Pong(ping_bytes));
        }
        Some(OwnedMessage::Pong(pong_bytes)) => {
          warn!("Got pong message from GDAX: {:?}", pong_bytes);
        }
        Some(OwnedMessage::Binary(data)) => {
          warn!("Got binary message from GDAX: {:?}", data);
        }
        Some(OwnedMessage::Text(data)) => {
          let incoming_message = Feed::deserialize(&data)?;
          warn_incomplete_deserialization(&data, &incoming_message);
          return Ok(Async::Ready(Some(incoming_message)));
        }
        None => return Ok(Async::Ready(None)),
      }
    }
  }
}

impl Sink for Feed {
  type SinkItem  = OutgoingMessage;
  type SinkError = FeedError;

  fn start_send(&mut self, item: OutgoingMessage) -> Result<AsyncSink<OutgoingMessage>, FeedError> {
    self.try_empty_buffer()?;
    self.buffer.push_back(Feed::serialize(item)?);
    Ok(AsyncSink::Ready)
  }

  fn poll_complete(&mut self) -> Result<Async<()>, FeedError> {
    try_ready!(self.try_empty_buffer());
    assert!(self.buffer.is_empty());
    self.sink.poll_complete().map_err(FeedError::WebSocket)
  }
}

fn json_to_map(text: &str) -> BTreeMap<String, serde_json::Value> {
  match serde_json::from_str(&text).expect("failed to re-deserialize message") {
    serde_json::Value::Object(items) => items.into_iter().collect(),
    reserialized => {
      warn!("reserialized text was not JSON object: {:?}", reserialized);
      BTreeMap::new()
    }
  }
}

fn warn_incomplete_deserialization<T: ::serde::Serialize>(received: &str, deserialized: &T) {
  let received_keys = json_to_map(received);
  let reserialized_keys = json_to_map(&serde_json::to_string(deserialized).expect("failed to re-serialize message"));
  let mut printed_message = false;
  for (key, value) in received_keys.into_iter() {
    if !reserialized_keys.contains_key(&key) {
      if !printed_message {
        warn!("deserialized message missing keys: {}", received);
        printed_message = true;
      }

      warn!("key: {} {}", key, value)
    }
  }
}
