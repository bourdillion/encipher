pub mod affine;
pub mod caesar;
pub mod error;
pub mod playfair;
pub mod utils;
pub mod vigenere;

pub trait Cipher {
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
}
