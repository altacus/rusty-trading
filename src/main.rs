use std::io;

#[derive(Debug, PartialEq)]
enum OrderType {
    // Undefined,
    Buy,
    Sell,
}

#[derive(Debug)]
struct Order {
    order_type: OrderType,
    price: i32,
}

fn main() {
    let mut buys: Vec<Order> = Vec::new();
    let mut sells: Vec<Order> = Vec::new();
    let mut is_valid_menu = false;

    loop {
        let mut input2 = String::new();

        let menu_input = display_menu();
        println!("    Menu entered: {}", menu_input);

        match menu_input.as_str() {
            "1" => is_valid_menu = true,
            "2" => is_valid_menu = true,
            _ => println!(" I don't understand, try again"),
        }
        if menu_input == "3" {
            break;
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
                "1" => intake_order(OrderType::Buy, price, &mut buys),
                "2" => intake_order(OrderType::Sell, price, &mut sells),
                _ => (),
            }

            fulfill_orders(&mut buys, &mut sells);
	    is_valid_menu = false;
        }
    }
}

fn display_menu() -> String {
    let mut input = String::new();
    println!("Basic trader!");
    println!("Menu options");
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

fn intake_order(new_order_type: OrderType, order_price: i32, book: &mut Vec<Order>) {
    println!("");
    println!("Intaking price {} book {:?}", order_price, book);
    let new_order = Order {
        order_type: new_order_type,
        price: order_price,
    };
    book.push(new_order);
    /* sort in ascending order */
    book.sort_by_key(|o| o.price);
    println!(" latest book state {book:?}");
    println!("");
}

/* Single pass fulfillment algo */
fn fulfill_orders(buys: &mut Vec<Order>, sells: &mut Vec<Order>) {
    println!("Entering fulfillment");
    println!(" Buys {:?}", buys);
    println!(" Sells {:?}", sells);
    let mut b_index = 0;
    let mut s_index = 0;

    let mut execute_trade = false;

    while b_index < buys.len() && s_index < sells.len() {
        if buys[b_index].price < sells[s_index].price {
            b_index = b_index + 1;
        } else if buys[b_index].price > sells[s_index].price {
            s_index = s_index + 1;
        } else {
            execute_trade = true;
            println!(" executing trade at b index {b_index} s index {s_index}");
            if buys[b_index].order_type == OrderType::Buy
                && sells[s_index].order_type == OrderType::Sell
            {
                buys.remove(b_index);
                sells.remove(s_index);
                break;
            }
        }
    }

    if execute_trade == false {
        println!("No trades executed");
    } else {
        println!(" Buys {:?}", buys);
        println!(" Sells {:?}", sells);
    }
    println!("");
}
