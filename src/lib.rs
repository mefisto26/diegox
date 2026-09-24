//! DIEGOX — Post-Quantum Encryption Suite (library root).
//!
//! Exposes the core cryptographic modules so that integration tests
//! (in `tests/`) and other consumers can link against `diegox::*`.

pub mod config;
pub mod engine;
pub mod gui;
pub mod keys;
