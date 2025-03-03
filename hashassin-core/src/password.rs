use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::{self, Write};

pub fn generate_passwords(chars: u8, out_file: Option<String>, threads: usize, num: usize) -> io::Result<()> {
    let passwords: Vec<String> = (0..num)
        .map(|_| rand::thread_rng().sample_iter(&Alphanumeric).take(chars as usize).map(char::from).collect())
        .collect();

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
