use rpassword::prompt_password;

pub fn ask_password() -> String {
    let password = prompt_password("Enter the password: ").unwrap();

    password.trim().to_string()
}
