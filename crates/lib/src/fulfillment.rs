use crate::Trade;

/// Trait that abstracts a fulfillment engine. Implementors provide the logic
/// to match and execute trades between buy and sell orders.
pub trait FulfillmentEngine {
    fn fulfill(&mut self) -> Option<Trade>;
}

/// An order book engine that implements the `FulfillmentEngine` trait.
pub struct OrderBookEngine<'a> {
    pub trades: &'a mut Trade,
}

impl<'a> OrderBookEngine<'a> {
    pub fn new(new_trades: &'a mut Trade) -> Self {
        Self { trades: new_trades }
    }
}

impl<'a> FulfillmentEngine for OrderBookEngine<'a> {
    fn fulfill(&mut self) -> Option<Trade> {
        self.trades.execute_trade()
    }
}

/// Convenience wrapper that keeps the original API: callers who have separate
/// `BuyOrders` and `SellOrders` can still call this function. It internally
/// constructs an `OrderBookEngine` and invokes the trait method.
pub fn fulfill_orders(trades: &mut Trade) {
    let mut engine = OrderBookEngine::new(trades);
    engine.fulfill();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Order, OrderType, Trade};

    #[test]
    fn no_trade_when_prices_differ() {
        let mut trades = Trade::new();

        trades
            .buy_orders
            .push(Order {
                order_type: OrderType::Buy,
                price: 100,
            })
            .unwrap();
        trades
            .sell_orders
            .push(Order {
                order_type: OrderType::Sell,
                price: 50,
            })
            .unwrap();

        fulfill_orders(&mut trades);

        assert_eq!(trades.buy_orders.len(), 1);
        assert_eq!(trades.sell_orders.len(), 1);
        assert_eq!(trades.buy_orders.as_slice()[0].price, 100);
        assert_eq!(trades.sell_orders.as_slice()[0].price, 50);
    }

    #[test]
    fn executes_trade_on_equal_price() {
        let mut trades = Trade::new();

        trades
            .buy_orders
            .push(Order {
                order_type: OrderType::Buy,
                price: 50,
            })
            .unwrap();
        trades
            .sell_orders
            .push(Order {
                order_type: OrderType::Sell,
                price: 50,
            })
            .unwrap();

        fulfill_orders(&mut trades);

        assert_eq!(trades.buy_orders.len(), 0);
        assert_eq!(trades.sell_orders.len(), 0);
    }

    #[test]
    fn removes_matching_middle_orders() {
        let mut trades = Trade::new();

        trades
            .buy_orders
            .push(Order {
                order_type: OrderType::Buy,
                price: 30,
            })
            .unwrap();
        trades
            .sell_orders
            .push(Order {
                order_type: OrderType::Sell,
                price: 30,
            })
            .unwrap();

        trades
            .buy_orders
            .push(Order {
                order_type: OrderType::Buy,
                price: 50,
            })
            .unwrap();
        trades
            .sell_orders
            .push(Order {
                order_type: OrderType::Sell,
                price: 60,
            })
            .unwrap();

        fulfill_orders(&mut trades);

        let buys = &trades.buy_orders;
        let sells = &trades.sell_orders;

        assert_eq!(buys.len(), 1);
        assert_eq!(sells.len(), 1);
        assert_eq!(buys.as_slice()[0].price, 50);
        assert_eq!(sells.as_slice()[0].price, 60);
    }
}
