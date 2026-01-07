use std::fs::File;
use std::fmt::*;
use std::env;
use std::io::{self, BufReader, BufRead, Read, Write};

// ==================
// |     STRUCTS    |
// ==================

enum Cipher {
    Caesar,
    Affinic,
    None,
}

enum Mode {
    Encryption,
    Decryption,
    Plaintext,
    Ciphertext,
    None,
}

// ==================
// |      MAIN      |
// ==================

fn main() -> std::io::Result<()> {
    let mut cipher_type: Cipher = Cipher::None;
    let mut mode: Mode = Mode::None;
    for argument in env::args().skip(1) {
        match argument.as_str() {
            "-c" => cipher_type = Cipher::Caesar,
            "-a" => cipher_type = Cipher::Affinic,
            "-e" => mode = Mode::Encryption,
            "-d" => mode = Mode::Decryption,
            "-j" => mode = Mode::Plaintext,         // Plaintext == Jawnytekst
            "-k" => mode = Mode::Ciphertext,        // Ciphertext == Kryptogram
            _ => panic!("Nie ma takiej opcji!"),
        }
    }

    match mode {
        Mode::Encryption => {
            let text = encrypt(cipher_type);
            write_to_file("crypto.txt", &text);
        },

        Mode::Decryption => {
            let text = decrypt(cipher_type);
            write_to_file("plain.txt", &text);
        },

        Mode::Plaintext => {
            let (text, analysis_key) = analysis_plaintext(cipher_type);
            write_to_file("decrypt.txt", &text);
            write_to_file("key-found.txt", &analysis_key);
        },

        Mode::Ciphertext => {
            let text = analysis_ciphertext(cipher_type);
            write_to_file("decrypt.txt", &text);
        }

        Mode::None => panic!("Nie podano trybu działania programu."),
    }

    Ok(())
}

fn encrypt(cipher_type: Cipher) -> String {
    let plain_file = File::open("plain.txt").unwrap();
    let key_file = File::open("key.txt").unwrap();

    let mut text_reader = BufReader::new(plain_file);
    let mut key_reader = BufReader::new(key_file);
    let mut buffer_str = String::new();

    match cipher_type {
        Cipher::Caesar => {
            key_reader.read_line(&mut buffer_str).expect("Nie dało się przeczytać kluczu.");
            let shift: u8 = buffer_str.trim().split_whitespace().next().unwrap().parse().expect("Nie dało się sparsować liczby.");
            if 1 > shift && shift > 25 {
                panic!("Niepoprawny zakres kluczu");
            }

            buffer_str.clear();
            text_reader.read_line(&mut buffer_str).expect("Nie dało się przeczytać tekstu jawnego.");

            buffer_str.chars().map(|c| {
                if c.is_ascii_uppercase() {
                    let base = 'A' as u8;
                    let shifted = ((c as u8 - base + shift) % 26) + base;
                    shifted as char
                } else if c.is_ascii_lowercase() {
                    let base = 'a' as u8;
                    let shifted = ((c as u8 - base + shift) % 26) + base;
                    shifted as char
                } else {
                    c
                }
            }).collect()
        },

        Cipher::Affinic => {
            key_reader.read_line(&mut buffer_str).expect("Nie dało się przeczytać kluczu.");
            let mut key: Vec<u8> = Vec::with_capacity(2);         // key[0] - shift (b), key[1] - koef. (a)
            key = buffer_str.trim().split_whitespace().map(|c| {c as u8}).collect();
            if 1 > key[0] && key[0] > 25 {
                panic!("Niepoprawny zakres kluczu");
            }

            buffer_str.clear();
            text_reader.read_line(&mut buffer_str).expect("Nie dało się przeczytać tekstu jawnego.");

            buffer_str.chars().map(|c| {
                if c.is_ascii_uppercase() {
                    let base = 'A' as u8;
                    let shifted = (((c as u8 - base)*key[1] + key[0]) % 26) + base;
                    shifted as char
                } else if c.is_ascii_lowercase() {
                    let base = 'a' as u8;
                    let shifted = (((c as u8 - base)*key[1] + key[0]) % 26) + base;
                    shifted as char
                } else {
                    c
                }
            }).collect()
        },
        
        Cipher::None => panic!("Nie podano typu szyfru."),
    }
}

