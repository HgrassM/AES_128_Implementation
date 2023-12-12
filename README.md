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
- Fizemos por funções bla bla bla na linguagem Rust que tem foco em segurança a tornando ideal para este projeto bla bla bla


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

- A função desloca_linha implementa a ShiftRows, ela realiza uma operação de reorganização (permutação) dos elementos de um vetor de bytes (registro). 
