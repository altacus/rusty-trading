#[derive(Debug, PartialEq, Clone)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub order_type: OrderType,
    pub price: i32,
}
