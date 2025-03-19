use std::fmt;

#[derive(Debug)]
pub enum HashassinError {
    IoError(std::io::Error),
    InvalidAlgorithm(String),
    InvalidOutputFormat,
    InvalidParameters(String),
    ThreadLockError,
    ThreadJoinError,
}

impl fmt::Display for HashassinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HashassinError::IoError(e) => write!(f, "IO error: {}", e),
            HashassinError::InvalidAlgorithm(a) => write!(f, "Invalid algorithm: {}", a),
            HashassinError::InvalidOutputFormat => write!(f, "Invalid output file format"),
            HashassinError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            HashassinError::ThreadLockError => write!(f, "Thread lock error"),
            HashassinError::ThreadJoinError => write!(f, "Thread join error"),
        }
    }
}

impl From<std::io::Error> for HashassinError {
    fn from(err: std::io::Error) -> Self {
        HashassinError::IoError(err)
    }
}
