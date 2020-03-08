use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

use super::hex_symbol::HexSymbol;

const PARSING_ERROR: &str = "Error parsing Hexadecimal";

/********************************** HEX **************************************/

#[derive(Debug, PartialEq, Clone)]
pub struct Hex(Vec<HexSymbol>);

impl Hex {
    pub fn new(symbols: &[HexSymbol]) -> Self {
        Self(symbols.to_vec())
    }

    /// encode an ascii encoded string to an hexadecimal type
    pub fn encode(input: &str) -> Self {
        Self(input.chars().map(|c| HexSymbol::new(c as u8)).collect())
    }

    /// decode the hexadecimal to an ascii encoded string
    pub fn decode(&self) -> String {
        self.iter().map(|h| h.decode() as char).collect()
    }

    pub fn rolling_xor(&self, key: &Self) -> Self {
        Self(self.0.chunks(key.0.len()).fold(Vec::new(), |tab, chunk| {
            [
                tab,
                chunk
                    .iter()
                    .zip(key.0.iter())
                    .map(|(self_symbol, key_symbol)| *self_symbol ^ *key_symbol)
                    .collect::<Vec<HexSymbol>>(),
            ]
            .concat()
        }))
    }

    pub fn hamming(&self, other: &Self) -> u32 {
        // panic if both Hex are not the same length
        assert_eq!(self.0.len(), other.0.len());

        self.0.iter().zip((*other).iter()).fold(
            0u32,
            |hamming_distance, (self_symbol, other_symbol)| {
                hamming_distance + self_symbol.hamming(other_symbol)
            },
        )
    }
}

/***************************** TRAITS *****************************************/

impl Deref for Hex {
    type Target = Vec<HexSymbol>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for Hex {
    type Error = &'static str;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        let length = hex.len();
        if length
            != hex
                .to_lowercase()
                .chars()
                .filter(|c| "1234567890abcdef".contains(*c))
                .count()
            || length % 2 != 0
        {
            return Err(PARSING_ERROR);
        }

        let symbols: Vec<HexSymbol> = hex
            .to_lowercase()
            .as_bytes()
            .chunks(2)
            .map(|tuple| HexSymbol::from((tuple[0] as char, tuple[1] as char)))
            .collect();

        Ok(Self(symbols))
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            (*self)
                .iter()
                .map(|hex| hex.to_string())
                .collect::<String>()
        )
    }
}

impl std::ops::BitXor for Hex {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> Self::Output {
        // panic if both vector do not have same size
        assert_eq!(self.0.len(), rhs.0.len());

        Self(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(self_symbol, rhs_symbol)| *self_symbol ^ *rhs_symbol)
                .collect::<Vec<HexSymbol>>(),
        )
    }
}

/***************************** TESTS *****************************************/

#[cfg(test)]
mod test {
    use super::Hex;
    use super::HexSymbol;
    use super::PARSING_ERROR;
    use std::convert::TryFrom;

    #[test]
    fn wrong_input_odd_length() {
        assert_eq!(Hex::try_from("aaa"), Err(PARSING_ERROR));
    }

    #[test]
    fn wrong_input_illegal_char() {
        assert_eq!(Hex::try_from("aay2"), Err(PARSING_ERROR));
    }

    #[test]
    fn valid_input() {
        assert_eq!(
            Hex::try_from("7a61"),
            Ok(Hex(vec!(HexSymbol::new(122), HexSymbol::new(97))))
        )
    }

    #[test]
    fn uppercase_input() {
        assert_eq!(
            Hex::try_from("7A6B"),
            Ok(Hex(vec!(HexSymbol::new(122), HexSymbol::new(107))))
        )
    }

    #[test]
    fn encode_input() {
        let input = "hello world!";
        let encoded = Hex::encode(&input);
        assert_eq!(encoded.to_string(), "68656c6c6f20776f726c6421".to_string());
    }

    #[test]
    fn decode_input() {
        let input = "68656c6c6f20776f726c6421";
        let decoded = Hex::try_from(input).unwrap().decode();
        assert_eq!(decoded, "hello world!".to_string())
    }

    #[test]
    fn hamming_distance() {
        let test = Hex::encode("this is a test");
        let wokka = Hex::encode("wokka wokka!!!");

        assert_eq!(test.hamming(&wokka), 37);
    }
}
