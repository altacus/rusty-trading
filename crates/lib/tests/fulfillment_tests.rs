use lib::{Trade, Order, OrderType, fulfill_orders};

#[test]
fn integration_no_trade_when_prices_differ() {
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
fn integration_executes_trade_on_equal_price() {
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
fn integration_removes_matching_middle_orders() {
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
