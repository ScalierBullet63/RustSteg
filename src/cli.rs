use std::io::{self, Write, stdin};

pub fn ask_password() -> String {
    print!("Enter the encryption password: ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    stdin()
        .read_line(&mut password)
        .expect("Failed to read the password");
    password.trim().to_string()
}
