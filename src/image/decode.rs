use super::{Byte, Image, utils};
use crate::{
    StegError,
    payload::{Flags, Payload},
};

impl Image {
    pub fn get_payload_from_image(&self) -> Result<Payload, StegError> {
        //Init all vecs
        let mut flags: u8 = 0;

        //If flags
        let mut salt: Vec<u8> = Vec::with_capacity(16);
        let mut nonce: Vec<u8> = Vec::with_capacity(24);

        let mut lenth: u32 = 0;

        check_magic(self.get_n_bytes(0, 8))?;
        handle_version(self.get_n_bytes(8, 1).pop().unwrap())?;

        //Check ver and flags
        dbg!(&self.pixel_matrix[0].iter().take(8).collect::<Vec<_>>());
        todo!("Return payload");

        // let extracted_payload = Payload::new(Flags::NONE);
    }

    fn get_n_bytes(&self, skip_n: usize, n: usize) -> Vec<Byte> {
        let mut buffer: Vec<Byte> = Vec::with_capacity(n);

        for shift in 0..n {
            let bits: Vec<u8> = self
                .pixel_matrix
                .iter()
                .flat_map(|row| row.iter())
                .flat_map(|pixel| pixel.iter().take(3))
                .skip(skip_n * 8 + shift * 8)
                .take(8)
                .copied()
                .collect();

            buffer.push(utils::to_byte(&bits));
        }

        return buffer;
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
        1 => return Ok(()),
        _ => return Err(StegError::PayloadVersionNotSupported),
    }
}
