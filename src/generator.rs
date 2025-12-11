use crate::wordbank::WordBank;
use rand::rng;
use rand::seq::{IndexedRandom, SliceRandom};
use std::io;

pub struct WeldGenerator {
    bank: WordBank,
}

impl WeldGenerator {
    pub fn new() -> Self {
        Self {
            bank: WordBank::new(),
        }
    }

    pub fn generate(&self, count: usize, separator: &str, capitalize: bool) -> String {
        let mut rng = rng();

        //Fetch words from bank
        let all_words = self.bank.get_all_words();

        //Safety check
        if all_words.is_empty() {
            return "ERROR: Word List is empty.".to_string();
        }

        //Cryptographically select 'count' unique words
        let mut selected_words: Vec<String> = all_words
            .choose_multiple(&mut rng, count)
            .map(|word| {
                //Apply capitalization if requested
                if capitalize {
                    let mut c = word.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                } else {
                    word.to_string()
                }
            })
            .collect();

        //Shuffle the selected words to randomize order
        selected_words.shuffle(&mut rng);

        //Weld them together
        selected_words.join(separator)
    }

    //Pass through fn to add words to bank
    pub fn add_word(&self, word: &str) -> io::Result<()> {
        self.bank.add_custom_word(word)
    }
}
