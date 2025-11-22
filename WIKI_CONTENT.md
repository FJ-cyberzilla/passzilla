# Home

Welcome to the PassZillA project wiki! This wiki provides comprehensive information about the PassZillA secure password manager.

---

## Table of Contents

1.  [Project Overview](#project-overview)
2.  [Getting Started](#getting-started)
    *   [Prerequisites](#prerequisites)
    *   [Building the Application](#building-the-application)
    *   [Running the Application](#running-the-application)
3.  [Core Features](#core-features)
4.  [Technical Design & Architecture](#technical-design--architecture)
    *   [Master Password Management](#master-password-management)
    *   [Encryption & Decryption](#encryption--decryption)
    *   [Data Persistence](#data-persistence)
5.  [Project Workflow: GitHub Flow](#project-workflow-github-flow)
6.  [Contributing](#contributing)
7.  [Future Enhancements / Roadmap](#future-enhancements--roadmap)
8.  [Troubleshooting](#troubleshooting)
9.  [License](#license)

---

## Project Overview

PassZillA is a command-line interface (CLI) application developed in Rust, designed for the secure management of personal passwords. It employs robust cryptographic primitives to ensure the confidentiality and integrity of your sensitive credential data stored locally.

**Current State:** The application provides a solid backend for secure password management, including hashing, key derivation, and encryption. The planned interactive Terminal User Interface (TUI) is currently disabled due to observed environmental compatibility issues on certain platforms (e.g., Android/Termux) that prevent `ratatui` from initializing correctly. The current version functions as a non-interactive CLI demonstrating the core security mechanisms and data persistence.

## Getting Started

Follow these steps to get PassZillA up and running on your system.

### Prerequisites

-   **Rust and Cargo:** Ensure you have the Rust programming language and its package manager, Cargo, installed. We recommend using `rustup` for installation:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    After installation, you might need to restart your terminal or run `source $HOME/.cargo/env`.

### Building the Application

1.  **Clone the repository:**
    Start by cloning the PassZillA GitHub repository to your local machine:
    ```bash
    git clone https://github.com/FJ-cyberzilla/passzilla
    cd passzilla
    ```

2.  **Compile the project:**
    Navigate into the cloned directory and build the project using Cargo:
    ```bash
    cargo build --release
    ```
    The `--release` flag compiles the project with optimizations, resulting in a faster executable.

### Running the Application

To run the compiled application, execute the following command:

```bash
cargo run --release
```

**Note on Interactive Input:**
In its current state, the `main.rs` contains a test suite that uses a hardcoded master password for demonstration purposes, primarily due to observed issues with interactive terminal input (`rpassword` crate) in certain environments like Android/Termux. When you run `cargo run --release`, it will execute this test suite, demonstrating password hashing, encryption/decryption, and database save/load operations.

To enable interactive master password input (which might fail in some environments):
1.  Open `src/main.rs`.
2.  Uncomment the lines related to `security::read_master_password()` and remove the hardcoded master password.
3.  Rebuild and run the application.

## Core Features

-   **Secure Master Password:** All stored data is protected by a single master password, which is never stored directly.
-   **Argon2 Hashing:** The master password undergoes robust hashing using the Argon2 algorithm, preventing brute-force attacks.
-   **AES-256 GCM Encryption:** Individual password entries are encrypted using the industry-standard AES-256 GCM symmetric encryption algorithm.
-   **Key Derivation:** Encryption keys are securely derived from the master password's hash using HKDF (HMAC-based Key Derivation Function) with SHA256.
-   **Local, Encrypted Storage:** The entire password database is stored locally in an encrypted file (`passwords.json.enc`), ensuring data at rest is protected.
-   **Demonstration of CRUD:** The current application structure includes logic for adding, viewing, and decrypting password entries, which are demonstrated in the `main.rs` test suite.

## Technical Design & Architecture

### Master Password Management

The master password is the cornerstone of PassZillA's security.
1.  **Input:** The master password is read securely from the terminal using `rpassword` (though currently bypassed in `main.rs` for compatibility reasons).
2.  **Hashing:** It is then passed to the `security::hash_password` function, which uses the `argon2` crate to generate a cryptographically strong hash. This hash includes a random salt to prevent rainbow table attacks.
3.  **Verification:** When the application needs to access the database, the entered master password is re-hashed and compared against the stored hash for authentication.

### Encryption & Decryption

PassZillA uses AES-256 GCM (Galois/Counter Mode), an authenticated encryption mode, for protecting password entries.
1.  **Key Derivation:** An encryption key is derived from the Argon2 hash of the master password using `hkdf` (HMAC-based Key Derivation Function) with `sha2` (SHA256). This ensures that even if the hash is compromised, the original master password isn't directly exposed.
2.  **Encryption (`security::encrypt`):**
    *   A unique 96-bit (12-byte) Nonce (Initialization Vector) is randomly generated using `OsRng` for each encryption operation.
    *   The plaintext password (or serialized database) is encrypted using `aes-gcm` with the derived key and the generated Nonce.
    *   The ciphertext and the Nonce are stored together.
3.  **Decryption (`security::decrypt`):**
    *   The stored Nonce and ciphertext are retrieved.
    *   The derived key is used along with the Nonce to decrypt the ciphertext.
    *   AES-GCM's authentication tag verifies the integrity and authenticity of the decrypted data, ensuring it hasn't been tampered with.

### Data Persistence

The password database is stored locally in a file named `passwords.json.enc`.
1.  **Serialization:** The `PasswordDatabase` struct (which contains `PasswordEntry` items) is serialized into JSON format using `serde` and `serde_json`.
2.  **Encryption:** The entire JSON byte stream is then encrypted using the AES-256 GCM scheme described above.
3.  **Storage Format:** The encrypted data is stored in the file prefixed by its unique Initialization Vector (IV), followed by the ciphertext.
4.  **Loading:** Upon loading, the IV is read first, then the ciphertext. The data is decrypted, and then deserialized back into `PasswordDatabase` objects.

## Project Workflow: GitHub Flow

PassZillA follows a simplified **GitHub Flow** for development, promoting a clean and collaborative process.

1.  **`master` is always deployable:** The `master` branch should always contain production-ready code.
2.  **Create feature branches:** For any new feature, bug fix, or experiment, create a new branch from `master` with a descriptive name (e.g., `feature/add-new-entry`, `fix/decryption-bug`).
3.  **Commit and push regularly:** As you work, commit your changes frequently and push them to your feature branch on GitHub.
4.  **Open a Pull Request (PR):** When your feature is complete (or you need feedback), open a Pull Request against the `master` branch.
5.  **Review and discuss:** Your code will be reviewed by collaborators. Discuss changes, address feedback, and make any necessary adjustments.
6.  **Merge when approved:** Once the PR is approved and all checks pass, merge your feature branch into `master`.
7.  **Clean up:** Delete your feature branch after merging to keep the repository tidy.

This workflow encourages small, frequent changes and ensures the `master` branch remains stable.

## Contributing

We welcome contributions! If you're interested in improving PassZillA, please:

1.  Fork the repository.
2.  Create a new feature branch (`git checkout -b feature/your-feature-name`).
3.  Implement your changes.
4.  Write tests for your changes.
5.  Ensure all existing tests pass (`cargo test`).
6.  Ensure the code compiles without warnings (`cargo check`).
7.  Commit your changes (`git commit -m "feat: Add new feature"`).
8.  Push to your fork (`git push origin feature/your-feature-name`).
9.  Open a Pull Request to the `master` branch of the main repository.

## Future Enhancements / Roadmap

-   **Interactive CLI:** Re-evaluate `rpassword` or explore alternative secure input methods to enable a fully interactive terminal experience across all environments.
-   **Command-Line Arguments:** Implement `clap` for a robust CLI interface to manage database operations (add, list, get, delete, update) via arguments, rather than requiring an interactive TUI.
-   **TUI (Terminal User Interface):** Implement the `ratatui` based TUI once interactive input issues are resolved or a compatible environment is identified.
-   **OS Keyring Integration:** Integrate with operating system keyrings for more secure storage and retrieval of the master encryption key.
-   **Export/Import Functionality:** Allow users to export their encrypted database to a portable format and import it.
-   **Search Functionality:** Implement search capabilities for password entries.
-   **Password Generation:** Add a secure password generator feature.

## Troubleshooting

-   **`Error: Os { code: 6, kind: Uncategorized, message: "No such device or address" }`:** This error often indicates a problem with interactive terminal input or raw mode capabilities in your environment.
    *   **Interactive Input (`rpassword`):** This might occur if `rpassword::read_password()` cannot interact with your terminal's standard input (e.g., in some Android/Termux setups or non-interactive shells).
    *   **TUI (`ratatui`):** If you attempt to enable the `ratatui`-based TUI, this error can also arise if `crossterm` cannot put the terminal into raw mode or use alternate screen buffers.
    *   **Workaround:** For interactive input, if `rpassword` fails, you may need to hardcode the master password temporarily for testing backend features (as done in `main.rs` currently), or use a simpler `std::io::stdin().read_line()` (though less secure as it echoes input). For the TUI, if `ratatui`/`crossterm` fails, a full interactive TUI might not be feasible in your specific environment.

-   **`Decryption failed` error when running after changes:** If you encounter a decryption failure, it's likely due to the encrypted `passwords.json.enc` file being incompatible with the current encryption key (derived from the master password). This can happen if the master password was changed or if the file was corrupted.
    *   **Solution:** Delete the `passwords.json.enc` file from the project directory and run the application again. A new, compatible encrypted database will be created.

## License

This project is licensed under the [MIT License](LICENSE).
