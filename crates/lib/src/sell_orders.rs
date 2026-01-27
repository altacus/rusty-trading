use crate::{Order, OrderType};

#[derive(Debug, Default)]
pub struct SellOrders {
    orders: Vec<Order>,
}

impl SellOrders {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
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
