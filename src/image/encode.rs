use super::{Bytes, Image, utils};
use crate::{StegError, payload::Payload};

impl Image {
    pub fn encode(&mut self, payload: Payload) -> Result<(), StegError> {
        let bytes = payload.into_bytes()?;
        self.has_enough_bits(&bytes)?;
        let mut bits = utils::to_bits(bytes).into_iter();
        let image = &mut self.pixel_matrix;

        for row in image.iter_mut() {
            for pixel in row.iter_mut() {
                for channel in pixel.iter_mut().take(3) {
                    let Some(next_bit) = bits.next() else {
                        #[cfg(debug_assertions)]
                        utils::debug_image(image);

                        return Ok(());
                    };

                    match next_bit {
                        0 => {
                            if channel.is_multiple_of(2) {
                                continue;
                            } else {
                                if *channel == 255 {
                                    *channel -= 1;
                                } else {
                                    *channel += 1;
                                }
                            }
                        }
                        1 => {
                            if !channel.is_multiple_of(2) {
                                continue;
                            } else {
                                *channel += 1;
                            }
                        }
                        _ => return Err(StegError::UnexpectedError),
                    }
                }
            }
        }
        Err(StegError::UnexpectedError)
    }

    fn has_enough_bits(&self, bytes: &Bytes) -> Result<(), StegError> {
        if (self.height * self.width) * 3 < bytes.len() as u32 * 8 {
            return Err(StegError::NotEnoughBits);
        }
        Ok(())
    }
}
