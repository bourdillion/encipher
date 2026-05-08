use crate::frequency::chi_squared;
use core::Cipher;
use core::affine::Affine;
use core::caesar::Caesar;
use core::utils::alphabet::ALPHABET_SIZE;

pub struct CrackResult {
    pub plaintext: String,
    pub score: f64,
    pub key_description: String,
}

/// Try all 25 Caesar shifts and return results ranked by chi-squared score (best first)
pub fn crack_caesar(ciphertext: &str) -> Vec<CrackResult> {
    let mut results: Vec<CrackResult> = (1..ALPHABET_SIZE as u8)
        .map(|shift| {
            let cipher = Caesar::new(shift);
            let decrypted = cipher.decrypt(ciphertext);
            let score = chi_squared(&decrypted);
            CrackResult {
                plaintext: decrypted,
                score,
                key_description: format!("shift={}", shift),
            }
        })
        .collect();

    results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    results
}

/// Try all 312 Affine key combinations and return results ranked by chi-squared score
pub fn crack_affine(ciphertext: &str) -> Vec<CrackResult> {
    let valid_mul_keys = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];

    let mut results: Vec<CrackResult> = Vec::new();

    for &a in &valid_mul_keys {
        for b in 0..ALPHABET_SIZE as u32 {
            let cipher = Affine::new(a, b).unwrap();
            let decrypted = cipher.decrypt(ciphertext);
            let score = chi_squared(&decrypted);
            results.push(CrackResult {
                plaintext: decrypted,
                score,
                key_description: format!("a={}, b={}", a, b),
            });
        }
    }

    results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cracks_caesar() {
        let cipher = Caesar::new(17);
        let ciphertext = cipher.encrypt("The secret message is hidden inside this text");
        let results = crack_caesar(&ciphertext);
        assert_eq!(results[0].key_description, "shift=17");
    }

    #[test]
    fn cracks_affine() {
        let cipher = Affine::new(5, 8).unwrap();
        let ciphertext = cipher.encrypt("The secret message is hidden inside this text");
        let results = crack_affine(&ciphertext);
        assert_eq!(results[0].key_description, "a=5, b=8");
    }
}
