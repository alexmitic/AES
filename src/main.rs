use std::io::Read;
use std::io::{self, BufReader, Write};

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
    let mut key_list: [[u8; 16]; 11] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let mut state: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let io = io::stdin();
    let in_buff = io.lock();

    let mut reader = BufReader::new(in_buff);

    read_bytes(&mut reader, &mut key_list[0]);
    for i in 0..10 {
        key_expansion(&mut key_list, i + 1);
    }

    while read_bytes(&mut reader, &mut state) > 0 {
        encrypt(&mut state, &key_list);
        write(&state);
    }
}

fn encrypt(mut state: &mut [u8], key_list: &[[u8; 16]; 11]) {
    add_roundkey(&mut state, &key_list[0]);

    for round in 0..9 {
        bytes_substitution(&mut state);
        shift_row(&mut state);
        mix_colums(&mut state);
        add_roundkey(&mut state, &key_list[round + 1]);
    }

    bytes_substitution(&mut state);
    shift_row(&mut state);
    add_roundkey(&mut state, &key_list[10]);
}

fn add_roundkey(state: &mut [u8], key: &[u8]) {
    for i in 0..key.len() {
        state[i] ^= key[i];
    }
}

fn bytes_substitution(state: &mut [u8]) {
    for i in 0..state.len() {
        state[i] = S_BOX[(state[i] >> 4) as usize][(state[i] & 15) as usize];
    }
}

