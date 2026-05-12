use crate::Cipher;
use crate::utils::alphabet::{ALPHABET_SIZE, char_to_index, index_to_char};

pub struct Playfair {
    grid: [[char; 5]; 5],
}

impl Playfair {
    pub fn new(keyword: &str) -> Self {
        let mut seen = [false; 26];
        let mut letters = Vec::with_capacity(25);

        // Step 1: add unique keyword letters
        for ch in keyword.chars() {
            let ch = ch.to_ascii_uppercase();
            if !ch.is_ascii_alphabetic() {
                continue;
            }
            // Treat J as I
            let ch = if ch == 'J' { 'I' } else { ch };
            let idx = (ch as u8 - b'A') as usize;
            if !seen[idx] {
                seen[idx] = true;
                letters.push(ch);
                // If we just added I, mark J as seen too
                if ch == 'I' {
                    seen[(b'J' - b'A') as usize] = true;
                }
            }
        }

        // Step 2: fill remaining alphabet (skip J entirely)
        for c in b'A'..=b'Z' {
            if c == b'J' {
                continue;
            }
            let idx = (c - b'A') as usize;
            if !seen[idx] {
                seen[idx] = true;
                letters.push(c as char);
            }
        }

        // Step 3: pour into 5x5 grid
        let mut grid = [[' '; 5]; 5];
        for (i, &ch) in letters.iter().enumerate() {
            grid[i / 5][i % 5] = ch;
        }

        Playfair { grid }
    }

    /// Find the (row, col) of a letter in the grid
    fn find_position(&self, ch: char) -> (usize, usize) {
        let ch = if ch == 'J' { 'I' } else { ch };
        for row in 0..5 {
            for col in 0..5 {
                if self.grid[row][col] == ch {
                    return (row, col);
                }
            }
        }
        panic!("Letter '{}' not found in grid", ch);
    }

    /// Split plaintext into digraphs (pairs of two letters)
    fn prepare_digraphs(&self, text: &str) -> Vec<(char, char)> {
        // Collect only alphabetic chars, uppercase, J -> I
        let chars: Vec<char> = text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| {
                let c = c.to_ascii_uppercase();
                if c == 'J' { 'I' } else { c }
            })
            .collect();

        let mut digraphs = Vec::new();
        let mut i = 0;

        while i < chars.len() {
            let first = chars[i];

            if i + 1 >= chars.len() {
                // Odd letter at the end, pad with X
                digraphs.push((first, 'X'));
                break;
            }

            let second = chars[i + 1];

            if first == second {
                // Double letter, insert X as partner
                digraphs.push((first, 'X'));
                i += 1; // only advance by 1, second letter starts next pair
            } else {
                digraphs.push((first, second));
                i += 2;
            }
        }

        digraphs
    }
}
