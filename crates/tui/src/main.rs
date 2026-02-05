use std::io;
use trading_lib::{FulfillmentEngine, OrderBookEngine, Trade};

fn main() {
    let mut unexecuted_trades: Trade = Trade::new();
    // let mut buys: BuyOrders = BuyOrders::new();
    // let mut sells: SellOrders = SellOrders::new();
    let mut is_valid_menu = false;

    loop {
        let menu_input = display_menu();

        match menu_input.as_str() {
            "1" => is_valid_menu = true,
            "2" => is_valid_menu = true,
            "3" => break,
            _ => println!(" I don't understand, try again"),
        }

        if is_valid_menu {
            let price: i32 = get_price_input();
            fulfill_orders(&menu_input, price, &mut unexecuted_trades);
            is_valid_menu = false;
        }
    }
}

fn display_menu() -> String {
    let mut input = String::new();
    println!("Basic trader - Menu options");
    println!("  1. Enter Buy Order ");
    println!("  2. Enter Sell Order ");
    println!("  3. Exit ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let value = input.trim();
    println!("    Menu entered: {}", value);
    value.to_string()
}

fn get_price_input() -> i32 {
    let mut input = String::new();
    println!(" Enter an integer price: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("     Price entered: {}", input);
    let price: i32 = input
        .trim()
        .parse()
        .expect("The input string was not a valid i32 number");
    price
}

fn fulfill_orders(menu_input: &str, price: i32, trades: &mut Trade) {
    match menu_input {
        "1" => trades
            .buy_orders
            .add_order(price)
            .expect("failed to add buy order"),
        "2" => trades
            .sell_orders
            .add_order(price)
            .expect("failed to add sell order"),
        _ => (),
    }

    println!(" Fulfilling\n   Trades {:?}", trades);
    // Use the trait-based engine instead of the free function.
    let mut engine = OrderBookEngine::new(trades);
    let trade = engine.fulfill();
    if let Some(trade) = trade {
        println!("Executed trade: {:?}", trade);
    }
    println!(" After fulfillment\n   Trades {:?}", trades);
}
