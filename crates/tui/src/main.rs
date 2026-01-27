use std::io;
use trading_lib::{fulfill_orders, Order, OrderType, BuyOrders, SellOrders};

fn main() {
    let mut buys: BuyOrders = BuyOrders::new();
    let mut sells: SellOrders = SellOrders::new();
    let mut is_valid_menu = false;

    loop {
        let mut input2 = String::new();
        let menu_input = display_menu();
        println!("    Menu entered: {}", menu_input);

        match menu_input.as_str() {
            "1" => is_valid_menu = true,
            "2" => is_valid_menu = true,
            "3" => break,
            _ => println!(" I don't understand, try again"),
        }

        if is_valid_menu {
            println!(" Enter an integer price: ");

            io::stdin()
                .read_line(&mut input2)
                .expect("Failed to read line");

            println!("     Price entered: {}", input2);
            let price: i32 = input2
                .trim()
                .parse()
                .expect("The input string was not a valid i32 number");


            match menu_input.as_str() {
                "1" => {
                    let new_order = Order { order_type: OrderType::Buy, price };
                    let _ = buys.push(new_order);
                }
                "2" => {
                    let new_order = Order { order_type: OrderType::Sell, price };
                    let _ = sells.push(new_order);
                }
                _ => (),
            }

            fulfill_orders(&mut buys, &mut sells);
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
    println!("");
    value.to_string()
}
