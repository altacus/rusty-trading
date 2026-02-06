use eframe::{egui, NativeOptions};
use egui::{CentralPanel, Slider, Ui};

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

#[derive(Default)]
struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here if you want
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(Slider::new(&mut self.age, 0..=120).text("Age"));
            if ui.button("Increment Age").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", &self.name, self.age));
        });
    }
}