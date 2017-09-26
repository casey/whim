use gdax::feed::message::*;
use std::collections::BTreeMap;

pub struct OrderBook {
  product_id: Product,
  bids:       BTreeMap<QuoteCurrencyPrice, BaseCurrencyAmount>,
  asks:       BTreeMap<QuoteCurrencyPrice, BaseCurrencyAmount>,
}

impl OrderBook {
  pub fn from_snapshot(snapshot: &OrderBookSnapshot) -> OrderBook {
    OrderBook {
      product_id: snapshot.product_id,
      bids:       snapshot.bids.iter().cloned().collect(),
      asks:       snapshot.asks.iter().cloned().collect(),
    }
  }

  pub fn update(&mut self, update: &OrderBookUpdate) {
    assert_eq!(self.product_id, update.product_id, "OrderBook/update product ID mismatch: {:?} != {:?}", self.product_id, update.product_id);
    for change in update.changes.iter().cloned() {
      match change {
        (Side::Buy,  price, amount) => self.bids.insert(price, amount),
        (Side::Sell, price, amount) => self.asks.insert(price, amount),
      };
    }
  }
}
