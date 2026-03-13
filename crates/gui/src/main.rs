use eframe::{egui, NativeOptions};
use egui::CentralPanel;
use lib::{FulfillmentEngine, OrderBookEngine, Trade};

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Rusty Trading GUI",
        native_options,
        Box::new(|cc| Ok(Box::new(TradingApp::new(cc)))),
    )
}

struct TradingApp {
    trades: Trade,
    price_input: String,
    executed_trades: Vec<String>,
}

impl TradingApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            trades: Trade::new(),
            price_input: String::new(),
            executed_trades: Vec::new(),
        }
    }

    fn add_order(&mut self, is_buy: bool) {
        if let Ok(price) = self.price_input.trim().parse::<i32>() {
            let result = if is_buy {
                self.trades.buy_orders.add_order(price)
            } else {
                self.trades.sell_orders.add_order(price)
            };

            if result.is_ok() {
                self.fulfill_orders();
                self.price_input.clear();
            }
        }
    }

    fn fulfill_orders(&mut self) {
        let mut engine = OrderBookEngine::new(&mut self.trades);
        while let Some(trade) = engine.fulfill() {
            self.executed_trades.push(format!("Executed trade: {:?}\n", trade));
        }
    }
}

impl eframe::App for TradingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rusty Trading GUI");

            ui.horizontal(|ui| {
                ui.label("Price:");
                ui.text_edit_singleline(&mut self.price_input);
            });

            ui.horizontal(|ui| {
                if ui.button("Enter Buy Order").clicked() {
                    self.add_order(true);
                }
                if ui.button("Enter Sell Order").clicked() {
                    self.add_order(false);
                }
            });

            ui.separator();

            ui.label("Buy Orders:");
            for order in self.trades.buy_orders.as_slice() {
                ui.label(format!("Price: {}", order.price));
            }

            ui.label("Sell Orders:");
            for order in self.trades.sell_orders.as_slice() {
                ui.label(format!("Price: {}", order.price));
            }

            ui.separator();

            ui.label("Executed Trades:");
            for trade in &self.executed_trades {
                ui.label(trade);
            }
        });
    }
}