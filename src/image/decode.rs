use super::{Byte, Image, utils};
use crate::{StegError, payload::Flags};

type SaltNonceOpt = Option<(Vec<u8>, Vec<u8>)>;

impl Image {
    pub fn get_payload_from_image(&self) -> Result<String, StegError> {
        check_magic(self.get_n_bytes(0, 8)?)?;
        handle_version(
            self.get_n_bytes(8, 1)?
                .pop()
                .ok_or(StegError::NotEnoughBits)?,
        )?;

        let salt_nonce_opt = self.parse_flags(
            Flags::from_bits(
                self.get_n_bytes(9, 1)?
                    .pop()
                    .ok_or(StegError::NotEnoughBits)?,
            )
            .ok_or(StegError::InvalidFlags)?,
        )?;

        let plain_message = self.get_message(salt_nonce_opt)?;

        Ok(plain_message)
    }

    fn get_n_bytes(&self, skip_n: usize, n: usize) -> Result<Vec<Byte>, StegError> {
        let flat_lsb_matrix: Vec<u8> = self
            .pixel_matrix
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(|pixel| pixel.iter().take(3).copied())
            .map(|channel| channel & 1)
            .collect();

        if flat_lsb_matrix.len() < (skip_n + n) * 8 {
            return Err(StegError::NotEnoughBits);
        }

        let mut buffer: Vec<Byte> = Vec::with_capacity(n);

        for shift in 0..n {
            let bits: Vec<u8> = flat_lsb_matrix
                .iter()
                .skip((skip_n + shift) * 8)
                .take(8)
                .copied()
                .collect();

            buffer.push(utils::to_byte(&bits));
        }

        Ok(buffer)
    }

    fn parse_flags(&self, flags: Flags) -> Result<SaltNonceOpt, StegError> {
        match flags {
            Flags::NONE => Ok(None),
            Flags::ENCRYPTED => {
                let salt: Vec<u8> = self.get_n_bytes(10, 16)?;
                let nonce: Vec<u8> = self.get_n_bytes(26, 24)?;
                Ok(Some((salt, nonce)))
            }
            _ => Err(StegError::UnsupportedFlag),
        }
    }

    fn get_message(&self, salt_nonce_opt: Option<(Vec<u8>, Vec<u8>)>) -> Result<String, StegError> {
        match salt_nonce_opt {
            Some((_salt, _nonce)) => {
                let length: u32 =
                    u32::from_be_bytes(self.get_n_bytes(50, 4)?.as_slice().try_into()?);
                let _encypted_bytes = self.get_n_bytes(54, length as usize);
                todo!("Implement decrypt");
            }
            None => {
                let length: u32 =
                    u32::from_be_bytes(self.get_n_bytes(10, 4)?.as_slice().try_into()?);
                let message_bytes = self.get_n_bytes(14, length as usize)?;
                Ok(utils::to_ascii(message_bytes))
            }
        }
    }
}

fn check_magic(magic: Vec<Byte>) -> Result<(), StegError> {
    if &utils::to_ascii(magic) != "RUSTSTEG" {
        return Err(StegError::NotRustStegFile);
    }
    Ok(())
}

fn handle_version(version: u8) -> Result<(), StegError> {
    match version {
        1 => Ok(()),
        _ => Err(StegError::UnsupportedPayloadVersion),
    }
}
