use eframe::egui::{self, Color32, RichText};

use crate::gui::state::GuiState;
use crate::gui::theme::{callout_frame, matrix_card_frame, MATRIX_BRIGHT_GREEN};

pub fn draw_guide(_state: &mut GuiState, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.heading("DIEGOX SUITE // FIELD OPERATIONS MANUAL");
    ui.label(
        RichText::new("Post-Quantum Plausible Deniability & Lattice-Based Encryption Architecture")
            .color(Color32::from_rgb(0x80, 0xDD, 0x80)),
    );
    ui.add_space(8.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            // SECTION 1: QUICKSTART WORKFLOW
            matrix_card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("⚡ SECTION 1: QUICKSTART WORKFLOW (1-MINUTE RUNTHROUGH)")
                        .strong()
                        .color(MATRIX_BRIGHT_GREEN),
                );
                ui.add_space(6.0);

                ui.label(
                    "Follow these fast steps to seal confidential data into a post-quantum container or unseal one:",
                );
                ui.add_space(4.0);

                callout_frame().show(ui, |ui| {
                    ui.label(RichText::new("🔒 Step A: Sealing Data (SHIELD Tab)").strong());
                    ui.label("1. Select Mode: Choose 'PERSONAL (VAULT)' for your own files, or 'FOR A CONTACT' using their public key.");
                    ui.label("2. Enter Passphrase: Type a high-entropy passphrase. This is derived via Argon2id into volatile RAM.");
                    ui.label("3. Choose Input: Select TEXT mode to type directly, or FILE mode to select any file/archive.");
                    ui.label("4. (Optional) Enable Decoy: Check 'ENABLE DECOY' to configure a second plausible alibi slot.");
                    ui.label("5. Name & Inject: Enter container name and click '[ INJECT IN BACKGROUND ]'. Output is saved to your configured Output Folder as a .diegox container.");
                });

                ui.add_space(6.0);
                callout_frame().show(ui, |ui| {
                    ui.label(RichText::new("🔓 Step B: Extracting Data (UNLOCK Tab)").strong());
                    ui.label("1. Load Container: Click '[ LOAD .DIEGOX CONTAINER ]' to select the target .diegox file.");
                    ui.label("2. Enter Passphrase: Type the real passphrase (or the decoy passphrase if compelled).");
                    ui.label("3. Filter KEM: Click '⚡ [ FILTER KEM ]' to initiate lattice blind-decryption in constant time.");
                    ui.label("4. Export or Inspect: Click '💾 [ EXPORT TO DISK ]' to write the recovered file to your output directory.");
                });
            });

            ui.add_space(10.0);

            // SECTION 2: THE CHAMELEON SYSTEM
            matrix_card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("🎭 SECTION 2: THE CHAMELEON SYSTEM (REAL VS. DECOY)")
                        .strong()
                        .color(MATRIX_BRIGHT_GREEN),
                );
                ui.add_space(6.0);

                ui.label(
                    "The Chameleon System provides true Plausible Deniability under physical coercion or adversarial inspection:"
                );
                ui.add_space(4.0);

                ui.label("• Dual Mathematical Slots (Alpha & Beta):");
                ui.label("  Every .diegox file contains exactly two identical mathematical slots: Slot Alpha and Slot Beta. Both slots are encrypted using Learning-With-Errors (LWE) lattice polynomials.");
                ui.add_space(3.0);

                ui.label("• Uniform Entropy & Indistinguishability:");
                ui.label("  Even if you choose not to use a decoy, the second slot is automatically filled with true quantum-safe pseudorandom padding. An external adversary examining the raw bytes cannot determine whether the container holds one message, two messages, or which slot is real.");
                ui.add_space(3.0);

                callout_frame().show(ui, |ui| {
                    ui.label(
                        RichText::new("🛡️ Coercion Defense Scenario")
                            .strong()
                            .color(Color32::from_rgb(0x00, 0xFF, 0xCC)),
                    );
                    ui.label(
                        "If you are forced under duress to disclose your password, provide your Decoy Passphrase. The software mathematically unlocks Slot Beta, recovering benign decoy data (e.g. travel recipes or non-sensitive notes). It is mathematically impossible for the adversary to prove that Slot Alpha exists or that another key exists for the container.",
                    );
                });
            });

            ui.add_space(10.0);

            // SECTION 3: IDENTITY & PUBLIC KEY MANAGEMENT
            matrix_card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("🔑 SECTION 3: IDENTITY & PUBLIC KEY MANAGEMENT")
                        .strong()
                        .color(MATRIX_BRIGHT_GREEN),
                );
                ui.add_space(6.0);

                ui.label("• Post-Quantum Asymmetric Encapsulation vs. Symmetric Vaulting:");
                ui.label("  - Symmetric Mode (Passphrase): Uses Argon2id memory-hard hashing combined with LWE lattice KEM to seal files for yourself.");
                ui.label("  - Asymmetric Mode (Public Key): Encapsulates shared secrets against a recipient's public polynomial vector b without ever knowing their private key.");
                ui.add_space(4.0);

                ui.label("• Exporting & Sharing Your Public Key:");
                ui.label("  1. Navigate to the [ 👥 ] IDENTITIES tab.");
                ui.label("  2. Enter an Alias and Passphrase, then click 'Generate and Register'.");
                ui.label("  3. Under the generated identity, expand 'View / Copy Public Key'.");
                ui.label("  4. Click 'Copy to Clipboard' or 'Save as File .diegoxkey' to export Armored Base64 text.");
                ui.label("  5. Transmit this public key to contacts via secure channels (Signal, email, QR code).");
            });

            ui.add_space(10.0);

            // SECTION 4: OPERATIONAL SECURITY (OPSEC) BEST PRACTICES
            matrix_card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("🛡️ SECTION 4: OPERATIONAL SECURITY (OPSEC) BEST PRACTICES")
                        .strong()
                        .color(MATRIX_BRIGHT_GREEN),
                );
                ui.add_space(6.0);

                ui.label("• Argon2id Resilience & Passphrase Selection:");
                ui.label("  Passphrase derivation enforces Argon2id with memory-hard cost parameters. Choose passphrases of at least 16-24 characters (e.g., 4-5 random words) to defeat GPU/ASIC dictionary attacks.");
                ui.add_space(3.0);

                ui.label("• Out-of-Band (OOB) Verification:");
                ui.label("  When receiving a contact's .diegoxkey, verify its integrity out-of-band (e.g., voice call or in-person verification) to eliminate Man-In-The-Middle (MITM) key substitutions.");
                ui.add_space(3.0);

                ui.label("• Volatile RAM & Zeroization on Exit:");
                ui.label("  Private key matrices and decrypted plaintext are held strictly in non-swappable volatile RAM and zeroized on drop. No unencrypted artifacts are persisted to disk unless you explicitly choose to export them.");
                ui.add_space(4.0);

                callout_frame().show(ui, |ui| {
                    ui.label(
                        RichText::new("⚠️ Standard Sanitization Advice")
                            .strong()
                            .color(Color32::from_rgb(0xFF, 0xCC, 0x00)),
                    );
                    ui.label(
                        "Always configure your Output Folder to an encrypted volume (e.g. BitLocker/VeraCrypt) for highest security. After extracting sensitive files, remember to securely wipe or shred them when no longer required.",
                    );
                });
            });

            ui.add_space(15.0);
        });
}
