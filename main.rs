mod key_expansion;

fn main() {
    //Primeiro, o código produz uma chave com bytes
    //expandidos a partir de uma chave primária de 16 bytes

    let primary_key: [u8;16] = [21,176,34,78,132,92,12,0,6,8,43,98,103,13,65,87];

    let exp_key: Vec<u8> = key_expansion::generate_exp_key(primary_key);
    
    let mut x = 0;

    //Printa os bytes em sistema decimal
    //Printa o tamanho da chave expandida
    while x < exp_key.len(){
        let a = exp_key[x];
        println!("Byte em decimal: {a}");
        x+=1;
    }

    let a = exp_key.len();
    println!("Tamanho: {a}");

    
}
