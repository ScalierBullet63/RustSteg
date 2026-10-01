use rpassword::prompt_password;
use std::{
    io::{self, Write},
    path::Path,
};

use crate::errors::StegError;

pub fn ask_password() -> String {
    let password = prompt_password("Enter the password: ").unwrap();

    password.trim().to_string()
}

pub fn ask_overwrite(path: &Path) -> Result<(), StegError> {
    print!(
        "The file {} already exists. Do you want to overwrite it? [y/N]: ",
        path.display()
    );
    io::stdout().flush().unwrap();

    let mut yes_or_no = String::new();
    io::stdin().read_line(&mut yes_or_no)?;

    if !yn::yes(yes_or_no) {
        return Err(StegError::NoOverWrite);
    }

    Ok(())
}
