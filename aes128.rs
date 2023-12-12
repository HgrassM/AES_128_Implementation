use super::funcoes;
use super::key_expansion;

pub fn encrypt_block(key: [u8; 16], block: Vec<u8>) -> Vec<u8> {
    let expanded_key = key_expansion::generate_exp_key(key);

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
pub fn decrypt_block(key: [u8; 16], block: Vec<u8>) -> Vec<u8> {
    let expanded_key = key_expansion::generate_exp_key(key);

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
