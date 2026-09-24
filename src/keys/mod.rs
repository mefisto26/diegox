pub mod derivation;
pub mod structures;

pub use derivation::{
    build_negacyclic_matrix, derive_keypair_from_passphrase, expand_matrix_a, generate_identity,
};
pub use structures::{PrivateKey, PublicKey};
