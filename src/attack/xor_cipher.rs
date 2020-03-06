use crate::structs::{Dictionary, Hex, HexSymbol};

use std::u8;

pub fn single_byte_xor_score_recognition(dictionary: &mut Dictionary, input: Hex) {
    let mut max_score: u32 = 1;
    let mut best_fit: String = String::new();
    let mut key: HexSymbol = HexSymbol::new(0);

    for byte in u8::MIN..=u8::MAX {
        let hs = HexSymbol::new(byte);

        let decoded = input.xor(hs);
        let decoded_str = decoded.decode();

        if decoded.iter().fold(true, |acc, elem| {
            acc && elem.is_printable() && !elem.is_extended()
        }) {
            let decoded_score = dictionary.letter_score(&decoded_str);
            if decoded_score >= max_score {
                max_score = decoded_score;
                best_fit = decoded_str;
                key = HexSymbol::new(byte);
            }
        }
    }
    if max_score > 1 {
        println!(
            "the input '{}', decoded with '{}' gave the result '{}'",
            input.to_string(),
            key.to_string(),
            best_fit
        );
    }
}

pub fn single_byte_xor_word_recognition(dictionary: &Dictionary, input: Hex) {
    for byte in u8::MIN..=u8::MAX {
        let hs = HexSymbol::new(byte);

        let decoded = input.xor(hs);
        let decoded_str = decoded.decode();

        if decoded.iter().fold(true, |acc, elem| {
            acc && elem.is_printable() && !elem.is_extended()
        }) && dictionary.hits(&decoded_str) > (&decoded_str.len() / 10) as u32
        {
            println!(
                "the input '{}' gave the result '{}'",
                input.to_string(),
                &decoded_str
            );
        }
    }
}
