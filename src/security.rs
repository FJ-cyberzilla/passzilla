use argon2::{
    password_hash::{SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key,
};
use hkdf::Hkdf;
use sha2::Sha256;
use rpassword::read_password;
use std::io::{self, Write};
use generic_array::{GenericArray, typenum::U32}; // Added GenericArray and U32

/// Securely reads a password from the terminal.
pub fn read_master_password() -> io::Result<String> {
    print!("Enter master password: ");
    io::stdout().flush()?;
    let password = read_password()?;
    print!("Confirm master password: ");
    io::stdout().flush()?;
    let confirm_password = read_password()?;

    if password != confirm_password {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Master passwords do not match.",
        ));
    }
    Ok(password)
}

/// Hashes a password using Argon2.
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

/// Verifies a password against a hash.
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).expect("Invalid hash format");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

/// Derives an AES-256 key from the Argon2 hash of the master password.
pub fn derive_key_from_hash(password_hash: &str) -> Key<Aes256Gcm> {
    let hkdf = Hkdf::<Sha256>::new(None, password_hash.as_bytes());
    let mut okm_bytes = [0u8; 32]; // Use a fixed-size array
    hkdf.expand(b"aes256-gcm encryption key", &mut okm_bytes)
        .expect("Failed to derive key");
    // Explicitly create GenericArray and then convert to Key
    let mut key_array = GenericArray::<u8, U32>::default();
    key_array.clone_from_slice(&okm_bytes);
    Key::<Aes256Gcm>::from(key_array)
}

/// Encrypts data using AES-256 GCM.
pub fn encrypt(key: &Key<Aes256Gcm>, plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit nonce
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .expect("Encryption failed!");
    (ciphertext, nonce.to_vec())
}

/// Decrypts data using AES-256 GCM.
pub fn decrypt(key: &Key<Aes256Gcm>, ciphertext: &[u8], nonce: &[u8]) -> io::Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Decryption failed"))
}

