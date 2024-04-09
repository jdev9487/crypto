use crate::{key::expand_key, shared::{add_round_key, inv_s_box, modular_multiply}};

pub fn inv_cipher(input: &mut [[u8; 4]; 4], key: [[u8; 4]; 4]) {
    let key_expansion = expand_key(key);
    add_round_key(input, key_expansion[10]);
    for i in (1..10).rev() {
        inv_shift_rows(input);
        inv_sub_bytes(input);
        add_round_key(input, key_expansion[i]);
        inv_mix_columns(input);
    }
    inv_shift_rows(input);
    inv_sub_bytes(input);
    add_round_key(input, key_expansion[0]);
}

fn inv_shift_rows(input: &mut [[u8; 4]; 4]) {
    (input[0][1], input[1][1], input[2][1], input[3][1]) = (input[3][1], input[0][1], input[1][1], input[2][1]);
    (input[0][2], input[1][2], input[2][2], input[3][2]) = (input[2][2], input[3][2], input[0][2], input[1][2]);
    (input[0][3], input[1][3], input[2][3], input[3][3]) = (input[1][3], input[2][3], input[3][3], input[0][3]);
}

fn inv_sub_bytes(input: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            let element = input[i][j];
            input[i][j] = inv_s_box(element);
        }
    }
}

fn inv_mix_columns(input: &mut [[u8; 4]; 4]) {
    for i in 0..4 {
        let col = input[i];
        let y_0 = modular_multiply(0x0e, col[0]) ^
                      modular_multiply(0x0b, col[1]) ^
                      modular_multiply(0x0d, col[2]) ^
                      modular_multiply(0x09, col[3]);
        let y_1 = modular_multiply(0x09, col[0]) ^
                      modular_multiply(0x0e, col[1]) ^
                      modular_multiply(0x0b, col[2]) ^
                      modular_multiply(0x0d, col[3]);
        let y_2 = modular_multiply(0x0d, col[0]) ^
                      modular_multiply(0x09, col[1]) ^
                      modular_multiply(0x0e, col[2]) ^
                      modular_multiply(0x0b, col[3]);
        let y_3 = modular_multiply(0x0b, col[0]) ^
                      modular_multiply(0x0d, col[1]) ^
                      modular_multiply(0x09, col[2]) ^
                      modular_multiply(0x0e, col[3]);
        input[i][0] = y_0;
        input[i][1] = y_1;
        input[i][2] = y_2;
        input[i][3] = y_3;
    }
}