use std::{fs, path::{Path, PathBuf}, sync::Arc};

use eframe::egui;
use reqwest::{Url, blocking::{Client, Response}, cookie::{CookieStore, Jar}, header::USER_AGENT};

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

struct App {
    pub client: Client,

    pub base_url: String,
    pub base_url_obj: Url,

    pub agent: String,
    pub storage: PathBuf,

    pub arcjar: Arc<Jar>,
}

impl Default for App {
    fn default() -> Self {
        let base_url = format!(
            "https://{}/api/v1/",
            include_str!("testing/domain.txt")
                .lines()
                .nth(0)
                .expect("File is too short!")
        )
        .to_string();

        let base_url_obj = base_url.parse::<Url>().unwrap();

        let agent = include_str!("testing/agent.txt")
            .lines()
            .nth(0)
            .expect("File is too short!")
            .to_string();

        let storage = dirs::data_local_dir()
            .expect("Error: no storage directory detected!")
            .join("canvascached");

        if !storage.exists() {
            fs::create_dir(storage.clone()).unwrap();
        }

        let cookies = fs::read_to_string(storage.join("cookies.txt")).unwrap_or_default();

        let jar = Jar::default();
        for cookie in cookies.clone().split("; ") {
            jar.add_cookie_str(cookie, &base_url_obj);
        }

        let arcjar = Arc::new(jar);
        let client: Client = Client::builder().cookie_provider(arcjar.clone()).build().unwrap();

        Self {
            client,

            base_url,
            base_url_obj,
            agent,
            storage,

            arcjar,
        }
    }
}

impl App {
    fn request_get(&mut self, endpoint: &'static str) -> Response {
        let url = format!("{}{}", self.base_url.clone(), endpoint);
        let agent = self.agent.clone();

        println!("Sending GET request to '{}'", url);
        let results = self.client.get(url).header(USER_AGENT, agent).send().unwrap();
        results
    }
    fn request_get_store(&mut self, endpoint: &'static str) {
        let mut results = self.request_get(endpoint);

        if !results.status().is_success() {
            println!("Error code met: {}. Not writing.", results.status());
            return
        }

        let results_path = self.storage.join(format!("{}{}", endpoint, ".json"));
        if !results_path.exists() {
            let newpath = Path::new(results_path.as_os_str()).parent().unwrap();
            fs::create_dir_all(newpath.as_os_str()).unwrap();
        }

        println!("Writing to '{}'", results_path.clone().into_string().unwrap());

        let mut w: Vec<u8> = Vec::new();
        results.copy_to(&mut w).unwrap();
        fs::write(results_path, w.as_slice()).unwrap();
    }

    fn store_cookies(&mut self, cookies: &str) {
        fs::write(self.storage.join("cookies.txt"), cookies.as_bytes()).unwrap();
    }

    fn set_cookies(&mut self, cookies: &str) -> String {
        for cookie in cookies.split("; ") {
            self.arcjar.add_cookie_str(cookie, &self.base_url_obj);
        }
        String::from(cookies)
    }

    fn get_cookies(&mut self) -> String {
        String::from(self.arcjar.cookies(&self.base_url_obj).unwrap().to_str().unwrap())
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_max_width(320.);
                if ui.add(egui::Button::new("Get Courses")).clicked() {
                    self.request_get_store("courses");
                }
            });
        });
    }
    fn on_exit(&mut self) {
        let cookies = self.get_cookies();
        self.store_cookies(cookies.as_str());
    }
}
