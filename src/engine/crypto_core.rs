use ndarray::Array1;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

use crate::config::{DELTA, DIMENSION, MODULUS};
use crate::engine::file_handler::QuantumCapsule;
use crate::engine::payload_builder::build_sealed_payload;
use crate::keys::{
    build_negacyclic_matrix, expand_matrix_a, generate_identity, PrivateKey, PublicKey,
};

pub struct QuantumEngine {
    pub chacha_rng: ChaCha20Rng,
}

impl QuantumEngine {
    pub fn new(rng: ChaCha20Rng) -> Self {
        QuantumEngine { chacha_rng: rng }
    }

    fn build_kem_block(
        &mut self,
        pub_key: &PublicKey,
        payload: &[u8],
        ext: &str,
    ) -> ((Array1<i64>, Array1<i64>), Vec<u8>) {
        let mut seed = [0u8; 32];
        self.chacha_rng.fill(&mut seed);

        let mut bits = Vec::new();
        for &byte in seed.iter() {
            for i in 0..8 {
                bits.push((byte >> i) & 1);
            }
        }

        let mut u_vec = Array1::<i64>::zeros(DIMENSION);
        let mut v_vec = Array1::<i64>::zeros(DIMENSION);
        let mut r_vec = Array1::<i64>::zeros(DIMENSION);
        for i in 0..DIMENSION {
            r_vec[i] = self.chacha_rng.gen_range(0..2);
        }

        // Expand matrix A deterministically from the public seed
        let pub_a = expand_matrix_a(&pub_key.public_seed);
        let b_mat = build_negacyclic_matrix(&pub_key.b);

        for i in 0..DIMENSION {
            let m_val = if i < bits.len() {
                (bits[i] as i64) * DELTA
            } else {
                0
            };

            let mut a_dot_r = 0i64;
            let mut b_dot_r = 0i64;
            for j in 0..DIMENSION {
                a_dot_r += pub_a[(i, j)] * r_vec[j];
                b_dot_r += b_mat[(i, j)] * r_vec[j];
            }
            let err1 = self.chacha_rng.gen_range(-1..=1);
            let err2 = self.chacha_rng.gen_range(-1..=1);

            u_vec[i] = ((a_dot_r + err1) % MODULUS + MODULUS) % MODULUS;
            v_vec[i] = ((b_dot_r + err2 + m_val) % MODULUS + MODULUS) % MODULUS;
        }

        let mut hasher = Sha256::new();
        hasher.update(seed);
        let sym_key: [u8; 32] = hasher.finalize().into();

        let mut block = build_sealed_payload(payload, ext);

        let mut stream = ChaCha20Rng::from_seed(sym_key);
        for byte in block.iter_mut() {
            *byte ^= stream.gen::<u8>();
        }

        ((u_vec, v_vec), block)
    }

    pub fn encrypt_dual(
        &mut self,
        pub_real: &PublicKey,
        data_real: &[u8],
        ext_real: &str,
        pub_dummy: Option<&PublicKey>,
        data_dummy: &[u8],
        ext_dummy: &str,
    ) -> QuantumCapsule {
        let (kem_alpha, block_alpha) = self.build_kem_block(pub_real, data_real, ext_real);

        let ghost_identity = generate_identity(&mut self.chacha_rng);
        let target_dummy_pub = pub_dummy.unwrap_or(&ghost_identity.0);

        let (kem_beta, block_beta) = self.build_kem_block(target_dummy_pub, data_dummy, ext_dummy);

        QuantumCapsule {
            kem_alpha,
            block_alpha,
            kem_beta,
            block_beta,
        }
    }

