use futures::prelude::*;
use std::collections::BTreeMap;
use tokio_core::reactor::Core;

use gdax::feed::message::*;
use gdax::order_book::OrderBook;

use gdax::feed::FeedBuilder;

use errors::*;

pub fn record(sandbox: bool) -> Result<(), Error> {
  info!("command: record");
  let mut core = Core::new().chain_err(|| "failed to create tokio Core")?;
  let mut order_books = BTreeMap::new();

  let feed_future = FeedBuilder::new()
    .sandbox(sandbox)
    .subscribe_to_all()
    .connect(&core.handle());

  let feed = core.run(feed_future.unwrap()).unwrap();

  let (sink, stream) = feed.split();

  let recorder = stream.filter_map(|message| match message {
    IncomingMessage::Snapshot(snapshot) => {
      info!("order book snapshot from GDAX: {:?}", snapshot);
      order_books.insert(snapshot.product_id, OrderBook::from_snapshot(&snapshot));
      None
    }
    IncomingMessage::L2update(update) => {
      info!("order book update from GDAX: {:?}", update);
      order_books
        .get_mut(&update.product_id)
        .ok_or_else(|| format!("No orderbook for {:?} but had no order book", update.product_id))
        // todo: deal with this
        .expect("Got update for missing orderbook")
        .update(&update);
      None
    }
    IncomingMessage::Error{message} => {
      error!("error message from GDAX: {}", message);
      None
    }
    other => {
      info!("message from GDAX: {:?}", other);
      None
    }
  }).forward(sink);

  // todo: deal with this
  core.run(recorder).unwrap();

  Ok(())
}
