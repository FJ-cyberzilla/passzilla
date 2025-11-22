# PassZillA - Secure Password Manager (CLI)

PassZillA is a command-line interface (CLI) application written in Rust for securely managing your passwords. It provides strong encryption and hashing mechanisms to protect your sensitive data.

**Note:** This is a basic implementation focused on core security features. The TUI (Terminal User Interface) is currently disabled due to environmental compatibility issues on certain platforms (e.g., Android/Termux). It operates as a non-interactive CLI for demonstration of the backend security.

## Features

- **Master Password Protection:** A single master password secures your entire password database.
- **Argon2 Hashing:** Master password is securely hashed using Argon2.
- **AES-256 GCM Encryption:** Password entries are encrypted using AES-256 GCM, with keys derived from your master password.
- **Local Storage:** Encrypted password database is stored locally in `passwords.json.enc`.
- **Basic CRUD Operations:** (Currently demonstrated in `main.rs` test suite) Add, view, update, and delete password entries.

## How to Build and Run (for development)

### Prerequisites

- Rust programming language and Cargo package manager (install via `rustup`).

### Steps

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/FJ-cyberzilla/passzilla
    cd passzilla
    ```

2.  **Build the project:**
    ```bash
    cargo build
    ```

3.  **Run the application (Development Mode):**
    For testing purposes, the `main.rs` file currently contains a hardcoded master password and a test suite for encryption, decryption, and database operations.

    ```bash
    cargo run
    ```
    This will output the results of the internal tests.

    **Important:** To run the application interactively with password prompts (which might not work on all environments due to `rpassword` limitations), you would uncomment the interactive password input in `src/main.rs`.

## Project Structure

- `src/main.rs`: Main application entry point, currently contains a test suite for core functionalities.
- `src/data.rs`: Defines data structures for `PasswordEntry` and `PasswordDatabase`, along with `load` and `save` operations.
- `src/security.rs`: Implements Argon2 password hashing, AES-256 GCM encryption/decryption, and key derivation.
- `Cargo.toml`: Project dependencies.
- `.gitignore`: Specifies files to be ignored by Git (including the encrypted database).
- `LICENSE`: Project license (MIT).

## Future Enhancements

- Full-fledged interactive CLI (once `rpassword` or an alternative is stable across environments).
- Command-line arguments for managing database entries (add, list, get, delete).
- Integration with OS keyring (for more secure master password handling).
- Export/Import functionality.

## Contributing

Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
