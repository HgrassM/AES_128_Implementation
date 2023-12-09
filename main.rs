mod key_expansion;

fn main() {
    //Primeiro, o código produz uma chave com bytes
    //expandidos a partir de uma chave primária de 16 bytes

    let primary_key: [u8;16] = [0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6d, 0x79, 0x20, 0x4b, 0x75,
                                0x6e, 0x67, 0x20, 0x46, 0x75];

    let exp_key: Vec<u8> = key_expansion::generate_exp_key(primary_key);
    
    let mut x = 0;

    //Printa os bytes em sistema decimal
    //Printa o tamanho da chave expandida
    while x < exp_key.len(){
        let b = exp_key[x];
        let a = format!("{b:02X}");
        println!("Byte em decimal: {a}");
        x+=1;
    }

    let a = exp_key.len();
    println!("Tamanho: {a}");

    
}
