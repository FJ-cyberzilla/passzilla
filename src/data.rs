use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::fs;
use std::path::Path;

use crate::security; // Import the security module

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    pub name: String,
    pub username: String,
    pub encrypted_password: Vec<u8>,
    pub iv: Vec<u8>, // Initialization Vector for AES-GCM
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordDatabase {
    pub entries: Vec<PasswordEntry>,
}

impl Default for PasswordDatabase {
    fn default() -> Self {
        PasswordDatabase {
            entries: Vec::new(),
        }
    }
}

impl PasswordDatabase {
    const DB_FILE_NAME: &str = "passwords.json.enc";

    /// Loads the password database from an encrypted file.
    pub fn load(master_password_hash: &str) -> io::Result<Self> {
        let path = Path::new(Self::DB_FILE_NAME);

        if !path.exists() {
            return Ok(PasswordDatabase::default());
        }

        let mut file = fs::File::open(path)?;
        let mut encrypted_data_with_iv = Vec::new();
        file.read_to_end(&mut encrypted_data_with_iv)?;

        // The stored format will be IV + Encrypted JSON.
        // Assuming IV is always 12 bytes for AES-256 GCM.
        let iv_len = 12;
        if encrypted_data_with_iv.len() < iv_len {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Corrupted database file: IV too short"));
        }

        let iv = encrypted_data_with_iv[..iv_len].to_vec();
        let ciphertext = encrypted_data_with_iv[iv_len..].to_vec();

        let key = security::derive_key_from_hash(master_password_hash);
        let decrypted_bytes = security::decrypt(&key, &ciphertext, &iv)?;

        serde_json::from_slice(&decrypted_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to deserialize database: {}", e)))
    }

    /// Saves the password database to an encrypted file.
    pub fn save(&self, master_password_hash: &str) -> io::Result<()> {
        let serialized_data = serde_json::to_vec(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to serialize database: {}", e)))?;

        let key = security::derive_key_from_hash(master_password_hash);
        let (ciphertext, iv) = security::encrypt(&key, &serialized_data);

        let mut encrypted_data_with_iv = iv;
        encrypted_data_with_iv.extend_from_slice(&ciphertext);

        let mut file = fs::File::create(Self::DB_FILE_NAME)?;
        file.write_all(&encrypted_data_with_iv)?;

        Ok(())
    }
}
