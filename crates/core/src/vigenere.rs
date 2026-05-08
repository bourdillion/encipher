use crate::utils::alphabet::{ALPHABET_SIZE, char_to_index, index_to_char};
use crate::utils::modular::{mod_inverse, mod_pos};

pub struct Vigenere {
    pub key: Vec<u32>,
}

impl Vigenere {
    pub fn new(_key: &str) -> Self {
        let mut key = Vec::new();
        _key.chars()
            .for_each(|c| key.push(char_to_index(c).unwrap() as u32));

        Vigenere { key }
    }
}
