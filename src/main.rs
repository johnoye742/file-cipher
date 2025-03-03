use std::{fs, io, thread};

use aes_gcm::{aead::{Aead, OsRng}, aes::cipher, AeadCore, Aes256Gcm, Key, KeyInit};


fn main() {
    println!("File Encryptor Prototype v1.0");
    println!("Input file location: ");

    let mut content = Vec::new();
    let mut file_path = String::new();
    let _ = io::stdin().read_line(&mut file_path);

    
    let key = Key::<Aes256Gcm>::from_slice(b"password");

    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    match fs::read(file_path.trim()) {
        Ok(lines) => {
            println!("{:?}", lines);
            for line in lines {
                if line < 1 {
                    &content.push(line);
                } else {
                    &content.push(line - 1);
                }
            }
        },
        Err(err) => println!("{:?}", err)
    }

    let encrypted = match cipher.encrypt(&nonce, content.as_ref()) {
        Ok(text) => text,
        Err(err) => panic!("{:?}", err)
    };
    fs::write("../moded-file", encrypted);
}
