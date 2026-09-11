use super::{Byte, Bytes};
use crate::StegError;

pub fn get_n_bytes(bytes: &Bytes, skip_n: usize, n: usize) -> Result<Vec<Byte>, StegError> {
    if bytes.len() < (skip_n + n) {
        return Err(StegError::NotEnoughBits);
    }

    Ok(bytes.iter().skip(skip_n).take(n).copied().collect())
}

pub fn to_ascii(bytes: &Bytes) -> String {
    bytes.iter().map(|byte| *byte as char).collect()
}
