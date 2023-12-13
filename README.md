-  ## **Este projeto tem como objetivo implementar o algoritmo de criptografia avançado (AES) com uma chave de tamanho 128 bits. Esta implementação visa garantir a segurança de dados sensíveis dos usuários.**


- Criptografia é uma prática que visa proteger dados sensíveis por meio da aplicação de algoritmos matemáticos. O método AES é um desses algoritmos, e é muito conhecido por sua eficiência e rebustez na proteção de informações, sendo utilizada em grandes empresas de tecnologia como a Google, Microsoft, Amazon e etc. Provando assim a sua eficiência no cenário mundial de segurança.

  O AES-128 é um algoritmo simétrico de criptografia de bloco que opera em blocos de 128 bits, e consiste em quatro operações principais que são aplicadas iterativamente em 10 rodadas. As operações são:
  
- SubBytes:
	Substituição não linear dos bytes do estado (um bloco de 4x4 bytes) usando uma tabela de substituição conhecida como S-Box.

- ShiftRows:
	Desloca as linhas do estado, garantindo a difusão na vertical.

- MixColumns:
	Operação de mistura nas colunas do estado, introduzindo difusão na horizontal.

- AddRoundKey:
	Cada byte do estado é combinado com um byte correspondente da chave de round usando a operação XOR.

