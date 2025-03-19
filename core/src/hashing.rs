use crate::errors::HashassinError;
use md5::Md5;
use scrypt::{scrypt, Params};
use sha2::{Digest, Sha256, Sha512};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::{error, info};

pub fn hash_passwords(
    in_file: String,
    out_file: String,
    threads: usize,
    algorithm: String,
) -> Result<(), HashassinError> {
    let input = File::open(&in_file)?;
    let reader = BufReader::new(input);

    let passwords: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    if passwords.is_empty() {
        return Err(HashassinError::InvalidParameters(
            "Input file is empty".into(),
        ));
    }

    let password_length = passwords[0].len();
    if passwords.iter().any(|p| p.len() != password_length) {
        return Err(HashassinError::InvalidOutputFormat);
    }

    let passwords = Arc::new(passwords);
    let hashed_passwords = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    let chunk_size = passwords.len() / threads + 1;

    for i in 0..threads {
        let passwords = Arc::clone(&passwords);
        let hashed_passwords = Arc::clone(&hashed_passwords);
        let algo_name = algorithm.clone();

        let handle = thread::spawn(move || -> Result<(), HashassinError> {
            let start = i * chunk_size;
            let end = std::cmp::min(start + chunk_size, passwords.len());

            let mut local_hashes = Vec::with_capacity(end - start);
            for password in &passwords[start..end] {
                let hash = match algo_name.as_str() {
                    "md5" => Md5::digest(password.as_bytes()).to_vec(),
                    "sha256" => Sha256::digest(password.as_bytes()).to_vec(),
                    "sha3-512" => Sha512::digest(password.as_bytes()).to_vec(),
                    "scrypt" => {
                        let mut output = [0u8; 64];
                        let params = Params::new(15, 8, 1)
                            .map_err(|_| HashassinError::InvalidAlgorithm(algo_name.clone()))?;
                        scrypt(password.as_bytes(), b"salt", &params, &mut output)
                            .map_err(|_| HashassinError::InvalidAlgorithm(algo_name.clone()))?;
                        output.to_vec()
                    }
                    _ => return Err(HashassinError::InvalidAlgorithm(algo_name.clone())),
                };
                local_hashes.push(hash);
            }

            hashed_passwords
                .lock()
                .map_err(|_| HashassinError::ThreadLockError)?
                .extend(local_hashes);

            Ok(())
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join().map_err(|_| HashassinError::ThreadJoinError)? {
            error!("Thread error: {:?}", e);
            return Err(e);
        }
    }

    let hashed_passwords = Arc::try_unwrap(hashed_passwords)
        .map_err(|_| HashassinError::ThreadLockError)?
        .into_inner()
        .map_err(|_| HashassinError::ThreadLockError)?;

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out_file)?;

    output.write_all(&[1])?; //version
    output.write_all(&[algorithm.len() as u8])?; //Algo len
    output.write_all(algorithm.as_bytes())?; // Algo name
    output.write_all(&[password_length as u8])?; //password len

    for hash in hashed_passwords {
        output.write_all(&hash)?;
    }

    info!("Hashes successfully written to file.");

    Ok(())
}

pub fn dump_hashes(in_file: String) -> Result<(), HashassinError> {
    let mut input = File::open(&in_file)?;
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    if buffer.len() < 3 {
        return Err(HashassinError::InvalidOutputFormat);
    }

    let version = buffer[0];
    let algo_length = buffer[1] as usize;
    let algo_name = String::from_utf8(buffer[2..2 + algo_length].to_vec())
        .map_err(|_| HashassinError::InvalidOutputFormat)?;
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
        _ => return Err(HashassinError::InvalidAlgorithm(algo_name)),
    };

    for chunk in hash_data.chunks(hash_size) {
        println!("{}", hex::encode(chunk));
    }

    info!("Hashes successfully dumped from file.");

    Ok(())
}
