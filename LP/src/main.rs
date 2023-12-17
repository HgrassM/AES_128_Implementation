use eframe::{egui, run_native, NativeOptions, App, CreationContext};

struct MyApp {
    input_path: String,
    output_path: String,
    encrypt: bool,
    decrypt: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            encrypt: true,
            decrypt: false,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Encriptação");
            ui.horizontal(|ui| {
                ui.label("Path:");
                ui.text_edit_singleline(&mut self.input_path);
            });
            ui.horizontal(|ui| {
                if ui.button("Encriptar").clicked() {
                    self.encrypt = true;
                    self.decrypt = false;
                }
                if ui.button("Desencriptar").clicked() {
                    self.decrypt = true;
                    self.encrypt = false;
                }
            });
            if ui.button("Enviar").clicked() {
                self.output_path = self.input_path.clone();
            }
            if !self.output_path.is_empty() {
                ui.label(format!("Caminho Inserido: {}", self.output_path));
            }
        });
    }
}

fn main() {
    let options = NativeOptions::default();
    run_native(
        "TRABALHO LP",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
