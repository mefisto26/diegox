//! DIEGOX — Cryptographic Integration Test Suite.
//!
//! Validates the post-quantum (LWE) core against the real public API
//! exposed by the `diegox` library root (`src/lib.rs`):
//!   * Argon2id passphrase derivation      -> `diegox::keys::derive_keypair_from_passphrase`
//!   * Public-seed matrix expansion        -> `diegox::keys::expand_matrix_a`
//!   * LWE Dual-KEM encrypt / blind decrypt -> `diegox::engine::QuantumEngine`
//!   * .diegox capsule container I/O        -> `diegox::engine::QuantumCapsule`

use std::fs;

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use diegox::config::{DIMENSION, MODULUS};
use diegox::engine::{QuantumCapsule, QuantumEngine};
use diegox::keys::{derive_keypair_from_passphrase, expand_matrix_a};

/// Deterministic engine factory (seeded, not entropy-based) so CI runs
/// are fully reproducible while still exercising real ChaCha20 randomness.
fn deterministic_engine(seed_byte: u8) -> QuantumEngine {
    let mut seed = [0u8; 32];
    seed[0] = seed_byte;
    QuantumEngine::new(ChaCha20Rng::from_seed(seed))
}

/// 1. Argon2id KDF determinism.
///
/// Identical passphrases MUST reconstruct bit-identical key material in RAM;
/// different passphrases MUST produce completely distinct material.
///
/// Note: `PrivateKey` implements `Drop` which zeroizes `sec_s` in place, so
/// every value we compare is snapshotted into owned vectors *before* the
/// key goes out of scope.
#[test]
fn test_argon2id_passphrase_determinism() {
    let passphrase = "CorrectHorseBatteryStaple2026!";
    let wrong_passphrase = "WrongHorseBatteryStaple2026!";

    // Derive the same passphrase twice: must be indistinguishable.
    let (pub_a, priv_a) = derive_keypair_from_passphrase(passphrase);
    let snapshot_a = (pub_a.public_seed, pub_a.b.clone(), priv_a.reveal().to_vec());

    let (pub_b, priv_b) = derive_keypair_from_passphrase(passphrase);
    let snapshot_b = (pub_b.public_seed, pub_b.b.clone(), priv_b.reveal().to_vec());

    // A different passphrase must yield different geometry.
    let (pub_wrong, priv_wrong) = derive_keypair_from_passphrase(wrong_passphrase);
    let snapshot_wrong = (
        pub_wrong.public_seed,
        pub_wrong.b,
        priv_wrong.reveal().to_vec(),
    );

    // Structural bounds: vectors match the schema dimension.
    assert_eq!(
        snapshot_a.1.len(),
        DIMENSION,
        "public vector b must have DIMENSION coefficients"
    );
    assert_eq!(
        snapshot_a.2.len(),
        DIMENSION,
        "secret vector s must have DIMENSION coefficients"
    );

    assert_eq!(
        snapshot_a, snapshot_b,
        "Identical passphrases MUST produce identical key matrices in RAM"
    );

    assert_ne!(
        snapshot_a.2, snapshot_wrong.2,
        "Different passphrases MUST produce different secret key vectors"
    );
    assert_ne!(
        snapshot_a.0, snapshot_wrong.0,
        "Different passphrases MUST produce different public seeds"
    );
}

/// 2. Public seed expansion integrity.
///
/// The 32-byte seed -> (DIMENSION x DIMENSION) negacyclic matrix expansion
/// must be 100% deterministic and every coefficient must stay within the
/// ring bounds [0, MODULUS).
#[test]
fn test_public_seed_expansion() {
    let public_seed = [0x42u8; 32];

    let matrix_1 = expand_matrix_a(&public_seed);
    let matrix_2 = expand_matrix_a(&public_seed);

    // Shape must be a square DIMENSION x DIMENSION matrix.
    assert_eq!(
        matrix_1.dim(),
        (DIMENSION, DIMENSION),
        "Expanded matrix must be DIMENSION x DIMENSION"
    );
    assert_eq!(
        matrix_1.nrows() * matrix_1.ncols(),
        matrix_2.nrows() * matrix_2.ncols(),
        "Expanded public matrix dimensions must be consistent"
    );

    // Determinism: same seed => bit-identical matrix.
    assert_eq!(
        matrix_1, matrix_2,
        "Public matrix expansion from 32-byte seed must be 100% deterministic"
    );

    // Bounds: every coefficient is reduced modulo MODULUS, i.e. in [0, MODULUS).
    for (idx, &coeff) in matrix_1.iter().enumerate() {
        assert!(
            (0..MODULUS).contains(&coeff),
            "Coefficient at flat index {} out of bounds [0, {}): {}",
            idx,
            MODULUS,
            coeff
        );
    }

    // A different seed must produce a different matrix.
    let matrix_other = expand_matrix_a(&[0x43u8; 32]);
    assert_ne!(
        matrix_1, matrix_other,
        "Distinct seeds MUST expand to distinct matrices"
    );
}

