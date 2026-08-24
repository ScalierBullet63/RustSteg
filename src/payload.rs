use argon2::Argon2;
use bitflags::bitflags;
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, Generate, Key, KeyInit},
};
use rand::Rng;
use std::io::{self, Write, stdin};
use zeroize::Zeroize;

use crate::errors::StegError;

type Binary = Vec<u8>;

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Flags : u8 {
        const NONE       = 0b0000_0000;
        const ENCRYPTED  = 0b0000_0001;
    }
}

#[derive(Debug)]
struct PayloadHeader {
    magic: [u8; 8],
    version: u8,
    flags: Flags,
    salt: [u8; 16],
    nonce: XNonce,
    length: u32,
}

#[derive(Debug)]
pub struct Payload {
    header: PayloadHeader,
    hidden_message: Vec<u8>,
    auth_tag: Vec<u8>,
}

impl Payload {
    pub fn new(flags: Flags) -> Self {
        let mut salt = [0u8; 16];
        rand::rng().fill_bytes(&mut salt);

        Self {
            header: PayloadHeader {
                magic: *b"RUSTSTEG",
                version: 1,
                flags,
                salt,
                nonce: XNonce::generate(),
                length: 0,
            },
            hidden_message: Vec::new(),
            auth_tag: Vec::new(),
        }
    }

    pub fn set_hidden_message(&mut self, plaintext: String) -> Result<(), StegError> {
        if self.header.flags.contains(Flags::ENCRYPTED) {
            let (ciphertext, auth_tag) = self.encrypt_hidden_message(plaintext)?;
            self.hidden_message = ciphertext.to_vec();
            self.auth_tag = auth_tag.to_vec();
        } else {
            self.hidden_message = plaintext.as_bytes().to_vec();
        }
        self.header.length = self.hidden_message.len() as u32;

        Ok(())
    }

    fn encrypt_hidden_message(
        &self,
        mut plaintext: String,
    ) -> Result<(Vec<u8>, Vec<u8>), StegError> {
        let password = ask_password();
        let key = self.derive_key_from_password(password)?;
        let mut key =
            Key::<XChaCha20Poly1305>::try_from(&key[..]).expect("Key must be exactly 32 bytes");
        let cipher = XChaCha20Poly1305::new(&key);
        let encrypted = cipher.encrypt(&self.header.nonce, plaintext.as_bytes())?;
        let (ciphertext, auth_tag) = encrypted.split_at(encrypted.len() - 16);

        plaintext.zeroize();
        key.zeroize();

        Ok((ciphertext.to_vec(), auth_tag.to_vec()))
    }

    fn derive_key_from_password(&self, mut password: String) -> Result<[u8; 32], StegError> {
        let mut key = [0u8; 32];
        Argon2::default().hash_password_into(password.as_bytes(), &self.header.salt, &mut key)?;
        password.zeroize();
        Ok(key)
    }

    pub fn into_bits(self) -> Binary {
        //Vec of bytes
        let mut bytes = Binary::new();
        bytes.extend_from_slice(&self.header.magic);
        bytes.push(self.header.version);
        bytes.extend_from_slice(&self.header.salt);
        bytes.extend_from_slice(&self.header.nonce);
        bytes.push(self.header.flags.bits());
        bytes.extend_from_slice(&self.header.length.to_be_bytes());
        bytes.extend_from_slice(&self.hidden_message);
        bytes.extend_from_slice(&self.auth_tag);

        //Vec of bits
        let mut bits = Binary::with_capacity(bytes.len() * 8);
        for mut byte in bytes {
            for _ in 0..8 {
                bits.push(byte % 2);
                byte /= 2;
            }
        }

        bits.reverse();

        bits
    }
}

fn ask_password() -> String {
    print!("Enter the encryption password: ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    stdin()
        .read_line(&mut password)
        .expect("Failed to read the password");
    password.trim().to_string()
}
