# Changelog

All notable changes to the DIEGOX project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0-alpha] - 2026-09-14

### Added
- **Post-Quantum LWE Engine:** Core mathematical backend using Learning With Errors in 1024 dimensions (`DIMENSION = 1024`).
- **The Chameleon System (Dual-KEM):** Plausible deniability architecture with `Alpha` (Real) and `Beta` (Decoy) encrypted blocks.
- **NIST-Style Public Seed Expansion:** Public keys reduced from ~21 MB to ~11 KB via 32-byte `public_seed` expansion in RAM.
- **Armored Base64 Key Encoding:** Standardized export/import format (`.diegoxkey`) and GUI copy-paste support.
- **Volatile RAM Protection:** Stateless private key derivation via **Argon2id** and explicit `Drop` trait implementation (`Zeroize` RAM cleanup).
- **Native GUI:** Multi-threaded `eframe`/`egui` desktop application with asynchronous channels (`mpsc::channel`).