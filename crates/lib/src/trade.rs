use crate::{OrdersVec,  order_vec};

#[derive(Clone, Debug)]
pub struct Trade {
    pub buy_orders:  order_vec::OrdersVec,
    pub sell_orders: order_vec::OrdersVec,
}

impl Trade {
    pub fn new() -> Self {
        Self {
            buy_orders: OrdersVec::new(crate::OrderType::Buy),
            sell_orders: OrdersVec::new(crate::OrderType::Sell),
        }
    }
}