    fn try_decrypt_block(
        &self,
        kem: &(Array1<i64>, Array1<i64>),
        block: &[u8],
        priv_key: &PrivateKey,
    ) -> Option<(Vec<u8>, String)> {
        let raw_key = priv_key.reveal();
        let (u, v) = kem;

        let s_mat = build_negacyclic_matrix(raw_key.as_slice().unwrap());
        let mut su = Array1::<i64>::zeros(DIMENSION);
        for i in 0..DIMENSION {
            let mut sum = 0i64;
            for j in 0..DIMENSION {
                sum += s_mat[(i, j)] * u[j];
            }
            su[i] = (sum % MODULUS + MODULUS) % MODULUS;
        }

        let mut bits = Vec::new();
        for i in 0..256 {
            let noisy = ((v[i] - su[i]) % MODULUS + MODULUS) % MODULUS;
            let shifted = (noisy - (DELTA / 2) + MODULUS) % MODULUS;
            bits.push(if shifted < DELTA { 1u8 } else { 0u8 });
        }

        let mut bytes = Vec::new();
        for chunk in bits.chunks(8) {
            let mut byte = 0u8;
            for (i, &b) in chunk.iter().enumerate() {
                byte |= b << i;
            }
            bytes.push(byte);
        }

        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let sym_key: [u8; 32] = hasher.finalize().into();

        let mut dec_block = block.to_vec();
        let mut stream = ChaCha20Rng::from_seed(sym_key);
        for byte in dec_block.iter_mut() {
            *byte ^= stream.gen::<u8>();
        }

        if dec_block.len() < 33 {
            return None;
        }
        let hash = dec_block[0..32].to_vec();

        let ext_len = dec_block[32] as usize;
        if dec_block.len() < 33 + ext_len {
            return None;
        }

        let ext = String::from_utf8(dec_block[33..33 + ext_len].to_vec())
            .unwrap_or_else(|_| "txt".to_string());
        let data = dec_block[33 + ext_len..].to_vec();

        let mut h = Sha256::new();
        h.update(&data);
        if h.finalize().as_slice() == hash {
            Some((data, ext))
        } else {
            None
        }
    }

    pub fn decrypt_blind(
        &self,
        file: &QuantumCapsule,
        phrase: &str,
    ) -> Result<(Vec<u8>, String), String> {
        let (_, priv_key) = crate::keys::derive_keypair_from_passphrase(phrase);

        if let Some(res) = self.try_decrypt_block(&file.kem_alpha, &file.block_alpha, &priv_key) {
            return Ok(res);
        }
        if let Some(res) = self.try_decrypt_block(&file.kem_beta, &file.block_beta, &priv_key) {
            return Ok(res);
        }
        Err("ACCESS DENIED: Collapsed Quantum Fluctuation or Invalid Passphrase.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::{derive_keypair_from_passphrase, generate_identity};

    #[test]
    fn test_public_key_size_and_text_roundtrip() {
        let mut rng = ChaCha20Rng::from_entropy();
        let (pub_k, _) = generate_identity(&mut rng);

        // Verify PublicKey structure contains public_seed and vector b of dimension 1024
        assert_eq!(pub_k.public_seed.len(), 32);
        assert_eq!(pub_k.b.len(), DIMENSION);

        let armored_text = pub_k.to_text();

        // Assert text armored size is far below 300 KB (actual is ~11 KB)
        assert!(
            armored_text.len() < 300 * 1024,
            "Public key text is {} bytes, exceeds 300KB",
            armored_text.len()
        );
        println!(
            "Public key armored text size: {} bytes (~{:.1} KB)",
            armored_text.len(),
            armored_text.len() as f64 / 1024.0
        );

        // Assert reconstruction from armored text
        let recovered_pub = PublicKey::from_text(&armored_text)
            .expect("Failed to deserialize public key from armored text");
        assert_eq!(pub_k.public_seed, recovered_pub.public_seed);
        assert_eq!(pub_k.b, recovered_pub.b);
    }

    #[test]
    fn test_encryption_and_decryption_roundtrip() {
        let passphrase_real = "super_secret_master_quantum_key_2026";
        let (pub_real, _) = derive_keypair_from_passphrase(passphrase_real);

        let message =
            b"Confidential quantum message to be encapsulated with post-quantum security.";
        let ext = "txt";

        let mut engine = QuantumEngine::new(ChaCha20Rng::from_entropy());
        let capsule = engine.encrypt_dual(&pub_real, message, ext, None, b"decoy", "txt");

        let (decrypted_bytes, decrypted_ext) = engine
            .decrypt_blind(&capsule, passphrase_real)
            .expect("Decryption failed!");

        assert_eq!(decrypted_bytes, message);
        assert_eq!(decrypted_ext, ext);
    }
}
