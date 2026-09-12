mod decode;
mod encode;
mod encryption;
mod utils;

use bitflags::bitflags;
use chacha20poly1305::{XNonce, aead::Generate};
use rand::Rng;

type Bytes = Vec<Byte>;
type Byte = u8;

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
    salt: Option<[u8; 16]>,
    nonce: Option<XNonce>,
    length: u32,
}

#[derive(Debug)]
pub struct Payload {
    header: PayloadHeader,
    hidden_message: Bytes,
    auth_tag: Option<[u8; 16]>,
}

impl Payload {
    pub fn new(flags: Flags) -> Self {
        let (salt, nonce) = if flags.contains(Flags::ENCRYPTED) {
            let mut salt = [0u8; 16];
            rand::rng().fill_bytes(&mut salt);
            let nonce = XNonce::generate();

            (Some(salt), Some(nonce))
        } else {
            (None, None)
        };

        Self {
            header: PayloadHeader {
                magic: *b"RUSTSTEG",
                version: 1,
                flags,
                salt,
                nonce,
                length: 0,
            },
            hidden_message: Bytes::new(),
            auth_tag: None,
        }
    }
}
