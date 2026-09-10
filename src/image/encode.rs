use super::{Binary, Image};
use crate::{StegError, payload::Payload};

#[cfg(debug_assertions)]
use super::utils;

impl Image {
    pub fn insert_hidden_message(&mut self, payload: Payload) -> Result<(), StegError> {
        let bits = payload.into_bits()?;
        self.are_bits_enough(&bits)?;
        let mut bits = bits.iter();
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

    fn are_bits_enough(&self, bits: &Binary) -> Result<(), StegError> {
        if (self.height * self.width) * 3 < bits.len() as u32 {
            return Err(StegError::NotEnoughBits);
        }
        Ok(())
    }
}
