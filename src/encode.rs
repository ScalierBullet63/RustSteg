use zeroize::Zeroize;

use crate::carrier::check_carrier_format;
use crate::cli::ask_password;
use crate::errors::StegError;
use crate::image::Image;
use crate::payload::{Flags, Payload};

pub fn encode(
    target_file: &str,
    msg: String,
    output_path: Option<String>,
    encrypt: bool,
) -> Result<(), StegError> {
    check_carrier_format(target_file)?;
    //Load image
    let mut image = Image::new();
    match image.load_image(target_file) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    //Process flags
    let mut flags = Flags::NONE;
    let mut password: Option<String> = if encrypt {
        flags.insert(Flags::ENCRYPTED);
        Some(ask_password())
    } else {
        None
    };

    //Process payload
    let mut payload = Payload::new(flags);
    match payload.set_hidden_message(msg, password.as_deref()) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    password.zeroize();

    //Process image
    match image.encode(payload) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    match image.save_image(output_path.as_deref()) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    Ok(())
}
