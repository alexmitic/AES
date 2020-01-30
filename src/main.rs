use std::io::Read;
use std::io::{self, BufReader, BufWriter, Write};

// const S_BOX: [[u8; 16]; 16] = [
//     [
//         99, 124, 119, 123, 242, 107, 111, 197, 48, 1, 103, 43, 254, 215, 171, 118,
//     ],
//     [
//         202, 130, 201, 125, 250, 89, 71, 240, 173, 212, 162, 175, 156, 164, 114, 192,
//     ],
//     [
//         183, 253, 147, 38, 54, 63, 247, 204, 52, 165, 229, 241, 113, 216, 49, 21,
//     ],
//     [
//         4, 199, 35, 195, 24, 150, 5, 154, 7, 18, 128, 226, 235, 39, 178, 117,
//     ],
//     [
//         9, 131, 44, 26, 27, 110, 90, 160, 82, 59, 214, 179, 41, 227, 47, 132,
//     ],
//     [
//         83, 209, 0, 237, 32, 252, 177, 91, 106, 203, 190, 57, 74, 76, 88, 207,
//     ],
//     [
//         208, 239, 170, 251, 67, 77, 51, 133, 69, 249, 2, 127, 80, 60, 159, 168,
//     ],
//     [
//         81, 163, 64, 143, 146, 157, 56, 245, 188, 182, 218, 33, 16, 255, 243, 210,
//     ],
//     [
//         205, 12, 19, 236, 95, 151, 68, 23, 196, 167, 126, 61, 100, 93, 25, 115,
//     ],
//     [
//         96, 129, 79, 220, 34, 42, 144, 136, 70, 238, 184, 20, 222, 94, 11, 219,
//     ],
//     [
//         224, 50, 58, 10, 73, 6, 36, 92, 194, 211, 172, 98, 145, 149, 228, 121,
//     ],
//     [
//         231, 200, 55, 109, 141, 213, 78, 169, 108, 86, 244, 234, 101, 122, 174, 8,
//     ],
//     [
//         186, 120, 37, 46, 28, 166, 180, 198, 232, 221, 116, 31, 75, 189, 139, 138,
//     ],
//     [
//         112, 62, 181, 102, 72, 3, 246, 14, 97, 53, 87, 185, 134, 193, 29, 158,
//     ],
//     [
//         225, 248, 152, 17, 105, 217, 142, 148, 155, 30, 135, 233, 206, 85, 40, 223,
//     ],
//     [
//         140, 161, 137, 13, 191, 230, 66, 104, 65, 153, 45, 15, 176, 84, 187, 22,
//     ],
// ];

static S_SUB: [u8; 256] = [
    99, 124, 119, 123, 242, 107, 111, 197, 48, 1, 103, 43, 254, 215, 171, 118, 202, 130, 201, 125,
    250, 89, 71, 240, 173, 212, 162, 175, 156, 164, 114, 192, 183, 253, 147, 38, 54, 63, 247, 204,
    52, 165, 229, 241, 113, 216, 49, 21, 4, 199, 35, 195, 24, 150, 5, 154, 7, 18, 128, 226, 235,
    39, 178, 117, 9, 131, 44, 26, 27, 110, 90, 160, 82, 59, 214, 179, 41, 227, 47, 132, 83, 209, 0,
    237, 32, 252, 177, 91, 106, 203, 190, 57, 74, 76, 88, 207, 208, 239, 170, 251, 67, 77, 51, 133,
    69, 249, 2, 127, 80, 60, 159, 168, 81, 163, 64, 143, 146, 157, 56, 245, 188, 182, 218, 33, 16,
    255, 243, 210, 205, 12, 19, 236, 95, 151, 68, 23, 196, 167, 126, 61, 100, 93, 25, 115, 96, 129,
    79, 220, 34, 42, 144, 136, 70, 238, 184, 20, 222, 94, 11, 219, 224, 50, 58, 10, 73, 6, 36, 92,
    194, 211, 172, 98, 145, 149, 228, 121, 231, 200, 55, 109, 141, 213, 78, 169, 108, 86, 244, 234,
    101, 122, 174, 8, 186, 120, 37, 46, 28, 166, 180, 198, 232, 221, 116, 31, 75, 189, 139, 138,
    112, 62, 181, 102, 72, 3, 246, 14, 97, 53, 87, 185, 134, 193, 29, 158, 225, 248, 152, 17, 105,
    217, 142, 148, 155, 30, 135, 233, 206, 85, 40, 223, 140, 161, 137, 13, 191, 230, 66, 104, 65,
    153, 45, 15, 176, 84, 187, 22,
];

