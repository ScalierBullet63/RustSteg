use crate::cli::ask_password;
use crate::errors::StegError;
use crate::image::Image;
use crate::payload::{Flags, Payload};

pub fn encode(target_file: &str, hidden_message: String, encrypt: bool) -> Result<(), StegError> {
    //Load image
    let mut image = Image::new();
    match image.load_image(&target_file) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    //Process flags
    let mut flags = Flags::NONE;
    let password: Option<String> = if encrypt {
        flags.insert(Flags::ENCRYPTED);
        Some(ask_password())
    } else {
        None
    };

    //Process payload
    let mut payload = Payload::new(flags);
    match payload.set_hidden_message(hidden_message, password.as_deref()) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    //Process image
    match image.encode(payload) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    match image.save_image() {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    Ok(())
}
