use crate::Cipher;
use crate::utils::alphabet::{ALPHABET_SIZE, char_to_index, index_to_char};
use crate::utils::modular::mod_pos;
/*
The Caesar cipher is a substitution cipher that shifts each letter in the plaintext by a fixed number of positions in the alphabet (e.g., shift of 3 turns 'A' to 'D').
*/
pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new(shift: u8) -> Self {
        let mod_shift = shift % ALPHABET_SIZE as u8;
        Caesar { shift: mod_shift }
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, plaintext: &str) -> String {
        plaintext
            .chars()
            .map(|c| {
                let is_lower = c.is_ascii_lowercase();
                match char_to_index(c) {
                    Some(i) => {
                        let shifted = (i + self.shift as usize) % ALPHABET_SIZE;
                        let out = index_to_char(shifted);
                        if is_lower {
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
        ciphertext
            .chars()
            .map(|c| {
                let is_lower = c.is_ascii_lowercase();
                match char_to_index(c) {
                    Some(i) => {
                        let temp: i32 = i as i32 - self.shift as i32;
                        let shifted = mod_pos(temp, ALPHABET_SIZE as i32);
                        let out = index_to_char(shifted);
                        if is_lower {
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

/* Test implementation for Caesar Cipher */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let cipher = Caesar::new(3);
        let plain = "Hello, World!";
        let encrypted = cipher.encrypt(plain);
        assert_eq!(encrypted, "Khoor, Zruog!");
        assert_eq!(cipher.decrypt(&encrypted), plain);
    }

    #[test]
    fn shift_wraps_around() {
        let cipher = Caesar::new(25);
        assert_eq!(cipher.encrypt("B"), "A");
        assert_eq!(cipher.decrypt("A"), "B");
    }
}
