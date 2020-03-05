pub mod structs;
use std::convert::TryFrom;
use structs::Base64;
use structs::Hex;

fn main() {
    let first = Hex::try_from("1c0111001f010100061a024b53535009181c").unwrap();
    let second = Hex::try_from("686974207468652062756c6c277320657965").unwrap();

    let result = first ^ second;

    println!("result : '{}'", &result.to_string());
    assert_eq!(
        result.to_string(),
        "746865206b696420646f6e277420706c6179".to_string()
    );
}
