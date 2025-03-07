use std::{fs, io::{self}};

use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
use argon2::Argon2;


fn main() {
    println!("File Encryptor Prototype v1.0");
    println!("Input file location: ");

    let mut file_path = String::new();
    let _ = io::stdin().read_line(&mut file_path);

    println!("Enter a strong passphrase you'll remember: ");

    let mut passphrase = String::new();

    let _ = io::stdin().read_line(&mut passphrase);

    println!("Encrypt File (y/n) [if n decrypt] :");
    let mut encrypt = String::new();
    
    io::stdin().read_line(&mut encrypt).expect("Didn't get that");

    if encrypt.to_lowercase().starts_with("y") {
        encrypt_file(&passphrase, &file_path);
    } else {
        decrypt_file(&passphrase, &file_path);
    }
    
}

fn encrypt_file(passphrase: &String, file_path: &String) {
    let mut content = Vec::new();
    let mut key_bytes = [0u8; 32];
    Argon2::default().hash_password_into(passphrase.trim().as_bytes(), b"file-cipher", &mut key_bytes).expect("Couldn't hash_password_into");
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&key_bytes[0..12]);
    match fs::read(file_path.trim()) {
        Ok(lines) => {
            for line in lines {
                content.push(line);
            }
        },
        Err(err) => println!("{:?}", err)
    }
    println!("nonce: {:?}", nonce);

    let encrypted = match cipher.encrypt(&nonce, content.as_ref()) {
        Ok(text) => text,
        Err(err) => panic!("{:?}", err)
    };
    fs::write(file_path.trim(), encrypted).expect("Couldn't write to file");
    println!("Nonce size: {}", nonce.len());
}

fn decrypt_file (passphrase: &String, file_path: &String) {
    let mut content = Vec::new();
    let mut key_bytes = [0u8; 32];
    Argon2::default().hash_password_into(passphrase.trim().as_bytes(), b"file-cipher", &mut key_bytes).expect("Couldn't hash_password_into");
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&key_bytes[0..12]);
    match fs::read(file_path.trim()) {
        Ok(bytes) => {
            for byte in bytes {
                content.push(byte);
            }
        },
        Err(err) => println!("{:?}", err)
    }

    let decrypted = cipher.decrypt(&nonce, content.as_ref());

    fs::write(file_path.trim(), match &decrypted {
        Ok(d) => d,
        Err(err) => panic!("{:?}", err)
    }).expect("couldn't write the file");
}
