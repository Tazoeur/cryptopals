use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

const PARSING_ERROR: &str = "Error parsing Base64";

/********************************** BASE64 ***********************************/
#[derive(Debug, PartialEq)]
pub struct Base64(Vec<u8>);

impl Base64 {
    fn alphabet() -> &'static str {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="
    }

    pub fn encode(input: &str) -> Self {
        Base64(
            input
                .as_bytes()
                .chunks(3)
                .map(|tab| Vec::from(tab))
                .collect::<Vec<Vec<u8>>>()
                .iter()
                .fold(Vec::new(), |acc, xs| {
                    let mut tab: Vec<u8> = Vec::new();
                    let last_byte = xs.iter().enumerate().fold(0u8, |rest, (i, byte)| {
                        tab.push((rest + (byte >> 2 * (i + 1))) & 0b0011_1111);
                        byte << 6 - (2 * (i + 1))
                    });
                    tab.push(last_byte & 0b0011_1111);
                    let padding = xs.len() % 3;
                    if padding == 1 {
                        tab.push(64u8);
                        tab.push(64u8);
                    } else if padding == 2 {
                        tab.push(64u8);
                    }
                    [acc, tab].concat()
                })
                .iter()
                .map(|c| *c)
                .collect(),
        )
    }

    pub fn decode(&self) -> String {
        self.0
            .chunks(4)
            .map(|tab| Vec::from(tab))
            .collect::<Vec<Vec<u8>>>()
            .iter()
            .fold(Vec::new(), |acc, xs| {
                let decoded = xs[1..]
                    .iter()
                    .zip(xs.iter())
                    .enumerate()
                    // b is the elem i+1 & a is the elem i of the chunk &[u8;4]
                    .map(|(i, (b, a))| ((a << ((i + 1) * 2)) + (b >> (6 - (i + 1) * 2))) as char)
                    .collect::<Vec<char>>();

                [&acc[..], &decoded[..]].concat()
            })
            .iter()
            .map(|c| *c)
            .collect()
    }
}

/***************************** TRAITS *****************************************/

impl Deref for Base64 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for Base64 {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut mut_input = input;

        // remove trailing '=' and count them to get padding
        let mut padding: usize = 0;
        while let Some(c) = mut_input.chars().last() {
            if c == '=' {
                mut_input = &mut_input[..mut_input.len() - 1];
                padding += 1;
            } else {
                break;
            }
        }

        // check lenght of input, valid characters & padding length
        //  - input length must be % 4 == 0
        //  - characters must be found in alphabet
        //  - padding length cannot be greater than 2
        let length = mut_input.len();
        let alphabet = &Base64::alphabet()[..Base64::alphabet().len() - 1];

        if padding > 2
            || length != mut_input.chars().filter(|c| alphabet.contains(*c)).count()
            || (length + padding) % 4 != 0
        {
            return Err(PARSING_ERROR);
        }

        // for each char, return the u8 position (<64) of the char in the given alphabet
        Ok(Base64(
            mut_input
                .chars()
                .map(|c| alphabet.find(c).unwrap() as u8)
                .collect(),
        ))
    }
}

impl fmt::Display for Base64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            (*self)
                .iter()
                .map(|c| Base64::alphabet().as_bytes()[*c as usize] as char)
                .collect::<String>()
        )
    }
}

/***************************** TESTS *****************************************/

#[cfg(test)]
mod test {

    use super::Base64;
    use super::PARSING_ERROR;
    use std::convert::TryFrom;

    #[test]
    fn count_alphabet() {
        // 64 letters in the alphabet + 1 for padding
        assert_eq!(Base64::alphabet().len(), 64 + 1);
    }

    #[test]
    fn alphabet_without_equal() {
        let alphabet = &Base64::alphabet()[..Base64::alphabet().len() - 1];

        assert!(!alphabet.contains('='));
    }

    #[test]
    fn encode_input_padding_two() {
        let value = "A";
        assert_eq!(Base64::encode(&value).to_string(), "QQ==".to_string());
    }

    #[test]
    fn encode_input_padding_one() {
        let value = "AA";
        assert_eq!(Base64::encode(&value).to_string(), "QUE=".to_string());
    }

    #[test]
    fn encode_input_no_padding() {
        let value = "AAA";
        assert_eq!(Base64::encode(&value).to_string(), "QUFB".to_string());
    }

    #[test]
    fn encode_input() {
        let value = "hello world!";
        assert_eq!(
            Base64::encode(&value).to_string(),
            "aGVsbG8gd29ybGQh".to_string()
        );

        let value = "hello world";
        assert_eq!(
            Base64::encode(&value).to_string(),
            "aGVsbG8gd29ybGQ=".to_string()
        );

        let value = "hello worl";
        assert_eq!(
            Base64::encode(&value).to_string(),
            "aGVsbG8gd29ybA==".to_string()
        );
    }

    #[test]
    fn from_input_padding_two() {
        let input = "QQ==";
        let base_64 = Base64::try_from(input).unwrap();
        assert_eq!(base_64, Base64(vec!(16, 16)));
    }

    #[test]
    fn from_input_padding_one() {
        let input = "QUE=";
        let base_64 = Base64::try_from(input).unwrap();
        assert_eq!(base_64, Base64(vec!(16, 20, 4)));
    }

    #[test]
    fn from_input_no_padding() {
        let input = "QUFB";
        let base_64 = Base64::try_from(input).unwrap();
        assert_eq!(base_64, Base64(vec!(16, 20, 5, 1)));
    }

    #[test]
    fn decode_input() {
        let input = "aGVsbG8gd29ybGQh";
        let decoded = Base64::try_from(input).unwrap().decode();
        assert_eq!(decoded, "hello world!".to_string());
    }

    #[test]
    fn encode_and_decode() {
        let input = "hello world!";

        let encoded = Base64::encode(&input);
        let decoded = encoded.decode();
        assert_eq!(decoded, input.to_string());
    }

    #[test]
    fn illegal_parsing() {
        let input = "g%x";
        assert_eq!(Base64::try_from(input), Err(PARSING_ERROR));

        let input = "QQ=";
        assert_eq!(Base64::try_from(input), Err(PARSING_ERROR));
    }
}
