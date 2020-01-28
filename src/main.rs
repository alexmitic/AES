use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

static S_BOX: [[u8; 16]; 16] = [
    [
        99, 124, 119, 123, 242, 107, 111, 197, 48, 1, 103, 43, 254, 215, 171, 118,
    ],
    [
        202, 130, 201, 125, 250, 89, 71, 240, 173, 212, 162, 175, 156, 164, 114, 192,
    ],
    [
        183, 253, 147, 38, 54, 63, 247, 204, 52, 165, 229, 241, 113, 216, 49, 21,
    ],
    [
        4, 199, 35, 195, 24, 150, 5, 154, 7, 18, 128, 226, 235, 39, 178, 117,
    ],
    [
        9, 131, 44, 26, 27, 110, 90, 160, 82, 59, 214, 179, 41, 227, 47, 132,
    ],
    [
        83, 209, 0, 237, 32, 252, 177, 91, 106, 203, 190, 57, 74, 76, 88, 207,
    ],
    [
        208, 239, 170, 251, 67, 77, 51, 133, 69, 249, 2, 127, 80, 60, 159, 168,
    ],
    [
        81, 163, 64, 143, 146, 157, 56, 245, 188, 182, 218, 33, 16, 255, 243, 210,
    ],
    [
        205, 12, 19, 236, 95, 151, 68, 23, 196, 167, 126, 61, 100, 93, 25, 115,
    ],
    [
        96, 129, 79, 220, 34, 42, 144, 136, 70, 238, 184, 20, 222, 94, 11, 219,
    ],
    [
        224, 50, 58, 10, 73, 6, 36, 92, 194, 211, 172, 98, 145, 149, 228, 121,
    ],
    [
        231, 200, 55, 109, 141, 213, 78, 169, 108, 86, 244, 234, 101, 122, 174, 8,
    ],
    [
        186, 120, 37, 46, 28, 166, 180, 198, 232, 221, 116, 31, 75, 189, 139, 138,
    ],
    [
        112, 62, 181, 102, 72, 3, 246, 14, 97, 53, 87, 185, 134, 193, 29, 158,
    ],
    [
        225, 248, 152, 17, 105, 217, 142, 148, 155, 30, 135, 233, 206, 85, 40, 223,
    ],
    [
        140, 161, 137, 13, 191, 230, 66, 104, 65, 153, 45, 15, 176, 84, 187, 22,
    ],
];

static R_CON: [u8; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 27, 54];

static FIXED_MATRIX: [u8; 16] = [2, 3, 1, 1, 1, 2, 3, 1, 1, 1, 2, 3, 3, 1, 1, 2];

fn main() {
    let (mut key, mut state) = read_input();

    print!("State in: ");
    print_hex(&state);

    add_roundkey(&mut state, &key);
    bytes_substitution(&mut state);
    shift_row(&mut state);
    mix_colums(&mut state);

    print!("State out: ");
    print_hex(&state);

    // print!("Message in: ");
    // print_hex(&_message);

    // for round in 0..10 {
    //     key_expansion(&mut key, round);
    //     print!("Round {:?}: ", round);
    //     print_hex(&key);
    // }
}

fn add_roundkey(state: &mut Vec<u8>, key: &Vec<u8>) {
    for i in 0..key.len() {
        state[i] ^= key[i];
    }
}

fn bytes_substitution(state: &mut Vec<u8>) {
    for i in 0..state.len() {
        state[i] = S_BOX[(state[i] >> 4) as usize][(state[i] & 15) as usize];
    }
}

fn shift_row(state: &mut Vec<u8>) {
    let mut tmp: u8;

    // Second row shift offset 1
    tmp = state[1];
    state[1] = state[5];
    state[5] = state[9];
    state[9] = state[13];
    state[13] = tmp;

    // Third row shift offset 2
    tmp = state[2];
    state[2] = state[10];
    state[10] = tmp;

    tmp = state[6];
    state[6] = state[14];
    state[14] = tmp;

    // Fourth row shift offset 3
    tmp = state[15];
    state[15] = state[11];
    state[11] = state[7];
    state[7] = state[3];
    state[3] = tmp;
}

