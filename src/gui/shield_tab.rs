use eframe::egui::{self, RichText};
use rand::SeedableRng;
use std::fs;
use std::path::Path;

use crate::engine::QuantumEngine;
use crate::gui::state::{GuiState, InputMode, TargetMode, WorkerResult};
use crate::gui::theme::{matrix_card_frame, MATRIX_BRIGHT_GREEN, MATRIX_NEON_GREEN};
use crate::keys::{derive_keypair_from_passphrase, PublicKey};

pub fn draw_shield(state: &mut GuiState, ui: &mut egui::Ui, ctx: &egui::Context) {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.heading("QUANTUM SHIELDING MODULE");
            ui.label("Encapsulate payloads with dual-KEM lattice polynomials and constant-time padding.");
            ui.add_space(8.0);

    // SECTION 1: KEY / TARGET SELECTION
    matrix_card_frame().show(ui, |ui| {
        ui.label(RichText::new("1. TARGET & KEY CONFIGURATION").strong().color(MATRIX_BRIGHT_GREEN));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut state.target_mode,
                TargetMode::Me,
                "🔒 PERSONAL (VAULT)",
            );
            ui.selectable_value(
                &mut state.target_mode,
                TargetMode::Contact,
                "✉️ FOR A CONTACT",
            );
        });
        ui.add_space(6.0);

        if state.target_mode == TargetMode::Me {
            ui.label("> REAL PASSPHRASE (Volatile generation in RAM):");
            ui.add(
                egui::TextEdit::singleline(&mut state.real_passphrase)
                    .password(true)
                    .desired_width(f32::INFINITY),
            );
        } else {
            ui.group(|ui| {
                ui.label("> RECIPIENT'S PUBLIC KEY (ARMORED BASE64):");
                ui.horizontal(|ui| {
                    if ui.button("📁 Load File .diegoxkey").clicked() {
                        let mut dialog = rfd::FileDialog::new().add_filter("DIEGOX Public Key", &["diegoxkey"]);
                        if state.output_dir.exists() {
                            dialog = dialog.set_directory(&state.output_dir);
                        }
                        if let Some(path) = dialog.pick_file() {
                            if let Ok(content) = fs::read_to_string(&path) {
                                state.contact_pub_key_path = path.display().to_string();
                                state.contact_pub_key_bytes = content;
                                state.status = "PUBLIC KEY LOADED FROM FILE INTO TEXT FIELD.".to_string();
                            }
                        }
                    }
                    if !state.contact_pub_key_path.is_empty() {
                        ui.label(format!("Loaded: {}", state.contact_pub_key_path));
                    }
                });

                ui.add_space(5.0);
                ui.label("Paste the armored Base64 key directly or load it from file above:");
                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut state.contact_pub_key_bytes)
                            .hint_text("-----BEGIN DIEGOX PUBLIC KEY-----\n...\n-----END DIEGOX PUBLIC KEY-----")
                            .desired_width(f32::INFINITY),
                    );
                });
            });
        }
    });

    ui.add_space(8.0);

    // SECTION 2: REAL PAYLOAD
    matrix_card_frame().show(ui, |ui| {
        ui.label(RichText::new("2. PRIMARY PAYLOAD (SLOT ALPHA)").strong().color(MATRIX_BRIGHT_GREEN));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut state.input_mode, InputMode::Text, "TEXT INPUT");
            ui.selectable_value(&mut state.input_mode, InputMode::File, "FILE / ARCHIVE");
        });
        ui.add_space(4.0);

        match state.input_mode {
            InputMode::Text => {
                ui.label("Enter confidential text:");
                ui.add(
                    egui::TextEdit::multiline(&mut state.input_real)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                );
            }
            InputMode::File => {
                ui.horizontal(|ui| {
                    if ui.button("[ LOAD REAL FILE ]").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            state.file_real = path.display().to_string();
                        }
                    }
                    if state.file_real.is_empty() {
                        ui.label("No file selected.");
                    } else {
                        ui.label(&state.file_real);
                    }
                });
            }
        }
    });

    ui.add_space(8.0);

    // SECTION 3: DECOY CONFIGURATION (CHAMELEON SYSTEM)
    matrix_card_frame().show(ui, |ui| {
        ui.label(RichText::new("3. PLAUSIBLE DENIABILITY (SLOT BETA)").strong().color(MATRIX_BRIGHT_GREEN));
        ui.add_space(4.0);

        ui.checkbox(
            &mut state.use_decoy,
            "ENABLE DECOY (DOUBLE SYMMETRIC CONTAINER)",
        );

        if state.use_decoy {
            ui.add_space(4.0);
            ui.label("> ALIBI / DECOY PASSPHRASE:");
            ui.add(
                egui::TextEdit::singleline(&mut state.decoy_passphrase)
                    .password(true)
                    .desired_width(f32::INFINITY),
            );

            ui.add_space(4.0);
            match state.input_mode {
                InputMode::Text => {
                    ui.label("Enter decoy cover text:");
                    ui.add(
                        egui::TextEdit::multiline(&mut state.input_decoy_text)
                            .desired_rows(2)
                            .desired_width(f32::INFINITY),
                    );
                }
                InputMode::File => {
                    ui.horizontal(|ui| {
                        if ui.button("[ LOAD DECOY FILE ]").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                state.file_decoy = path.display().to_string();
                            }
                        }
                        if state.file_decoy.is_empty() {
                            ui.label("No decoy file selected.");
                        } else {
                            ui.label(&state.file_decoy);
                        }
                    });
                }
            }
        }
    });

    ui.add_space(8.0);

    // SECTION 4: OUTPUT CONTAINER
    matrix_card_frame().show(ui, |ui| {
        ui.label(RichText::new("4. CONTAINER INJECTION & EXPORT").strong().color(MATRIX_BRIGHT_GREEN));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Output Filename:");
            ui.add(
                egui::TextEdit::singleline(&mut state.output_name)
                    .hint_text("e.g. confidential_archive")
                    .desired_width(260.0),
            );
            ui.label(".diegox");
        });

        let target_display = state.output_dir.join(format!(
            "{}.diegox",
            if state.output_name.trim().is_empty() {
                "output_filename"
            } else {
                state.output_name.trim()
            }
        ));
        ui.label(
            RichText::new(format!("Target path: {}", target_display.display()))
                .small()
                .color(MATRIX_NEON_GREEN),
        );

        ui.add_space(8.0);

        if ui.button(RichText::new("🚀 [ INJECT IN BACKGROUND ]").strong()).clicked() {
            let trimmed_name = state.output_name.trim();
            if trimmed_name.is_empty() {
                state.status = "ERROR: Please specify an output container name.".to_string();
                return;
            }

            if state.target_mode == TargetMode::Me && state.real_passphrase.is_empty() {
                state.status = "ERROR: Real passphrase cannot be empty.".to_string();
                return;
            }

            if state.target_mode == TargetMode::Contact && state.contact_pub_key_bytes.trim().is_empty() {
                state.status = "ERROR: Contact public key cannot be empty.".to_string();
                return;
            }

            state.is_loading = true;
            state.status =
                "Calculating matrices and deriving entropy in background thread...".to_string();

            let tx = state.tx.clone();
            let ui_ctx = ctx.clone();

            let real_passphrase = state.real_passphrase.clone();
            let target_mode = state.target_mode;
            let pub_key_bytes = state.contact_pub_key_bytes.clone();
            let use_decoy = state.use_decoy;
            let decoy_passphrase = state.decoy_passphrase.clone();
            let input_mode = state.input_mode;
            let input_real = state.input_real.clone();
            let file_real = state.file_real.clone();
            let input_decoy_text = state.input_decoy_text.clone();
            let file_decoy = state.file_decoy.clone();
            let out_name = format!("{}.diegox", trimmed_name);
            let target_path = state.output_dir.join(&out_name);

            std::thread::spawn(move || {
                let pub_real = if target_mode == TargetMode::Me {
                    derive_keypair_from_passphrase(&real_passphrase).0
                } else {
                    match PublicKey::from_text(&pub_key_bytes) {
                        Ok(pk) => pk,
                        Err(_) => {
                            let _ = tx.send(WorkerResult::Failure(
                                "ERROR: Corrupted contact key.".to_string(),
                            ));
                            ui_ctx.request_repaint();
                            return;
                        }
                    }
                };

                let opt_pub_dummy = if use_decoy && !decoy_passphrase.is_empty() {
                    Some(derive_keypair_from_passphrase(&decoy_passphrase).0)
                } else {
                    None
                };

                // Smart extraction of extensions without revealing full names
                let ext_real = match input_mode {
                    InputMode::Text => "txt".to_string(),
                    InputMode::File => Path::new(&file_real)
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("bin")
                        .to_string(),
                };

                let ext_dummy = match input_mode {
                    InputMode::Text => "txt".to_string(),
                    InputMode::File => Path::new(&file_decoy)
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("txt")
                        .to_string(),
                };

                let real_bytes = match input_mode {
                    InputMode::Text => input_real.as_bytes().to_vec(),
                    InputMode::File => fs::read(&file_real).unwrap_or_default(),
                };

                let dummy_bytes = if use_decoy {
                    match input_mode {
                        InputMode::Text => input_decoy_text.as_bytes().to_vec(),
                        InputMode::File => fs::read(&file_decoy).unwrap_or_default(),
                    }
                } else {
                    b"FULL_RANDOM_QUANTUM_NOISE_PADDING".to_vec()
                };

                let mut worker_app = QuantumEngine::new(rand_chacha::ChaCha20Rng::from_entropy());
                let file_out = worker_app.encrypt_dual(
                    &pub_real,
                    &real_bytes,
                    &ext_real,
                    opt_pub_dummy.as_ref(),
                    &dummy_bytes,
                    &ext_dummy,
                );

                match file_out.save_to_disk(&target_path) {
                    Ok(()) => {
                        let _ = tx.send(WorkerResult::EncryptionSuccess(format!(
                            "SUCCESS: CAPSULE SAVED TO {}",
                            target_path.display()
                        )));
                    }
                    Err(e) => {
                        let _ = tx.send(WorkerResult::Failure(format!(
                            "ERROR WRITING CAPSULE: {}",
                            e
                        )));
                    }
                }
                ui_ctx.request_repaint();
            });
        }
    });

    ui.add_space(10.0);
    });
}
