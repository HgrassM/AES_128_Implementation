//Função que faz um shift circular no bytes expandidos
pub fn rot_word(exp_bytes:[u8;4]) -> [u8;4] {

    let shifted_array: [u8;4] = [exp_bytes[1],exp_bytes[2],exp_bytes[3],exp_bytes[0]];

    return shifted_array;

}

//Função sub_word, que funciona da mesma forma que a função byte sub,
//ou seja, pega os bytes expandidos e troca os valores na s-box.
pub fn sub_word (exp_bytes:[u8;4]) -> [u8;4] {

    let mut sub_array: [u8;4] = [0,0,0,0];

    let s_box = [
        "63", "7C", "77", "7B", "F2", "6B", "6F", "C5", "30", "01", "67", "2B", "FE", "D7", "AB", "76",
        "CA", "82", "C9", "7D", "FA", "59", "47", "F0", "AD", "D4", "A2", "AF", "9C", "A4", "72", "C0",
        "B7", "FD", "93", "26", "36", "3F", "F7", "CC", "34", "A5", "E5", "F1", "71", "D8", "31", "15",
        "04", "C7", "23", "C3", "18", "96", "05", "9A", "07", "12", "80", "E2", "EB", "27", "B2", "75",
        "09", "83", "2C", "1A", "1B", "6E", "5A", "A0", "52", "3B", "D6", "B3", "29", "E3", "2F", "84",
        "53", "D1", "00", "ED", "20", "FC", "B1", "5B", "6A", "CB", "BE", "39", "4A", "4C", "58", "CF",
        "D0", "EF", "AA", "FB", "43", "4D", "33", "85", "45", "F9", "02", "7F", "50", "3C", "9F", "A8",
        "51", "A3", "40", "8F", "92", "9D", "38", "F5", "BC", "B6", "DA", "21", "10", "FF", "F3", "D2",
        "CD", "0C", "13", "EC", "5F", "97", "44", "17", "C4", "A7", "7E", "3D", "64", "5D", "19", "73",
        "60", "81", "4F", "DC", "22", "2A", "90", "88", "46", "EE", "B8", "14", "DE", "5E", "0B", "DB",
        "E0", "32", "3A", "0A", "49", "06", "24", "5C", "C2", "D3", "AC", "62", "91", "95", "E4", "79",
        "E7", "C8", "37", "6D", "8D", "D5", "4E", "A9", "6C", "56", "F4", "EA", "65", "7A", "AE", "08",
        "BA", "78", "25", "2E", "1C", "A6", "B4", "C6", "E8", "DD", "74", "1F", "4B", "BD", "8B", "8A",
        "70", "3E", "B5", "66", "48", "03", "F6", "0E", "61", "35", "57", "B9", "86", "C1", "1D", "9E",
        "E1", "F8", "98", "11", "69", "D9", "8E", "94", "9B", "1E", "87", "E9", "CE", "55", "28", "DF",
        "8C", "A1", "89", "0D", "BF", "E6", "42", "68", "41", "99", "2D", "0F", "B0", "54", "BB", "16"
    ];

    let lc = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    let mut hex_units: [char; 2] = ['0', '0'];

    let mut y = 0;

    let mut column = 0;
    let mut line = 0;


    for b in exp_bytes {
        let hex_value = format!("{b:02X}");
        let mut x = 0;
        
        for c in hex_value.chars() {
            hex_units[x] = c;
            x+=1
        }
     
        x = 0;

        for c in lc {
            if c == hex_units[0] {
                line = x+1;
                break;
            }
            x+=1;
        }

        x = 0;

        for c in lc {
            if c == hex_units[1] {
                column = x+1;
                break;
            }
            x+=1;
        }

        let index = (((line*16)-16) + (column))-1;
        sub_array[y] = u8::from_str_radix(s_box[index], 16).unwrap();

        y+=1;
    }

    return sub_array;
}

//Função que retorna bytes de acordo com o cálcuo
//realizado na função
pub fn rcon(round:usize) -> [u8;4] {
    let rcon_calc = (round/(16/4))-1;
    let hex_value: &str = match rcon_calc {
        0=> "01",
        1=> "02",
        2=> "04",
        3=> "08",
        4=> "10",
        5=> "20",
        6=> "40",
        7=> "80",
        8=> "1B",
        9=> "36",
        10=> "6C",
        11=> "D8",
        12=> "AB",
        13=> "4D",
        14=> "9A",
        _=> "00"
    };

    let mut x = 0;
    let mut result: [u8;4] = [0;4];
    while x < 4 {
        if x == 0{
            result[x] = u8::from_str_radix(hex_value, 16).unwrap();
        }else{
            result[x] = u8::from_str_radix("00", 16).unwrap();
        }
        x+=1;
    }
    return result;
}


//Função que retorna bytes de acordo com o offset
//da chave primária
pub fn k_off(offset: usize, primary_key: [u8;16]) -> [u8;4] {
    let mut result: [u8;4] = [0;4];
    let mut x = 0; 
    while x < 4 {
        result[x] = primary_key[offset + x];
        x+=1;
    }

    return result;
}

//Função que retorna bytes de acordo com o offset
//da chave expandida
pub fn ek_off(offset: usize, exp_key: Vec<u8>) -> [u8;4] {
    let mut result: [u8;4] = [0;4];
    let mut x = 0;
    while x < 4 {
        result[x] = exp_key[offset + x];
        x+=1;
    }

    return result;
}

//Função que gera a chave expandida
pub fn generate_exp_key(primary_key: [u8;16]) -> Vec<u8> {
    let mut round = 0;
    let mut exp_key: Vec<u8> = Vec::new();

    //Rounds 0-3
    {   
        let mut offset = 0;
        while round < 4 {
            let added_bytes: [u8;4] = k_off(offset, primary_key);

            for b in added_bytes {
                exp_key.push(b);
            }

            offset += 4;
            round += 1;
        }
    }

    //Rounds 4-43
    {   
        let mut added_bytes: [u8;4] = [0;4];
        let mut sub_round = 4;

        while round < 44{
            
            if round == sub_round {
                //Realiza as operações do round

                let mut x = 0;
                let sub_bytes: [u8;4] = sub_word(rot_word(ek_off((round-1)*4, exp_key.clone())));
                let rcon_bytes: [u8;4] = rcon(round);
                let ek_bytes: [u8;4] = ek_off((round-4)*4, exp_key.clone());
                
                while x < 4 {
                    added_bytes[x] = sub_bytes[x] ^ rcon_bytes[x] ^ ek_bytes[x];
                    x+=1;
                }

                x = 0;
                //Adiciona os bytes resultantes da operação
                //na chave expandida
                while x < 4{
                    exp_key.push(added_bytes[x]);
                    x+=1;
                }

                //Atualiza o sub round
                sub_round += 4;

            } else {
                let ek_bytes1 = ek_off((round-1)*4, exp_key.clone());
                let ek_bytes2 = ek_off((round-4)*4, exp_key.clone());
                let mut x = 0;

                while x < 4 {
                    added_bytes[x] = ek_bytes1[x] ^ ek_bytes2[x];
                    x+=1;
                }

                x = 0;
                //Adiciona os bytes resultantes da operação
                //na chave expandida
                while x < 4{
                    exp_key.push(added_bytes[x]);
                    x+=1;
                }

            }

            round += 1;           
        }

    }

    return exp_key;
}