fn mix_colums(state: &mut Vec<u8>) {
    let old_state = state.clone();

    state[0] =  gmul(FIXED_MATRIX[0], old_state[0]) ^ 
                gmul(FIXED_MATRIX[1], old_state[1]) ^ 
                gmul(FIXED_MATRIX[2], old_state[2]) ^ 
                gmul(FIXED_MATRIX[3], old_state[3]);

    state[4] =  gmul(FIXED_MATRIX[0], old_state[4]) ^ 
                gmul(FIXED_MATRIX[1], old_state[5]) ^ 
                gmul(FIXED_MATRIX[2], old_state[6]) ^ 
                gmul(FIXED_MATRIX[3], old_state[7]);

    state[8] =  gmul(FIXED_MATRIX[0], old_state[8]) ^ 
                gmul(FIXED_MATRIX[1], old_state[9]) ^ 
                gmul(FIXED_MATRIX[2], old_state[10]) ^ 
                gmul(FIXED_MATRIX[3], old_state[11]);

    state[12] = gmul(FIXED_MATRIX[0], old_state[12]) ^ 
                gmul(FIXED_MATRIX[1], old_state[13]) ^ 
                gmul(FIXED_MATRIX[2], old_state[14]) ^ 
                gmul(FIXED_MATRIX[3], old_state[15]);

    // ************************************************

    state[1] =  gmul(FIXED_MATRIX[4], old_state[0]) ^ 
                gmul(FIXED_MATRIX[5], old_state[1]) ^ 
                gmul(FIXED_MATRIX[6], old_state[2]) ^ 
                gmul(FIXED_MATRIX[7], old_state[3]);

    state[5] =  gmul(FIXED_MATRIX[4], old_state[4]) ^ 
                gmul(FIXED_MATRIX[5], old_state[5]) ^ 
                gmul(FIXED_MATRIX[6], old_state[6]) ^ 
                gmul(FIXED_MATRIX[7], old_state[7]);

    state[9] =  gmul(FIXED_MATRIX[4], old_state[8]) ^ 
                gmul(FIXED_MATRIX[5], old_state[9]) ^ 
                gmul(FIXED_MATRIX[6], old_state[10]) ^ 
                gmul(FIXED_MATRIX[7], old_state[11]);

    state[13] = gmul(FIXED_MATRIX[4], old_state[12]) ^ 
                gmul(FIXED_MATRIX[5], old_state[13]) ^ 
                gmul(FIXED_MATRIX[6], old_state[14]) ^ 
                gmul(FIXED_MATRIX[7], old_state[15]);

    // ************************************************
    
    state[2] =  gmul(FIXED_MATRIX[8], old_state[0]) ^ 
                gmul(FIXED_MATRIX[9], old_state[1]) ^ 
                gmul(FIXED_MATRIX[10], old_state[2]) ^ 
                gmul(FIXED_MATRIX[11], old_state[3]);

    state[6] =  gmul(FIXED_MATRIX[8], old_state[4]) ^ 
                gmul(FIXED_MATRIX[9], old_state[5]) ^ 
                gmul(FIXED_MATRIX[10], old_state[6]) ^ 
                gmul(FIXED_MATRIX[11], old_state[7]);

    state[10] = gmul(FIXED_MATRIX[8], old_state[8]) ^ 
                gmul(FIXED_MATRIX[9], old_state[9]) ^ 
                gmul(FIXED_MATRIX[10], old_state[10]) ^ 
                gmul(FIXED_MATRIX[11], old_state[11]);

    state[14] = gmul(FIXED_MATRIX[8], old_state[12]) ^ 
                gmul(FIXED_MATRIX[9], old_state[13]) ^ 
                gmul(FIXED_MATRIX[10], old_state[14]) ^ 
                gmul(FIXED_MATRIX[11], old_state[15]);

    // ************************************************
    
    state[3] =  gmul(FIXED_MATRIX[12], old_state[0]) ^ 
                gmul(FIXED_MATRIX[13], old_state[1]) ^ 
                gmul(FIXED_MATRIX[14], old_state[2]) ^ 
                gmul(FIXED_MATRIX[15], old_state[3]);

    state[7] =  gmul(FIXED_MATRIX[12], old_state[4]) ^ 
                gmul(FIXED_MATRIX[13], old_state[5]) ^ 
                gmul(FIXED_MATRIX[14], old_state[6]) ^ 
                gmul(FIXED_MATRIX[15], old_state[7]);

    state[11] = gmul(FIXED_MATRIX[12], old_state[8]) ^ 
                gmul(FIXED_MATRIX[13], old_state[9]) ^ 
                gmul(FIXED_MATRIX[14], old_state[10]) ^ 
                gmul(FIXED_MATRIX[15], old_state[11]);

    state[15] = gmul(FIXED_MATRIX[12], old_state[12]) ^ 
                gmul(FIXED_MATRIX[13], old_state[13]) ^ 
                gmul(FIXED_MATRIX[14], old_state[14]) ^ 
                gmul(FIXED_MATRIX[15], old_state[15]);
}

