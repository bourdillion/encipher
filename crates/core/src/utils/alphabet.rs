pub const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHABET_SIZE: usize = 26;

pub fn char_to_index(c: char) -> Option<usize> {
    let c = c.to_ascii_uppercase();
    ALPHABET.find(c)
}

pub fn index_to_char(i: usize) -> char {
    ALPHABET.as_bytes()[i % ALPHABET_SIZE] as char
}
