use std::fs::File;
use std::io::{BufReader, BufRead, Write};

// ==================
// |      MAIN      |
// ==================

fn main() -> std::io::Result<()> {
    
    let data = calculate_difference();
    let _ = write_to_file("diff.txt", data);

    Ok(())
}

fn calculate_difference() -> Vec<String> {
    let file_wrapper = File::open("hash.txt");
    let hashes_file = if file_wrapper.is_ok() {
        file_wrapper.unwrap()
    } else {
        let _ = write_to_file("hash.txt", create_hash());
        File::open("hash.txt").unwrap()
    };

    let mut hash_reader = BufReader::new(hashes_file);
    let mut data: Vec<String> = Vec::new();

    // read and compare 2 lines block
    while {
        let mut bits: u64 = 0;
        let mut diff_bits: u64 = 0;
        let mut hash_function = String::new();
        let mut buf_str1 = String::new();
        let mut buf_str2 = String::new();
        
        hash_reader.read_line(&mut buf_str1).expect("Nie dało się przeczytać wierszu.");
        hash_reader.read_line(&mut buf_str2).expect("Nie dało się przeczytać wierszu.");
        
        buf_str1 = buf_str1.split_whitespace().next().unwrap().to_string();
        buf_str2 = buf_str2.split_whitespace().next().unwrap().to_string();
        let buf_str1_bytes = buf_str1.bytes();
        let buf_str2_bytes = buf_str2.bytes();

        for (first_byte, second_byte) in buf_str1_bytes.zip(buf_str2_bytes) {
            for bit_pos in 0..8 {
                if ((first_byte >> bit_pos) & 1) != ((second_byte >> bit_pos) & 1) {
                    diff_bits += 1;
                }
            }
            bits += 4;
        }

        let procent_diff = (diff_bits*100)/bits;

        match bits {
        	128 => hash_function = "md5sum\n".to_string(),
        	160 => hash_function = "sha1sum\n".to_string(),
        	224 => hash_function = "sha224sum\n".to_string(),
        	256 => hash_function = "sha256sum\n".to_string(),
        	384 => hash_function = "sha384sum\n".to_string(),
        	512 => hash_function = "sha512sum/b2sum\n".to_string(),
            _ => hash_function = "Funkcja hashująca jest nieznana dla takiej liczby bitów.\n".to_string(),
        }

        buf_str1 = format!("{buf_str1}\n");
        buf_str2 = format!("{buf_str2}\n");
        data.push(hash_function);
        data.push(buf_str1);
        data.push(buf_str2);
        let results = format!("Liczba różniących się bitów: {diff_bits} z {bits}, procentowo: {procent_diff}%.\n\n");
        data.push(results);

        !hash_reader.buffer().is_empty()
    } {}

    data
}

fn write_to_file(filename: &str, content: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for line in content.iter() {
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}

fn create_hash() -> Vec<String> {
	let mut hash_data: Vec<String> = Vec::new();
	hash_data.push("59e87d5b72daa5f6d6c2f984c93e7537\n".to_string());
	hash_data.push("6536f5acf7b084361842f481d4a3f4be\n".to_string());
	hash_data.push("4a9538e0b4c90137c53a74c4c874673d6ffbb4ff\n".to_string());
	hash_data.push("085d71b326ef162737df74b84c5050fd6296d795\n".to_string());
	hash_data.push("b5cb4322895af3bc95557b0263da2923aeb34fbcdb28c6ea1d249570\n".to_string());
	hash_data.push("3f447962022d589123c36cdc4f403b7440f6069e7552611d139c258a\n".to_string());
	hash_data.push("9b71824acfa408172f7231fc59e3c2a1ec957090b3a14f7e7835ccb1f6680ae6\n".to_string());
	hash_data.push("112b40152333b4ab91c64e80d00c5eb727017d7b6d776e46dfead91da6de40ec\n".to_string());
	hash_data.push("05f41ab59ea3cef43e6dd6ca5b45b4c182efdc0d1c2961b4bae138c58ff6dc6b16c7a5260f295eac3207efd4c094f63c\n".to_string());
	hash_data.push("d322fbe8937c6a69f747ff494861efbdbe16043c30af3acbc9fa72a1cfa6d41ff3d8701ac0e3ff08990c6aef1ed42864\n".to_string());
	hash_data.push("65202b5fea0dddb7ff52eaa65a7fc27fd433f3025a5e2deb7d2f9d1c83f17b9fbf8f79c03ea30ae2893fecb7b4403dd27ed99ab531dd7ef83600ad772322d453\n".to_string());
	hash_data.push("77e1d1be1b2d1b732efd994f514f2963cfcafdb8ccf534282abf35d1a27681e432be32d2cb25dcbda4802447daf21959eea5bf7fcc89e9b675fe6072ae792488\n".to_string());
	hash_data.push("860fef8be6468820ba6046fb121d148c4cb99e10a8d1f499d239ff8eb6199a0f992e01dfd0024f8edbacce1c5bf7785757ac17cb3f29f7895446fdfc2acda59a\n".to_string());
	hash_data.push("ab6252d07dc34f3859e7e9c8fb5d1413e5a8be8ce37e7373b117bcb1fd7d7518f8a2c260174ec6cf04d5c0be77aad681768895da9f6cae3032da659e325fd51c\n".to_string());

	hash_data
}