mod key_expansion;
mod aes128;
mod funcoes;

use std::fs;
use std::env;


fn main() {
    
    let method = env::args().nth(1).expect("Missing arguments\nUse -h to get help").to_lowercase();
    if method == "-h" {
        
        println!(
"Usage: .\\aes128.exe ... [option] ... [arg]
Options:
-e    : Encrypts a file, a file path is required, if no key is specified a random one will be generated
-d    : Decode an encrypted file, a file path and a key is required
-h    : Show this page
Arguments:
file  : Path to a specific file for encrypting or decrypting
key   : Numeric key used for decrypting, optional for encrypting, must not be longer than 128 bits"
        );

    } else if method == "-e" {
        //encrypt
        let dir = env::args().nth(2).expect("Missing arguments\nUse -h to get help");
        let file = fs::read(dir).expect("No file found");
        
        let key = (0x5468617473206D79204B756E67204675 as u128).to_be_bytes();

        let codificado = aes128::encrypt(file, key);
        println!("{:0x?}", codificado);
        


    } else if method == "-d" {
        //decrypt
        let dir = env::args().nth(2).expect("Missing arguments\nUse -h to get help");
        let file = fs::read(dir).expect("No file found");
        
        let key = (0x5468617473206D79204B756E67204675 as u128).to_be_bytes();

        println!("{:0x?}", aes128::decrypt(file, key));

    } else {
        println!("Unkown argument: {}\nUse -h to get help", method);
    }
}
