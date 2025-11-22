mod data;
mod security;

use std::io;

fn main() -> io::Result<()> {
    // TODO: Re-enable interactive master password input when environment allows.
    // Temporarily hardcode for development due to stdin issues on Android/Termux.
    let master_password = "my_secure_dev_password".to_string(); // Replace with a strong dev password
    println!("\n--- Master Password (Hardcoded for Dev) ---");
    println!("Using hardcoded master password for development.");

    let hashed_password = security::hash_password(&master_password);
    println!("Master password hashed.");

    // No interactive verification needed for hardcoded password, but keep the logic for future.
    println!("\n--- Master Password Verification (Simulated) ---");
    if security::verify_password(&master_password, &hashed_password) {
        println!("Master password verified successfully (simulated)!");
    } else {
        eprintln!("Error: Master password verification failed (simulated).");
        return Err(io::Error::new(io::ErrorKind::Other, "Hardcoded master password verification failed."));
    }

    // --- Test Encryption/Decryption ---
    println!("\n--- Testing Encryption/Decryption ---");
    let key = security::derive_key_from_hash(&hashed_password);
    let plaintext = "This is a secret message for testing encryption.".as_bytes();
    println!("Original plaintext: {:?}", String::from_utf8_lossy(plaintext));

    let (ciphertext, iv) = security::encrypt(&key, plaintext);
    println!("Encrypted ciphertext (hex): {}", hex::encode(&ciphertext));
    println!("IV (hex): {}", hex::encode(&iv));

    let decrypted_bytes = security::decrypt(&key, &ciphertext, &iv)?;
    let decrypted_text = String::from_utf8(decrypted_bytes)
        .expect("Failed to convert decrypted bytes to UTF-8");
    println!("Decrypted plaintext: {}", decrypted_text);

    if String::from_utf8_lossy(plaintext) == decrypted_text {
        println!("Encryption and Decryption successful!");
    } else {
        eprintln!("Error: Encryption/Decryption verification failed.");
    }

    // --- Test Database Save/Load ---
    println!("\n--- Testing Database Save/Load ---");

    let mut db = data::PasswordDatabase::load(&hashed_password)?;
    println!("Database loaded with {} entries.", db.entries.len());

    if db.entries.is_empty() {
        println!("Database is empty. Adding a sample entry.");
        let sample_password = "password123".as_bytes();
        let (enc_pass, iv_pass) = security::encrypt(&key, sample_password);

        db.entries.push(data::PasswordEntry {
            name: "sample_service".to_string(),
            username: "test_user".to_string(),
            encrypted_password: enc_pass,
            iv: iv_pass,
        });
        db.save(&hashed_password)?;
        println!("Sample entry added and database saved.");
    } else {
        println!("Database contains entries. Adding another sample entry.");
        let sample_password = "another_password".as_bytes();
        let (enc_pass, iv_pass) = security::encrypt(&key, sample_password);

        db.entries.push(data::PasswordEntry {
            name: "another_service".to_string(),
            username: "another_user".to_string(),
            encrypted_password: enc_pass,
            iv: iv_pass,
        });
        db.save(&hashed_password)?;
        println!("Another sample entry added and database saved.");
    }

    let loaded_db = data::PasswordDatabase::load(&hashed_password)?;
    println!("\n--- Loaded Database Contents ({} entries) ---", loaded_db.entries.len());
    for entry in loaded_db.entries {
        let decrypted_pass_bytes = security::decrypt(&key, &entry.encrypted_password, &entry.iv)?;
        let decrypted_pass = String::from_utf8(decrypted_pass_bytes)
            .expect("Failed to convert decrypted password to UTF-8");
        println!("  Name: {}", entry.name);
        println!("  Username: {}", entry.username);
        println!("  Password: {}", decrypted_pass);
        println!("---");
    }

    Ok(())
}