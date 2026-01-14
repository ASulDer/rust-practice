use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_folder = env::var("OUT_DIR").unwrap();
    let manifest_folder = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    let input_file = Path::new(&manifest_folder).join("src").join("wejscie.txt");
    let dest_dir = Path::new(&out_folder).parent().unwrap().parent().unwrap().parent().unwrap();
    let copied_input = dest_dir.join("wejscie.txt");
    
    if input_file.exists() {
        println!("cargo:warning=Copying {:?} to {:?}", input_file, copied_input);
        fs::copy(&input_file, &copied_input).unwrap_or_else(|err| {
            println!("cargo:warning=Nie dalo sie skopiowac pliku: {}", err);
            0
        });
    } else {
        println!("cargo:warning=Nie znaleziono pliku wejsciowego: {:?}", input_file);
    }
    
    // Re-run if input file changes
    println!("cargo:rerun-if-changed=src/wejscie.txt");
}