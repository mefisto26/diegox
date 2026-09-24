use ndarray::Array1;
use serde::{Deserialize, Serialize};

/// Public key representation containing polynomial vector for the post-quantum KEM scheme.
#[derive(Serialize, Deserialize, Clone)]
pub struct PublicKey {
    pub public_seed: [u8; 32],
    pub b: Vec<i64>,
}

impl PublicKey {
    /// Serializes and encodes the Public Key into an armored Base64 text format for sharing.
    pub fn to_text(&self) -> String {
        let encoded = bincode::serialize(self).unwrap();
        use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
        let base64_data = BASE64.encode(encoded);

        format!(
            "-----BEGIN DIEGOX PUBLIC KEY-----\n{}\n-----END DIEGOX PUBLIC KEY-----",
            base64_data
        )
    }

    /// Decodes and reconstructs a Public Key received in armored Base64 text format.
    pub fn from_text(text: &str) -> Result<Self, String> {
        let header = "-----BEGIN DIEGOX PUBLIC KEY-----";
        let footer = "-----END DIEGOX PUBLIC KEY-----";

        if !text.contains(header) || !text.contains(footer) {
            return Err("Invalid armored format: missing headers".to_string());
        }

        let start = text.find(header).unwrap() + header.len();
        let end = text.find(footer).unwrap();
        let base64_data = text[start..end].trim();

        let clean = base64_data
            .replace("\n", "")
            .replace("\r", "")
            .replace(" ", "");
        use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
        let decoded = BASE64
            .decode(clean)
            .map_err(|_| "Invalid or corrupted Base64".to_string())?;
        bincode::deserialize(&decoded).map_err(|_| "Invalid public key structure".to_string())
    }
}

/// Private key representation containing the secret small binary polynomial vector.
pub struct PrivateKey {
    pub sec_s: Array1<i64>,
}

impl PrivateKey {
    /// Reveals a reference to the secret vector for KEM engine operations.
    pub fn reveal(&self) -> &Array1<i64> {
        &self.sec_s
    }
}

// =========================================================================
// CHAMELEON SYSTEM: Automatic Private Key destruction in RAM
// =========================================================================
impl Drop for PrivateKey {
    fn drop(&mut self) {
        // As soon as the key goes out of scope, we flood
        // the vector buffer with pure zeros to neutralize forensic RAM analysis.
        self.sec_s.fill(0);
    }
}
