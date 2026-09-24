use ndarray::Array1;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct QuantumCapsule {
    pub kem_alpha: (Array1<i64>, Array1<i64>),
    pub block_alpha: Vec<u8>,
    pub kem_beta: (Array1<i64>, Array1<i64>),
    pub block_beta: Vec<u8>,
}

impl QuantumCapsule {
    pub fn save_to_disk<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), String> {
        let encoded = bincode::serialize(self).map_err(|e| format!("Serialization error: {}", e))?;
        let mut file = File::create(path.as_ref()).map_err(|e| format!("Failed to create capsule file: {}", e))?;
        file.write_all(&encoded).map_err(|e| format!("Failed to write capsule data: {}", e))?;
        Ok(())
    }

    pub fn load_from_disk<P: AsRef<std::path::Path>>(path: P) -> Result<Self, String> {
        let mut file = File::open(path.as_ref()).map_err(|e| format!("Capsule not found: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read capsule: {}", e))?;
        bincode::deserialize(&buffer).map_err(|e| format!("Corrupt capsule or invalid format: {}", e))
    }
}

pub fn save_extracted_file<P: AsRef<std::path::Path>>(path: P, bytes: &[u8]) -> Result<(), String> {
    let mut file = File::create(path.as_ref()).map_err(|e| format!("Failed to create output file: {}", e))?;
    file.write_all(bytes).map_err(|e| format!("Failed to write output file: {}", e))?;
    Ok(())
}
