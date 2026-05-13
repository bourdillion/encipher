use crate::Cipher;
use crate::utils::alphabet::{ALPHABET_SIZE, char_to_index, index_to_char};

pub struct Playfair {
    grid: [[char; 5]; 5],
}

impl Playfair {
    pub fn new(keyword: &str) -> Self {
        let mut seen = [false; 26];
        let mut letters = Vec::with_capacity(25);

        // firstly, add unique keyword letters
        for ch in keyword.chars() {
            let ch = ch.to_ascii_uppercase();
            if !ch.is_ascii_alphabetic() {
                continue;
            }
            // then treat J as I
            let ch = if ch == 'J' { 'I' } else { ch };
            let idx = (ch as u8 - b'A') as usize;
            if !seen[idx] {
                seen[idx] = true;
                letters.push(ch);
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

    fn prepare_digraphs(&self, text: &str) -> Vec<(char, char)> {
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
                i += 1;
            } else {
                digraphs.push((first, second));
                i += 2;
            }
        }

        digraphs
    }

    fn encrypt_pair(&self, a: char, b: char) -> (char, char) {
        let (r1, c1) = self.find_position(a);
        let (r2, c2) = self.find_position(b);

        if r1 == r2 {
            // for same row, shift right
            (self.grid[r1][(c1 + 1) % 5], self.grid[r2][(c2 + 1) % 5])
        } else if c1 == c2 {
            // for same column, shift down
            (self.grid[(r1 + 1) % 5][c1], self.grid[(r2 + 1) % 5][c2])
        } else {
            // for rectangle, swap columns
            (self.grid[r1][c2], self.grid[r2][c1])
        }
    }

    fn decrypt_pair(&self, a: char, b: char) -> (char, char) {
        let (r1, c1) = self.find_position(a);
        let (r2, c2) = self.find_position(b);

        if r1 == r2 {
            // Same row, shift left (add 4 instead of subtracting 1 to avoid underflow)
            (self.grid[r1][(c1 + 4) % 5], self.grid[r2][(c2 + 4) % 5])
        } else if c1 == c2 {
            // Same column, shift up
            (self.grid[(r1 + 4) % 5][c1], self.grid[(r2 + 4) % 5][c2])
        } else {
            // Rectangle, swap columns (same as encrypt)
            (self.grid[r1][c2], self.grid[r2][c1])
        }
    }
}

impl Cipher for Playfair {
    fn encrypt(&self, plaintext: &str) -> String {
        let digraphs = self.prepare_digraphs(plaintext);
        let mut result = String::new();

        for (a, b) in digraphs {
            let (ea, eb) = self.encrypt_pair(a, b);
            result.push(ea);
            result.push(eb);
        }

        result
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let digraphs = self.prepare_digraphs(ciphertext);
        let mut result = String::new();

        for (a, b) in digraphs {
            let (da, db) = self.decrypt_pair(a, b);
            result.push(da);
            result.push(db);
        }

        result
    }
}

//Test implementation for Playfair Cipher

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cipher;

    #[test]
    fn grid_construction() {
        let pf = Playfair::new("MONARCHY");
        assert_eq!(pf.grid[0], ['M', 'O', 'N', 'A', 'R']);
        assert_eq!(pf.grid[1], ['C', 'H', 'Y', 'B', 'D']);
        assert_eq!(pf.grid[2], ['E', 'F', 'G', 'I', 'K']);
        assert_eq!(pf.grid[3], ['L', 'P', 'Q', 'S', 'T']);
        assert_eq!(pf.grid[4], ['U', 'V', 'W', 'X', 'Z']);
    }

    #[test]
    fn same_row_pair() {
        let pf = Playfair::new("MONARCHY");
        // S(3,3) T(3,4) -> same row, shift right -> T L
        assert_eq!(pf.encrypt_pair('S', 'T'), ('T', 'L'));
    }

    #[test]
    fn same_column_pair() {
        let pf = Playfair::new("MONARCHY");
        // M(0,0) C(1,0) -> same col, shift down -> C E
        assert_eq!(pf.encrypt_pair('M', 'C'), ('C', 'E'));
    }

    #[test]
    fn rectangle_pair() {
        let pf = Playfair::new("MONARCHY");
        // R(0,4) U(4,0) -> rectangle -> M Z
        assert_eq!(pf.encrypt_pair('R', 'U'), ('M', 'Z'));
    }

    #[test]
    fn encrypt_rust() {
        let pf = Playfair::new("MONARCHY");
        assert_eq!(pf.encrypt("RUST"), "MZTL");
    }

    #[test]
    fn roundtrip() {
        let pf = Playfair::new("MONARCHY");
        let ciphertext = pf.encrypt("RUST");
        assert_eq!(pf.decrypt(&ciphertext), "RUST");
    }

    #[test]
    fn double_letter_handling() {
        let pf = Playfair::new("MONARCHY");
        // HELLO -> HE LX LO (X inserted between LL)
        let digraphs = pf.prepare_digraphs("HELLO");
        assert_eq!(digraphs, vec![('H', 'E'), ('L', 'X'), ('L', 'O')]);
    }

    #[test]
    fn odd_length_padding() {
        let pf = Playfair::new("MONARCHY");
        // ABC -> AB CX (X padded at end)
        let digraphs = pf.prepare_digraphs("ABC");
        assert_eq!(digraphs, vec![('A', 'B'), ('C', 'X')]);
    }
}
