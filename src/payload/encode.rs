use super::{Bytes, Flags, Payload};
use crate::errors::StegError;

impl Payload {
    pub fn set_hidden_message(&mut self, plaintext: String) -> Result<(), StegError> {
        if self.header.flags.contains(Flags::ENCRYPTED) {
            let (ciphertext, auth_tag) = self.encrypt(plaintext)?;
            self.hidden_message = ciphertext.to_vec();
            self.auth_tag = Some(auth_tag.to_vec());
        } else {
            self.hidden_message = plaintext.as_bytes().to_vec();
        }
        self.header.length = self.hidden_message.len() as u32;

        Ok(())
    }

    pub fn into_bytes(self) -> Result<Bytes, StegError> {
        let mut bytes = Bytes::new();

        bytes.extend_from_slice(&self.header.magic);
        bytes.push(self.header.version);
        bytes.push(self.header.flags.bits());

        if self.header.flags.contains(Flags::ENCRYPTED) {
            bytes.extend_from_slice(&self.header.salt.ok_or(StegError::InvalidPayloadState)?);
            bytes.extend_from_slice(&self.header.nonce.ok_or(StegError::InvalidPayloadState)?);
        }

        bytes.extend_from_slice(&self.header.length.to_be_bytes());
        bytes.extend_from_slice(&self.hidden_message);

        if self.header.flags.contains(Flags::ENCRYPTED) {
            bytes.extend_from_slice(&self.auth_tag.ok_or(StegError::InvalidPayloadState)?);
        }

        Ok(bytes)
    }
}
