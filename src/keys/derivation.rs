use argon2::Argon2;
use ndarray::{Array1, Array2};
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::config::{DIMENSION, MODULUS};
use crate::keys::structures::{PrivateKey, PublicKey};

/// Constructs the negacyclic matrix of size (N, N) from a polynomial of N coefficients.
/// In the ring Z_q[X]/(X^N + 1), polynomial multiplication corresponds exactly
/// to matrix-vector multiplication with this negacyclic matrix.
pub fn build_negacyclic_matrix(poly: &[i64]) -> Array2<i64> {
    let n = poly.len();
    let mut mat = Array2::<i64>::zeros((n, n));
    for i in 0..n {
        for j in 0..n {
            if i >= j {
                mat[(i, j)] = poly[i - j];
            } else {
                mat[(i, j)] = (MODULUS - (poly[n + i - j] % MODULUS)) % MODULUS;
            }
        }
    }
    mat
}

/// Expands a 32-byte public seed into the full negacyclic matrix A deterministically using ChaCha20Rng.
/// This allows the matrix A to be reconstructed on-demand without storing it in the public key.
pub fn expand_matrix_a(public_seed: &[u8; 32]) -> Array2<i64> {
    let mut rng = ChaCha20Rng::from_seed(*public_seed);
    let mut a_poly = Vec::with_capacity(DIMENSION);

    // Fill a_poly with uniform random values mod MODULUS
    for _ in 0..DIMENSION {
        a_poly.push(rng.gen_range(0..MODULUS));
    }

    build_negacyclic_matrix(&a_poly)
}

/// Generation of the initial quantum identity using entropy provided by the TRNG.
pub fn generate_identity(rng: &mut ChaCha20Rng) -> (PublicKey, PrivateKey) {
    // Generate a random 32-byte public seed
    let mut public_seed = [0u8; 32];
    rng.fill(&mut public_seed);

    // Expand the matrix A deterministically from the public seed
    let pub_a = expand_matrix_a(&public_seed);

    let mut sec_s = Array1::<i64>::zeros(DIMENSION);
    let mut b = Vec::with_capacity(DIMENSION);

    // Fill sec_s with small binary secret keys (0 or 1)
    for item in sec_s.iter_mut() {
        *item = rng.gen_range(0..2);
    }

    // Calculate vector b = A * s + e in the polynomial ring with discrete noise
    for i in 0..DIMENSION {
        let mut sum = 0i64;
        for j in 0..DIMENSION {
            sum += pub_a[(i, j)] * sec_s[j];
        }
        let err = rng.gen_range(-1..=1);
        b.push(((sum + err) % MODULUS + MODULUS) % MODULUS);
    }

    (PublicKey { public_seed, b }, PrivateKey { sec_s })
}

// =========================================================================
// KDF ENGINE: Ephemeral key reconstruction from passphrases
// =========================================================================
pub fn derive_keypair_from_passphrase(passphrase: &str) -> (PublicKey, PrivateKey) {
    let mut seed = [0u8; 32];
    let salt = b"diegox_quantum_salt_v1__"; // Static salt for ecosystem cryptographic isolation

    // Argon2id processes the phrase consuming time and memory to neutralize GPU/ASIC brute force attacks
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt, &mut seed)
        .expect("Critical error in quantum KDF derivation");

    // We use the seed stretched by the KDF to raise the same molecular geometry in RAM
    let mut rng = ChaCha20Rng::from_seed(seed);
    generate_identity(&mut rng)
}
