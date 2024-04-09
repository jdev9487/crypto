use cipher::cipher;
use serde_derive::{Deserialize, Serialize};
use warp::{http::Response, Filter};
use base64::{engine::general_purpose, Engine as _};

mod cipher;
mod key;
mod shared;

#[derive(Deserialize, Serialize)]
struct EncryptionPayload {
    message: String
}

#[tokio::main]
async fn main() {
    let aes_encrypt = warp::get()
        .and(warp::path("aes")
        .and(warp::query::<EncryptionPayload>())
        .map(|ep: EncryptionPayload| Response::builder().body(format!("{}", encrypt(ep.message)))));
    warp::serve(aes_encrypt).run(([127, 0, 0, 1], 7878)).await;
}

fn encrypt(message: String) -> String {
    let key: [[u8; 4]; 4] = [
        [0x2b, 0x7e, 0x15, 0x16],
        [0x28, 0xae, 0xd2, 0xa6],
        [0xab, 0xf7, 0x15, 0x88],
        [0x09, 0xcf, 0x4f, 0x3c]
    ];
    let mut encrypted_message: String = String::from("");
    message.as_bytes().chunks(16).for_each(|c| {
        let mut input: [[u8; 4]; 4] = [[0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                input[i][j] = c[4*i + j]
            }
        }
        cipher(&mut input, key);
        let mut flattened:[u8; 16] = [0; 16];
        for i in 0..4 {
            for j in 0..4 {
                flattened[4*i + j] = input[i][j];
            }
        }
        encrypted_message.push_str(&general_purpose::STANDARD.encode(flattened));
    });
    encrypted_message
}

// fn print(y: &[u8; 4]) {
//     println!("{:#04x} {:#04x} {:#04x} {:#04x}", y[0], y[1], y[2], y[3]);
// }
    // let key: [[u8; 4]; 4] = [
    //     [0x2b, 0x7e, 0x15, 0x16],
    //     [0x28, 0xae, 0xd2, 0xa6],
    //     [0xab, 0xf7, 0x15, 0x88],
    //     [0x09, 0xcf, 0x4f, 0x3c]
    // ];
    // let mut input: [[u8; 4]; 4] = [
    //     [0x32, 0x43, 0xf6, 0xa8],
    //     [0x88, 0x5a, 0x30, 0x8d],
    //     [0x31, 0x31, 0x98, 0xa2],
    //     [0xe0, 0x37, 0x07, 0x34]
    // ];
    // input.iter().for_each(print);
    // println!();
    // cipher(&mut input, key);
    // input.iter().for_each(print);
    // println!();
    // inv_cipher(&mut input, key);
    // input.iter().for_each(print);