![all 4 steps](https://github.com/HgrassM/AES_128_Implementation/assets/102628611/31534957-578a-4606-b106-2213e5ef340d)

___

- A operação SubBytes é uma substituição não linear que ocorre em cada byte do estado usando uma tabela de substituição conhecida como S-Box. A S-Box é uma matriz 16x16 que mapeia cada byte de entrada para um byte de saída, proporcionando confusão não linear nos dados. A operação SubBytes é aplicada da seguinte maneira:

	- Cada byte no estado é substituído pelo valor correspondente na S-Box.
	- A S-Box é projetada para espalhar os bits de forma não linear, aumentando a resistência contra análises criptográficas.

- Essa etapa ajuda a garantir que pequenas mudanças nos dados de entrada causem grandes mudanças na saída, aumentando a segurança do algoritmo.

![sub bytes](https://github.com/HgrassM/AES_128_Implementation/assets/102628611/0fc52d60-d729-4984-b759-af5198dd3f41)


___
- A operação ShiftRows é uma operação de permutação que desloca as linhas do estado. A ideia é garantir que cada byte do estado seja influenciado por bytes diferentes nas operações subsequentes. O deslocamento é realizado da seguinte maneira:

	- A primeira linha não é alterada.
	- A segunda linha é deslocada circularmente para a esquerda em uma posição.
	- A terceira linha é deslocada circularmente para a esquerda em duas posições.
	- A quarta linha é deslocada circularmente para a esquerda em três posições.

- Essa operação proporciona difusão vertical nos dados, garantindo que as mudanças em um byte afetem outros bytes em operações futuras.



![shift rows](https://github.com/HgrassM/AES_128_Implementation/assets/102628611/19f241c6-5eea-4fff-b25f-9d74ef1ce2bc)



___

- A operação MixColumns é uma transformação linear que ocorre nas colunas do estado. Ela é projetada para proporcionar difusão horizontal nos dados. Cada coluna é tratada como um polinômio de grau três sobre o corpo finito de dois elementos (GF(2^8)), e é multiplicada por uma matriz fixa.

	- Cada byte em uma coluna é multiplicado por um número fixo e, em seguida, somado aos resultados dos outros bytes da mesma coluna.
	- A multiplicação é realizada no corpo finito GF(2^8) usando uma técnica chamada "multiplicação na Galois Field".
	- A operação MixColumns fornece uma propriedade importante de difusão, garantindo que mudanças em um byte afetem vários bytes nas colunas subsequentes.



![mix columns](https://github.com/HgrassM/AES_128_Implementation/assets/102628611/75e82f48-d98f-4699-830c-5fe86ae37f61)


___

- A operação AddRoundKey é uma operação de chave que ocorre em cada rodada do AES. Ela consiste na operação de ou exclusivo (XOR) entre cada byte do estado e o byte correspondente da chave de round.

	- Cada byte do estado é combinado com o byte correspondente da subchave gerada a partir da chave principal.
	- Essa operação introduz a influência da chave na transformação, tornando a operação dependente da chave utilizada.

- O processo de adição da chave é bidirecional, ou seja, para descriptografar, você simplesmente executa novamente a operação AddRoundKey usando a chave de round apropriada.


![addroundkey](https://github.com/HgrassM/AES_128_Implementation/assets/102628611/8eac0ddf-74e1-4612-ab6d-1aa1e6e36c2c)



___


- Essas quatro operações (SubBytes, ShiftRows, MixColumns e AddRoundKey) são aplicadas de forma iterativa em cada rodada, exceto na última rodada. O último round do AES é projetado de maneira diferente para otimizar a eficiência da descriptografia, simplificando o processo e economizando recursos computacionais. A operação MixColumns não é necessária no último round porque a difusão horizontal já foi alcançada nas rodadas anteriores, e sua omissão simplifica a implementação e acelera a descriptografia. Essas operações proporcionam confusão e difusão necessárias para garantir a segurança e a resistência do AES a ataques criptoanalíticos.


___

- A descriptografia no AES (Advanced Encryption Standard) segue basicamente o mesmo processo da criptografia, porém os passos são invertidos.

## Operações Inversas do AES:

 - **AddRoundKey**
 - **Inverse MixColumns**
 - **ShiftRows**
 - **Inverse SubByte**


- O processo é semelhante em termos de estrutura, mas as operações são aplicadas de maneira inversa para desfazer as transformações realizadas durante a criptografia. A descriptografia é projetada para recuperar os dados originais a partir dos dados criptografados, garantindo que o processo seja reversível quando a chave correta é usada.


---
## Implementação
- Fizemos o projeto através de funções, ou seja, cada etapa de criptografia foi criada dentro de uma função, depois chamamos elas atráves de um arquivo 'aes128.rs'. A linguagem escolhida foi Rust, que desde o inicio da sua criação tem seu foco em segurança, a tornando ideal para implementações de criptografia.


## Função trocar_bytes
	
	pub fn trocar_bytes(mut registro: Vec<u8>)-> Vec<u8>{
    // tabela sbox, byte > byte_trocado, usado na codificação
    let sbox = vec![
      0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01,   0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
      0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
      0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
      0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
      0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
      0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
      0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
      0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
      0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
      0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
      0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
      0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
      0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
      0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
      0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
      0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
    ];

    //substitui cada byte pelo correspondente na sbox usado na decodificação
    for n in 0..=15{
        registro[n] = sbox[registro[n] as usize]
    }
    
    return registro

    }


- A função trocar_bytes implementa a etapa de substituição de bytes usando uma tabela S-Box. A função aceita um vetor de bytes chamado registro como entrada e realiza a substituição de cada byte no vetor de acordo com os valores predefinidos na tabela S-Box. A tabela S-Box é uma matriz de valores que são usados para substituir os bytes originais durante a operação de substituição.

- O loop for itera sobre os primeiros 16 bytes do vetor registro, substituindo cada byte pelo valor correspondente na tabela S-Box. Após o loop, o vetor modificado é retornado.

___

## Função desloca_linha

    pub fn desloca_linha(registro: Vec<u8>)-> Vec<u8>{
      let temp: Vec<u8> = vec![
        registro[0], registro[5], registro[10], registro[15],
        registro[4], registro[9], registro[14], registro[3],
        registro[8], registro[13], registro[2], registro[7],
        registro[12], registro[1], registro[6], registro[11]
    ];


    return temp
    }

- A função desloca_linha implementa a ShiftRows, ela aceita um vetor de 16 bytes (registro) representando a matriz de estado antes da operação ShiftRows. O vetor temp é usado para armazenar a matriz de estado após a operação ShiftRows.

___

## Função embaralha_colunas
	
	 // tabela campo de Galois multiplica 2,
    let mult2 = vec![0x00,0x02,0x04,0x06,0x08,0x0a,0x0c,0x0e,0x10,0x12,0x14,0x16,0x18,0x1a,0x1c,0x1e,
    0x20,0x22,0x24,0x26,0x28,0x2a,0x2c,0x2e,0x30,0x32,0x34,0x36,0x38,0x3a,0x3c,0x3e,
    0x40,0x42,0x44,0x46,0x48,0x4a,0x4c,0x4e,0x50,0x52,0x54,0x56,0x58,0x5a,0x5c,0x5e,
    0x60,0x62,0x64,0x66,0x68,0x6a,0x6c,0x6e,0x70,0x72,0x74,0x76,0x78,0x7a,0x7c,0x7e,	
    0x80,0x82,0x84,0x86,0x88,0x8a,0x8c,0x8e,0x90,0x92,0x94,0x96,0x98,0x9a,0x9c,0x9e,
    0xa0,0xa2,0xa4,0xa6,0xa8,0xaa,0xac,0xae,0xb0,0xb2,0xb4,0xb6,0xb8,0xba,0xbc,0xbe,
    0xc0,0xc2,0xc4,0xc6,0xc8,0xca,0xcc,0xce,0xd0,0xd2,0xd4,0xd6,0xd8,0xda,0xdc,0xde,
    0xe0,0xe2,0xe4,0xe6,0xe8,0xea,0xec,0xee,0xf0,0xf2,0xf4,0xf6,0xf8,0xfa,0xfc,0xfe,
    0x1b,0x19,0x1f,0x1d,0x13,0x11,0x17,0x15,0x0b,0x09,0x0f,0x0d,0x03,0x01,0x07,0x05,
    0x3b,0x39,0x3f,0x3d,0x33,0x31,0x37,0x35,0x2b,0x29,0x2f,0x2d,0x23,0x21,0x27,0x25,
    0x5b,0x59,0x5f,0x5d,0x53,0x51,0x57,0x55,0x4b,0x49,0x4f,0x4d,0x43,0x41,0x47,0x45,
    0x7b,0x79,0x7f,0x7d,0x73,0x71,0x77,0x75,0x6b,0x69,0x6f,0x6d,0x63,0x61,0x67,0x65,
    0x9b,0x99,0x9f,0x9d,0x93,0x91,0x97,0x95,0x8b,0x89,0x8f,0x8d,0x83,0x81,0x87,0x85,
    0xbb,0xb9,0xbf,0xbd,0xb3,0xb1,0xb7,0xb5,0xab,0xa9,0xaf,0xad,0xa3,0xa1,0xa7,0xa5,
    0xdb,0xd9,0xdf,0xdd,0xd3,0xd1,0xd7,0xd5,0xcb,0xc9,0xcf,0xcd,0xc3,0xc1,0xc7,0xc5,
    0xfb,0xf9,0xff,0xfd,0xf3,0xf1,0xf7,0xf5,0xeb,0xe9,0xef,0xed,0xe3,0xe1,0xe7,0xe5
	];

    // tabela campo de Galois multiplica 3,
    let mult3 = vec![0x00,0x03,0x06,0x05,0x0c,0x0f,0x0a,0x09,0x18,0x1b,0x1e,0x1d,0x14,0x17,0x12,0x11,
    0x30,0x33,0x36,0x35,0x3c,0x3f,0x3a,0x39,0x28,0x2b,0x2e,0x2d,0x24,0x27,0x22,0x21,
    0x60,0x63,0x66,0x65,0x6c,0x6f,0x6a,0x69,0x78,0x7b,0x7e,0x7d,0x74,0x77,0x72,0x71,
    0x50,0x53,0x56,0x55,0x5c,0x5f,0x5a,0x59,0x48,0x4b,0x4e,0x4d,0x44,0x47,0x42,0x41,
    0xc0,0xc3,0xc6,0xc5,0xcc,0xcf,0xca,0xc9,0xd8,0xdb,0xde,0xdd,0xd4,0xd7,0xd2,0xd1,
    0xf0,0xf3,0xf6,0xf5,0xfc,0xff,0xfa,0xf9,0xe8,0xeb,0xee,0xed,0xe4,0xe7,0xe2,0xe1,
    0xa0,0xa3,0xa6,0xa5,0xac,0xaf,0xaa,0xa9,0xb8,0xbb,0xbe,0xbd,0xb4,0xb7,0xb2,0xb1,
    0x90,0x93,0x96,0x95,0x9c,0x9f,0x9a,0x99,0x88,0x8b,0x8e,0x8d,0x84,0x87,0x82,0x81,	
    0x9b,0x98,0x9d,0x9e,0x97,0x94,0x91,0x92,0x83,0x80,0x85,0x86,0x8f,0x8c,0x89,0x8a,
    0xab,0xa8,0xad,0xae,0xa7,0xa4,0xa1,0xa2,0xb3,0xb0,0xb5,0xb6,0xbf,0xbc,0xb9,0xba,
    0xfb,0xf8,0xfd,0xfe,0xf7,0xf4,0xf1,0xf2,0xe3,0xe0,0xe5,0xe6,0xef,0xec,0xe9,0xea,	
    0xcb,0xc8,0xcd,0xce,0xc7,0xc4,0xc1,0xc2,0xd3,0xd0,0xd5,0xd6,0xdf,0xdc,0xd9,0xda,	
    0x5b,0x58,0x5d,0x5e,0x57,0x54,0x51,0x52,0x43,0x40,0x45,0x46,0x4f,0x4c,0x49,0x4a,
    0x6b,0x68,0x6d,0x6e,0x67,0x64,0x61,0x62,0x73,0x70,0x75,0x76,0x7f,0x7c,0x79,0x7a,	
    0x3b,0x38,0x3d,0x3e,0x37,0x34,0x31,0x32,0x23,0x20,0x25,0x26,0x2f,0x2c,0x29,0x2a,
    0x0b,0x08,0x0d,0x0e,0x07,0x04,0x01,0x02,0x13,0x10,0x15,0x16,0x1f,0x1c,0x19,0x1a
	];

    // embaralha os elementos
    let elementos_embaralhado0 = mult2[coluna[0] as usize] ^  mult3[coluna[1] as usize] ^ coluna[2] ^ coluna[3];
    let elementos_embaralhado1 = coluna[0] ^  mult2[coluna[1] as usize] ^ mult3[coluna[2] as usize] ^ coluna[3];
    let elementos_embaralhado2 = coluna[0] ^  coluna[1] ^ mult2[coluna[2] as usize] ^ mult3[coluna[3] as usize];
    let elementos_embaralhado3 = mult3[coluna[0] as usize] ^  coluna[1] ^ coluna[2] ^ mult2[coluna[3] as usize];

    // retorna a coluna embaralhada
    let col_embaralhada = vec![elementos_embaralhado0, elementos_embaralhado1, elementos_embaralhado2, elementos_embaralhado3];

    //println!("Meu vetor apos o embaralhamento: {:x?}",col_embaralhada);

    return col_embaralhada     
	}

 ___
## Função embaralha_bloco
    pub fn embaralha_bloco(registro: Vec<u8>) -> Vec<u8> {
	    let mut temp: Vec<u8> = Vec::new();
	    temp.extend(embaralha_colunas(registro[0..4].to_vec()));
	    temp.extend(embaralha_colunas(registro[4..8].to_vec()));
	    temp.extend(embaralha_colunas(registro[8..12].to_vec()));
	    temp.extend(embaralha_colunas(registro[12..16].to_vec()));
	    return temp;
	}

___

## Função xor_chave
	pub fn xor_chave(mut registro: Vec<u8>, chave: Vec<u8>)-> Vec<u8>{
    //operação xor bitwise
    for n in 0..=15{
        registro[n] = registro[n] ^ chave[n]
    }
    

    return registro
	}


 ___

 ## Para a descriptografia foram implementadas funções inversas dos passos usados para criptografar

___
 ## Função trocar_inverse_bytes
    pub fn trocar_inverse_bytes(mut registro: Vec<u8>)-> Vec<u8>{
    // inverso da tabela sbox,  byte_trocado > byte
	    let inversesbox = vec![0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
	    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
	    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
	    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
	    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
	    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
	    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
	    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
	    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
	    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
	    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
	    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
	    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
	    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
	    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
	    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d
		];
	
	    //substitui cada byte pelo correspondente na inversesbox
	    for n in 0..=15{
	        registro[n] = inversesbox[registro[n] as usize]
	    }
	    return registro
		}

___

## Função desloca_inverse_linha
    pub fn desloca_inverse_linha(mut registro: Vec<u8>)-> Vec<u8>{
	    let temp: Vec<u8> = vec![
	        registro[0], registro[13], registro[10], registro[7],
	        registro[4], registro[1], registro[14], registro[11],
	        registro[8], registro[5], registro[2], registro[15],
	        registro[12], registro[9], registro[6], registro[3]
	    ];

	    return temp
	    }

___


## Função embaralha_inverse_colunas
    fn embaralha_inverse_colunas(mut registro: Vec<u8>)-> Vec<u8>{

    // tabela campo de Galois multiplica 9,
	    let mult9 = vec![0x00,0x09,0x12,0x1b,0x24,0x2d,0x36,0x3f,0x48,0x41,0x5a,0x53,0x6c,0x65,0x7e,0x77,
	    0x90,0x99,0x82,0x8b,0xb4,0xbd,0xa6,0xaf,0xd8,0xd1,0xca,0xc3,0xfc,0xf5,0xee,0xe7,
	    0x3b,0x32,0x29,0x20,0x1f,0x16,0x0d,0x04,0x73,0x7a,0x61,0x68,0x57,0x5e,0x45,0x4c,
	    0xab,0xa2,0xb9,0xb0,0x8f,0x86,0x9d,0x94,0xe3,0xea,0xf1,0xf8,0xc7,0xce,0xd5,0xdc,
	    0x76,0x7f,0x64,0x6d,0x52,0x5b,0x40,0x49,0x3e,0x37,0x2c,0x25,0x1a,0x13,0x08,0x01,
	    0xe6,0xef,0xf4,0xfd,0xc2,0xcb,0xd0,0xd9,0xae,0xa7,0xbc,0xb5,0x8a,0x83,0x98,0x91,
	    0x4d,0x44,0x5f,0x56,0x69,0x60,0x7b,0x72,0x05,0x0c,0x17,0x1e,0x21,0x28,0x33,0x3a,
	    0xdd,0xd4,0xcf,0xc6,0xf9,0xf0,0xeb,0xe2,0x95,0x9c,0x87,0x8e,0xb1,0xb8,0xa3,0xaa,	
	    0xec,0xe5,0xfe,0xf7,0xc8,0xc1,0xda,0xd3,0xa4,0xad,0xb6,0xbf,0x80,0x89,0x92,0x9b,	
	    0x7c,0x75,0x6e,0x67,0x58,0x51,0x4a,0x43,0x34,0x3d,0x26,0x2f,0x10,0x19,0x02,0x0b,
	    0xd7,0xde,0xc5,0xcc,0xf3,0xfa,0xe1,0xe8,0x9f,0x96,0x8d,0x84,0xbb,0xb2,0xa9,0xa0,
	    0x47,0x4e,0x55,0x5c,0x63,0x6a,0x71,0x78,0x0f,0x06,0x1d,0x14,0x2b,0x22,0x39,0x30,
	    0x9a,0x93,0x88,0x81,0xbe,0xb7,0xac,0xa5,0xd2,0xdb,0xc0,0xc9,0xf6,0xff,0xe4,0xed,
	    0x0a,0x03,0x18,0x11,0x2e,0x27,0x3c,0x35,0x42,0x4b,0x50,0x59,0x66,0x6f,0x74,0x7d,	
	    0xa1,0xa8,0xb3,0xba,0x85,0x8c,0x97,0x9e,0xe9,0xe0,0xfb,0xf2,0xcd,0xc4,0xdf,0xd6,
	    0x31,0x38,0x23,0x2a,0x15,0x1c,0x07,0x0e,0x79,0x70,0x6b,0x62,0x5d,0x54,0x4f,0x46
		];
	
	    // tabela campo de Galois multiplica 11,
	    let mult11 = vec![0x00,0x0b,0x16,0x1d,0x2c,0x27,0x3a,0x31,0x58,0x53,0x4e,0x45,0x74,0x7f,0x62,0x69,
	    0xb0,0xbb,0xa6,0xad,0x9c,0x97,0x8a,0x81,0xe8,0xe3,0xfe,0xf5,0xc4,0xcf,0xd2,0xd9,
	    0x7b,0x70,0x6d,0x66,0x57,0x5c,0x41,0x4a,0x23,0x28,0x35,0x3e,0x0f,0x04,0x19,0x12,
	    0xcb,0xc0,0xdd,0xd6,0xe7,0xec,0xf1,0xfa,0x93,0x98,0x85,0x8e,0xbf,0xb4,0xa9,0xa2,
	    0xf6,0xfd,0xe0,0xeb,0xda,0xd1,0xcc,0xc7,0xae,0xa5,0xb8,0xb3,0x82,0x89,0x94,0x9f,
	    0x46,0x4d,0x50,0x5b,0x6a,0x61,0x7c,0x77,0x1e,0x15,0x08,0x03,0x32,0x39,0x24,0x2f,
	    0x8d,0x86,0x9b,0x90,0xa1,0xaa,0xb7,0xbc,0xd5,0xde,0xc3,0xc8,0xf9,0xf2,0xef,0xe4,
	    0x3d,0x36,0x2b,0x20,0x11,0x1a,0x07,0x0c,0x65,0x6e,0x73,0x78,0x49,0x42,0x5f,0x54,
	    0xf7,0xfc,0xe1,0xea,0xdb,0xd0,0xcd,0xc6,0xaf,0xa4,0xb9,0xb2,0x83,0x88,0x95,0x9e,
	    0x47,0x4c,0x51,0x5a,0x6b,0x60,0x7d,0x76,0x1f,0x14,0x09,0x02,0x33,0x38,0x25,0x2e,
	    0x8c,0x87,0x9a,0x91,0xa0,0xab,0xb6,0xbd,0xd4,0xdf,0xc2,0xc9,0xf8,0xf3,0xee,0xe5,
	    0x3c,0x37,0x2a,0x21,0x10,0x1b,0x06,0x0d,0x64,0x6f,0x72,0x79,0x48,0x43,0x5e,0x55,
	    0x01,0x0a,0x17,0x1c,0x2d,0x26,0x3b,0x30,0x59,0x52,0x4f,0x44,0x75,0x7e,0x63,0x68,
	    0xb1,0xba,0xa7,0xac,0x9d,0x96,0x8b,0x80,0xe9,0xe2,0xff,0xf4,0xc5,0xce,0xd3,0xd8,
	    0x7a,0x71,0x6c,0x67,0x56,0x5d,0x40,0x4b,0x22,0x29,0x34,0x3f,0x0e,0x05,0x18,0x13,
	    0xca,0xc1,0xdc,0xd7,0xe6,0xed,0xf0,0xfb,0x92,0x99,0x84,0x8f,0xbe,0xb5,0xa8,0xa3
		];
	
	    // tabela campo de Galois multiplica 11,
	    let mult13 = vec![0x00,0x0d,0x1a,0x17,0x34,0x39,0x2e,0x23,0x68,0x65,0x72,0x7f,0x5c,0x51,0x46,0x4b,
	    0xd0,0xdd,0xca,0xc7,0xe4,0xe9,0xfe,0xf3,0xb8,0xb5,0xa2,0xaf,0x8c,0x81,0x96,0x9b,
	    0xbb,0xb6,0xa1,0xac,0x8f,0x82,0x95,0x98,0xd3,0xde,0xc9,0xc4,0xe7,0xea,0xfd,0xf0,
	    0x6b,0x66,0x71,0x7c,0x5f,0x52,0x45,0x48,0x03,0x0e,0x19,0x14,0x37,0x3a,0x2d,0x20,
	    0x6d,0x60,0x77,0x7a,0x59,0x54,0x43,0x4e,0x05,0x08,0x1f,0x12,0x31,0x3c,0x2b,0x26,
	    0xbd,0xb0,0xa7,0xaa,0x89,0x84,0x93,0x9e,0xd5,0xd8,0xcf,0xc2,0xe1,0xec,0xfb,0xf6,
	    0xd6,0xdb,0xcc,0xc1,0xe2,0xef,0xf8,0xf5,0xbe,0xb3,0xa4,0xa9,0x8a,0x87,0x90,0x9d,
	    0x06,0x0b,0x1c,0x11,0x32,0x3f,0x28,0x25,0x6e,0x63,0x74,0x79,0x5a,0x57,0x40,0x4d,
	    0xda,0xd7,0xc0,0xcd,0xee,0xe3,0xf4,0xf9,0xb2,0xbf,0xa8,0xa5,0x86,0x8b,0x9c,0x91,
	    0x0a,0x07,0x10,0x1d,0x3e,0x33,0x24,0x29,0x62,0x6f,0x78,0x75,0x56,0x5b,0x4c,0x41,
	    0x61,0x6c,0x7b,0x76,0x55,0x58,0x4f,0x42,0x09,0x04,0x13,0x1e,0x3d,0x30,0x27,0x2a,
	    0xb1,0xbc,0xab,0xa6,0x85,0x88,0x9f,0x92,0xd9,0xd4,0xc3,0xce,0xed,0xe0,0xf7,0xfa,
	    0xb7,0xba,0xad,0xa0,0x83,0x8e,0x99,0x94,0xdf,0xd2,0xc5,0xc8,0xeb,0xe6,0xf1,0xfc,
	    0x67,0x6a,0x7d,0x70,0x53,0x5e,0x49,0x44,0x0f,0x02,0x15,0x18,0x3b,0x36,0x21,0x2c,
	    0x0c,0x01,0x16,0x1b,0x38,0x35,0x22,0x2f,0x64,0x69,0x7e,0x73,0x50,0x5d,0x4a,0x47,
	    0xdc,0xd1,0xc6,0xcb,0xe8,0xe5,0xf2,0xff,0xb4,0xb9,0xae,0xa3,0x80,0x8d,0x9a,0x97
		];
	
	    // tabela campo de Galois multiplica 11,
	    let mult14 = vec![0x00,0x0e,0x1c,0x12,0x38,0x36,0x24,0x2a,0x70,0x7e,0x6c,0x62,0x48,0x46,0x54,0x5a,
	    0xe0,0xee,0xfc,0xf2,0xd8,0xd6,0xc4,0xca,0x90,0x9e,0x8c,0x82,0xa8,0xa6,0xb4,0xba,
	    0xdb,0xd5,0xc7,0xc9,0xe3,0xed,0xff,0xf1,0xab,0xa5,0xb7,0xb9,0x93,0x9d,0x8f,0x81,
	    0x3b,0x35,0x27,0x29,0x03,0x0d,0x1f,0x11,0x4b,0x45,0x57,0x59,0x73,0x7d,0x6f,0x61,
	    0xad,0xa3,0xb1,0xbf,0x95,0x9b,0x89,0x87,0xdd,0xd3,0xc1,0xcf,0xe5,0xeb,0xf9,0xf7,
	    0x4d,0x43,0x51,0x5f,0x75,0x7b,0x69,0x67,0x3d,0x33,0x21,0x2f,0x05,0x0b,0x19,0x17,
	    0x76,0x78,0x6a,0x64,0x4e,0x40,0x52,0x5c,0x06,0x08,0x1a,0x14,0x3e,0x30,0x22,0x2c,
	    0x96,0x98,0x8a,0x84,0xae,0xa0,0xb2,0xbc,0xe6,0xe8,0xfa,0xf4,0xde,0xd0,0xc2,0xcc,
	    0x41,0x4f,0x5d,0x53,0x79,0x77,0x65,0x6b,0x31,0x3f,0x2d,0x23,0x09,0x07,0x15,0x1b,
	    0xa1,0xaf,0xbd,0xb3,0x99,0x97,0x85,0x8b,0xd1,0xdf,0xcd,0xc3,0xe9,0xe7,0xf5,0xfb,
	    0x9a,0x94,0x86,0x88,0xa2,0xac,0xbe,0xb0,0xea,0xe4,0xf6,0xf8,0xd2,0xdc,0xce,0xc0,
	    0x7a,0x74,0x66,0x68,0x42,0x4c,0x5e,0x50,0x0a,0x04,0x16,0x18,0x32,0x3c,0x2e,0x20,
	    0xec,0xe2,0xf0,0xfe,0xd4,0xda,0xc8,0xc6,0x9c,0x92,0x80,0x8e,0xa4,0xaa,0xb8,0xb6,
	    0x0c,0x02,0x10,0x1e,0x34,0x3a,0x28,0x26,0x7c,0x72,0x60,0x6e,0x44,0x4a,0x58,0x56,
	    0x37,0x39,0x2b,0x25,0x0f,0x01,0x13,0x1d,0x47,0x49,0x5b,0x55,0x7f,0x71,0x63,0x6d,
	    0xd7,0xd9,0xcb,0xc5,0xef,0xe1,0xf3,0xfd,0xa7,0xa9,0xbb,0xb5,0x9f,0x91,0x83,0x8d
		];
	
	    // embaralha os elementos
	    let elementos_embaralhado0 = mult14[registro[0] as usize] ^  mult11[registro[1] as usize] ^ mult13[registro[2] as usize] ^ mult9[registro[3] as usize];
	    let elementos_embaralhado1 = mult9[registro[0] as usize] ^  mult14[registro[1] as usize] ^ mult11[registro[2] as usize] ^ mult13[registro[3] as usize];
	    let elementos_embaralhado2 = mult13[registro[0] as usize] ^  mult9[registro[1] as usize] ^ mult14[registro[2] as usize] ^ mult11[registro[3] as usize];
	    let elementos_embaralhado3 = mult11[registro[0] as usize] ^  mult13[registro[1] as usize] ^ mult9[registro[2] as usize] ^ mult14[registro[3] as usize];
	
	    // retorna a coluna embaralhada
	    let mut col_embaralhada = vec![elementos_embaralhado0, elementos_embaralhado1, elementos_embaralhado2, elementos_embaralhado3];
	    
	
	    return col_embaralhada     
		}


___


## Função embaralha_bloco_inverso
    pub fn embaralha_bloco_inverso(mut registro: Vec<u8>) -> Vec<u8> {
	    let mut temp: Vec<u8> = Vec::new();
	    temp.extend(embaralha_inverse_colunas(registro[0..4].to_vec()));
	    temp.extend(embaralha_inverse_colunas(registro[4..8].to_vec()));
	    temp.extend(embaralha_inverse_colunas(registro[8..12].to_vec()));
	    temp.extend(embaralha_inverse_colunas(registro[12..16].to_vec()));
	    
	    return temp;
		}


___


# Arquivo key_expansion.rs
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


___


# Arquivo aes128.rs
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
