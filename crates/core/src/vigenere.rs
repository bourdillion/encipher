use crate::Cipher;
use crate::error::CipherError;
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

impl Cipher for Vigenere {
    fn encrypt(&self, plaintext: &str) -> String {
        let mut key_index = 0;

        plaintext
            .chars()
            .map(|c| {
                let is_lowercase = c.is_ascii_lowercase();
                match char_to_index(c) {
                    Some(i) => {
                        let shift = self.key[key_index % self.key.len()] as usize;
                        key_index += 1;
                        let result = (i + shift) % ALPHABET_SIZE;
                        let out = index_to_char(result);
                        if is_lowercase {
                            out.to_ascii_lowercase()
                        } else {
                            out
                        }
                    }
                    None => c,
                }
            })
            .collect()
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let mut key_index = 0;

        ciphertext
            .chars()
            .map(|c| {
                let is_lowercase = c.is_ascii_lowercase();
                match char_to_index(c) {
                    Some(i) => {
                        let shift = self.key[key_index % self.key.len()] as i32;
                        key_index += 1;
                        let result = mod_pos(i as i32 - shift, ALPHABET_SIZE as i32);
                        let out = index_to_char(result);
                        if is_lowercase {
                            out.to_ascii_lowercase()
                        } else {
                            out
                        }
                    }
                    None => c,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_known_output() {
        let cipher = Vigenere::new("CAT");
        assert_eq!(cipher.encrypt("HELLO WORLD"), "JEENO PQREF");
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let cipher = Vigenere::new("SECRET");
        let plain = "The quick brown fox jumps over the lazy dog";
        let encrypted = cipher.encrypt(plain);
        assert_eq!(cipher.decrypt(&encrypted), plain);
    }

    #[test]
    fn preserves_non_alpha() {
        let cipher = Vigenere::new("KEY");
        let plain = "Hello, World! 123";
        let decrypted = cipher.decrypt(&cipher.encrypt(plain));
        assert_eq!(decrypted, plain);
    }

    #[test]
    fn empty_string() {
        let cipher = Vigenere::new("ABC");
        assert_eq!(cipher.encrypt(""), "");
        assert_eq!(cipher.decrypt(""), "");
    }

    #[test]
    fn single_char_key_behaves_like_caesar() {
        // Key "D" means every letter shifts by 3
        let cipher = Vigenere::new("D");
        assert_eq!(cipher.encrypt("ABC"), "DEF");
        assert_eq!(cipher.decrypt("DEF"), "ABC");
    }

    #[test]
    fn key_a_is_identity() {
        // A = 0 shift, so plaintext should come back unchanged
        let cipher = Vigenere::new("A");
        let plain = "Nothing changes here";
        assert_eq!(cipher.encrypt(plain), plain);
    }

    #[test]
    fn key_longer_than_plaintext() {
        let cipher = Vigenere::new("ABCDEFGHIJ");
        let plain = "Hi";
        let decrypted = cipher.decrypt(&cipher.encrypt(plain));
        assert_eq!(decrypted, plain);
    }

    #[test]
    fn wraps_around_z() {
        let cipher = Vigenere::new("Z");
        // Z = 25 shift, so A (0 + 25 = 25) = Z, B (1 + 25 = 0) = A
        assert_eq!(cipher.encrypt("AB"), "ZA");
        assert_eq!(cipher.decrypt("ZA"), "AB");
    }

    #[test]
    fn lowercase_key_works() {
        let upper = Vigenere::new("KEY");
        let lower = Vigenere::new("key");
        let plain = "Hello World";
        assert_eq!(upper.encrypt(plain), lower.encrypt(plain));
    }

    #[test]
    fn keyword_cycles_correctly() {
        // Key "AB" means shift 0, shift 1, shift 0, shift 1...
        let cipher = Vigenere::new("AB");
        // H+0=H, E+1=F, L+0=L, L+1=M, O+0=O
        assert_eq!(cipher.encrypt("HELLO"), "HFLMO");
    }
}