fn shift_row(state: &mut [u8]) {
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

fn mix_colums(state: &mut [u8]) {
    let mut old_state: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    old_state.copy_from_slice(state);

    state[0] = gmul(FIXED_MATRIX[0], old_state[0])
        ^ gmul(FIXED_MATRIX[1], old_state[1])
        ^ gmul(FIXED_MATRIX[2], old_state[2])
        ^ gmul(FIXED_MATRIX[3], old_state[3]);

    state[4] = gmul(FIXED_MATRIX[0], old_state[4])
        ^ gmul(FIXED_MATRIX[1], old_state[5])
        ^ gmul(FIXED_MATRIX[2], old_state[6])
        ^ gmul(FIXED_MATRIX[3], old_state[7]);

    state[8] = gmul(FIXED_MATRIX[0], old_state[8])
        ^ gmul(FIXED_MATRIX[1], old_state[9])
        ^ gmul(FIXED_MATRIX[2], old_state[10])
        ^ gmul(FIXED_MATRIX[3], old_state[11]);

    state[12] = gmul(FIXED_MATRIX[0], old_state[12])
        ^ gmul(FIXED_MATRIX[1], old_state[13])
        ^ gmul(FIXED_MATRIX[2], old_state[14])
        ^ gmul(FIXED_MATRIX[3], old_state[15]);

    // ************************************************

    state[1] = gmul(FIXED_MATRIX[4], old_state[0])
        ^ gmul(FIXED_MATRIX[5], old_state[1])
        ^ gmul(FIXED_MATRIX[6], old_state[2])
        ^ gmul(FIXED_MATRIX[7], old_state[3]);

    state[5] = gmul(FIXED_MATRIX[4], old_state[4])
        ^ gmul(FIXED_MATRIX[5], old_state[5])
        ^ gmul(FIXED_MATRIX[6], old_state[6])
        ^ gmul(FIXED_MATRIX[7], old_state[7]);

    state[9] = gmul(FIXED_MATRIX[4], old_state[8])
        ^ gmul(FIXED_MATRIX[5], old_state[9])
        ^ gmul(FIXED_MATRIX[6], old_state[10])
        ^ gmul(FIXED_MATRIX[7], old_state[11]);

    state[13] = gmul(FIXED_MATRIX[4], old_state[12])
        ^ gmul(FIXED_MATRIX[5], old_state[13])
        ^ gmul(FIXED_MATRIX[6], old_state[14])
        ^ gmul(FIXED_MATRIX[7], old_state[15]);

    // ************************************************

    state[2] = gmul(FIXED_MATRIX[8], old_state[0])
        ^ gmul(FIXED_MATRIX[9], old_state[1])
        ^ gmul(FIXED_MATRIX[10], old_state[2])
        ^ gmul(FIXED_MATRIX[11], old_state[3]);

    state[6] = gmul(FIXED_MATRIX[8], old_state[4])
        ^ gmul(FIXED_MATRIX[9], old_state[5])
        ^ gmul(FIXED_MATRIX[10], old_state[6])
        ^ gmul(FIXED_MATRIX[11], old_state[7]);

    state[10] = gmul(FIXED_MATRIX[8], old_state[8])
        ^ gmul(FIXED_MATRIX[9], old_state[9])
        ^ gmul(FIXED_MATRIX[10], old_state[10])
        ^ gmul(FIXED_MATRIX[11], old_state[11]);

    state[14] = gmul(FIXED_MATRIX[8], old_state[12])
        ^ gmul(FIXED_MATRIX[9], old_state[13])
        ^ gmul(FIXED_MATRIX[10], old_state[14])
        ^ gmul(FIXED_MATRIX[11], old_state[15]);

    // ************************************************

    state[3] = gmul(FIXED_MATRIX[12], old_state[0])
        ^ gmul(FIXED_MATRIX[13], old_state[1])
        ^ gmul(FIXED_MATRIX[14], old_state[2])
        ^ gmul(FIXED_MATRIX[15], old_state[3]);

    state[7] = gmul(FIXED_MATRIX[12], old_state[4])
        ^ gmul(FIXED_MATRIX[13], old_state[5])
        ^ gmul(FIXED_MATRIX[14], old_state[6])
        ^ gmul(FIXED_MATRIX[15], old_state[7]);

    state[11] = gmul(FIXED_MATRIX[12], old_state[8])
        ^ gmul(FIXED_MATRIX[13], old_state[9])
        ^ gmul(FIXED_MATRIX[14], old_state[10])
        ^ gmul(FIXED_MATRIX[15], old_state[11]);

    state[15] = gmul(FIXED_MATRIX[12], old_state[12])
        ^ gmul(FIXED_MATRIX[13], old_state[13])
        ^ gmul(FIXED_MATRIX[14], old_state[14])
        ^ gmul(FIXED_MATRIX[15], old_state[15]);
}

fn key_expansion(key_list: &mut [[u8; 16]; 11], round: usize) {
    // Circular byte left shift of w[3]
    key_list[round][12] = key_list[round - 1][13];
    key_list[round][13] = key_list[round - 1][14];
    key_list[round][14] = key_list[round - 1][15];
    key_list[round][15] = key_list[round - 1][12];

    // Byte Substitution (S-Box) of w[3]
    key_list[round][12] =
        S_BOX[(key_list[round][12] >> 4) as usize][(key_list[round][12] & 15) as usize];
    key_list[round][13] =
        S_BOX[(key_list[round][13] >> 4) as usize][(key_list[round][13] & 15) as usize];
    key_list[round][14] =
        S_BOX[(key_list[round][14] >> 4) as usize][(key_list[round][14] & 15) as usize];
    key_list[round][15] =
        S_BOX[(key_list[round][15] >> 4) as usize][(key_list[round][15] & 15) as usize];

    // Round constant
    key_list[round][12] ^= R_CON[round - 1];

    // w[4] = w[0] ⊕ g(w[3])
    key_list[round][0] = key_list[round - 1][0] ^ key_list[round][12];
    key_list[round][1] = key_list[round - 1][1] ^ key_list[round][13];
    key_list[round][2] = key_list[round - 1][2] ^ key_list[round][14];
    key_list[round][3] = key_list[round - 1][3] ^ key_list[round][15];

    // w[5] = w[4] ⊕ w[1]
    key_list[round][4] = key_list[round - 1][4] ^ key_list[round][0];
    key_list[round][5] = key_list[round - 1][5] ^ key_list[round][1];
    key_list[round][6] = key_list[round - 1][6] ^ key_list[round][2];
    key_list[round][7] = key_list[round - 1][7] ^ key_list[round][3];

    // w[6] = w[5] ⊕ w[2]
    key_list[round][8] = key_list[round - 1][8] ^ key_list[round][4];
    key_list[round][9] = key_list[round - 1][9] ^ key_list[round][5];
    key_list[round][10] = key_list[round - 1][10] ^ key_list[round][6];
    key_list[round][11] = key_list[round - 1][11] ^ key_list[round][7];

    // w[7] = w[6] ⊕ w[3]
    key_list[round][12] = key_list[round][8] ^ key_list[round - 1][12];
    key_list[round][13] = key_list[round][9] ^ key_list[round - 1][13];
    key_list[round][14] = key_list[round][10] ^ key_list[round - 1][14];
    key_list[round][15] = key_list[round][11] ^ key_list[round - 1][15];
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

fn read_bytes(in_buff: &mut io::BufReader<io::StdinLock>, mut state: &mut [u8]) -> i8 {
    let e = in_buff.read_exact(&mut state);
    let e = match e {
        Ok(_) => 1,
        Err(_) => -1,
    };

    return e;
}

fn write(state: &[u8]) {
    io::stdout().write_all(state).expect("Error while writing!");
}

fn print_hex(bytes: &[u8; 16]) {
    for x in bytes {
        print!("{:X} ", x);
    }

    print!("\n");
}
