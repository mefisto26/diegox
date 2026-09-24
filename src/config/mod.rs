/// Fixed dimension for the polynomial matrices in the post-quantum schema.
pub const DIMENSION: usize = 1024;

/// Modulus used for all polynomial arithmetic operations.
pub const MODULUS: i64 = 12289;

/// Delta value used for error correction and signal scaling.
pub const DELTA: i64 = MODULUS / 2;
