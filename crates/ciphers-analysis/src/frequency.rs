use core::utils::alphabet::{ALPHABET, ALPHABET_SIZE, char_to_index, index_to_char};

// English alphabet frequency from a to z
pub const ENGLISH_FREQ: [f64; 26] = [
    8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 6.966, 0.153, 0.772, 4.025, 2.406,
    6.749, 7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758, 0.978, 2.360, 0.150, 1.974, 0.074,
];

/// Count how many times each letter appears in the text sent
/// Returns an array of the 26 letters with how many times they appear
pub fn letter_counts(text: &str) -> [usize; ALPHABET_SIZE] {
    let mut counts = [0usize; ALPHABET_SIZE];
    for c in text.chars() {
        if let Some(i) = char_to_index(c) {
            counts[i] += 1;
        }
    }
    counts
}

/// Convert the count to percentage
pub fn letter_frequencies(text: &str) -> [f64; ALPHABET_SIZE] {
    let counts = letter_counts(text);
    let total: usize = counts.iter().sum();

    let mut freqs = [0.0f64; ALPHABET_SIZE];
    if total == 0 {
        return freqs;
    }

    for i in 0..ALPHABET_SIZE {
        freqs[i] = (counts[i] as f64 / total as f64) * 100.0;
    }
    freqs
}

/// Chi-squared score is a means of comparing observed text frequencies against english letters
pub fn chi_squared(text: &str) -> f64 {
    let counts = letter_counts(text);
    let total: usize = counts.iter().sum();

    if total == 0 {
        return f64::MAX;
    }

    let mut score = 0.0;
    for i in 0..ALPHABET_SIZE {
        let expected = ENGLISH_FREQ[i] / 100.0 * total as f64;
        if expected > 0.0 {
            let diff = counts[i] as f64 - expected;
            score += (diff * diff) / expected;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_basic() {
        let counts = letter_counts("AABBC");
        assert_eq!(counts[0], 2); // A
        assert_eq!(counts[1], 2); // B
        assert_eq!(counts[2], 1); // C
        assert_eq!(counts[3], 0); // D
    }

    #[test]
    fn counts_ignores_non_alpha() {
        let counts = letter_counts("A B! C?");
        assert_eq!(counts[0], 1);
        assert_eq!(counts[1], 1);
        assert_eq!(counts[2], 1);
    }

    #[test]
    fn counts_case_insensitive() {
        let counts = letter_counts("aAbB");
        assert_eq!(counts[0], 2);
        assert_eq!(counts[1], 2);
    }

    #[test]
    fn frequencies_sum_to_100() {
        let freqs = letter_frequencies("The quick brown fox jumps over the lazy dog");
        let sum: f64 = freqs.iter().sum();
        assert!((sum - 100.0).abs() < 0.01);
    }

    #[test]
    fn empty_text_returns_zero() {
        let freqs = letter_frequencies("");
        assert!(freqs.iter().all(|&f| f == 0.0));
    }

    #[test]
    fn chi_squared_english_text_scores_low() {
        let english = "The quick brown fox jumps over the lazy dog and the cow";
        let gibberish = "ZZZQQJJJXXXKKK";
        assert!(chi_squared(english) < chi_squared(gibberish));
    }
}
