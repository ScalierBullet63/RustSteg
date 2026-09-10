mod decode;
mod encode;
mod encryption;

use bitflags::bitflags;
use chacha20poly1305::{XNonce, aead::Generate};
use rand::Rng;

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
}
