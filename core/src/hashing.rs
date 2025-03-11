use md5::Md5;
use scrypt::{scrypt, Params};
use sha2::{Digest, Sha256, Sha512};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};

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
        .truncate(true)
        .open(&out_file)?;

    let algo_name = algorithm.to_lowercase();
    let algo_bytes = algo_name.as_bytes();
    let algo_len = algo_bytes.len() as u8;

    let mut passwords: Vec<String> = Vec::new();
    let mut expected_length: Option<usize> = None;

    // Read passwords and enforce length consistency
    for line in reader.lines() {
        let password = line?.trim().to_string(); // Trim spaces & newlines

        // Set expected length based on first password
        match expected_length {
            None => {
                expected_length = Some(password.len());
                println!("✅ Debug: Detected password length: {}", password.len()); // Debug
            }
            Some(len) => {
                if password.len() != len {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "❌ Error: Password length mismatch in `{}` (Expected: {}, Found: {})",
                            in_file, len, password.len()
                        ),
                    ));
                }
            }
        }

        passwords.push(password);
    }

    let password_length = expected_length.unwrap_or(0) as u8;

    // Writing output format header
    output.write_all(&[1])?; // Version
    output.write_all(&[algo_len])?; // Algorithm length
    output.write_all(algo_bytes)?;
    output.write_all(&[password_length])?; // ✅ No newline after password length

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
    let mut reader = BufReader::new(input);

    let mut version = [0u8; 1];
    let mut algo_len = [0u8; 1];

    // Read version
    reader.read_exact(&mut version)?;

    // Read algorithm length
    reader.read_exact(&mut algo_len)?;

    // Read algorithm name
    let mut algo_name = vec![0u8; algo_len[0] as usize];
    reader.read_exact(&mut algo_name)?;

    // Read password length
    let mut password_length = [0u8; 1];
    reader.read_exact(&mut password_length)?;

    println!("VERSION: {}", version[0]);
    println!("ALGORITHM: {}", String::from_utf8_lossy(&algo_name));
    println!("PASSWORD LENGTH: {}", password_length[0]);

    // Read and print hash data line by line
    for line in reader.lines() {
        let hash = line?;
        println!("{}", hash);
    }

    Ok(())
}
