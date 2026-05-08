use clap::{Parser, Subcommand};
use core::Cipher;
use core::affine::Affine;
use core::caesar::Caesar;
use core::vigenere::Vigenere;

#[derive(Parser)]
#[command(name = "encipher")]
#[command(about = "A Rust implementation of classical cryptographic ciphers")]
struct Cli {
    #[command(subcommand)]
    cipher: CipherCommand,
}

#[derive(Subcommand)]
enum CipherCommand {
    /// Caesar cipher with a single shift value
    Caesar {
        #[arg(short, long)]
        shift: u8,

        #[arg(short, long)]
        decrypt: bool,

        /// The text to encrypt or decrypt
        text: String,
    },

    /// Affine cipher with multiplicative and additive keys
    Affine {
        #[arg(short, long)]
        mul_key: u32,

        #[arg(short, long)]
        add_key: u32,

        #[arg(short, long)]
        decrypt: bool,

        text: String,
    },

    /// Vigenere cipher with a keyword
    Vigenere {
        #[arg(short, long)]
        key: String,

        #[arg(short, long)]
        decrypt: bool,

        text: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let output = match cli.cipher {
        CipherCommand::Caesar {
            shift,
            decrypt,
            text,
        } => {
            let cipher = Caesar::new(shift);
            if decrypt {
                cipher.decrypt(&text)
            } else {
                cipher.encrypt(&text)
            }
        }

        CipherCommand::Affine {
            mul_key,
            add_key,
            decrypt,
            text,
        } => match Affine::new(mul_key, add_key) {
            Ok(cipher) => {
                if decrypt {
                    cipher.decrypt(&text)
                } else {
                    cipher.encrypt(&text)
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },

        CipherCommand::Vigenere { key, decrypt, text } => {
            let cipher = Vigenere::new(&key);
            if decrypt {
                cipher.decrypt(&text)
            } else {
                cipher.encrypt(&text)
            }
        }
    };

    println!("{}", output);
}
