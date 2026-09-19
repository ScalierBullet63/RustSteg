use crate::errors::StegError;
use std::path::Path;

pub fn check_carrier_format(source_path: &str) -> Result<(), StegError> {
    let extension = Path::new(source_path)
        .extension()
        .and_then(|ext| ext.to_str());

    match extension {
        Some("png") | Some("bmp") => Ok(()),
        _ => Err(StegError::UnsupportedCarrierFormat),
    }
}
