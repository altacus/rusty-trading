use crate::{Order, OrderType};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SellOrders {
    orders: Vec<Order>,
}

impl SellOrders {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn add_order(&mut self, price: i32) -> Result<(), &'static str> {
        if price <= 0 {
            Err("price cannot be negative")
        } else {
            let new_order = Order {
                order_type: OrderType::Sell,
                price,
            };
            // Propagate the error from `push` instead of unwrapping.
            self.push(new_order)
        }
    }

    pub fn push(&mut self, order: Order) -> Result<(), &'static str> {
        if order.order_type == OrderType::Sell {
            self.orders.push(order);
            self.orders.sort_by_key(|o| o.price);
            Ok(())
        } else {
            Err("only Sell orders can be added to SellOrders")
        }
    }

    pub fn len(&self) -> usize {
        self.orders.len()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn as_slice(&self) -> &[Order] {
        &self.orders
    }

    pub fn remove(&mut self, index: usize) -> Option<Order> {
        if index < self.orders.len() {
            Some(self.orders.remove(index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Order, OrderType};

    #[test]
    fn push_rejects_non_sell() {
        let mut s = SellOrders::new();
        assert!(s.is_empty());

        let ok = s.push(Order {
            order_type: OrderType::Sell,
            price: 10,
        });
        assert!(ok.is_ok());
        assert_eq!(s.len(), 1);

        let err = s.push(Order {
            order_type: OrderType::Buy,
            price: 20,
        });
        assert!(err.is_err());
        assert_eq!(s.len(), 1, "length should not increase after rejected push");
    }

    #[test]
    fn maintains_sorted_order() {
        let mut s = SellOrders::new();
        s.push(Order {
            order_type: OrderType::Sell,
            price: 100,
        })
        .unwrap();
        s.push(Order {
            order_type: OrderType::Sell,
            price: 50,
        })
        .unwrap();
        s.push(Order {
            order_type: OrderType::Sell,
            price: 75,
        })
        .unwrap();

        let prices: Vec<i32> = s.as_slice().iter().map(|o| o.price).collect();
        assert_eq!(prices, vec![50, 75, 100]);
    }

    #[test]
    fn remove_returns_element_and_updates_len() {
        let mut s = SellOrders::new();
        s.push(Order {
            order_type: OrderType::Sell,
            price: 1,
        })
        .unwrap();
        s.push(Order {
            order_type: OrderType::Sell,
            price: 2,
        })
        .unwrap();

        assert_eq!(s.len(), 2);
        let removed = s.remove(0).expect("should remove element");
        assert_eq!(removed.price, 1);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn add_order_validates_and_sorts() {
        let mut s = SellOrders::new();

        // invalid prices (<= 0) are rejected
        assert!(s.add_order(0).is_err());
        assert!(s.add_order(-10).is_err());

        // valid orders are accepted and kept sorted
        s.add_order(30).unwrap();
        s.add_order(20).unwrap();

        let prices: Vec<i32> = s.as_slice().iter().map(|o| o.price).collect();
        assert_eq!(prices, vec![20, 30]);
        assert_eq!(s.len(), 2);
    }
}
