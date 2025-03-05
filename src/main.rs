use std::{fs, io::{self, Read}};

use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, Key, KeyInit, Nonce};


fn main() {
    println!("File Encryptor Prototype v1.0");
    println!("Input file location: ");

    let mut file_path = String::new();
    let _ = io::stdin().read_line(&mut file_path);

    println!("Enter 32 character long passphrase: ");

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
    let key = Key::<Aes256Gcm>::from_slice(passphrase.trim().as_bytes());

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&passphrase[0..12].as_bytes());
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
    let key = Key::<Aes256Gcm>::from_slice(passphrase.trim().as_bytes());
    let cipher = Aes256Gcm::new(key);
    let mut content = Vec::new();
    let nonce = Nonce::from_slice(&passphrase[0..12].as_bytes());

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
