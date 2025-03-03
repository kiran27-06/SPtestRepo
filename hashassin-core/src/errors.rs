use std::fmt;

#[derive(Debug)]
pub enum HashassinError {
    IoError(std::io::Error),
    InvalidAlgorithm(String),
}

impl fmt::Display for HashassinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HashassinError::IoError(e) => write!(f, "IO error: {}", e),
            HashassinError::InvalidAlgorithm(a) => write!(f, "Invalid algorithm: {}", a),
        }
    }
}

impl From<std::io::Error> for HashassinError {
    fn from(err: std::io::Error) -> Self {
        HashassinError::IoError(err)
    }
}
