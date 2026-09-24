use sha2::{Digest, Sha256};

/// The sealing now safely stores the extension length and the extension itself
pub fn build_sealed_payload(data: &[u8], ext: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let ext_bytes = ext.as_bytes();

    let mut sealed = Vec::with_capacity(32 + 1 + ext_bytes.len() + data.len());
    sealed.extend_from_slice(&hasher.finalize());
    sealed.push(ext_bytes.len() as u8);
    sealed.extend_from_slice(ext_bytes);
    sealed.extend_from_slice(data);
    sealed
}
