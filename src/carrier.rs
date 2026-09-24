use crate::errors::StegError;
use std::path::Path;

pub fn check_carrier_format(source_path: &str) -> Result<(), StegError> {
    let path = Path::new(source_path);

    if let Err(error) = std::fs::metadata(path) {
        if error.kind() == std::io::ErrorKind::NotFound {
            return Err(StegError::PathNotFound);
        }
        return Err(error.into());
    }

    let extension = path.extension().and_then(|ext| ext.to_str());

    match extension {
        Some("png") | Some("bmp") => Ok(()),
        _ => Err(StegError::UnsupportedCarrierFormat),
    }
}
