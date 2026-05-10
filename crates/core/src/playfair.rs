use crate::Cipher;
use crate::error::CipherError;
use crate::utils::alphabet::{ALPHABET_SIZE, char_to_index, index_to_char};

pub struct Playfair {
    pub key: [[char; 5]; 5],
}

