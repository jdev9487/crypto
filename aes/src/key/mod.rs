use crate::shared::s_box;

pub fn expand_key(key: [[u8; 4]; 4]) -> [[[u8; 4]; 4]; 11] {
    let mut result = [[[0; 4]; 4]; 11];
    let mut previous = key;
    result[0] = key;
    for i in 1..11 {
        let new = generate_round_key(previous, i - 1);
        result[i as usize] = new;
        previous = new;
    }
    result
}

fn generate_round_key(previous: [[u8; 4]; 4], index: i32) -> [[u8; 4]; 4] {
    let mut new: [[u8; 4]; 4] = [[0; 4]; 4];
    let mut temp = previous[3];
    rotate_word(&mut temp);
    sub_word(&mut temp);
    x_or_word(&mut temp, get_constant(index));
    for i in 0..4 {
        x_or_word(&mut temp, previous[i]);
        new[i] = temp;
    }
    new
}

fn get_constant(index: i32) -> [u8; 4] {
    match index {
        0 => [0x01, 0, 0, 0],
        1 => [0x02, 0, 0, 0],
        2 => [0x04, 0, 0, 0],
        3 => [0x08, 0, 0, 0],
        4 => [0x10, 0, 0, 0],
        5 => [0x20, 0, 0, 0],
        6 => [0x40, 0, 0, 0],
        7 => [0x80, 0, 0, 0],
        8 => [0x1b, 0, 0, 0],
        9 => [0x36, 0, 0, 0],
        _ => panic!()
    }
}

fn rotate_word(input: &mut [u8; 4]) {
    (input[0], input[1], input[2], input[3]) = (input[1], input[2], input[3], input[0])
}

fn sub_word(input: &mut [u8; 4]) {
    (input[0], input[1], input[2], input[3]) =
    (s_box(input[0]), s_box(input[1]), s_box(input[2]), s_box(input[3]));
}

fn x_or_word(input: &mut [u8; 4], constant: [u8; 4]) {
    for i in 0..4 {
        input[i] ^= constant[i];
    }
}