use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Shield,
    Unlock,
    IdentityManager,
    HowToUse,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode {
    Text,
    File,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TargetMode {
    Me,
    Contact,
}

pub enum WorkerResult {
    EncryptionSuccess(String),
    DecryptionSuccess(Vec<u8>, String),
    IdentitySuccess(IdentityItem),
    Failure(String),
}

pub struct IdentityItem {
    pub alias: String,
    pub public_key_text: String,
    pub public_key_display: String,
}

pub struct GuiState {
    pub current_tab: Tab,
    pub input_mode: InputMode,
    pub target_mode: TargetMode,

    pub real_passphrase: String,
    pub decoy_passphrase: String,
    pub contact_pub_key_path: String,
    pub contact_pub_key_bytes: String,

    pub input_real: String,
    pub input_decoy_text: String,
    pub file_real: String,
    pub file_decoy: String,
    pub output_name: String,
    pub use_decoy: bool,

    pub file_to_decrypt: String,
    pub unlock_passphrase: String,

    pub decrypted_bytes_output: Option<Vec<u8>>,
    pub decrypted_ext_output: String, // Stores the filtered extension in RAM

    pub alias_vault: String,
    pub passphrase_vault: String,
    pub identities: Vec<IdentityItem>,
    pub selected_identity_idx: Option<usize>,
    pub selected_identity_pub_key: String,

    pub tx: Sender<WorkerResult>,
    pub rx: Receiver<WorkerResult>,
    pub is_loading: bool,
    pub status: String,
    pub output_dir: PathBuf,
    pub logo_texture: Option<eframe::egui::TextureHandle>,
}
