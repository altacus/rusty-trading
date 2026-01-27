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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BuyOrders, SellOrders, Order, OrderType};

    #[test]
    fn no_trade_when_prices_differ() {
        let mut buys = BuyOrders::new();
        let mut sells = SellOrders::new();

        buys.push(Order { order_type: OrderType::Buy, price: 100 }).unwrap();
        sells.push(Order { order_type: OrderType::Sell, price: 50 }).unwrap();

        fulfill_orders(&mut buys, &mut sells);

        assert_eq!(buys.len(), 1);
        assert_eq!(sells.len(), 1);
        assert_eq!(buys.as_slice()[0].price, 100);
        assert_eq!(sells.as_slice()[0].price, 50);
    }

    #[test]
    fn executes_trade_on_equal_price() {
        let mut buys = BuyOrders::new();
        let mut sells = SellOrders::new();

        buys.push(Order { order_type: OrderType::Buy, price: 50 }).unwrap();
        sells.push(Order { order_type: OrderType::Sell, price: 50 }).unwrap();

        fulfill_orders(&mut buys, &mut sells);

        assert_eq!(buys.len(), 0);
        assert_eq!(sells.len(), 0);
    }

    #[test]
    fn removes_matching_middle_orders() {
        let mut buys = BuyOrders::new();
        let mut sells = SellOrders::new();

        buys.push(Order { order_type: OrderType::Buy, price: 30 }).unwrap();
        buys.push(Order { order_type: OrderType::Buy, price: 50 }).unwrap();

        sells.push(Order { order_type: OrderType::Sell, price: 50 }).unwrap();
        sells.push(Order { order_type: OrderType::Sell, price: 60 }).unwrap();

        fulfill_orders(&mut buys, &mut sells);

        assert_eq!(buys.len(), 1);
        assert_eq!(sells.len(), 1);
        assert_eq!(buys.as_slice()[0].price, 30);
        assert_eq!(sells.as_slice()[0].price, 60);
    }
}
