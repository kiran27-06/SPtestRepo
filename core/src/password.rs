use crate::errors::HashassinError;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::{error, info};

pub fn generate_passwords(
    chars: u8,
    out_file: Option<String>,
    threads: usize,
    num: usize,
) -> Result<(), HashassinError> {
    if chars == 0 || threads == 0 || num == 0 {
        return Err(HashassinError::InvalidParameters(
            "Parameters must be greater than zero".into(),
        ));
    }

    let num_per_thread = num / threads;
    let remainder = num % threads;

    let passwords = Arc::new(Mutex::new(Vec::with_capacity(num)));
    let mut handles = Vec::with_capacity(threads);

    for i in 0..threads {
        let passwords = Arc::clone(&passwords);
        let chars = chars as usize;
        let count = num_per_thread + if i < remainder { 1 } else { 0 };

        let handle = thread::spawn(move || {
            let mut local_passwords = Vec::with_capacity(count);
            let mut rng = rand::thread_rng();

            for _ in 0..count {
                let password: String = (0..chars)
                    .map(|_| rng.sample(Alphanumeric) as char)
                    .collect();

                local_passwords.push(password);
            }

            passwords
                .lock()
                .map_err(|_| HashassinError::ThreadLockError)?
                .extend(local_passwords);

            Ok::<(), HashassinError>(())
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join().map_err(|_| HashassinError::ThreadJoinError)? {
            error!("Thread error: {:?}", e);
            return Err(e);
        }
    }

    let passwords = Arc::try_unwrap(passwords)
        .map_err(|_| HashassinError::ThreadLockError)?
        .into_inner()
        .map_err(|_| HashassinError::ThreadLockError)?;

    if let Some(file) = out_file {
        let file = File::create(file)?;
        let mut writer = BufWriter::new(file);
        for password in &passwords {
            writeln!(writer, "{}", password)?;
        }
        info!("Passwords successfully written to file.");
    } else {
        for password in passwords {
            println!("{}", password);
        }
        info!("Passwords successfully printed to stdout.");
    }

    Ok(())
}
