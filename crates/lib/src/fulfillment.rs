use crate::{BuyOrders, OrderType, SellOrders, Trade};

/// Trait that abstracts a fulfillment engine. Implementors provide the logic
/// to match and execute trades between buy and sell orders.
pub trait FulfillmentEngine {
    fn fulfill(&mut self) -> Option<Trade>;
}

/// Adapter that allows using the existing `fulfill_orders` logic over
/// mutable references to `BuyOrders` and `SellOrders` via the
/// `FulfillmentEngine` trait.
pub struct OrderBookEngine<'a> {
    pub buys: &'a mut BuyOrders,
    pub sells: &'a mut SellOrders,
}

impl<'a> OrderBookEngine<'a> {
    pub fn new(buys: &'a mut BuyOrders, sells: &'a mut SellOrders) -> Self {
        Self { buys, sells }
    }
}

impl<'a> FulfillmentEngine for OrderBookEngine<'a> {
    fn fulfill(&mut self) -> Option<Trade> {
        let mut b_index = 0;
        let mut s_index = 0;

        while b_index < self.buys.len() && s_index < self.sells.len() {
            if self.buys.as_slice()[b_index].price < self.sells.as_slice()[s_index].price {
                b_index = b_index + 1;
            } else if self.buys.as_slice()[b_index].price > self.sells.as_slice()[s_index].price {
                s_index = s_index + 1;
            } else {
                if self.buys.as_slice()[b_index].order_type == OrderType::Buy
                    && self.sells.as_slice()[s_index].order_type == OrderType::Sell
                {
                    let mut to_buy: BuyOrders = BuyOrders::new();
                    let mut to_sells: SellOrders = SellOrders::new();

                    to_buy.push(self.buys.remove(b_index).unwrap()).unwrap();
                    to_sells.push(self.sells.remove(s_index).unwrap()).unwrap();

                    let trade = Trade {
                        buy_orders: to_buy,
                        sell_orders: to_sells,
                    };

                    return Some(trade);
                }
            }
        }

        None
    }
}

/// Convenience wrapper that keeps the original API: callers who have separate
/// `BuyOrders` and `SellOrders` can still call this function. It internally
/// constructs an `OrderBookEngine` and invokes the trait method.
pub fn fulfill_orders(buys: &mut BuyOrders, sells: &mut SellOrders) {
    let mut engine = OrderBookEngine::new(buys, sells);
    engine.fulfill();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BuyOrders, Order, OrderType, SellOrders};

    #[test]
    fn no_trade_when_prices_differ() {
        let mut buys = BuyOrders::new();
        let mut sells = SellOrders::new();

        buys.push(Order {
            order_type: OrderType::Buy,
            price: 100,
        })
        .unwrap();
        sells
            .push(Order {
                order_type: OrderType::Sell,
                price: 50,
            })
            .unwrap();

        fulfill_orders(&mut buys, &mut sells);

        assert_eq!(buys.len(), 1);
        assert_eq!(sells.len(), 1);
        assert_eq!(buys.as_slice()[0].price, 100);
        assert_eq!(sells.as_slice()[0].price, 50);
    }

    #[test]
    fn executes_trade_on_equal_price() {
        let mut buys = BuyOrders::new();
        let mut sells = SellOrders::new();

        buys.push(Order {
            order_type: OrderType::Buy,
            price: 50,
        })
        .unwrap();
        sells
            .push(Order {
                order_type: OrderType::Sell,
                price: 50,
            })
            .unwrap();

        fulfill_orders(&mut buys, &mut sells);

        assert_eq!(buys.len(), 0);
        assert_eq!(sells.len(), 0);
    }

    #[test]
    fn removes_matching_middle_orders() {
        let mut buys = BuyOrders::new();
        let mut sells = SellOrders::new();

        buys.push(Order {
            order_type: OrderType::Buy,
            price: 30,
        })
        .unwrap();
        buys.push(Order {
            order_type: OrderType::Buy,
            price: 50,
        })
        .unwrap();

        sells
            .push(Order {
                order_type: OrderType::Sell,
                price: 50,
            })
            .unwrap();
        sells
            .push(Order {
                order_type: OrderType::Sell,
                price: 60,
            })
            .unwrap();

        fulfill_orders(&mut buys, &mut sells);

        assert_eq!(buys.len(), 1);
        assert_eq!(sells.len(), 1);
        assert_eq!(buys.as_slice()[0].price, 30);
        assert_eq!(sells.as_slice()[0].price, 60);
    }
}
