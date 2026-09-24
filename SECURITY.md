# Security Policy

## Supported Versions

As DIEGOX is currently in active pre-release / experimental development, security updates and patches are applied exclusively to the latest release on the `main` branch.

| Version | Supported          |
| ------- | ------------------ |
| v0.1.x  | :white_check_mark: |
| < 0.1.0 | :x:                |

---

## Security Model & Known Scope

DIEGOX is designed around a post-quantum Lattice-based cryptography scheme (Learning With Errors / LWE) combined with a Dual-KEM "Chameleon System" for plausible deniability.

When auditing or reviewing the software, please take note of the following intentional design parameters:

1. **Passphrase-Derived Key Entropy:** Private keys are derived statelessly in RAM using **Argon2id**. The effective security margin of the underlying LWE lattice against brute-force dictionary attacks is bound by the entropy of the user-provided passphrase.
2. **Volatile Memory Handling:** Private key structures implement the explicit `Drop` trait to overwrite sensitive key material with zeroes (`zeroize`) when going out of scope.
3. **Public Key Expansion:** Public keys utilize a 32-byte `public_seed` to deterministically expand matrix $\mathbf{A}$ in RAM, adhering to NIST-style public key compression standards.

---

## Reporting a Vulnerability

**Please DO NOT report security vulnerabilities through public GitHub Issues or Pull Requests.**

If you discover a security flaw, mathematical weakness, side-channel leak, or implementation bug in DIEGOX, please follow responsible disclosure guidelines:

1. **Private Contact:** Send an email directly to **[your-email@example.com]** (or submit a private security advisory via GitHub's "Security" tab if enabled).
2. **Details to Include:**
   - A clear description of the vulnerability and its potential impact (e.g., side-channel leakage, improper memory zeroization, KEM state collision).
   - Step-by-step instructions or a Proof of Concept (PoC) demonstrating the issue.
   - Any proposed remediation or patch recommendations, if available.
3. **Response Timeline:** Acknowledgment of your report will be sent within **48 to 72 hours**, followed by status updates as the finding is evaluated.

---

## Disclosure Process

- **Assessment:** The report will be reviewed to determine validity and severity.
- **Remediation:** Fixes will be developed in a private staging branch.
- **Public Release:** Once a patch is released, a security advisory will be published. Security researchers who report valid findings will be publicly credited in the release notes (unless anonymity is requested).

---

## Security Disclaimer

> **Experimental Post-Quantum Cryptography Notice**
>
> DIEGOX is an independent, open-source project created for research, privacy advocacy, and post-quantum cryptographic experimentation.
>
> ⚠️ **This codebase HAS NOT been audited by an independent third-party security firm.** It is provided "as is" without warranty of any kind. It should not be deployed in high-risk environments, critical infrastructure, or scenarios where human life or safety depends on its cryptographic guarantees.