fn key_expansion(key: &mut Vec<u8>, round: usize) {
    // Save w[3] for last XOR
    let w30 = key[12];
    let w31 = key[13];
    let w32 = key[14];
    let w33 = key[15];

    // Circular byte left shift of w[3]
    let tmp = key[12];
    key[12] = key[13];
    key[13] = key[14];
    key[14] = key[15];
    key[15] = tmp;

    // Byte Substitution (S-Box) of w[3]
    key[12] = S_BOX[(key[12] >> 4) as usize][(key[12] & 15) as usize];
    key[13] = S_BOX[(key[13] >> 4) as usize][(key[13] & 15) as usize];
    key[14] = S_BOX[(key[14] >> 4) as usize][(key[14] & 15) as usize];
    key[15] = S_BOX[(key[15] >> 4) as usize][(key[15] & 15) as usize];

    // Round constant
    key[12] ^= R_CON[round];

    // w[4] = w[0] ⊕ g(w[3])
    key[0] ^= key[12];
    key[1] ^= key[13];
    key[2] ^= key[14];
    key[3] ^= key[15];

    // w[5] = w[4] ⊕ w[1]
    key[4] ^= key[0];
    key[5] ^= key[1];
    key[6] ^= key[2];
    key[7] ^= key[3];

    // w[6] = w[5] ⊕ w[2]
    key[8] ^= key[4];
    key[9] ^= key[5];
    key[10] ^= key[6];
    key[11] ^= key[7];

    // w[7] = w[6] ⊕ w[3]
    key[12] = key[8] ^ w30;
    key[13] = key[9] ^ w31;
    key[14] = key[10] ^ w32;
    key[15] = key[11] ^ w33;
}

fn read_input() -> (Vec<u8>, Vec<u8>) {
    let mut in_buff = io::stdin();

    let mut key = [0; 16];
    in_buff.read(&mut key).expect("Failed reading key");

    let mut reader = BufReader::new(in_buff);

    let mut message = vec![];
    let _num_bytes = reader
        .read_until(b'-', &mut message)
        .expect("Failed reading message");

    return (key.to_vec(), message);
}

fn print_hex(bytes: &Vec<u8>) {
    for x in bytes {
        print!("{:X} ", x);
    }

    print!("\n");
}

fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
 
    for _counter in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }

        let hi_bit_set = (a & 0x80) != 0;
        a <<= 1;
        if hi_bit_set {
            a ^= 0x1B; 
        }
        b >>= 1;
    }
 
    return p;
}
