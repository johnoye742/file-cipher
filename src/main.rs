use std::{fs, io::{self, Read}};

use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, Key, KeyInit};


fn main() {
    println!("File Encryptor Prototype v1.0");
    println!("Input file location: ");

    let mut content = Vec::new();
    let mut file_path = String::new();
    let _ = io::stdin().read_line(&mut file_path);

    println!("Enter 32 character long passphrase: ");

    let mut passphrase = String::new();

    let _ = io::stdin().read_line(&mut passphrase);

    
    let key = Key::<Aes256Gcm>::from_slice(passphrase.trim().as_bytes());

    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
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
    fs::write("../moded-file", encrypted).expect("Cpuldn't write to file");
    println!("{:?}", key);
}


fn decrypt_file () {
}
