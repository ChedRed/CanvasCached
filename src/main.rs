use std::{
    io::{Write, stdout},
    path::PathBuf,
};

use eframe::egui;
use reqwest::Client;

// Define eframe options here
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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

struct App {
    pub client: Client,

    pub config: String,
    pub cookies: String,
    pub storage: PathBuf,
}

impl Default for App {
    fn default() -> Self {
        let client: Client = Client::new();

        let config = format!(
            "https://{}/api/v1/",
            include_str!("testing/config.txt")
                .lines()
                .nth(0)
                .expect("File is too short!")
        )
        .to_string();
        let cookies = include_str!("testing/cookies.txt")
            .lines()
            .nth(0)
            .expect("File is too short!")
            .to_string();
        let storage = dirs::data_local_dir()
            .expect("Error: no storage directory detected!")
            .join("canvascached");

        let mut newself = Self {
            curler,

            config,
            cookies,
            storage,
        };

        newself.get_courses();
        newself
    }
}

impl App {
    fn get_courses(&mut self) {
        let transfer = self.curler.transfer();

        let mut list: easy::List = easy::List::new();
        _ = list.append(&format!("Cookie: {}", self.cookies.as_str()).to_string());
        _ = transfer.header_function(list);

        _ = transfer.url(&format!("{}courses", self.config));

        let mut response: Vec<u8> = Vec::new();
        let mut transfer = self.curler.transfer();
        transfer
            .write_function(|data| {
                response.extend(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Canvas Cached");
        });
    }
}
