
pub mod password;
pub mod hashing;
pub mod errors;

pub use password::generate_passwords;
pub use hashing::{hash_passwords, dump_hashes}; // Fix re-export

