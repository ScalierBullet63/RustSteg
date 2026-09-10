use std::{error::Error, fmt};

#[derive(Debug)]
pub enum StegError {
    NotEnoughBits,
    UnexpectedError,
    NotRustStegFile,
    UnsupportedPayloadVersion,
    InvalidFlags,
    UnsupportedFlag,
    IoError(std::io::Error),
    TryFromSliceError(std::array::TryFromSliceError),
    ImageError(image::ImageError),
    ChaCha20Error(chacha20poly1305::aead::Error),
    Argon2Error(argon2::Error),
}

impl Error for StegError {}

impl fmt::Display for StegError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StegError::NotEnoughBits => write!(f, "Not enough bits in the image"),
            StegError::UnexpectedError => write!(f, "Unexpected error"),
            StegError::NotRustStegFile => {
                write!(f, "This file doesn't seem to be encoded with RustSteg")
            }
            StegError::UnsupportedPayloadVersion => {
                write!(f, "This payload version isn't supported yet")
            }
            StegError::InvalidFlags => write!(f, "Invalid flags"),
            StegError::UnsupportedFlag => write!(f, "Unsupported playlaod flag"),
            StegError::IoError(e) => write!(f, "Io error: {e}"),
            StegError::TryFromSliceError(e) => write!(f, "TryFromVec error: {e}"),
            StegError::ImageError(e) => write!(f, "Image error: {e}"),
            StegError::ChaCha20Error(e) => write!(f, "ChaCha20 error: {e}"),
            StegError::Argon2Error(e) => write!(f, "Argon2 error: {e}"),
        }
    }
}

impl From<std::io::Error> for StegError {
    fn from(error: std::io::Error) -> Self {
        StegError::IoError(error)
    }
}

impl From<image::ImageError> for StegError {
    fn from(error: image::ImageError) -> Self {
        StegError::ImageError(error)
    }
}

impl From<std::array::TryFromSliceError> for StegError {
    fn from(error: std::array::TryFromSliceError) -> Self {
        StegError::TryFromSliceError(error)
    }
}

impl From<chacha20poly1305::Error> for StegError {
    fn from(error: chacha20poly1305::Error) -> Self {
        StegError::ChaCha20Error(error)
    }
}

impl From<argon2::Error> for StegError {
    fn from(error: argon2::Error) -> Self {
        StegError::Argon2Error(error)
    }
}
