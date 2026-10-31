pub mod app;
use crate::app::App;
use eframe::egui;

impl eframe::App for app::App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.setup_ui(ui, frame);
    }
    fn on_exit(&mut self) {
        let cookie_results = self.save_cookies();
        if let Err(cookie_err) = cookie_results {
            eprintln!("{cookie_err}");
        }
    }
}

// Define eframe options here
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([340.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Canvas Cached",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<App>::default())
        }),
    )
}
