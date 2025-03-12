use md5::Md5;
use scrypt::{scrypt, Params};
use sha2::{Digest, Sha256, Sha512};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub fn hash_passwords(
    in_file: String,
    out_file: String,
    _threads: usize,
    algorithm: String,
) -> io::Result<()> {
    let input = File::open(&in_file)?;
    let reader = BufReader::new(input);
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Ensure file is overwritten
        .open(&out_file)?;

    let algo_name = algorithm.to_lowercase();
    let mut passwords: Vec<String> = Vec::new();
    let mut expected_length: Option<usize> = None;

    // Read passwords and enforce length consistency
    for line in reader.lines() {
        let password = line?.trim().to_string(); // Trim spaces & newlines

        // Set expected length based on first password
        match expected_length {
            None => {
                expected_length = Some(password.len());
                println!("✅ Debug: Detected password length: {}", password.len());
            }
            Some(len) => {
                if password.len() != len {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "❌ Error: Password length mismatch in `{}` (Expected: {}, Found: {})",
                            in_file,
                            len,
                            password.len()
                        ),
                    ));
                }
            }
        }

        passwords.push(password);
    }

    let password_length = expected_length.unwrap_or(0);

    // ✅ Write in the **expected human-readable format**
    writeln!(output, "VERSION: 1")?;
    writeln!(output, "ALGORITHM: {}", algo_name)?;
    writeln!(output, "PASSWORD LENGTH: {}", password_length)?;

    for password in passwords {
        let hash = match algo_name.as_str() {
            "md5" => format!("{:x}", Md5::digest(password.as_bytes())),
            "sha256" => format!("{:x}", Sha256::digest(password.as_bytes())),
            "sha3-512" => format!("{:x}", Sha512::digest(password.as_bytes())),
            "scrypt" => {
                let mut output = [0u8; 64];
                let params = Params::new(15, 8, 1).unwrap();
                scrypt(password.as_bytes(), b"salt", &params, &mut output)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "Scrypt hashing failed"))?;

                hex::encode(output)
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unknown algorithm",
                ));
            }
        };

        writeln!(output, "{}", hash)?;
    }

    Ok(())
}

pub fn dump_hashes(in_file: String) -> std::io::Result<()> {
    let input = File::open(&in_file)?;
    let reader = BufReader::new(input);

    let mut lines = reader.lines();

    // Read and print required headers
    if let Some(Ok(version_line)) = lines.next() {
        println!("{}", version_line);
    }
    if let Some(Ok(algo_line)) = lines.next() {
        println!("{}", algo_line);
    }
    if let Some(Ok(length_line)) = lines.next() {
        println!("{}", length_line);
    }

    // Read and print hashes line by line
    for line in lines {
        let hash = line?;
        println!("{}", hash);
    }

    Ok(())
}
