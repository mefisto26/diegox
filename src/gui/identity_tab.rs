use eframe::egui::{self, RichText};
use std::fs;

use crate::gui::state::{GuiState, IdentityItem, WorkerResult};
use crate::gui::theme::{matrix_card_frame, MATRIX_BRIGHT_GREEN, MATRIX_NEON_GREEN};
use crate::keys::derive_keypair_from_passphrase;

pub fn draw_identity_manager(state: &mut GuiState, ui: &mut egui::Ui, ctx: &egui::Context) {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.heading("ASYNCHRONOUS IDENTITY MANAGER");
            ui.label("Manage your key profiles and export public keys for your trusted contacts.");
            ui.add_space(8.0);

            // SECTION 1: CREATE NEW IDENTITY
            matrix_card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("» CREATE NEW QUANTUM IDENTITY")
                        .strong()
                        .color(MATRIX_BRIGHT_GREEN),
                );
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label("Alias/Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut state.alias_vault)
                            .hint_text("e.g. Alice_Work")
                            .desired_width(220.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Passphrase:");
                    ui.add(
                        egui::TextEdit::singleline(&mut state.passphrase_vault)
                            .password(true)
                            .desired_width(220.0),
                    );
                });
                ui.add_space(6.0);

                if ui
                    .button(RichText::new("⚡ Generate and Register").strong())
                    .clicked()
                {
                    if !state.alias_vault.is_empty() && !state.passphrase_vault.is_empty() {
                        state.is_loading = true;
                        state.status = "Deriving post-quantum polynomial matrices in background..."
                            .to_string();

                        let tx = state.tx.clone();
                        let ui_ctx = ctx.clone();
                        let alias = state.alias_vault.clone();
                        let phrase = state.passphrase_vault.clone();

                        // Delegation to secondary thread to avoid micro-freezes in GUI
                        std::thread::spawn(move || {
                            let (pub_k, _) = derive_keypair_from_passphrase(&phrase);
                            let key_text = pub_k.to_text();
                            let _ = tx.send(WorkerResult::IdentitySuccess(IdentityItem {
                                alias,
                                public_key_text: key_text.clone(),
                                public_key_display: key_text,
                            }));
                            ui_ctx.request_repaint();
                        });
                    }
                }
            });

            ui.add_space(8.0);

            // SECTION 2: ACTIVE KEYRING
            matrix_card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("» ACTIVE IDENTITY KEYRING")
                        .strong()
                        .color(MATRIX_BRIGHT_GREEN),
                );
                ui.add_space(4.0);

                let mut item_to_remove: Option<usize> = None;

                if state.identities.is_empty() {
                    ui.label("No identities created in this session. Generate one above.");
                } else {
                    egui::ScrollArea::vertical()
                        .max_height(140.0)
                        .show(ui, |ui| {
                            for (index, id) in state.identities.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    let is_selected = state.selected_identity_idx == Some(index);
                                    if ui
                                        .selectable_label(is_selected, format!("• [{}]", id.alias))
                                        .clicked()
                                    {
                                        state.selected_identity_idx = Some(index);
                                        state.selected_identity_pub_key =
                                            id.public_key_text.clone();
                                    }

                                    let btn_delete = egui::Button::new(
                                        RichText::new("Delete")
                                            .color(egui::Color32::from_rgb(255, 70, 70)),
                                    );
                                    if ui.add(btn_delete).clicked() {
                                        item_to_remove = Some(index);
                                    }
                                });
                            }
                        });
                }

                if let Some(idx) = item_to_remove {
                    if state.selected_identity_idx == Some(idx) {
                        state.selected_identity_idx = None;
                        state.selected_identity_pub_key.clear();
                    } else if let Some(sel_idx) = state.selected_identity_idx {
                        if sel_idx > idx {
                            state.selected_identity_idx = Some(sel_idx - 1);
                        }
                    }
                    state.identities.remove(idx);
                    state.status = "IDENTITY DELETED FROM EPHEMERAL KEYRING.".to_string();
                }
            });

            // Show selected identity's public key inside a collapsible section
            if let Some(idx) = state.selected_identity_idx {
                if let Some(id) = state.identities.get(idx) {
                    let alias = id.alias.clone();
                    ui.add_space(8.0);
                    matrix_card_frame().show(ui, |ui| {
                        egui::CollapsingHeader::new(
                            RichText::new(format!("🔑 View / Copy Public Key [{}]", alias))
                                .color(MATRIX_NEON_GREEN)
                                .strong(),
                        )
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.add_space(4.0);

                            // Read-only text area with vertical scroll
                            egui::ScrollArea::vertical()
                                .max_height(160.0)
                                .show(ui, |ui| {
                                    let mut key_display = state.selected_identity_pub_key.clone();
                                    ui.add(
                                        egui::TextEdit::multiline(&mut key_display)
                                            .desired_width(f32::INFINITY)
                                            .interactive(false),
                                    );
                                });

                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                if ui.button("📋 Copy to Clipboard").clicked() {
                                    ctx.output_mut(|o| {
                                        o.copied_text = state.selected_identity_pub_key.clone()
                                    });
                                    state.status = "PUBLIC KEY COPIED TO CLIPBOARD.".to_string();
                                }

                                if ui.button("💾 Save as File .diegoxkey").clicked() {
                                    let mut dialog = rfd::FileDialog::new()
                                        .set_file_name(&format!("{}_public_key.diegoxkey", alias))
                                        .add_filter("DIEGOX Public Key", &["diegoxkey"]);
                                    if state.output_dir.exists() {
                                        dialog = dialog.set_directory(&state.output_dir);
                                    }
                                    if let Some(path) = dialog.save_file() {
                                        let _ = fs::write(path, &state.selected_identity_pub_key);
                                        state.status = "PUBLIC KEY EXPORTED TO FILE.".to_string();
                                    }
                                }
                            });
                        });
                    });
                }
            }

            ui.add_space(10.0);
        });
}
