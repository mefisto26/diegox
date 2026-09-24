pub mod guide_tab;
pub mod header;
pub mod identity_tab;
pub mod shield_tab;
pub mod state;
pub mod theme;
pub mod unlock_tab;

use eframe::egui;
use std::sync::mpsc::channel;

use crate::engine::QuantumEngine;
use crate::gui::state::{GuiState, InputMode, Tab, TargetMode, WorkerResult};

pub struct CsrpGui {
    state: GuiState,
}

impl CsrpGui {
    pub fn new(_engine: QuantumEngine) -> Self {
        let (tx, rx) = channel();
        let output_dir = dirs::download_dir().unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
        });

        Self {
            state: GuiState {
                current_tab: Tab::Shield,
                input_mode: InputMode::Text,
                target_mode: TargetMode::Me,
                real_passphrase: String::new(),
                decoy_passphrase: String::new(),
                contact_pub_key_path: String::new(),
                contact_pub_key_bytes: String::new(),
                input_real: String::new(),
                input_decoy_text: String::new(),
                file_real: String::new(),
                file_decoy: String::new(),
                output_name: String::new(),
                use_decoy: false,
                file_to_decrypt: String::new(),
                unlock_passphrase: String::new(),
                decrypted_bytes_output: None,
                decrypted_ext_output: String::from("txt"),
                alias_vault: String::new(),
                passphrase_vault: String::new(),
                identities: Vec::new(),
                selected_identity_idx: None,
                selected_identity_pub_key: String::new(),
                tx,
                rx,
                is_loading: false,
                status: String::from("CSRP SYSTEM INITIALIZED. SECURE ENVIRONMENT."),
                output_dir,
                logo_texture: None,
            },
        }
    }

    fn check_worker_channels(&mut self) {
        if let Ok(res) = self.state.rx.try_recv() {
            self.state.is_loading = false;
            match res {
                WorkerResult::EncryptionSuccess(msg) => {
                    self.state.status = msg;
                    self.state.real_passphrase.clear();
                    self.state.decoy_passphrase.clear();
                    self.state.input_real.clear();
                    self.state.input_decoy_text.clear();
                    self.state.file_real.clear();
                    self.state.file_decoy.clear();
                    self.state.output_name.clear();
                }
                WorkerResult::DecryptionSuccess(bytes, ext) => {
                    self.state.decrypted_bytes_output = Some(bytes);
                    self.state.decrypted_ext_output = ext;
                    self.state.status = "SUCCESS: BLIND CONTAINER EXTRACTED CORRECTLY.".to_string();
                    self.state.unlock_passphrase.clear();
                    self.state.file_to_decrypt.clear();
                }
                WorkerResult::IdentitySuccess(item) => {
                    let key_text = item.public_key_text.clone();
                    self.state.identities.push(item);
                    self.state.selected_identity_idx = Some(self.state.identities.len() - 1);
                    self.state.selected_identity_pub_key = key_text;
                    self.state.status = "NEW IDENTITY BOUND TO EPHEMERAL KEYRING.".to_string();
                    self.state.alias_vault.clear();
                    self.state.passphrase_vault.clear();
                }
                WorkerResult::Failure(err) => {
                    self.state.status = err;
                    self.state.unlock_passphrase.clear();
                }
            }
        }
    }
}

impl eframe::App for CsrpGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_worker_channels();

        // 1. Custom Cyberpunk Titlebar (branding, draggable frame, window controls)
        header::draw_custom_titlebar(&mut self.state, ctx);

        // 2. Output Directory configuration bar
        egui::TopBottomPanel::top("header_bar").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("📁 OUTPUT DIR:")
                        .strong()
                        .color(egui::Color32::from_rgb(0, 255, 65)),
                );

                let mut path_str = self.state.output_dir.display().to_string();
                let edit_resp = ui.add(
                    egui::TextEdit::singleline(&mut path_str)
                        .desired_width(ui.available_width() - 110.0),
                );
                if edit_resp.changed() {
                    self.state.output_dir = std::path::PathBuf::from(path_str);
                }

                if ui.button("📂 Browse...").clicked() {
                    let mut dialog = rfd::FileDialog::new();
                    if self.state.output_dir.exists() {
                        dialog = dialog.set_directory(&self.state.output_dir);
                    }
                    if let Some(folder) = dialog.pick_folder() {
                        self.state.output_dir = folder;
                    }
                }
            });
            ui.add_space(4.0);
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("SYS_STATUS:");
                if self.state.is_loading {
                    ui.spinner();
                }
                ui.colored_label(egui::Color32::from_rgb(0, 255, 65), &self.state.status);
            });
            ui.add_space(5.0);
        });

        egui::SidePanel::left("nav")
            .default_width(170.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.heading("> .DIEGOX_OS");
                ui.separator();
                ui.add_space(10.0);

                ui.add_enabled_ui(!self.state.is_loading, |ui| {
                    if ui
                        .selectable_label(self.state.current_tab == Tab::Shield, "[ 🛡 ] SHIELD")
                        .clicked()
                    {
                        self.state.current_tab = Tab::Shield;
                    }
                    if ui
                        .selectable_label(self.state.current_tab == Tab::Unlock, "[ 🔓 ] UNLOCK")
                        .clicked()
                    {
                        self.state.current_tab = Tab::Unlock;
                    }
                    if ui
                        .selectable_label(
                            self.state.current_tab == Tab::IdentityManager,
                            "[ 👥 ] IDENTITIES",
                        )
                        .clicked()
                    {
                        self.state.current_tab = Tab::IdentityManager;
                    }
                    if ui
                        .selectable_label(
                            self.state.current_tab == Tab::HowToUse,
                            "[ 📖 ] HOW TO USE",
                        )
                        .clicked()
                    {
                        self.state.current_tab = Tab::HowToUse;
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_enabled_ui(!self.state.is_loading, |ui| match self.state.current_tab {
                Tab::Shield => shield_tab::draw_shield(&mut self.state, ui, ctx),
                Tab::Unlock => unlock_tab::draw_unlock(&mut self.state, ui, ctx),
                Tab::IdentityManager => {
                    identity_tab::draw_identity_manager(&mut self.state, ui, ctx)
                }
                Tab::HowToUse => guide_tab::draw_guide(&mut self.state, ui, ctx),
            });
        });
    }
}
