use clap::{Parser, Subcommand};
use hashassin_core::{dump_hashes, generate_passwords, hash_passwords};

#[derive(Parser)]
#[command(name = "hashassin")]
#[command(about = "A simple password hashing tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate random passwords
    GenPasswords {
        #[arg(short, long, default_value_t = 4)]
        chars: u8,
        #[arg(short, long)]
        out_file: Option<String>,
        #[arg(short, long, default_value_t = 1)]
        threads: usize,
        #[arg(short, long)]
        num: usize,
    },
    /// Generate password hashes
    GenHashes {
        #[arg(short, long)]
        in_file: String,
        #[arg(short, long)]
        out_file: String,
        #[arg(short, long, default_value_t = 1)]
        threads: usize,
        #[arg(short, long)]
        algorithm: String,
    },
    /// Dump hashed passwords
    DumpHashes {
        #[arg(short, long)]
        in_file: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::GenPasswords {
            chars,
            out_file,
            threads,
            num,
        } => {
            if chars == 0 || num == 0 || threads == 0 {
                eprintln!("Error: All values must be greater than zero!");
                return;
            }
            generate_passwords(chars, out_file, threads, num)
                .expect("Failed to generate passwords");
        }
        Commands::GenHashes {
            in_file,
            out_file,
            threads,
            algorithm,
        } => {
            hash_passwords(in_file, out_file, threads, algorithm)
                .expect("Failed to hash passwords");
        }
        Commands::DumpHashes { in_file } => {
            dump_hashes(in_file).expect("Failed to dump hashes");
        }
    }
}
