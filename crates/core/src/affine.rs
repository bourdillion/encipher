use crate::Cipher;
use crate::error::CipherError;
use crate::utils::alphabet::{ALPHABET_SIZE, char_to_index, index_to_char};
use crate::utils::modular::{mod_inverse, mod_pos};

#[derive(Debug)]
pub struct Affine {
    pub mul_key: u32,
    pub add_key: u32,
    pub mul_inv: u32,
}

impl Affine {
    pub fn new(mul_key: u32, add_key: u32) -> Result<Self, CipherError> {
        let mul_inv =
            mod_inverse(mul_key as i32, ALPHABET_SIZE as i32).ok_or(CipherError::InvalidMulKey {
                key: mul_key,
                modulus: ALPHABET_SIZE,
            })? as u32;

        Ok(Affine {
            mul_key,
            add_key,
            mul_inv,
        })
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

    /// decrypt cipher
    fn decrypt(&self, ciphertext: &str) -> String {
        ciphertext
            .chars()
            .map(|c| {
                let is_lowercase = c.is_ascii_lowercase();
                match char_to_index(c) {
                    Some(i) => {
                        let temp = self.mul_inv as i32 * (i as i32 - self.add_key as i32);
                        let result = mod_pos(temp, ALPHABET_SIZE as i32);
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
        let affine = Affine::new(5, 8).unwrap();
        assert_eq!(affine.encrypt("Hello"), "Rclla");
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let affine = Affine::new(5, 8).unwrap();
        let plain = "Hello world, stuff is serious here";
        let cipher = affine.encrypt(plain);
        assert_eq!(affine.decrypt(&cipher), plain);
    }

    #[test]
    fn preserves_non_alpha() {
        let affine = Affine::new(7, 3).unwrap();
        let plain = "Test 123! @#$ end.";
        let decrypted = affine.decrypt(&affine.encrypt(plain));
        assert_eq!(decrypted, plain);
    }

    #[test]
    fn empty_string() {
        let affine = Affine::new(5, 8).unwrap();
        assert_eq!(affine.encrypt(""), "");
        assert_eq!(affine.decrypt(""), "");
    }

    #[test]
    fn mul_key_one_behaves_like_caesar() {
        // a=1 reduces to E(x) = (x + b) mod 26, which is Caesar with shift b
        let affine = Affine::new(1, 3).unwrap();
        assert_eq!(affine.encrypt("ABC"), "DEF");
        assert_eq!(affine.decrypt("DEF"), "ABC");
    }

    #[test]
    fn wraps_around_z() {
        let affine = Affine::new(5, 8).unwrap();
        // Z = 25, so (5 * 25 + 8) mod 26 = 133 mod 26 = 3 = D
        assert_eq!(affine.encrypt("Z"), "D");
        assert_eq!(affine.decrypt("D"), "Z");
    }

    #[test]
    fn all_valid_mul_keys_roundtrip() {
        let valid_keys = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
        let plain = "The quick brown fox jumps over the lazy dog";
        for &a in &valid_keys {
            let affine = Affine::new(a, 10).unwrap();
            let decrypted = affine.decrypt(&affine.encrypt(plain));
            assert_eq!(decrypted, plain, "roundtrip failed for mul_key={}", a);
        }
    }

    #[test]
    fn invalid_mul_key_returns_error() {
        let result = Affine::new(2, 5);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CipherError::InvalidMulKey {
                key: 2,
                modulus: ALPHABET_SIZE
            }
        );
    }

    #[test]
    fn invalid_mul_key_thirteen() {
        assert!(Affine::new(13, 0).is_err());
    }
}
