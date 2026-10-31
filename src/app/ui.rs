use crate::app;

impl app::App {
    pub fn setup_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_max_width(320.);
                ui.columns(3, |ui| {
                    if ui[0].add(egui::Button::new("Get Courses")).clicked() {
                        let cookie_results = self.request_get_store("courses");
                        if let Err(err) = cookie_results {
                            eprintln!("{err}");
                        }
                    }
                    if ui[1].add(egui::Button::new("Set Cookies")).clicked() {
                        let cookie_results = self.set_cookies(&self.singleline_input.clone());
                        if let Err(err) = cookie_results {
                            eprintln!("{err}");
                        }

                        self.store_cookies(&self.singleline_input.clone());
                    }
                    if ui[2].add(egui::Button::new("Set Domain")).clicked() {
                        let domain_results = self.set_domain(&self.singleline_input.clone());
                        if let Err(err) = domain_results {
                            eprintln!("{err}");
                        }
                    }
                });
                ui.add_space(4.);
                ui.add(egui::TextEdit::singleline(&mut self.singleline_input).desired_width(320.));
            });
        });
    }
}
