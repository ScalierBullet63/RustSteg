use crate::{
    errors::StegError,
    payload::{Flags, Payload},
};
use image::{DynamicImage, ImageBuffer, ImageReader, Rgba};

//Custom types
type ImageMatrix = Vec<ImageRow>;
type ImageRow = Vec<Pixel>;
type Pixel = [u8; 4];
type Binary = Vec<u8>;

pub struct Image {
    pixel_matrix: ImageMatrix,
    source_path: String,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new() -> Self {
        Self {
            pixel_matrix: ImageMatrix::new(),
            source_path: String::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn load_image(&mut self, target_file: &str) -> Result<(), StegError> {
        let (image, width, height) = image_reader(target_file)?;

        self.pixel_matrix = image;
        self.source_path = target_file.to_string();
        self.width = width;
        self.height = height;

        #[cfg(debug_assertions)]
        debug_image(&self.pixel_matrix);

        Ok(())
    }

    pub fn insert_hidden_message(&mut self, payload: Payload) -> Result<(), StegError> {
        let bits = payload.into_bits();
        self.are_bits_enough(&bits)?;
        let mut bits = bits.iter();
        let image = &mut self.pixel_matrix;

        for row in image.iter_mut() {
            for pixel in row.iter_mut() {
                for channel in pixel.iter_mut().take(3) {
                    let Some(next_bit) = bits.next() else {
                        #[cfg(debug_assertions)]
                        debug_image(image);

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

    pub fn save_image(&self) -> Result<(), StegError> {
        let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(self.width, self.height);

        let flat_pixel_matrix: Vec<u8> = self
            .pixel_matrix
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(|pixel| pixel.iter())
            .copied()
            .collect();

        for (dst, src) in image_buffer.iter_mut().zip(&flat_pixel_matrix) {
            *dst = *src;
        }

        let mut output_path = self.source_path.to_string();

        if let Some(point_pos) = self.source_path.rfind(".") {
            output_path.truncate(point_pos);
            output_path += "_steg.";
            let extension = &self.source_path[&point_pos + 1..];
            output_path += extension;
        }

        image_buffer.save(output_path)?;
        Ok(())
    }

    fn are_bits_enough(&self, bits: &Binary) -> Result<(), StegError> {
        if (self.height * self.width) * 3 < bits.len() as u32 {
            return Err(StegError::NotEnoughBits);
        }
        Ok(())
    }
}

fn image_reader(target_file: &str) -> Result<(ImageMatrix, u32, u32), StegError> {
    let img: DynamicImage = ImageReader::open(target_file)?.decode()?;

    let rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>> = img.to_rgba8();
    let (width, height): (u32, u32) = rgba_image.dimensions();

    let mut rgba_image_matrix: ImageMatrix = ImageMatrix::new();

    for y in 0..height {
        let mut row: ImageRow = ImageRow::new();
        for x in 0..width {
            let pixel: &Rgba<u8> = rgba_image.get_pixel(x, y);
            row.push(pixel.0);
        }
        rgba_image_matrix.push(row);
    }

    Ok((rgba_image_matrix, width, height))
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
    dbg!(to_byte(&[1u8; 8].to_vec()));

    dbg!(&buffer);
    return buffer;
}

//Use this
fn to_byte(bits: &Vec<u8>) -> u8 {
    let mut byte: u8 = 0;
    let mut exp: u8 = 7;
    for bit in bits {
        byte += bit * u8::pow(2, exp as u32);
        exp -= 1;
    }
    byte
}

#[cfg(debug_assertions)]
fn debug_image(img: &ImageMatrix) {
    for y in img.iter() {
        for pixel in y {
            print!("Red: {:?} ", pixel[0]);
            print!("Green: {:?} ", pixel[1]);
            println!("Blue: {:?}", pixel[2]);
        }
        println!();
    }
}