fn decrypt(cipher_type: Cipher) -> String {
    let encrypted_file = File::open("crypto.txt").unwrap();
    let key_file = File::open("key.txt").unwrap();

    let mut text_reader = BufReader::new(encrypted_file);
    let mut key_reader = BufReader::new(key_file);
    let mut buffer_str = String::new();

    match cipher_type {
        Cipher::Caesar => {
            key_reader.read_line(&mut buffer_str).expect("Nie dało się przeczytać kluczu.");
            let shift: u8 = buffer_str.trim().split_whitespace().next().unwrap().parse().expect("Nie dało się sparsować liczby przesunięcia.");
            if 1 > shift && shift > 25 {
                panic!("Niepoprawny zakres kluczu");
            }

            buffer_str.clear();
            text_reader.read_line(&mut buffer_str).expect("Nie dało się przeczytać tekstu zaszyfrowanego.");

            buffer_str.chars().map(|c| {
                if c.is_ascii_uppercase() {
                    let base = 'A' as u8;
                    let shifted = ((c as u8 - base + ((26 - shift) % 26)) % 26) + base;
                    shifted as char
                } else if c.is_ascii_lowercase() {
                    let base = 'a' as u8;
                    let shifted = ((c as u8 - base + ((26 - shift) % 26)) % 26) + base;
                    shifted as char
                } else {
                    c
                }
            }).collect()
        },

        Cipher::Affinic => panic!("Not implemented!"),
        
        Cipher::None => panic!("Nie podano typu szyfru."),
    }
}

fn analysis_plaintext(cipher_type: Cipher) -> (String, String) {
    let encrypted_file = File::open("crypto.txt").unwrap();
    let help_file = File::open("extra.txt").unwrap();

    let mut text_reader = BufReader::new(encrypted_file);
    let mut extra_reader = BufReader::new(help_file);
    let mut text_buffer = String::new();
    let mut extra_buffer = String::new();

    match cipher_type {
        Cipher::Caesar => {
            text_reader.read_line(&mut text_buffer).expect("Nie dało się przeczytać tekstu zaszyfrowanego.");
            let encrypted_letter: u8 = text_buffer.chars().next().unwrap() as u8;
            extra_reader.read_line(&mut extra_buffer).expect("Nie dało się przeczytać tekstu pomocniczego.");
            let plain_letter: u8 = extra_buffer.chars().next().unwrap() as u8;

            let shift: u8 = (encrypted_letter + 26 - plain_letter) % 26;
            let text: String = text_buffer.chars().map(|c| {
                if c.is_ascii_uppercase() {
                    let base = 'A' as u8;
                    let shifted = ((c as u8 - base + ((26 - shift) % 26)) % 26) + base;
                    shifted as char
                } else if c.is_ascii_lowercase() {
                    let base = 'a' as u8;
                    let shifted = ((c as u8 - base + ((26 - shift) % 26)) % 26) + base;
                    shifted as char
                } else {
                    c
                }
            }).collect();
            
            let found_key: String = shift.to_string();
            let return_args_tuple: (String, String) = (text, found_key);
            return_args_tuple
        },

        Cipher::Affinic => panic!("Not implemented!"),
        
        Cipher::None => panic!("Nie podano typu szyfru."),
    }
}

// | "Bruteforce"
fn analysis_ciphertext(cipher_type: Cipher) -> String {
    let encrypted_file = File::open("crypto.txt").unwrap();

    let mut text_reader = BufReader::new(encrypted_file);
    let mut buffer_str = String::new();

    match cipher_type {
        Cipher::Caesar => {
            let mut brutforce_decryption: String = String::from("");
            text_reader.read_line(&mut buffer_str).expect("Nie dało się przeczytać tekstu zaszyfrowanego.");
            for shift in 1..27 {
                let mut version: String = buffer_str.chars().map(|c| {
                    if c.is_ascii_uppercase() {
                        let base = 'A' as u8;
                        let shifted = ((c as u8 - base + ((26 - shift) % 26)) % 26) + base;
                        shifted as char
                    } else if c.is_ascii_lowercase() {
                        let base = 'a' as u8;
                        let shifted = ((c as u8 - base + ((26 - shift) % 26)) % 26) + base;
                        shifted as char
                    } else {
                        c
                    }
                }).collect();
                brutforce_decryption = brutforce_decryption + &version + "\n";
            };
            brutforce_decryption
        },

        Cipher::Affinic => panic!("Not implemented!"),
        
        Cipher::None => panic!("Nie podano typu szyfru."),
    }
}

fn write_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}