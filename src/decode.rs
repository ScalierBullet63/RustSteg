use crate::errors::StegError;
use crate::image::Image;
use crate::payload::Payload;

pub fn decode(target_file: String) -> Result<(), StegError> {
    //Load image
    let mut image = Image::new();
    match image.load_image(&target_file) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    let image_bytes = image.to_bytes();
    let extracted_payload: Payload = Payload::from_bytes(image_bytes)?;

    println!("Extracted payload: {:?}", extracted_payload);
    Ok(())
}
