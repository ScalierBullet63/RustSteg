use crate::cli::ask_password;
use crate::errors::StegError;
use crate::image::Image;
use crate::payload::{Flags, Payload};

pub fn decode(target_file: &str) -> Result<(), StegError> {
    //Load image
    let mut image = Image::new();
    match image.load_image(target_file) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    let image_bytes = image.to_bytes();
    let mut extracted_payload: Payload = Payload::from_bytes(image_bytes)?;

    if extracted_payload.flags().contains(Flags::ENCRYPTED) {
        extracted_payload.decrypt(&ask_password())?;
    };

    println!("Extracted payload: {:?}", extracted_payload);
    Ok(())
}
