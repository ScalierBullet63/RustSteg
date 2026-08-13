use std::{error::Error, fmt};

#[derive(Debug)]
pub struct NotEnoughBits;

impl Error for NotEnoughBits {}

impl fmt::Display for NotEnoughBits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not enough bits in the image")
    }
}
