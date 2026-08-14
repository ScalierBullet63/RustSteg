use std::{error::Error, fmt};

#[derive(Debug)]
pub enum StegError {
    NotEnoughBits,
    UnexpectedError,
}

impl Error for StegError {}

impl fmt::Display for StegError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StegError::NotEnoughBits => write!(f, "Not enough bits in the image"),
            StegError::UnexpectedError => write!(f, "Unexpected error"),
        }
    }
}
