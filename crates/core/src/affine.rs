use crate::Cipher;
use crate::utils::alphabet::{ALPHABET_SIZE, char_to_index, index_to_char};
use crate::utils::modular::{mod_inverse, mod_pos};
pub struct Affine {
    pub mul_key: u32,
    pub add_key: u32,
    pub mul_inv: u32,
}

impl Affine {
    pub fn new(mul_key: u32, add_key: u32) -> Self {
        let mul_inv = mod_inverse(mul_key as i32, ALPHABET_SIZE as i32).unwrap() as u32;
        Affine {
            mul_key,
            add_key,
            mul_inv,
        }
    }
}

impl Cipher for Affine {
    fn encrypt(&self, plaintext: &str) -> String {
        plaintext
            .chars()
            .map(|c| {
                let is_lowercase = c.is_ascii_lowercase();
                match char_to_index(c) {
                    Some(i) => {
                        // recall affine cipher formular to encrypt is mul_key * c + add_key
                        let result: usize =
                            (self.mul_key as usize * i + self.add_key as usize) % ALPHABET_SIZE;
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
        ciphertext
            .chars()
            .map(|c| {
                let is_lowercase = c.is_ascii_lowercase();
                match char_to_index(c) {
                    Some(i) => {
                        // recall affine cipher formular to decrypt is mul_inv * (i - add_key)
                        let temp = self.mul_inv as i32 * (i as i32 - self.add_key as i32);
                        let result = mod_pos(temp as i32, ALPHABET_SIZE as i32);
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
    fn test_encryption_works() {
        let affine = Affine::new(5, 8);
        let plain_text = "Hello";
        let expected_result = "Rclla";
        let result = affine.encrypt(plain_text);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let affine = Affine::new(5, 8);
        let plain_text = "Hello world, stuff is serious here";
        let cipher = affine.encrypt(plain_text);
        let result = affine.decrypt(cipher.as_str());

        assert_eq!(plain_text, result);
    }
}
