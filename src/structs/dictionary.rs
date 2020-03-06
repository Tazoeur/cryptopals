use std::collections::HashMap;
use std::fs;

const DICTIONARY_PATH: &'static str = "data/dictionary/words_alpha.txt";

#[derive(Debug)]
pub struct Dictionary {
    words: Vec<String>,
    letter_stats: Option<HashMap<char, u32>>,
}

impl Dictionary {
    pub fn new() -> Self {
        let raw_content = fs::read_to_string(DICTIONARY_PATH).expect("Error loading dictionary");
        Self {
            words: raw_content
                .replace("\r", "")
                .split("\n")
                .map(|w| w.to_owned())
                .collect(),
            letter_stats: None,
        }
    }

    /// Count the number of *words* (substring separated by anything that is not alphanumerical)
    pub fn hits(&self, input: &str) -> u32 {
        input
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|&&w| w != "")
            .fold(0u32, |hits, word| {
                hits + if self.words.iter().any(|w| w == word) {
                    1
                } else {
                    0
                }
            })
    }

    fn letter_counts(&mut self) {
        let mut counts: HashMap<char, u32> = HashMap::new();
        for w in self.words.iter() {
            for c in w.chars() {
                let n = counts.entry(c).or_insert(0);
                *n += 1;
            }
        }
        self.letter_stats = Some(counts);
    }

    pub fn letter_score(&mut self, input: &str) -> u32 {
        // calculate the letter stats if not already done
        if self.letter_stats.is_none() {
            self.letter_counts();
        }

        let letter_stats = self.letter_stats.as_ref().unwrap();

        input.chars().fold(0u32, |acc, c| {
            acc + match letter_stats.get(&c) {
                Some(n) => *n,
                None => 0u32,
            }
        })
    }
}
