mod key_expansion;

fn main() {
    //Testando as funções
    
    let string: String = String::from("D5fg");
    let mut array:[u8;4] = [0;4];
    let mut x = 0;
    for b in string.bytes() {
        array[x] = b;
        x += 1;
    }

    let bytes: [u8;4] = key_expansion::rot_word(array);
    let mut vector: Vec<u8> = Vec::new();
    vector.push(bytes[0]);
    vector.push(bytes[1]);
    vector.push(bytes[2]);
    vector.push(bytes[3]);
    let s = String::from_utf8_lossy(&vector);
    println!("{s} rot word");

    let bytes2 = key_expansion::sub_word(array);
    let mut vector2: Vec<u8> = Vec::new();
    vector2.push(bytes2[0]);
    vector2.push(bytes2[1]);
    vector2.push(bytes2[2]);
    vector2.push(bytes2[3]);
    let s2 = String::from_utf8_lossy(&vector2);
    println!("{s2} sub word");

}
