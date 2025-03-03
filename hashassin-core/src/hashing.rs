use md5::Md5;
use sha2::{Sha256, Sha512, digest::Digest}; 


use scrypt::{Params, scrypt};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub fn hash_passwords(in_file: String, out_file: String, _threads: usize, algorithm: String) -> io::Result<()> {
    let input = File::open(in_file)?;
    let reader = BufReader::new(input);
    let mut output = OpenOptions::new().write(true).create(true).truncate(true).open(out_file)?;

    writeln!(output, "VERSION: 1")?;
    writeln!(output, "ALGORITHM: {}", algorithm)?;
    
    for line in reader.lines() {
        let password = line?;
        let hash = match algorithm.as_str() {
            "md5" => format!("{:x}", Md5::digest(password.as_bytes())),
            "sha256" => format!("{:x}", Sha256::digest(password.as_bytes())),
            "sha3-512" => format!("{:x}", Sha512::digest(password.as_bytes())),

            "scrypt" => {
                let mut output = [0u8; 64];
                let params = Params::new(15, 8, 1).unwrap();
                scrypt(password.as_bytes(), b"salt", &params, &mut output);

                hex::encode(output)
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown algorithm")),
        };

        writeln!(output, "{}", hash)?;
    }

    Ok(())
}

pub fn dump_hashes(in_file: String) -> std::io::Result<()> {
    // TODO: Implement hash dumping logic here
    Ok(())
}
