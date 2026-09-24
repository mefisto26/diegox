# DIEGOX - Post-Quantum Cryptographic Suite (CSRP)

![Language](https://img.shields.io/badge/Language-Rust--2021-orange.svg)
![Security](https://img.shields.io/badge/Security-Post--Quantum%20Lattice%20(LWE)-brightgreen.svg)
![Architecture](https://img.shields.io/badge/Architecture-Ultra--Modular%20%26%20Asynchronous-blue.svg)
![License](https://img.shields.io/badge/License-Apache--2.0-lightgrey.svg)

**DIEGOX** is an industrial-grade, post-quantum cryptographic suite engineered from the ground up in Rust. Designed for ultra-high-threat environments, hostile surveillance, and the post-quantum computational era, DIEGOX bridges advanced lattice-based mathematics, hardware-level side-channel mitigations, forensic plausible deniability, and an ultra-responsive native desktop interface.

---

## Table of Contents

- [DIEGOX - Post-Quantum Cryptographic Suite (CSRP)](#diegox---post-quantum-cryptographic-suite-csrp)
  - [Table of Contents](#table-of-contents)
  - [1. Project Overview \& Philosophy](#1-project-overview--philosophy)
  - [2. Post-Quantum Lattice Foundation (LWE)](#2-post-quantum-lattice-foundation-lwe)
    - [Mathematical Parameters](#mathematical-parameters)
    - [True Hardware Entropy \& Deterministic KDF](#true-hardware-entropy--deterministic-kdf)
  - [3. The Chameleon System (Side-Channel \& Hardware Security)](#3-the-chameleon-system-side-channel--hardware-security)
    - [Constant-Time Stream Execution](#constant-time-stream-execution)
    - [Volatile RAM Destruction (Chameleon Zeroization)](#volatile-ram-destruction-chameleon-zeroization)
  - [4. Dual-KEM Decoy System \& Plausible Deniability](#4-dual-kem-decoy-system--plausible-deniability)
    - [The Mechanism](#the-mechanism)
  - [5. Metadata \& Forensic Leak Mitigation](#5-metadata--forensic-leak-mitigation)
  - [6. Asynchronous Multi-threaded Architecture](#6-asynchronous-multi-threaded-architecture)
  - [7. Project Structure](#7-project-structure)
  - [8. Build \& Verification Guide](#8-build--verification-guide)
    - [Prerequisites](#prerequisites)
    - [Terminal Commands](#terminal-commands)

---

## 1. Project Overview & Philosophy

Modern classical public-key cryptosystems (such as RSA, Diffie-Hellman, and ECDSA/ECDH) rely fundamentally on the computational difficulty of integer factorization and discrete logarithms. Shor's Algorithm running on a sufficiently large fault-tolerant quantum computer will solve these problems in polynomial time, rendering existing asymmetric encryption completely obsolete.

DIEGOX addresses this imminent threat through two core doctrines:
* **Quantum-Resistant Foundations:** Employing lattice problems believed to be immune to both quantum and classical algorithmic acceleration.
* **Plausible Deniability & Forensic Hardening:** Assuming that cryptographic endpoints may be compromised, stolen, or subjected to coercive interrogation ("rubber-hose cryptanalysis"), the data format itself must conceal whether encrypted payloads even exist.

---

## 2. Post-Quantum Lattice Foundation (LWE)

DIEGOX is based on the **Learning With Errors (LWE)** paradigm over high-dimensional polynomial rings and matrices:

### Mathematical Parameters
* **Dimension ($n$):** `1024` (`DIMENSION = 1024`). The high dimensionality guarantees exponential search space complexity against lattice reduction algorithms such as BKZ 2.0.
* **Modulus ($q$):** `12289` (`MODULUS = 12289`), a prime modulus chosen to ensure efficient arithmetic reductions while preserving strict hardness bounds against dual-lattice attacks.
* **Signal Delta ($\Delta$):** `MODULUS / 2 = 6144`, separating the bit signaling thresholds to maximize error-correction tolerances during decapsulation.

### True Hardware Entropy & Deterministic KDF
* **Hardware-Seeded PRNG:** Generation of ad-hoc keys and blinding matrices utilizes `ChaCha20Rng` seeded directly from operating system entropy (`ChaCha20Rng::from_entropy()`), leveraging hardware random number generators (RDRAND/RDSEED on x86_64, or OS cryptographic CSPRNGs).
* **Memory-Hard Password Derivation (Argon2id):** When generating ephemeral or persistent quantum identities from user passphrases, DIEGOX applies the `Argon2id` key derivation function with an isolated domain salt (`diegox_quantum_salt_v1__`). Argon2id provides maximum resistance against GPU, ASIC, and FPGA brute-force dictionary attacks through strict memory-hardness and side-channel immunity. The stretched 256-bit seed is then deterministically expanded into high-dimensional matrix rings in volatile RAM.

---

## 3. The Chameleon System (Side-Channel & Hardware Security)

High-grade mathematical algorithms often fail in the physical world due to side-channel leakage—variations in power consumption, electromagnetic radiation, cache timing, or execution duration that correlate with secret key material. DIEGOX employs rigorous hardware-level defenses:

### Constant-Time Stream Execution
All symmetric stream encryption and decryption steps apply bitwise `XOR` transformations directly across byte slices. By eliminating conditional branching (`if`/`else`) and secret-dependent array lookups during the cryptographic cycle:
* Execution time is completely invariant with respect to secret data.
* Cache-timing attacks (such as Prime+Probe or Flush+Reload) are prevented from observing memory access patterns.

### Volatile RAM Destruction (Chameleon Zeroization)
To counter cold-boot attacks and post-execution RAM dumps, the `PrivateKey` structure explicitly implements Rust's `Drop` trait:

```rust
impl Drop for PrivateKey {
    fn drop(&mut self) {
        // As soon as the private key goes out of scope, flood
        // the underlying matrix buffer with pure zeros.
        self.sec_s.fill(0);
    }
}
```

The moment a private key goes out of scope—whether through normal completion or thread termination—the sensitive secret matrix (`sec_s`) is immediately overwritten with zeros, leaving zero retrievable artifacts in volatile memory.

---

## 4. Dual-KEM Decoy System & Plausible Deniability

Standard encrypted containers reveal their structure: an adversary who forces the user to yield a key can verify that the key decrypts the contents. If the user yields a dummy key that fails, the attacker knows they are withholding the true key.

DIEGOX eliminates this threat using a **Symmetric Dual-Slot Architecture (`CSRPFile`)**:

```
+-----------------------------------------------------------------------+
|                 .diegox Sealed Container                              |
+-----------------------------------+-----------------------------------+
|             Slot Alpha            |             Slot Beta             |
+-----------------------------------+-----------------------------------+
| KEM Ciphertext: (Array1, Array1)  | KEM Ciphertext: (Array1, Array1)  |
| Ciphertext Block: [Payload Alpha] | Ciphertext Block: [Payload Beta]  |
+-----------------------------------+-----------------------------------+
```

### The Mechanism
1. **Identical Mathematical Layout:** Both `block_alpha` and `block_beta` possess identical cryptographic envelopes. An outside analyst cannot determine which slot is real and which slot is a decoy.
2. **Ghost Identities:** If the user elects not to specify a decoy passphrase, the engine automatically derives a random "ghost" quantum identity and encapsulates true uniform random quantum noise. The container always retains two fully valid cryptographic slots.
3. **Blind Decapsulation (*Decrypt Blind*):** When unlocking a container:
   * The candidate passphrase derives an ephemeral private key.
   * The engine evaluates Slot Alpha. If the internal SHA-256 seal matches, it outputs Slot Alpha.
   * The engine evaluates Slot Beta. If the seal matches, it outputs Slot Beta.
   * If neither matches, it returns a generic access error.

**Operational Security:** Under duress, an operator can provide the decoy passphrase. The system decrypts harmless decoy material, leaving no computational evidence that Slot Alpha even contains meaningful data.

---

## 5. Metadata & Forensic Leak Mitigation

File systems leak extensive metadata: original filenames, creation timestamps, directory structures, and file attributes. DIEGOX enforces a strict metadata sanitization boundary:

1. **Filename Stripping:** The original filename is discarded before encapsulation.
2. **Sealed Payload Framing:**
   ```text
   [ 32 Bytes: SHA-256 Checksum ] + [ 1 Byte: Extension Length (N) ] + [ N Bytes: Extension ASCII ] + [ Data Bytes ]
   ```
3. **Isolated Extension Preservation:** Only the file extension (e.g., `pdf`, `kdbx`, `zip`, `txt`) is preserved in the sealed payload. When decapsulated in RAM, the GUI automatically configures the native OS save dialog to suggest the correct extension while leaving zero forensic traces of the original identity of the file on disk.

---

## 6. Asynchronous Multi-threaded Architecture

Cryptographic lattice mathematics (especially matrix multiplications in 1024 dimensions) and Argon2id memory-hard iterations are computationally intensive. Executing these operations on the main GUI thread would cause severe frame drops and window freezing ("Not Responding").

DIEGOX implements an asynchronous multi-threaded pipeline:

```
+--------------------------+                         +-----------------------------+
|    Main GUI Thread       |                         |    Background Worker Pool   |
|   (eframe / egui UI)     |                         |    (Dedicated std::thread)  |
+--------------------------+                         +-----------------------------+
             |                                                      |
             |  1. Dispatches task via std::thread::spawn           |
             |----------------------------------------------------->|
             |                                                      |
             |                                                      | 2. Computes Argon2id / LWE
             |                                                      |    or Blind Decapsulation
             |                                                      |
             |  3. Sends WorkerResult via mpsc::Sender              |
             |<-----------------------------------------------------|
             |                                                      |
             |  4. UI polls channel (try_recv)                      |
             |     and updates SYS_STATUS without frame drops       |
             v                                                      v
```

* **GUI Responsiveness:** The UI loop runs at full native framerate, providing real-time animated spinners and status telemetry.
* **Non-Blocking Messaging:** `std::sync::mpsc::channel` coordinates messages between worker threads and the GUI, guaranteeing thread safety without lock contention or deadlocks.

---

## 7. Project Structure

Adhering strictly to ultra-modular architectural guidelines:

```text
diegox/
├── Cargo.toml                  # Project manifest and dependencies
├── build.rs                    # Windows executable icon embedding via winres
├── app_icon.ico                # Native application icon
├── README.md                   # Repository landing page
├── DOCUMENTATION.md            # In-depth technical specification
├── tests/
│   └── crypto_tests.rs         # Integration and security validation suite
└── src/
    ├── lib.rs                  # Core library interface
    ├── main.rs                 # Native runtime entrypoint & window icon initialization
    ├── config/
    │   └── mod.rs              # Fixed quantum parameters (DIMENSION, MODULUS, DELTA)
    ├── keys/
    │   ├── mod.rs              # Keys module entry point
    │   ├── structures.rs       # PublicKey and PrivateKey (with Chameleon Drop)
    │   └── derivation.rs       # Argon2id KDF & LWE identity derivation
    ├── engine/
    │   ├── mod.rs              # Cryptographic engine module entry point
    │   ├── file_handler.rs     # CSRPFile (.diegox) disk I/O operations
    │   ├── payload_builder.rs  # Forensic payload packaging & extension sealing
    │   └── crypto_core.rs       # Dual-encryption and blind decapsulation logic
    └── gui/
        ├── mod.rs              # eframe::App implementation & event dispatcher
        ├── state.rs            # Centralized GUI state & worker channel types
        ├── shield_tab.rs       # Quantum Shielding Module UI
        ├── unlock_tab.rs       # Asynchronous Extraction Module UI
        └── identity_tab.rs     # Asynchronous Identity Manager UI
```

---

## 8. Build & Verification Guide

### Prerequisites
* Rust toolchain 1.70.0+ (2021 edition compatible)
* `cargo` package manager
* Windows target (MSVC or GNU) for executable icon embedding

### Terminal Commands

1. **Verify Syntax & Type Safety:**
   ```bash
   cargo check
   ```

2. **Execute Full Test Suite:**
   ```bash
   cargo test
   ```
   *Runs all 7 unit and integration tests covering Argon2id, LWE matrices, and Dual-KEM isolation.*

3. **Run in Development Mode:**
   ```bash
   cargo run
   ```
   *Launches the cyberpunk-themed GUI with titlebar and taskbar icons enabled.*

4. **Build Final Optimized Release Executable:**
   ```bash
   cargo build --release
   ```
   *Generates the standalone binary at `target/release/diegox.exe` with `app_icon.ico` compiled directly into the executable resources.*