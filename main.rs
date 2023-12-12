mod key_expansion;
mod aes128;
mod funcoes;

fn main() {
    let key = (0x5468617473206D79204B756E67204675 as u128).to_be_bytes();
    let message = (0x54776F204F6E65204E696E652054776F as u128).to_be_bytes().to_vec();
    println!("Original: {:x?}", message);

    let cypher = aes128::encrypt_block(key, message);
    
    println!("Codificada: {:0x?}", cypher);

    println!("Decodificada: {:0x?}", aes128::decrypt_block(key, cypher));
}
