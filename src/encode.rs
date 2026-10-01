use std::path::{Path, PathBuf};

use zeroize::Zeroize;

use crate::carrier::{check_carrier_format, check_path_metadata};
use crate::cli::{self, ask_password};
use crate::errors::StegError;
use crate::image::Image;
use crate::payload::{Flags, Payload};

pub fn encode(
    target_file: &Path,
    msg: String,
    output_path: Option<&Path>,
    encrypt: bool,
) -> Result<(), StegError> {
    check_carrier_format(target_file)?;
    check_path_metadata(target_file)?;

    let output_path: PathBuf = match output_path {
        Some(output_path) => output_path.to_path_buf(),
        None => get_default_output(target_file.to_path_buf())?,
    };

    check_carrier_format(&output_path)?;

    if output_path.exists() {
        cli::ask_overwrite(&output_path)?;
    }

    //Load image
    let mut image = Image::new();
    image.load_image(target_file)?;

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
    payload.set_hidden_message(msg, password.as_deref())?;

    password.zeroize();

    //Process image
    image.encode(payload)?;

    image.save_image(&output_path)?;

    Ok(())
}

fn get_default_output(path_buf: PathBuf) -> Result<PathBuf, StegError> {
    let mut path = path_buf;
    let stem = path
        .file_stem()
        .ok_or(StegError::InvalidPath)?
        .to_string_lossy();
    let extension = path
        .extension()
        .ok_or(StegError::InvalidPath)?
        .to_string_lossy();

    path.set_file_name(format!("{stem}_steg.{extension}"));

    Ok(path)
}