static R_CON: [u8; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 27, 54];

static FIXED_MATRIX: [u8; 16] = [2, 3, 1, 1, 1, 2, 3, 1, 1, 1, 2, 3, 3, 1, 1, 2];

static G_MUL: [[u8; 256]; 3] = [
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
        131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148,
        149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
        167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
        185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
        203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
        221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238,
        239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
    ],
    [
        0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46,
        48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92,
        94, 96, 98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 126, 128, 130,
        132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 166,
        168, 170, 172, 174, 176, 178, 180, 182, 184, 186, 188, 190, 192, 194, 196, 198, 200, 202,
        204, 206, 208, 210, 212, 214, 216, 218, 220, 222, 224, 226, 228, 230, 232, 234, 236, 238,
        240, 242, 244, 246, 248, 250, 252, 254, 27, 25, 31, 29, 19, 17, 23, 21, 11, 9, 15, 13, 3,
        1, 7, 5, 59, 57, 63, 61, 51, 49, 55, 53, 43, 41, 47, 45, 35, 33, 39, 37, 91, 89, 95, 93,
        83, 81, 87, 85, 75, 73, 79, 77, 67, 65, 71, 69, 123, 121, 127, 125, 115, 113, 119, 117,
        107, 105, 111, 109, 99, 97, 103, 101, 155, 153, 159, 157, 147, 145, 151, 149, 139, 137,
        143, 141, 131, 129, 135, 133, 187, 185, 191, 189, 179, 177, 183, 181, 171, 169, 175, 173,
        163, 161, 167, 165, 219, 217, 223, 221, 211, 209, 215, 213, 203, 201, 207, 205, 195, 193,
        199, 197, 251, 249, 255, 253, 243, 241, 247, 245, 235, 233, 239, 237, 227, 225, 231, 229,
    ],
    [
        0, 3, 6, 5, 12, 15, 10, 9, 24, 27, 30, 29, 20, 23, 18, 17, 48, 51, 54, 53, 60, 63, 58, 57,
        40, 43, 46, 45, 36, 39, 34, 33, 96, 99, 102, 101, 108, 111, 106, 105, 120, 123, 126, 125,
        116, 119, 114, 113, 80, 83, 86, 85, 92, 95, 90, 89, 72, 75, 78, 77, 68, 71, 66, 65, 192,
        195, 198, 197, 204, 207, 202, 201, 216, 219, 222, 221, 212, 215, 210, 209, 240, 243, 246,
        245, 252, 255, 250, 249, 232, 235, 238, 237, 228, 231, 226, 225, 160, 163, 166, 165, 172,
        175, 170, 169, 184, 187, 190, 189, 180, 183, 178, 177, 144, 147, 150, 149, 156, 159, 154,
        153, 136, 139, 142, 141, 132, 135, 130, 129, 155, 152, 157, 158, 151, 148, 145, 146, 131,
        128, 133, 134, 143, 140, 137, 138, 171, 168, 173, 174, 167, 164, 161, 162, 179, 176, 181,
        182, 191, 188, 185, 186, 251, 248, 253, 254, 247, 244, 241, 242, 227, 224, 229, 230, 239,
        236, 233, 234, 203, 200, 205, 206, 199, 196, 193, 194, 211, 208, 213, 214, 223, 220, 217,
        218, 91, 88, 93, 94, 87, 84, 81, 82, 67, 64, 69, 70, 79, 76, 73, 74, 107, 104, 109, 110,
        103, 100, 97, 98, 115, 112, 117, 118, 127, 124, 121, 122, 59, 56, 61, 62, 55, 52, 49, 50,
        35, 32, 37, 38, 47, 44, 41, 42, 11, 8, 13, 14, 7, 4, 1, 2, 19, 16, 21, 22, 31, 28, 25, 26,
    ],
];

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

    let out = io::stdout();
    let out_buff = out.lock();
    let mut writer = BufWriter::new(out_buff);

    reader.read_exact(&mut key_list[0]).is_ok();

    for i in 0..10 {
        key_expansion(&mut key_list, i + 1);
    }

    while let Ok(_) = reader.read_exact(&mut state) {
        encrypt(&mut state, &key_list);
        writer.write_all(&state).expect("Error while writing!");
    }

    writer.flush();
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
    // This is some times faster for some reason...
    // for i in 0..key.len() {
    //     state[i] ^= key[i];
    // }

    for (i, k) in key.iter().enumerate() {
        state[i] ^= *k;
    }
}

