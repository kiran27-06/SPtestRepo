use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn generate_passwords(
    chars: u8,
    out_file: Option<String>,
    threads: usize,
    num: usize,
) -> io::Result<()> {
    let num_per_thread = num / threads; // Divide workload
    let remainder = num % threads;

    let passwords = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    for i in 0..threads {
        let passwords = Arc::clone(&passwords);
        let chars = chars as usize;

        let count = if i == 0 {
            num_per_thread + remainder
        } else {
            num_per_thread
        };

        let handle = thread::spawn(move || {
            let mut local_passwords = Vec::new();
            let mut rng = rand::thread_rng();

            for _ in 0..count {
                let password: String = (0..chars)
                    .map(|_| rng.sample(Alphanumeric) as char)
                    .collect();

                local_passwords.push(password);
            }

            let mut shared_passwords = passwords.lock().unwrap();
            shared_passwords.extend(local_passwords);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let passwords = Arc::try_unwrap(passwords).unwrap().into_inner().unwrap();

    if let Some(file) = out_file {
        let mut file = File::create(file)?;
        for password in &passwords {
            writeln!(file, "{}", password)?;
        }
    } else {
        for password in passwords {
            println!("{}", password);
        }
    }

    Ok(())
}
