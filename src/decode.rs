use crate::errors::StegError;
use crate::image::Image;
// use crate::payload::Payload;

pub fn decode(target_file: String) -> Result<(), StegError> {
    //Load image
    let mut image = Image::new();
    match image.load_image(&target_file) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    match image.get_payload_from_image() {
        Ok(_extracted_payload) => (),
        Err(e) => return Err(e),
    }
    todo!("Decode {target_file}")
}
