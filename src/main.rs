use std::{fs, io};



fn main() {
    println!("File Encryptor Prototype v1.0");
    println!("Input file location: ");

    let mut file_path = String::new();
    let _ = io::stdin().read_line(&mut file_path);
    
    match fs::read(file_path.trim()) {
        Ok(lines) => {
            println!("{:?}", lines);
        },
        Err(err) => println!("{:?}", err)
    }
}
