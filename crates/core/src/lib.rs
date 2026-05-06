pub mod affine;
pub mod caesar;
pub mod error;
pub mod utils;

pub trait Cipher {
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
}
