use crate::errors::StegError;
use crate::image::Image;
use crate::payload::{Flags, Payload};

pub fn encode(target_file: String, hidden_message: String, encrypt: bool) -> Result<(), StegError> {
    println!("Selected {}", target_file);

    //Load image
    let mut image = Image::new();
    match image.load_image(&target_file) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    //Process flags
    let mut flags = Flags::NONE;
    if encrypt {
        flags.insert(Flags::ENCRYPTED);
    }

    //Process payload
    let mut payload = Payload::new(flags);
    match payload.set_hidden_message(hidden_message) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    //Process image
    match image.insert_hidden_message(payload) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    match image.save_image() {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    Ok(())
}
