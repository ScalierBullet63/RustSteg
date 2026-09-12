use super::{Bytes, Image, utils};

impl Image {
    pub fn to_bytes(&self) -> Bytes {
        //Flat array of bytes
        let bits: Vec<u8> = self
            .pixel_matrix
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(|pixel| pixel.iter().take(3).copied())
            .map(|channel| channel & 1)
            .collect();

        let bytes: Bytes = utils::to_bytes(&bits);

        bytes
    }
}
