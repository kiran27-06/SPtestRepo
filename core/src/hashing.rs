use md5::Md5;
use scrypt::{scrypt, Params};
use sha2::{Digest, Sha256, Sha512};
use std::fs::{File, OpenOptions}; // ✅ Added OpenOptions
use std::io::{self, BufRead, BufReader, Read, Write};
use std::sync::{Arc, Mutex}; // ✅ Added Arc and Mutex
use std::thread; // ✅ Added thread

pub fn hash_passwords(
    in_file: String,
    out_file: String,
    threads: usize,
    algorithm: String,
) -> io::Result<()> {
    let input = File::open(&in_file)?;
    let reader = BufReader::new(input);
    
    let passwords: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let password_length = passwords.first().map_or(0, |p| p.len());

    let passwords = Arc::new(passwords);
    let hashed_passwords = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    let chunk_size = passwords.len() / threads + 1;

    for i in 0..threads {
        let passwords = Arc::clone(&passwords);
        let hashed_passwords = Arc::clone(&hashed_passwords);
        let algo_name = algorithm.clone();

        let handle = thread::spawn(move || {
            let mut local_hashes = Vec::new();
            let start = i * chunk_size;
            let end = std::cmp::min(start + chunk_size, passwords.len());

            for j in start..end {
                let password = &passwords[j];
                let hash = match algo_name.as_str() {
                    "md5" => Md5::digest(password.as_bytes()).to_vec(),
                    "sha256" => Sha256::digest(password.as_bytes()).to_vec(),
                    "sha3-512" => Sha512::digest(password.as_bytes()).to_vec(),
                    "scrypt" => {
                        let mut output = [0u8; 64];
                        let params = Params::new(15, 8, 1).unwrap();
                        scrypt(password.as_bytes(), b"salt", &params, &mut output).unwrap();
                        output.to_vec()
                    }
                    _ => return,
                };
                local_hashes.push(hash);
            }

            let mut global_hashes = hashed_passwords.lock().unwrap();
            global_hashes.extend(local_hashes);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let hashed_passwords = Arc::try_unwrap(hashed_passwords).unwrap().into_inner().unwrap();

    let mut output = OpenOptions::new().write(true).create(true).truncate(true).open(out_file)?;
    output.write_all(&[1])?; // VERSION
    output.write_all(&[algorithm.len() as u8])?; // ALGORITHM LENGTH
    output.write_all(algorithm.as_bytes())?; // ALGORITHM NAME
    output.write_all(&[password_length as u8])?; // PASSWORD LENGTH

    for hash in hashed_passwords {
        output.write_all(&hash)?;  // ✅ Correctly store binary hash data
    }

    Ok(())
}

pub fn dump_hashes(in_file: String) -> io::Result<()> {
    let mut input = File::open(&in_file)?;
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    if buffer.len() < 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid hash file format",
        ));
    }

    let version = buffer[0];
    let algo_length = buffer[1] as usize;
    let algo_name = String::from_utf8(buffer[2..2 + algo_length].to_vec()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid algorithm name encoding",
        )
    })?;
    let password_length = buffer[2 + algo_length];

    println!("VERSION: {}", version);
    println!("ALGORITHM: {}", algo_name);
    println!("PASSWORD LENGTH: {}", password_length);

    let hash_data = &buffer[(3 + algo_length)..];
    let hash_size = match algo_name.as_str() {
        "md5" => 16,
        "sha256" => 32,
        "sha3-512" => 64,
        "scrypt" => 64,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unknown algorithm",
            ))
        }
    };

    for chunk in hash_data.chunks(hash_size) {
        println!("{}", hex::encode(chunk));
    }

    Ok(())
}
