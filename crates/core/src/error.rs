use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CipherError {
    InvalidMulKey { key: u32, modulus: usize },
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherError::InvalidMulKey { key, modulus } => {
                write!(
                    f,
                    "multiplicative key {} is not coprime with {}. Valid keys are: 1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25",
                    key, modulus
                )
            }
        }
    }
}

impl std::error::Error for CipherError {}
