use std::fmt;

#[derive(Debug)]
pub enum HashassinError {
    IoError(std::io::Error),
    InvalidAlgorithm(String),
    InvalidOutputFormat,
}

impl fmt::Display for HashassinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HashassinError::IoError(e) => write!(f, "IO error: {}", e),
            HashassinError::InvalidAlgorithm(a) => write!(f, "Invalid algorithm: {}", a),
            HashassinError::InvalidOutputFormat => write!(f, "Invalid output file format"),
        }
    }
}

impl From<std::io::Error> for HashassinError {
    fn from(err: std::io::Error) -> Self {
        HashassinError::IoError(err)
    }
}
