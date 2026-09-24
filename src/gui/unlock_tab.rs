use eframe::egui::{self, RichText};
use rand::SeedableRng;
use std::path::Path;

use crate::engine::file_handler::{save_extracted_file, QuantumCapsule};
use crate::engine::QuantumEngine;
use crate::gui::state::{GuiState, WorkerResult};
use crate::gui::theme::{matrix_card_frame, MATRIX_BRIGHT_GREEN, MATRIX_NEON_GREEN};

pub fn draw_unlock(state: &mut GuiState, ui: &mut egui::Ui, ctx: &egui::Context) {
    let mut clear_decrypted = false;
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.heading("ASYNCHRONOUS EXTRACTION MODULE");
            ui.label(
                "Blind-decrypt Chameleon containers in constant time without metadata leakage.",
            );
            ui.add_space(8.0);

            // SECTION 1: CONTAINER SELECTION
            matrix_card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("1. CONTAINER INPUT")
                        .strong()
                        .color(MATRIX_BRIGHT_GREEN),
                );
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    if ui.button("📦 [ LOAD .DIEGOX CONTAINER ]").clicked() {
                        let mut dialog = rfd::FileDialog::new().add_filter("Capsule", &["diegox"]);
                        if state.output_dir.exists() {
                            dialog = dialog.set_directory(&state.output_dir);
                        }
                        if let Some(path) = dialog.pick_file() {
                            state.file_to_decrypt = path.display().to_string();
                        }
                    }
                    if state.file_to_decrypt.is_empty() {
                        ui.label("No container loaded.");
                    } else {
                        ui.label(&state.file_to_decrypt);
                    }
                });
            });

            ui.add_space(8.0);

            // SECTION 2: EXTRACTION PASSPHRASE
            matrix_card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("2. EXTRACTION PASSPHRASE")
                        .strong()
                        .color(MATRIX_BRIGHT_GREEN),
                );
                ui.add_space(4.0);

                ui.label("> ENTER PASSPHRASE (REAL OR DECOY):");
                ui.add(
                    egui::TextEdit::singleline(&mut state.unlock_passphrase)
                        .password(true)
                        .desired_width(f32::INFINITY),
                );
                ui.add_space(8.0);

                if ui
                    .button(RichText::new("⚡ [ FILTER KEM ]").strong())
                    .clicked()
                {
                    if state.file_to_decrypt.is_empty() {
                        state.status =
                            "ERROR: Please select a .diegox container to decrypt.".to_string();
                        return;
                    }
                    if state.unlock_passphrase.is_empty() {
                        state.status = "ERROR: Extraction passphrase cannot be empty.".to_string();
                        return;
                    }

                    state.is_loading = true;
                    state.status =
                        "Executing constant time (XOR) operations in background thread..."
                            .to_string();
                    state.decrypted_bytes_output = None;

                    let tx = state.tx.clone();
                    let ui_ctx = ctx.clone();
                    let path = state.file_to_decrypt.clone();
                    let phrase = state.unlock_passphrase.clone();

                    std::thread::spawn(move || {
                        match QuantumCapsule::load_from_disk(&path) {
                            Ok(file_data) => {
                                let worker_app =
                                    QuantumEngine::new(rand_chacha::ChaCha20Rng::from_entropy());
                                match worker_app.decrypt_blind(&file_data, &phrase) {
                                    Ok((bytes, ext)) => {
                                        let _ =
                                            tx.send(WorkerResult::DecryptionSuccess(bytes, ext));
                                    }
                                    Err(e) => {
                                        let _ = tx.send(WorkerResult::Failure(e));
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(WorkerResult::Failure(e));
                            }
                        }
                        ui_ctx.request_repaint();
                    });
                }
            });

            // SECTION 3: UNSEALED PAYLOAD RECOVERY
            if let Some(bytes) = state.decrypted_bytes_output.as_ref() {
                ui.add_space(8.0);
                matrix_card_frame().show(ui, |ui| {
                    ui.label(
                        RichText::new("» CONTAINER SUCCESSFULLY UNSEALED")
                            .strong()
                            .color(MATRIX_BRIGHT_GREEN),
                    );
                    ui.add_space(4.0);

                    ui.label(format!(
                        "Payload size: {} bytes | Recovered extension: .{}",
                        bytes.len(),
                        state.decrypted_ext_output
                    ));
                    ui.add_space(6.0);

                    // Determine base name from input container if possible
                    let base_stem = Path::new(&state.file_to_decrypt)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("extracted_payload");
                    let target_filename =
                        format!("{}_unsealed.{}", base_stem, state.decrypted_ext_output);
                    let target_write_path = state.output_dir.join(&target_filename);

                    ui.label(
                        RichText::new(format!(
                            "Default output destination: {}",
                            target_write_path.display()
                        ))
                        .small()
                        .color(MATRIX_NEON_GREEN),
                    );
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui
                            .button(RichText::new("💾 [ QUICK EXPORT TO OUTPUT DIR ]").strong())
                            .clicked()
                        {
                            let bytes_to_write = bytes.clone();
                            let out_path = target_write_path.clone();
                            let tx = state.tx.clone();
                            let ui_ctx = ctx.clone();

                            std::thread::spawn(move || {
                                match save_extracted_file(&out_path, &bytes_to_write) {
                                    Ok(()) => {
                                        let _ = tx.send(WorkerResult::EncryptionSuccess(format!(
                                            "SUCCESS: EXTRACTED FILE SAVED TO {}",
                                            out_path.display()
                                        )));
                                    }
                                    Err(e) => {
                                        let _ = tx.send(WorkerResult::Failure(format!(
                                            "ERROR WRITING EXTRACTED FILE: {}",
                                            e
                                        )));
                                    }
                                }
                                ui_ctx.request_repaint();
                            });
                            clear_decrypted = true;
                        }

                        if ui.button("📁 [ CHOOSE DESTINATION... ]").clicked() {
                            let mut dialog = rfd::FileDialog::new()
                                .set_file_name(&target_filename)
                                .add_filter("Decoded File", &[&state.decrypted_ext_output]);
                            if state.output_dir.exists() {
                                dialog = dialog.set_directory(&state.output_dir);
                            }
                            if let Some(custom_path) = dialog.save_file() {
                                let bytes_to_write = bytes.clone();
                                let tx = state.tx.clone();
                                let ui_ctx = ctx.clone();

                                std::thread::spawn(move || {
                                    match save_extracted_file(&custom_path, &bytes_to_write) {
                                        Ok(()) => {
                                            let _ =
                                                tx.send(WorkerResult::EncryptionSuccess(format!(
                                                    "SUCCESS: EXTRACTED FILE SAVED TO {}",
                                                    custom_path.display()
                                                )));
                                        }
                                        Err(e) => {
                                            let _ = tx.send(WorkerResult::Failure(format!(
                                                "ERROR WRITING EXTRACTED FILE: {}",
                                                e
                                            )));
                                        }
                                    }
                                    ui_ctx.request_repaint();
                                });
                                clear_decrypted = true;
                            }
                        }
                    });
                });
            }

            ui.add_space(10.0);
        });

    if clear_decrypted {
        state.decrypted_bytes_output = None;
    }
}
