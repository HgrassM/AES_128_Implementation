use super::funcoes;
use super::key_expansion;

fn encrypt_block(expanded_key: Vec<u8>, block: Vec<u8>) -> Vec<u8> {
    let mut state = block;
    let mut key_slice = 0;

    //primeiramente se adiciona a chave[0] "round 0"
    state = crate::funcoes::xor_chave(state, expanded_key[key_slice..(key_slice+16)].to_vec()); 
    
    //rounds 1 a 9
    for round in 0..9 {
        key_slice = 16*(round + 1);
        state = funcoes::trocar_bytes(state);
        state = funcoes::desloca_linha(state);
        state = funcoes::embaralha_bloco(state);
        state = funcoes::xor_chave(state, expanded_key[key_slice..(key_slice+16)].to_vec());
    }
    
    //ultimo round não executa o passa de embaralhar
    key_slice += 16;
    state = funcoes::trocar_bytes(state);
    state = funcoes::desloca_linha(state);
    state = funcoes::xor_chave(state, expanded_key[key_slice..(key_slice+16)].to_vec());

    return state;
}



//funcao de decodificar segue o mesmo fluxo que a de codificar porem de forma inversa:
fn decrypt_block(expanded_key: Vec<u8>, block: Vec<u8>) -> Vec<u8> {
    let mut state = block;
    let mut key_slice = 160;

    state = funcoes::xor_chave(state, expanded_key[key_slice..(key_slice+16)].to_vec());
    state = funcoes::desloca_inverse_linha(state);
    state = funcoes::trocar_inverse_bytes(state);
    
    for round in 0..9 {
        key_slice = 16*(9 - round);
        state = funcoes::xor_chave(state, expanded_key[key_slice..(key_slice+16)].to_vec());
        state = funcoes::embaralha_bloco_inverso(state);
        state = funcoes::desloca_inverse_linha(state);
        state = funcoes::trocar_inverse_bytes(state);
    }
    
    key_slice -= 16;   
    state = crate::funcoes::xor_chave(state, expanded_key[key_slice..(key_slice+16)].to_vec());

    return state;
}

//adiciona um padding ao final
fn add_padding(mut byte_array: Vec<u8>) -> Vec<u8> {
    let mut padding_size = 16 - (byte_array.len() % 16);

    if padding_size == 0 {
        padding_size = 16;
    }

    byte_array.push(0x80);
    for _i in 1..padding_size{
        byte_array.push(0x00);
    }

    return byte_array;
}

//remove o padding
fn remove_padding(mut byte_array: Vec<u8>) -> Vec<u8> {
    while byte_array.pop().unwrap() == 0 {}
    return byte_array;
}

pub fn encrypt(mut byte_array: Vec<u8>, key: [u8; 16]) -> Vec<u8>{
    byte_array = add_padding(byte_array);
    let exp_key = key_expansion::generate_exp_key(key);

    let mut result: Vec<u8> = Vec::new();

    for i in 0..byte_array.len()/16 {
        result.append(&mut encrypt_block(exp_key.clone(), byte_array[(i*16)..(i*16 + 16)].to_vec()));
    }

    return result;
}


pub fn decrypt(byte_array: Vec<u8>, key: [u8; 16]) -> Vec<u8>{
    let exp_key = key_expansion::generate_exp_key(key);

    let mut result: Vec<u8> = Vec::new();

    for i in 0..byte_array.len()/16 {
        result.append(&mut decrypt_block(exp_key.clone(), byte_array[(i*16)..(i*16 + 16)].to_vec()));
    }

    result = remove_padding(result);
    return result;
}