use crate::{Order, OrderType};

#[derive(Debug, Default)]
pub struct BuyOrders {
    orders: Vec<Order>,
}

impl BuyOrders {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn push(&mut self, order: Order) -> Result<(), &'static str> {
        if order.order_type == OrderType::Buy {
            self.orders.push(order);
            self.orders.sort_by_key(|o| o.price);
            Ok(())
        } else {
            Err("only Buy orders can be added to BuyOrders")
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
    fn push_rejects_non_buy() {
        let mut b = BuyOrders::new();
        assert!(b.is_empty());

        let ok = b.push(Order { order_type: OrderType::Buy, price: 10 });
        assert!(ok.is_ok());
        assert_eq!(b.len(), 1);

        let err = b.push(Order { order_type: OrderType::Sell, price: 20 });
        assert!(err.is_err());
        assert_eq!(b.len(), 1, "length should not increase after rejected push");
    }

    #[test]
    fn maintains_sorted_order() {
        let mut b = BuyOrders::new();
        b.push(Order { order_type: OrderType::Buy, price: 100 }).unwrap();
        b.push(Order { order_type: OrderType::Buy, price: 50 }).unwrap();
        b.push(Order { order_type: OrderType::Buy, price: 75 }).unwrap();

        let prices: Vec<i32> = b.as_slice().iter().map(|o| o.price).collect();
        assert_eq!(prices, vec![50, 75, 100]);
    }

    #[test]
    fn remove_returns_element_and_updates_len() {
        let mut b = BuyOrders::new();
        b.push(Order { order_type: OrderType::Buy, price: 1 }).unwrap();
        b.push(Order { order_type: OrderType::Buy, price: 2 }).unwrap();

        assert_eq!(b.len(), 2);
        let removed = b.remove(0).expect("should remove element");
        assert_eq!(removed.price, 1);
        assert_eq!(b.len(), 1);
    }
}

