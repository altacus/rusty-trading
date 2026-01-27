use crate::{BuyOrders, SellOrders, OrderType};

pub fn fulfill_orders(buys: &mut BuyOrders, sells: &mut SellOrders) {
    println!("Entering fulfillment");
    println!(" Buys {:?}", buys);
    println!(" Sells {:?}", sells);
    let mut b_index = 0;
    let mut s_index = 0;

    let mut execute_trade = false;

    while b_index < buys.len() && s_index < sells.len() {
        if buys.as_slice()[b_index].price < sells.as_slice()[s_index].price {
            b_index = b_index + 1;
        } else if buys.as_slice()[b_index].price > sells.as_slice()[s_index].price {
            s_index = s_index + 1;
        } else {
            execute_trade = true;
            println!(" executing trade at b index {b_index} s index {s_index}");
            if buys.as_slice()[b_index].order_type == OrderType::Buy
                && sells.as_slice()[s_index].order_type == OrderType::Sell
            {
                let _ = buys.remove(b_index);
                let _ = sells.remove(s_index);
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
