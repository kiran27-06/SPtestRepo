use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::{self, Write};
use std::thread;

pub fn generate_passwords(
    chars: u8,
    out_file: Option<String>,
    threads: usize,
    num: usize,
) -> io::Result<()> {
    let chunk_size = num / threads;
    let mut handles = vec![];

    for _ in 0..threads {
        let handle = thread::spawn(move || {
            (0..chunk_size)
                .map(|_| {
                    rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(chars as usize)
                        .map(char::from)
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
        });
        handles.push(handle);
    }

    let mut passwords = Vec::new();
    for handle in handles {
        passwords.extend(handle.join().unwrap());
    }

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
