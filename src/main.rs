use eframe::{egui, run_native, NativeOptions, App};
mod aes128;
use aes128::*; 
mod funcoes;
use funcoes::*;
mod key_expansion;
use key_expansion::*;
use std::fs;

struct MyApp {
    input: String,
    output_path: String,
    decrypt_key: String,
    encrypt_key: String,
    encrypt: bool,
    decrypt: bool,
    is_file: bool,
    message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output_path: String::new(),
            decrypt_key: String::new(),
            encrypt_key: String::new(),
            encrypt: true,
            decrypt: false,
            is_file: true,
            message: String::new(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let title = if self.decrypt {
            "Desencriptador"
        } else {
            "Encriptador"
        };
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(title);
            ui.separator();
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(if self.is_file { "Path:" } else { "Texto/Número:" });
                    ui.text_edit_singleline(&mut self.input);
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Arquivo").clicked() {
                        self.is_file = true;
                    }
                    if ui.button("Texto/Número").clicked() {
                        self.is_file = false;
                    }
                });
                ui.separator();
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
                ui.separator();
                if self.encrypt {
                    ui.horizontal(|ui| {
                        ui.label("Chave de Encriptação:");
                        ui.text_edit_singleline(&mut self.encrypt_key);
                    });
                    ui.separator();
                }
                if self.decrypt {
                    ui.horizontal(|ui| {
                        ui.label("Chave de Desencriptação:");
                        ui.text_edit_singleline(&mut self.decrypt_key);
                    });
                    ui.separator();
                }
                if ui.button("Enviar").clicked() {
                    if !self.input.is_empty() && (!self.encrypt_key.is_empty() || !self.decrypt_key.is_empty()) {
                        let mut key = [0u8; 16];
                        let input_data = if self.is_file {
                            match fs::read(&self.input) {
                                Ok(data) => data,
                                Err(e) => {
                                    self.message = format!("Erro ao ler o arquivo: {}", e);
                                    return;
                                }
                            }
                        } else {
                            self.input.as_bytes().to_vec()
                        };
                        if self.encrypt {
                            let encrypt_key_bytes = self.encrypt_key.as_bytes();
                            for i in 0..16 {
                                key[i] = *encrypt_key_bytes.get(i).unwrap_or(&0);
                            }
                            let encrypted_data = encrypt(input_data, key);
                            match fs::write(&self.output_path, encrypted_data) {
                                Ok(_) => self.message = "Operação realizada com sucesso".to_string(),
                                Err(e) => self.message = format!("Erro ao escrever no arquivo: {}", e),
                            }
                        } else if self.decrypt {
                            let decrypt_key_bytes = self.decrypt_key.as_bytes();
                            for i in 0..16 {
                                key[i] = *decrypt_key_bytes.get(i).unwrap_or(&0);
                            }
                            let decrypted_data = decrypt(input_data, key);
                            match fs::write(&self.output_path, decrypted_data) {
                                Ok(_) => self.message = "Operação realizada com sucesso".to_string(),
                                Err(e) => self.message = format!("Erro ao escrever no arquivo: {}", e),
                            }
                        }
                    } else {
                        self.message = "Por favor, insira um caminho válido e uma chave".to_string();
                    }
                }
                ui.separator();
                if !self.output_path.is_empty() {
                    ui.label(format!("Caminho Inserido: {}", self.output_path));
                }
                if !self.message.is_empty() {
                    ui.label(&self.message);
                }
            });
        });
    }
}

fn main() {
    let options = NativeOptions::default();
    let _ = run_native(
        "TRABALHO LP",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
