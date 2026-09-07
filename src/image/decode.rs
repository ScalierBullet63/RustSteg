use super::{Image, ImageMatrix, utils};
use crate::{
    StegError,
    payload::{Flags, Payload},
};

impl Image {
    pub fn get_payload_from_image(self) -> Result<Payload, StegError> {
        let extracted_payload = Payload::new(Flags::NONE); //Farlo alla fine!!!
        let image = self.pixel_matrix;

        //Init all vecs
        let mut magic: Vec<u8> = Vec::with_capacity(8);
        let mut version: u8 = 0;
        let mut flags: u8 = 0;
        let mut salt: Vec<u8> = Vec::with_capacity(16);
        let mut nonce: Vec<u8> = Vec::with_capacity(24); //Verify
        let mut lenth: u32 = 0;

        //Get magic
        let _ = get_n_bits(&image, 1, 8);
        todo!("Return payload");
    }
}

//Get bytes?
fn get_n_bits(image: &ImageMatrix, start_n: usize, n: usize) -> Vec<u8> {
    let buffer: Vec<u8> = image
        .iter()
        .flat_map(|row| row.iter())
        .flat_map(|pixel| pixel.iter().take(3))
        .skip(start_n)
        .take(n)
        .copied()
        .collect();

    dbg!(&[1u8; 8]);
    dbg!(utils::to_byte(&[1u8; 8].to_vec()));

    dbg!(&buffer);
    return buffer;
}
