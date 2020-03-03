pub mod structs;
use std::convert::TryFrom;
use structs::hex::Hex;

fn main() {
    let test = Hex::try_from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();

    println!("{}", test.decode());
}
