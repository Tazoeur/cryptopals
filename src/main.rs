pub mod attack;
pub mod structs;

use std::convert::TryFrom;

use std::fs;

use attack::xor_cipher;
use structs::{Base64, Dictionary, Hex};

fn main() {
    let test_strings = vec![
        "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal",
    ];

    let key = Hex::encode("ICE");

    for xored in test_strings
        .iter()
        .map(|text| (Hex::encode(text).rolling_xor(&key)).to_string())
        .collect::<Vec<String>>()
    {
        println!("{}", xored);
    }
}
