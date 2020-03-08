pub mod attack;
pub mod structs;

use std::convert::TryFrom;

use std::fs;

use attack::xor_cipher;
use structs::{Base64, Dictionary, Hex};

fn main() {
    let test = Hex::encode("this is a test");
    let wokka = Hex::encode("wokka wokka!!!");

    let result = test.hamming(&wokka);

    println!("hamming = {}", result);
}
