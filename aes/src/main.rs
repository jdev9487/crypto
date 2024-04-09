use cipher::cipher;
use inv_cipher::inv_cipher;

pub mod cipher;
pub mod inv_cipher;
pub mod key;

mod shared;

fn main() {
    let key: [[u8; 4]; 4] = [
        [0x2b, 0x7e, 0x15, 0x16],
        [0x28, 0xae, 0xd2, 0xa6],
        [0xab, 0xf7, 0x15, 0x88],
        [0x09, 0xcf, 0x4f, 0x3c]
    ];
    let mut input: [[u8; 4]; 4] = [
        [0x32, 0x43, 0xf6, 0xa8],
        [0x88, 0x5a, 0x30, 0x8d],
        [0x31, 0x31, 0x98, 0xa2],
        [0xe0, 0x37, 0x07, 0x34]
    ];
    input.iter().for_each(print);
    println!();
    cipher(&mut input, key);
    input.iter().for_each(print);
    println!();
    inv_cipher(&mut input, key);
    input.iter().for_each(print);
}

fn print(y: &[u8; 4]) {
    println!("{:#04x} {:#04x} {:#04x} {:#04x}", y[0], y[1], y[2], y[3]);
}