use std::{fs, io, thread};
use rsa::RsaPrivateKey;
use rand;


fn main() {
    println!("File Encryptor Prototype v1.0");
    println!("Input file location: ");

    let mut content = Vec::new();
    let mut file_path = String::new();
    let _ = io::stdin().read_line(&mut file_path);

    let mut rng = rand::thread_rng();
    let privateKey = RsaPrivateKey::new(&mut rng, 2048);
    
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

    fs::write("../moded-file", content);
}
