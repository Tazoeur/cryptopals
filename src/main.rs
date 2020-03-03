pub mod structs;
use std::convert::TryFrom;
use structs::Base64;
use structs::Hex;

fn main() {
    let hexadecimal = Hex::try_from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    let base_64 =
        Base64::try_from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
            .unwrap();

    assert_eq!(hexadecimal.decode(), base_64.decode());

    println!("{}", Base64::encode(&hexadecimal.decode()).to_string());
}
