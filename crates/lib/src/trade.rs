use crate::{OrdersVec, order_vec};

#[derive(Clone, Debug)]
pub struct Trade {
    pub buy_orders: order_vec::OrdersVec,
    pub sell_orders: order_vec::OrdersVec,
}

impl Trade {
    pub fn execute_trade(&mut self) -> Option<Trade> {
        let mut b_index = 0;
        let mut s_index = 0;

        while b_index < self.buy_orders.len() && s_index < self.sell_orders.len() {
            if self.buy_orders.as_slice()[b_index].price
                < self.sell_orders.as_slice()[s_index].price
            {
                b_index += 1;
            } else if self.buy_orders.as_slice()[b_index].price
                > self.sell_orders.as_slice()[s_index].price
            {
                s_index += 1;
            } else {
                let mut to_execute = Trade::new();

                to_execute
                    .buy_orders
                    .push(self.buy_orders.remove(b_index).unwrap())
                    .unwrap();
                to_execute
                    .sell_orders
                    .push(self.sell_orders.remove(s_index).unwrap())
                    .unwrap();

                return Some(to_execute);
            }
        }

        None
    }

    pub fn new() -> Self {
        Self {
            buy_orders: OrdersVec::new(crate::OrderType::Buy),
            sell_orders: OrdersVec::new(crate::OrderType::Sell),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Order, OrderType};

    #[test]
    fn execute_trade_returns_none_when_no_match() {
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
                price: 200,
            })
            .unwrap();

        let result = trades.execute_trade();
        assert!(result.is_none());
        assert_eq!(trades.buy_orders.len(), 1);
        assert_eq!(trades.sell_orders.len(), 1);
    }

    #[test]
    fn execute_trade_returns_trade_and_removes_orders() {
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

        let result = trades.execute_trade();
        assert!(result.is_some());

        let executed = result.unwrap();
        assert_eq!(executed.buy_orders.len(), 1);
        assert_eq!(executed.sell_orders.len(), 1);
        assert_eq!(executed.buy_orders.as_slice()[0].price, 50);
        assert_eq!(executed.sell_orders.as_slice()[0].price, 50);

        // original orders removed
        assert_eq!(trades.buy_orders.len(), 0);
        assert_eq!(trades.sell_orders.len(), 0);
    }

    #[test]
    fn execute_trade_removes_matching_middle_orders() {
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

        let result = trades.execute_trade();
        assert!(result.is_some());

        // after execution, remaining orders should be the middle ones
        assert_eq!(trades.buy_orders.len(), 1);
        assert_eq!(trades.sell_orders.len(), 1);
        assert_eq!(trades.buy_orders.as_slice()[0].price, 50);
        assert_eq!(trades.sell_orders.as_slice()[0].price, 60);
    }

    #[test]
    fn execute_trade_on_empty_returns_none() {
        let mut trades = Trade::new();
        let result = trades.execute_trade();
        assert!(result.is_none());
    }

    #[test]
    fn execute_trade_with_multiple_equal_prices_removes_one_pair() {
        let mut trades = Trade::new();

        trades
            .buy_orders
            .push(Order {
                order_type: OrderType::Buy,
                price: 50,
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

        let result = trades.execute_trade();
        assert!(result.is_some());

        // one pair executed, one remaining on each side
        assert_eq!(trades.buy_orders.len(), 1);
        assert_eq!(trades.sell_orders.len(), 1);
        assert_eq!(trades.buy_orders.as_slice()[0].price, 50);
        assert_eq!(trades.sell_orders.as_slice()[0].price, 50);
    }

    #[test]
    fn execute_trade_matches_late_indices() {
        let mut trades = Trade::new();

        trades
            .buy_orders
            .push(Order {
                order_type: OrderType::Buy,
                price: 10,
            })
            .unwrap();
        trades
            .buy_orders
            .push(Order {
                order_type: OrderType::Buy,
                price: 20,
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
                price: 5,
            })
            .unwrap();
        trades
            .sell_orders
            .push(Order {
                order_type: OrderType::Sell,
                price: 25,
            })
            .unwrap();
        trades
            .sell_orders
            .push(Order {
                order_type: OrderType::Sell,
                price: 50,
            })
            .unwrap();

        let result = trades.execute_trade();
        assert!(result.is_some());

        // matched at price 50; remaining orders are the earlier ones
        assert_eq!(trades.buy_orders.len(), 2);
        assert_eq!(trades.sell_orders.len(), 2);
        assert_eq!(trades.buy_orders.as_slice()[0].price, 10);
        assert_eq!(trades.buy_orders.as_slice()[1].price, 20);
        assert_eq!(trades.sell_orders.as_slice()[0].price, 5);
        assert_eq!(trades.sell_orders.as_slice()[1].price, 25);
    }
}
