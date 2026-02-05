use crate::{Order, OrderType};

#[derive(Clone, Debug, PartialEq)]
pub struct OrdersVec {
    order_type: OrderType,
    orders: Vec<Order>,
}

impl OrdersVec {
    pub fn new(order_type: OrderType) -> Self {
        Self { order_type, orders: Vec::new() }
    }

    pub fn add_order(&mut self, price: i32) -> Result<(), &'static str> {
        if price <= 0 {
            Err("price cannot be negative")
        } else {
            let new_order = Order {
                order_type: self.order_type.clone(),
                price,
            };
            // Propagate the error from `push` instead of unwrapping.
            self.push(new_order)
        }
    }

    pub fn push(&mut self, order: Order) -> Result<(), &'static str> {
        if order.order_type == self.order_type {
            self.orders.push(order);
            self.orders.sort_by_key(|o| o.price);
            Ok(())
        } else {
            Err("order type does not match OrdersVec type")
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

    #[test]
    fn test_new_creates_empty_ordersvec() {
        let orders_vec = OrdersVec::new(OrderType::Buy);
        assert!(orders_vec.is_empty());
        assert_eq!(orders_vec.len(), 0);
    }

    #[test]
    fn test_add_order_with_valid_price() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        let result = orders_vec.add_order(100);
        assert!(result.is_ok());
        assert_eq!(orders_vec.len(), 1);
    }

    #[test]
    fn test_add_order_with_zero_price() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        let result = orders_vec.add_order(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "price cannot be negative");
        assert!(orders_vec.is_empty());
    }

    #[test]
    fn test_add_order_with_negative_price() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        let result = orders_vec.add_order(-50);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "price cannot be negative");
        assert!(orders_vec.is_empty());
    }

    #[test]
    fn test_push_matching_order_type_succeeds() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        let order = Order {
            order_type: OrderType::Buy,
            price: 150,
        };
        let result = orders_vec.push(order);
        assert!(result.is_ok());
        assert_eq!(orders_vec.len(), 1);
    }

    #[test]
    fn test_push_non_matching_order_type_fails() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        let order = Order {
            order_type: OrderType::Sell,
            price: 150,
        };
        let result = orders_vec.push(order);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "order type does not match OrdersVec type");
        assert!(orders_vec.is_empty());
    }

    #[test]
    fn test_orders_are_sorted_by_price() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 150,
            })
            .ok();
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 100,
            })
            .ok();
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 200,
            })
            .ok();

        let slice = orders_vec.as_slice();
        assert_eq!(slice[0].price, 100);
        assert_eq!(slice[1].price, 150);
        assert_eq!(slice[2].price, 200);
    }

    #[test]
    fn test_as_slice_returns_orders() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 100,
            })
            .ok();
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 200,
            })
            .ok();

        let slice = orders_vec.as_slice();
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0].price, 100);
        assert_eq!(slice[1].price, 200);
    }

    #[test]
    fn test_remove_valid_index() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 100,
            })
            .ok();
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 200,
            })
            .ok();

        let removed = orders_vec.remove(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().price, 100);
        assert_eq!(orders_vec.len(), 1);
    }

    #[test]
    fn test_remove_invalid_index() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 100,
            })
            .ok();

        let removed = orders_vec.remove(5);
        assert!(removed.is_none());
        assert_eq!(orders_vec.len(), 1);
    }

    #[test]
    fn test_remove_from_empty() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        let removed = orders_vec.remove(0);
        assert!(removed.is_none());
        assert!(orders_vec.is_empty());
    }

    #[test]
    fn test_clone() {
        let mut orders_vec = OrdersVec::new(OrderType::Buy);
        orders_vec
            .push(Order {
                order_type: OrderType::Buy,
                price: 100,
            })
            .ok();

        let cloned = orders_vec.clone();
        assert_eq!(orders_vec, cloned);
        assert_eq!(cloned.len(), 1);
    }

    #[test]
    fn test_add_order_creates_correct_order_type() {
        let mut buy_orders = OrdersVec::new(OrderType::Buy);
        buy_orders.add_order(100).ok();
        
        let slice = buy_orders.as_slice();
        assert_eq!(slice[0].order_type, OrderType::Buy);
    }

    #[test]
    fn test_add_order_with_sell_type() {
        let mut sell_orders = OrdersVec::new(OrderType::Sell);
        let result = sell_orders.add_order(100);
        assert!(result.is_ok());
        assert_eq!(sell_orders.len(), 1);
        
        let slice = sell_orders.as_slice();
        assert_eq!(slice[0].order_type, OrderType::Sell);
    }
}
