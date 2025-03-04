use std::{fs, io::{self, Read}};

use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, Key, KeyInit};


fn main() {
    println!("File Encryptor Prototype v1.0");
    println!("Input file location: ");

    let mut content = Vec::new();
    let mut file_path = String::new();
    let _ = io::stdin().read_line(&mut file_path);

    
    let key_bytes = [0u8; 32];
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    key.bytes().map(|byte| content.push(byte.expect("couldn't read byte")));
    nonce.bytes().map(|byte| content.push(byte.expect("couldn't read byte")));
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
