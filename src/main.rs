mod aes128;
mod funcoes;
mod key_expansion;
use std::env;
use std::fs;


fn main() {
    let key = env::args().nth(3).unwrap();
    let mut key_array: [u8; 16] = [0; 16];
    hex::decode_to_slice(key, &mut key_array).expect("Decoding failed");
    let mut path = env::args().nth(2).unwrap();
    let data_vector = fs::read(path.clone()).unwrap(); 
    
    if env::args().nth(1).unwrap() == "e" {
        let encrypted_data = aes128::encrypt(data_vector, key_array);
        fs::write(path+".cypher", encrypted_data);
    } else if env::args().nth(1).unwrap() == "d" {
        let decrypted_data = aes128::decrypt(data_vector, key_array);
        for _i in 0..7 {
            path.pop();
        }
        fs::write(path, decrypted_data);
    }
}
