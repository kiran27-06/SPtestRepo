use clap::{Parser, Subcommand};
use hashassin_core::{dump_hashes, generate_passwords, hash_passwords};
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "hashassin")]
#[command(about = "A simple password hashing tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    //generating random passwords
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
    //generating the hashes
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
    //dumping the hashed passwords
    DumpHashes {
        #[arg(short, long)]
        in_file: String,
    },
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match cli.command {
        Commands::GenPasswords {
            chars,
            out_file,
            threads,
            num,
        } => {
            if chars == 0 || num == 0 || threads == 0 {
                error!("Error: All values must be greater than zero!");
                return;
            }
            if let Err(e) = generate_passwords(chars, out_file.clone(), threads, num) {
                error!("Failed to generate passwords: {:?}", e);
            } else {
                info!("Passwords generated successfully.{:?}", out_file);
            }
        }
        Commands::GenHashes {
            in_file,
            out_file,
            threads,
            algorithm,
        } => {
            if let Err(e) = hash_passwords(
                in_file.clone(),
                out_file.clone(),
                threads,
                algorithm.clone(),
            ) {
                error!("Failed to hash passwords: {:?}", e);
            } else {
                info!(
                    "Hashes generated successfully. Algorithm: {} Output: {}",
                    algorithm, out_file
                );
            }
        }
        Commands::DumpHashes { in_file } => {
            if let Err(e) = dump_hashes(in_file.clone()) {
                error!("Failed to dump hashes: {:?}", e);
            } else {
                info!("Hashes dumped successfully from {}", in_file);
            }
        }
    }
}