/// 3. Full LWE encryption/decryption roundtrip through a .diegox container.
///
/// Encrypts a payload with a passphrase-derived public key, serializes the
/// capsule to disk and reloads it (validating the container format), then
/// blind-decrypts with the same passphrase and verifies byte equality.
#[test]
fn test_lwe_encryption_decryption_roundtrip() {
    let secret_payload = b"Top Secret Post-Quantum File Content 2026";
    let passphrase = "MyUltraSecurePassphrase#2026";
    let ext = "txt";

    let (pub_real, _priv_real) = derive_keypair_from_passphrase(passphrase);

    let mut engine = deterministic_engine(0x01);
    let capsule = engine.encrypt_dual(&pub_real, secret_payload, ext, None, b"decoy", ext);

    // Container I/O: persist as a .diegox capsule and reload it from disk.
    let container_path = std::env::temp_dir().join("diegox_roundtrip_test.diegox");
    capsule
        .save_to_disk(&container_path)
        .expect("Failed to save .diegox capsule to disk");
    let reloaded = QuantumCapsule::load_from_disk(&container_path)
        .expect("Failed to reload .diegox capsule from disk");
    let _ = fs::remove_file(&container_path);

    // Blind decryption with the valid passphrase must restore the payload.
    let (decrypted, decrypted_ext) = engine
        .decrypt_blind(&reloaded, passphrase)
        .expect("Decryption with the valid passphrase failed");

    assert_eq!(
        decrypted.as_slice(),
        &secret_payload[..],
        "Decrypted payload must match original secret payload exactly"
    );
    assert_eq!(
        decrypted_ext, ext,
        "Restored extension must match the sealed one"
    );
}

/// 4. Chameleon System (Dual-KEM plausible deniability).
///
/// A capsule carrying a real block (Alpha) and a decoy block (Beta) bound to
/// two independent passphrases must unlock ONLY the matching block per key:
/// Passphrase Alpha -> real data, Passphrase Beta -> decoy data, and neither
/// phrase may leak the other's payload.
#[test]
fn test_chameleon_dual_kem_isolation() {
    let real_payload = b"REAL SECRET DATA";
    let decoy_payload = b"DECOY COAXIAL DATA";

    let passphrase_alpha = "RealPassphraseAlpha2026!";
    let passphrase_beta = "DecoyPassphraseBeta2026!";

    let (pub_alpha, _priv_alpha) = derive_keypair_from_passphrase(passphrase_alpha);
    let (pub_beta, _priv_beta) = derive_keypair_from_passphrase(passphrase_beta);

    let mut engine = deterministic_engine(0x02);
    // Block Alpha is sealed to pub_alpha, Block Beta to pub_beta.
    let capsule = engine.encrypt_dual(
        &pub_alpha,
        real_payload,
        "txt",
        Some(&pub_beta),
        decoy_payload,
        "txt",
    );

    // Passphrase Alpha decrypts Block Alpha.
    let (alpha_result, _) = engine
        .decrypt_blind(&capsule, passphrase_alpha)
        .expect("Alpha passphrase must decrypt the capsule");
    assert_eq!(
        alpha_result.as_slice(),
        &real_payload[..],
        "Passphrase Alpha must recover the real payload"
    );

    // Passphrase Beta decrypts Block Beta.
    let (beta_result, _) = engine
        .decrypt_blind(&capsule, passphrase_beta)
        .expect("Beta passphrase must decrypt the capsule");
    assert_eq!(
        beta_result.as_slice(),
        &decoy_payload[..],
        "Passphrase Beta must recover the decoy payload"
    );

    // Isolation: the two blocks are independent secrets.
    assert_ne!(
        alpha_result, beta_result,
        "Alpha and Beta blocks must be cryptographically isolated"
    );
}

/// 5. Invalid passphrase rejection.
///
/// Decryption with a wrong passphrase must fail gracefully with `Err`
/// (no panic, no silent corruption, no plaintext leak).
#[test]
fn test_invalid_passphrase_failure() {
    let secret_payload = b"Confidential Data";
    let correct_passphrase = "ValidPassphrase123!";
    let attacker_passphrase = "AttackerGuess456!";

    let (pub_real, _priv_real) = derive_keypair_from_passphrase(correct_passphrase);

    let mut engine = deterministic_engine(0x03);
    let capsule = engine.encrypt_dual(&pub_real, secret_payload, "txt", None, b"decoy", "txt");

    let result = engine.decrypt_blind(&capsule, attacker_passphrase);

    assert!(
        result.is_err(),
        "Decryption with an invalid passphrase MUST return an Error"
    );
    let err = result.unwrap_err();
    assert!(
        err.contains("ACCESS DENIED"),
        "Expected the Chameleon access-denied error, got: {}",
        err
    );
}
