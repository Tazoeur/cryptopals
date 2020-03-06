pub mod attack;
pub mod structs;

use std::convert::TryFrom;

use std::fs;

use attack::xor_cipher;
use structs::{Base64, Dictionary, Hex};

fn main() {
    let file = fs::read_to_string("data/challenges/4.txt").expect("Error loading file");
    let mut dictionary = Dictionary::new();

    for line in file.split("\n").collect::<Vec<&str>>().iter() {
        xor_cipher::single_byte_xor_score_recognition(
            &mut dictionary,
            Hex::try_from(*line).unwrap(),
        );
    }
}