fn bytes_substitution(state: &mut [u8]) {
    for element in state.iter_mut() {
        // *element = S_BOX[(*element >> 4) as usize][(*element & 15) as usize];
        *element = S_SUB[*element as usize];
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

    state[0] = G_MUL[(FIXED_MATRIX[0] - 1) as usize][old_state[0] as usize]
        ^ G_MUL[(FIXED_MATRIX[1] - 1) as usize][old_state[1] as usize]
        ^ G_MUL[(FIXED_MATRIX[2] - 1) as usize][old_state[2] as usize]
        ^ G_MUL[(FIXED_MATRIX[3] - 1) as usize][old_state[3] as usize];

    state[4] = G_MUL[(FIXED_MATRIX[0] - 1) as usize][old_state[4] as usize]
        ^ G_MUL[(FIXED_MATRIX[1] - 1) as usize][old_state[5] as usize]
        ^ G_MUL[(FIXED_MATRIX[2] - 1) as usize][old_state[6] as usize]
        ^ G_MUL[(FIXED_MATRIX[3] - 1) as usize][old_state[7] as usize];

    state[8] = G_MUL[(FIXED_MATRIX[0] - 1) as usize][old_state[8] as usize]
        ^ G_MUL[(FIXED_MATRIX[1] - 1) as usize][old_state[9] as usize]
        ^ G_MUL[(FIXED_MATRIX[2] - 1) as usize][old_state[10] as usize]
        ^ G_MUL[(FIXED_MATRIX[3] - 1) as usize][old_state[11] as usize];

    state[12] = G_MUL[(FIXED_MATRIX[0] - 1) as usize][old_state[12] as usize]
        ^ G_MUL[(FIXED_MATRIX[1] - 1) as usize][old_state[13] as usize]
        ^ G_MUL[(FIXED_MATRIX[2] - 1) as usize][old_state[14] as usize]
        ^ G_MUL[(FIXED_MATRIX[3] - 1) as usize][old_state[15] as usize];

    // ************************************************

    state[1] = G_MUL[(FIXED_MATRIX[4] - 1) as usize][old_state[0] as usize]
        ^ G_MUL[(FIXED_MATRIX[5] - 1) as usize][old_state[1] as usize]
        ^ G_MUL[(FIXED_MATRIX[6] - 1) as usize][old_state[2] as usize]
        ^ G_MUL[(FIXED_MATRIX[7] - 1) as usize][old_state[3] as usize];

    state[5] = G_MUL[(FIXED_MATRIX[4] - 1) as usize][old_state[4] as usize]
        ^ G_MUL[(FIXED_MATRIX[5] - 1) as usize][old_state[5] as usize]
        ^ G_MUL[(FIXED_MATRIX[6] - 1) as usize][old_state[6] as usize]
        ^ G_MUL[(FIXED_MATRIX[7] - 1) as usize][old_state[7] as usize];

    state[9] = G_MUL[(FIXED_MATRIX[4] - 1) as usize][old_state[8] as usize]
        ^ G_MUL[(FIXED_MATRIX[5] - 1) as usize][old_state[9] as usize]
        ^ G_MUL[(FIXED_MATRIX[6] - 1) as usize][old_state[10] as usize]
        ^ G_MUL[(FIXED_MATRIX[7] - 1) as usize][old_state[11] as usize];

    state[13] = G_MUL[(FIXED_MATRIX[4] - 1) as usize][old_state[12] as usize]
        ^ G_MUL[(FIXED_MATRIX[5] - 1) as usize][old_state[13] as usize]
        ^ G_MUL[(FIXED_MATRIX[6] - 1) as usize][old_state[14] as usize]
        ^ G_MUL[(FIXED_MATRIX[7] - 1) as usize][old_state[15] as usize];

    // ************************************************

    state[2] = G_MUL[(FIXED_MATRIX[8] - 1) as usize][old_state[0] as usize]
        ^ G_MUL[(FIXED_MATRIX[9] - 1) as usize][old_state[1] as usize]
        ^ G_MUL[(FIXED_MATRIX[10] - 1) as usize][old_state[2] as usize]
        ^ G_MUL[(FIXED_MATRIX[11] - 1) as usize][old_state[3] as usize];

    state[6] = G_MUL[(FIXED_MATRIX[8] - 1) as usize][old_state[4] as usize]
        ^ G_MUL[(FIXED_MATRIX[9] - 1) as usize][old_state[5] as usize]
        ^ G_MUL[(FIXED_MATRIX[10] - 1) as usize][old_state[6] as usize]
        ^ G_MUL[(FIXED_MATRIX[11] - 1) as usize][old_state[7] as usize];

    state[10] = G_MUL[(FIXED_MATRIX[8] - 1) as usize][old_state[8] as usize]
        ^ G_MUL[(FIXED_MATRIX[9] - 1) as usize][old_state[9] as usize]
        ^ G_MUL[(FIXED_MATRIX[10] - 1) as usize][old_state[10] as usize]
        ^ G_MUL[(FIXED_MATRIX[11] - 1) as usize][old_state[11] as usize];

    state[14] = G_MUL[(FIXED_MATRIX[8] - 1) as usize][old_state[12] as usize]
        ^ G_MUL[(FIXED_MATRIX[9] - 1) as usize][old_state[13] as usize]
        ^ G_MUL[(FIXED_MATRIX[10] - 1) as usize][old_state[14] as usize]
        ^ G_MUL[(FIXED_MATRIX[11] - 1) as usize][old_state[15] as usize];

    // ************************************************

    state[3] = G_MUL[(FIXED_MATRIX[12] - 1) as usize][old_state[0] as usize]
        ^ G_MUL[(FIXED_MATRIX[13] - 1) as usize][old_state[1] as usize]
        ^ G_MUL[(FIXED_MATRIX[14] - 1) as usize][old_state[2] as usize]
        ^ G_MUL[(FIXED_MATRIX[15] - 1) as usize][old_state[3] as usize];

    state[7] = G_MUL[(FIXED_MATRIX[12] - 1) as usize][old_state[4] as usize]
        ^ G_MUL[(FIXED_MATRIX[13] - 1) as usize][old_state[5] as usize]
        ^ G_MUL[(FIXED_MATRIX[14] - 1) as usize][old_state[6] as usize]
        ^ G_MUL[(FIXED_MATRIX[15] - 1) as usize][old_state[7] as usize];

    state[11] = G_MUL[(FIXED_MATRIX[12] - 1) as usize][old_state[8] as usize]
        ^ G_MUL[(FIXED_MATRIX[13] - 1) as usize][old_state[9] as usize]
        ^ G_MUL[(FIXED_MATRIX[14] - 1) as usize][old_state[10] as usize]
        ^ G_MUL[(FIXED_MATRIX[15] - 1) as usize][old_state[11] as usize];

    state[15] = G_MUL[(FIXED_MATRIX[12] - 1) as usize][old_state[12] as usize]
        ^ G_MUL[(FIXED_MATRIX[13] - 1) as usize][old_state[13] as usize]
        ^ G_MUL[(FIXED_MATRIX[14] - 1) as usize][old_state[14] as usize]
        ^ G_MUL[(FIXED_MATRIX[15] - 1) as usize][old_state[15] as usize];
}

fn key_expansion(key_list: &mut [[u8; 16]; 11], round: usize) {
    // Circular byte left shift of w[3]
    key_list[round][12] = key_list[round - 1][13];
    key_list[round][13] = key_list[round - 1][14];
    key_list[round][14] = key_list[round - 1][15];
    key_list[round][15] = key_list[round - 1][12];

    // Byte Substitution (S-Box) of w[3]
    key_list[round][12] = S_SUB[key_list[round][12] as usize];
    key_list[round][13] = S_SUB[key_list[round][13] as usize];
    key_list[round][14] = S_SUB[key_list[round][14] as usize];
    key_list[round][15] = S_SUB[key_list[round][15] as usize];

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

// fn gmul(mut a: u8, mut b: u8) -> u8 {
//     if a == 1 { return b };
//     // if a == 2 { return (b << 1) ^ b };
//     if a == 3 { return gmul(2, b) ^ b };

//     let mut p: u8 = 0;

//     for _counter in 0..8 {
//         if b & 1 != 0 {
//             p ^= a;
//         }

//         let hi_bit_set = (a & 0x80) != 0;
//         a <<= 1;
//         if hi_bit_set {
//             a ^= 0x1B;
//         }
//         b >>= 1;
//     }

//     return p;
// }

fn print_hex(bytes: &[u8; 16]) {
    for x in bytes {
        print!("{:X} ", x);
    }

    print!("\n");
}
