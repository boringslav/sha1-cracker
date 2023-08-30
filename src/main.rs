use std::{env, error::Error, fs::File, io::BufRead, io::BufReader};
use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <hash>");
    }

    let hash_to_crack: &str = args[2].trim();

    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlist_file: File = File::open(&args[1])?;
    let reader: BufReader<File> = BufReader::new(wordlist_file);

    for line in reader.lines() {
        let password: String = line?.trim().to_string();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(password.as_bytes())) {
            println!("Password found: {}", &password);
            return Ok(());
        }
    }

    println!("Password not found");
    
    Ok(())
}
