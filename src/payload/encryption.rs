use super::Payload;
use crate::errors::StegError;

use argon2::Argon2;
use chacha20poly1305::{
    XChaCha20Poly1305,
    aead::{Aead, Key, KeyInit},
};
use std::io::{self, Write, stdin};
use zeroize::Zeroize;

impl Payload {
    pub fn encrypt(&self, mut plaintext: String) -> Result<(Vec<u8>, Vec<u8>), StegError> {
        let password = ask_password();
        let key = self.derive_key_from_password(password)?;
        let mut key = Key::<XChaCha20Poly1305>::try_from(&key[..])?;
        let cipher = XChaCha20Poly1305::new(&key);
        let encrypted = cipher.encrypt(
            &self.header.nonce.ok_or(StegError::InvalidPayloadState)?,
            plaintext.as_bytes(),
        )?;
        let (ciphertext, auth_tag) = encrypted.split_at(encrypted.len() - 16);

        plaintext.zeroize();
        key.zeroize();

        Ok((ciphertext.to_vec(), auth_tag.to_vec()))
    }

    fn derive_key_from_password(&self, mut password: String) -> Result<[u8; 32], StegError> {
        let mut key = [0u8; 32];
        Argon2::default().hash_password_into(
            password.as_bytes(),
            &self.header.salt.ok_or(StegError::InvalidPayloadState)?,
            &mut key,
        )?;
        password.zeroize();
        Ok(key)
    }
}

pub fn ask_password() -> String {
    print!("Enter the encryption password: ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    stdin()
        .read_line(&mut password)
        .expect("Failed to read the password");
    password.trim().to_string()
}
