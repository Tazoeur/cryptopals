use std::fmt;
use std::ops::Deref;

/******************************* HEX SYMBOL **********************************/

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HexSymbol(u8);

impl HexSymbol {
    pub fn new(i: u8) -> Self {
        HexSymbol(i)
    }

    fn encode_partial(input: u8) -> char {
        match input % 16 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => panic!("Impossible to have something greater than 15!"),
        }
    }

    fn decode_partial(input: char) -> u8 {
        match input {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => panic!("Impossible to have an input different than '0123456789abcdef'!"),
        }
    }

    pub fn encode(input: u8) -> Self {
        HexSymbol(input)
    }

    pub fn decode(&self) -> u8 {
        self.0
    }

    pub fn is_printable(&self) -> bool {
        self.0 > 31
    }

    pub fn is_extended(&self) -> bool {
        self.0 > 127
    }

    pub fn hamming(&self, other: &HexSymbol) -> u32 {
        let x = self.0 ^ other.0;
        (0..8).fold(0u32, |dist, i| dist + (x >> i & 0b1) as u32)
    }
}

/***************************** TRAITS *****************************************/

impl Deref for HexSymbol {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for HexSymbol {
    fn from(input: u8) -> Self {
        Self::encode(input)
    }
}

impl From<char> for HexSymbol {
    fn from(input: char) -> Self {
        Self::encode(input as u8)
    }
}

impl From<(char, char)> for HexSymbol {
    fn from(input: (char, char)) -> Self {
        let hv = Self::decode_partial(input.0) << 4;
        let lv = Self::decode_partial(input.1);

        Self(hv + lv)
    }
}

impl fmt::Display for HexSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hv = **self >> 4;
        let lv = **self & 0b0000_1111;

        write!(
            f,
            "{}{}",
            HexSymbol::encode_partial(hv),
            HexSymbol::encode_partial(lv)
        )
    }
}

impl std::ops::BitXor for HexSymbol {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

/***************************** TESTS *****************************************/

#[cfg(test)]
mod test {
    use super::HexSymbol;

    #[test]
    fn hex_unit_from_u8_small() {
        let value: u8 = b'a';
        assert_eq!(value, 97);
        let hex_value = HexSymbol::from(value);
        assert_eq!(*hex_value, 97);
        let display: String = hex_value.to_string();

        // 'a' is written as '61' in hex
        assert_eq!(String::from("61"), display);
    }

    #[test]
    fn hex_unit_from_u8_big() {
        let value: u8 = b'z';
        assert_eq!(value, 122);
        let hex_value = HexSymbol::from(value);
        assert_eq!(*hex_value, 122);
        let display: String = hex_value.to_string();

        // 'z' is '7a' in hex
        assert_eq!(String::from("7a"), display);
    }

    #[test]
    fn hex_unit_from_char_small() {
        let value: char = 'a';
        let hex_value = HexSymbol::from(value);
        assert_eq!(*hex_value, 97);
        let display: String = hex_value.to_string();
        assert_eq!(String::from("61"), display);
    }

    #[test]
    fn hex_unit_from_char_big() {
        let value: char = 'z';
        let hex_value = HexSymbol::from(value);
        assert_eq!(*hex_value, 122);
        let display: String = hex_value.to_string();
        assert_eq!(String::from("7a"), display);
    }

    #[test]
    fn hex_unit_encode_small() {
        let value: u8 = b'a';
        assert_eq!(value, 97);
        let hex_value = HexSymbol::encode(value);
        assert_eq!(*hex_value, 97);
        let hex_decoded = hex_value.decode();
        assert_eq!(hex_decoded, 97);
        let display: String = hex_value.to_string();
        assert_eq!(String::from("61"), display);
    }

    #[test]
    fn hex_unit_encode_big() {
        let value: u8 = b'z';
        assert_eq!(value, 122);
        let hex_value = HexSymbol::encode(value);
        assert_eq!(*hex_value, 122);
        let hex_decoded = hex_value.decode();
        assert_eq!(hex_decoded, 122);
        let display: String = hex_value.to_string();
        assert_eq!(String::from("7a"), display);
    }

    #[test]
    fn hex_unit_from_char_tuple() {
        assert_eq!(HexSymbol(122), HexSymbol::from(('7', 'a')));
        assert_eq!(HexSymbol(97), HexSymbol::from(('6', '1')));
        assert_eq!(HexSymbol(122), HexSymbol::from(('7', 'a')));
        assert_eq!(HexSymbol(48), HexSymbol::from(('3', '0')));
        assert_eq!(HexSymbol(65), HexSymbol::from(('4', '1')));
        assert_eq!(HexSymbol(117), HexSymbol::from(('7', '5')));
    }

    #[test]
    fn hamming_distance() {
        assert_eq!(HexSymbol::new(9).hamming(&HexSymbol::new(14)), 3);
        assert_eq!(HexSymbol::new(4).hamming(&HexSymbol::new(8)), 2);
    }
}
