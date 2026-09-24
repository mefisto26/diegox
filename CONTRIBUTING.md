# Contributing to DIEGOX

First off, thank you for considering contributing to DIEGOX! Projects in the post-quantum cryptography space rely heavily on community review, testing, and collaborative improvements.

---

## Code of Conduct

By participating in this project, you agree to abide by our standards of professional and respectful interaction. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

---

## Development Environment Setup

### Prerequisites
* **Rust Toolchain:** Version `1.75.0` or higher (install via [rustup.rs](https://rustup.rs/)).
* **Git:** Installed and configured.

### Local Setup Steps
```bash
# 1. Fork and clone the repository
git clone [https://github.com/YOUR_USERNAME/diegox.git](https://github.com/YOUR_USERNAME/diegox.git)
cd diegox

# 2. Create a feature branch
git checkout -b feature/your-feature-name

# 3. Verify the build
cargo build
```

---

## Quality Assurance & Testing Standards

Before submitting a Pull Request, you **must** ensure that all automated checks pass locally.

### 1. Formatting & Linting
We enforce standard Rust formatting and strict clippy lints.
```bash
# Check code formatting
cargo fmt --all -- --check

# Run Clippy lints
cargo clippy -- -D warnings
```

### 2. Unit & Integration Testing
All core cryptographic operations, KEM operations, and file handling must pass tests.
```bash
cargo test --all
```

---

## Commit & Pull Request Guidelines

1. **Keep Commits Atomic:** Make small, focused commits with clear imperative titles (e.g., `feat: implement Base64 armored public key export` or `fix: resolve memory leak in KEM buffer`).
2. **Never Commit Secrets:** Ensure no private test keys, binary payloads, or temporary files are committed.
3. **Link Issues:** Reference any relevant issue in your PR description (e.g., `Closes #12`).
4. **Pass CI:** All GitHub Actions workflows must be green before merge approval.