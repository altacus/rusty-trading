use crate::{buy_orders, sell_orders};

#[derive(Clone, Debug, Default)]
pub struct Trade {
    pub buy_orders: buy_orders::BuyOrders,
    pub sell_orders: sell_orders::SellOrders,
}

impl Trade {
    pub fn new() -> Self {
        Self {
            buy_orders: buy_orders::BuyOrders::new(),
            sell_orders: sell_orders::SellOrders::new(),
        }
    }
}
