use super::{
    Byte, Bytes, Flags, Payload, PayloadHeader, XNonce,
    utils::{get_n_bytes, to_ascii},
};
use crate::StegError;

type SaltNonceOpt = (Option<[u8; 16]>, Option<XNonce>);

impl Payload {
    pub fn from_bytes(bytes: Bytes) -> Result<Payload, StegError> {
        let magic = get_magic(&get_n_bytes(&bytes, 0, 8)?)?;
        let version = get_version(
            get_n_bytes(&bytes, 8, 1)?
                .pop()
                .ok_or(StegError::NotEnoughBits)?,
        )?;

        let (flags, salt_nonce_opt) = parse_flags(
            &bytes,
            Flags::from_bits(
                get_n_bytes(&bytes, 9, 1)?
                    .pop()
                    .ok_or(StegError::NotEnoughBits)?,
            )
            .ok_or(StegError::InvalidFlags)?,
        )?;

        let length: u32 = get_length(&bytes, flags)?;

        let (salt, nonce) = salt_nonce_opt;

        let (hidden_message, auth_tag) = get_message(&bytes, length, salt, nonce)?;

        Ok(Self {
            header: PayloadHeader {
                magic,
                version,
                flags,
                salt,
                nonce,
                length,
            },
            hidden_message,
            auth_tag,
        })
    }
}

fn get_magic(magic: &Vec<Byte>) -> Result<[u8; 8], StegError> {
    if to_ascii(magic) != "RUSTSTEG" {
        return Err(StegError::NotRustStegFile);
    }
    Ok(to_ascii(magic).as_bytes().try_into()?)
}

fn get_version(version: u8) -> Result<u8, StegError> {
    match version {
        1 => Ok(version),
        _ => Err(StegError::UnsupportedPayloadVersion),
    }
}

fn parse_flags(bytes: &Bytes, flags: Flags) -> Result<(Flags, SaltNonceOpt), StegError> {
    match flags {
        Flags::NONE => Ok((flags, (None, None))),
        Flags::ENCRYPTED => {
            let salt: [u8; 16] = get_n_bytes(bytes, 10, 16)?.as_slice().try_into()?;
            let nonce: XNonce = get_n_bytes(bytes, 26, 24)?.as_slice().try_into()?;
            Ok((flags, (Some(salt), Some(nonce))))
        }
        _ => Err(StegError::UnsupportedFlag),
    }
}

fn get_length(bytes: &Bytes, flags: Flags) -> Result<u32, StegError> {
    match flags {
        Flags::NONE => Ok(u32::from_be_bytes(
            get_n_bytes(bytes, 10, 4)?.as_slice().try_into()?,
        )),
        Flags::ENCRYPTED => Ok(u32::from_be_bytes(
            get_n_bytes(bytes, 50, 4)?.as_slice().try_into()?,
        )),
        _ => Err(StegError::UnsupportedFlag),
    }
}

fn get_message(
    bytes: &Bytes,
    length: u32,
    salt: Option<[u8; 16]>,
    nonce: Option<XNonce>,
) -> Result<(Bytes, Option<[u8; 16]>), StegError> {
    match (salt, nonce) {
        (Some(_salt), Some(_nonce)) => {
            let _encypted_bytes = get_n_bytes(bytes, 54, length as usize);
            todo!("Implement decrypt");
        }
        (None, None) => {
            let message_bytes = get_n_bytes(bytes, 14, length as usize)?;
            Ok((message_bytes, None))
        }
        _ => Err(StegError::InvalidPayloadState),
    }
}
