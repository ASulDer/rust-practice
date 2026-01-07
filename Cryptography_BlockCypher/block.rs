use std::fs::File;
use std::env;
use std::io::{BufReader, Read, Write};

// ==================
// |      MAIN      |
// ==================

fn main() -> std::io::Result<()> {
    
    let mut data = encrypt_ECB();
    let _ = write_to_file("ecb_crypto.bmp", data);
    data = encrypt_CBC();
    let _ = write_to_file("cbc_crypto.bmp", data);

    Ok(())
}

fn encrypt_ECB() -> Vec<Vec<u8>> {
    let plain_file = File::open("plain.bmp").unwrap();

    let mut image_reader = BufReader::new(plain_file);
    let mut header_chunk = vec![0u8; 54]; // deklaracja zmiennej naglowka
    image_reader.read_exact(&mut header_chunk).unwrap();    // wczytywanie naglowka

    // dlugosc kluczu = długosc bloku = 64 bajty
    let key: &str = "portas fas fdsghd lfaspqwe sar lsardsrdszualtds dfghlfdg uutrtzb";
    let encryption_key: &[u8] = key.as_bytes();

    let mut image_chunks: Vec<Vec<u8>> = Vec::new();
    let mut chunk_buf: [u8; 64] = [0; 64];

    let metadata = image_reader.get_ref().metadata().unwrap();
    let total_size = metadata.len() - 54;
    let off_bytes = total_size % 8;

    // Wczytywanie danych
    while {
        image_reader.read_exact(&mut chunk_buf);
        image_chunks.push(chunk_buf.to_vec());
        !image_reader.buffer().is_empty()
    } {}

    // Szyfrowanie
    for bytes in image_chunks.iter_mut() {
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte ^= encryption_key[i % encryption_key.len()];
        }
    }

    let mut encrypted_image: Vec<Vec<u8>> = Vec::new();
    encrypted_image.push(header_chunk);
    encrypted_image.append(&mut image_chunks);

    encrypted_image
}

fn encrypt_CBC() -> Vec<Vec<u8>> {
    let plain_file = File::open("plain.bmp").unwrap();

    let mut image_reader = BufReader::new(plain_file);
    let mut header_chunk = vec![0u8; 54]; // deklaracja zmiennej naglowka
    image_reader.read_exact(&mut header_chunk).unwrap();    // wczytywanie naglowka

    // dlugosc kluczu = długosc bloku = 64 bajty
    let key: &str = "portas fas fdsghd lfaspqwe sar lsardsrdszualtds dfghlfdg uutrtzb";
    let mut encryption_key: Vec<u8> = key.as_bytes().to_vec();

    let mut image_chunks: Vec<Vec<u8>> = Vec::new();
    let mut chunk_buf: [u8; 64] = [0; 64];

    let metadata = image_reader.get_ref().metadata().unwrap();
    let total_size = metadata.len() - 54;
    let off_bytes = total_size % 8;

    // Wczytywanie danych + szyfrowanie
    while {
        image_reader.read_exact(&mut chunk_buf);
        for (i, byte) in chunk_buf.iter_mut().enumerate() {
            *byte ^= encryption_key[i % encryption_key.len()];
        }
        encryption_key = chunk_buf.to_vec();
        image_chunks.push(chunk_buf.to_vec());
        !image_reader.buffer().is_empty()
    } {}

    let mut encrypted_image: Vec<Vec<u8>> = Vec::new();
    encrypted_image.push(header_chunk);
    encrypted_image.append(&mut image_chunks);

    encrypted_image
}

fn write_to_file(filename: &str, content: Vec<Vec<u8>>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for bytes in content.iter() {
        file.write_all(bytes.as_slice())?;
    }
    Ok(())
}