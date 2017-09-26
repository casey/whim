use decimal::Decimal;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum Product {
  BtcUsd,
  EthUsd,
  LtcUsd,
}

impl Product {
  pub fn all() -> Vec<Product> {
    vec![
      Product::BtcUsd,
      Product::EthUsd,
      Product::LtcUsd,
    ]
  }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
  Full,
  Heartbeat,
  Level2,
  Matches,
  Ticker,
}

impl Channel {
  pub fn all() -> Vec<Channel> {
    vec![
      Channel::Full,
      Channel::Heartbeat,
      Channel::Level2,
      Channel::Matches,
      Channel::Ticker,
    ]
  }
}


#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Side {
  Buy,
  Sell,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
  Limit,
  Market,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Reason {
  Canceled,
  Filled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription {
  pub name:        Channel,
  pub product_ids: Vec<Product>,
}

pub type DateTime = ::chrono::DateTime<::chrono::offset::Utc>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct QuoteCurrencyPrice(Decimal);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct QuoteCurrencyAmount(Decimal);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BaseCurrencyAmount(Decimal);

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IncomingMessage {
  Ticker(Ticker),
  Error{message: String},
  Subscriptions{channels: Vec<Subscription>},
  Done {
    product_id:     Product,
    order_id:       Uuid,
    side:           Side,
    reason:         Reason,
    sequence:       u64,
    price:          Option<QuoteCurrencyPrice>,
    time:           DateTime,
    remaining_size: Option<BaseCurrencyAmount>,
  },
  Received {
    product_id: Product,
    client_oid: Option<Uuid>,
    order_id:   Uuid,
    order_type: OrderType,
    side:       Side,
    sequence:   u64,
    time:       DateTime,
    price:      Option<QuoteCurrencyPrice>,
    size:       Option<BaseCurrencyAmount>,
    funds:      Option<QuoteCurrencyAmount>,
  },
  Open {
    product_id:     Product,
    order_id:       Uuid,
    side:           Side,
    sequence:       u64,
    price:          QuoteCurrencyPrice,
    time:           DateTime,
    remaining_size: BaseCurrencyAmount,
  },
  Match {
    product_id:     Product,
    maker_order_id: Uuid,
    taker_order_id: Uuid,
    price:          QuoteCurrencyPrice,
    sequence:       u64,
    side:           Side,
    size:           BaseCurrencyAmount,
    time:           DateTime,
    trade_id:       u64,
  },
  LastMatch {
    product_id:     Product,
    maker_order_id: Uuid,
    taker_order_id: Uuid,
    price:          QuoteCurrencyPrice,
    sequence:       u64,
    side:           Side,
    size:           BaseCurrencyAmount,
    time:           DateTime,
    trade_id:       u64,
  },
  Change {
    product_id: Product,
    order_id:   Uuid,
  },
  MarginProfileUpdate {
    product_id: Product,
    order_id:   Uuid,
  },
  Activate {
    product_id: Product,
    order_id:   Uuid,
  },
  Heartbeat {
    sequence:      u64,
    last_trade_id: u64,
    product_id:    Product,
    time:          DateTime,
  },
  Snapshot(OrderBookSnapshot),
  L2update(OrderBookUpdate),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookSnapshot {
  pub product_id: Product,
  pub bids:       Vec<(QuoteCurrencyPrice, BaseCurrencyAmount)>,
  pub asks:       Vec<(QuoteCurrencyPrice, BaseCurrencyAmount)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookUpdate {
  pub product_id: Product,
  pub changes:    Vec<(Side, QuoteCurrencyPrice, BaseCurrencyAmount)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
  product_id: Product,
  time:       Option<DateTime>,
  price:      QuoteCurrencyPrice,
  side:       Option<Side>,
  last_size:  Option<BaseCurrencyAmount>,
  trade_id:   Option<u64>,
  sequence:   u64,
  best_bid:   QuoteCurrencyPrice,
  best_ask:   QuoteCurrencyPrice,
  high_24h:   QuoteCurrencyPrice,
  open_24h:   QuoteCurrencyPrice,
  low_24h:    QuoteCurrencyPrice,
  volume_24h: BaseCurrencyAmount,
  volume_30d: BaseCurrencyAmount,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OutgoingMessage {
  Subscribe{channels: Vec<Subscription>}
}
