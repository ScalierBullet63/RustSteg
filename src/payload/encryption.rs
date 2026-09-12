use super::Payload;
use crate::errors::StegError;

use argon2::Argon2;
use chacha20poly1305::{
    XChaCha20Poly1305,
    aead::{Aead, Key, KeyInit},
};
use zeroize::Zeroize;

impl Payload {
    pub fn encrypt(&mut self, password: &str) -> Result<(), StegError> {
        let key = self.derive_key_from_password(password.to_string())?;
        let mut key = Key::<XChaCha20Poly1305>::try_from(&key[..])?;
        let cipher = XChaCha20Poly1305::new(&key);
        let encrypted = cipher.encrypt(
            &self.header.nonce.ok_or(StegError::InvalidPayloadState)?,
            self.hidden_message.as_slice(),
        )?;
        let (ciphertext, auth_tag) = encrypted.split_at(encrypted.len() - 16);

        key.zeroize();

        self.hidden_message = ciphertext.to_vec();
        self.auth_tag = Some(auth_tag.try_into()?);

        Ok(())
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
