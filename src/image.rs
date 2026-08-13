use crate::errors::NotEnoughBits;
use image::{DynamicImage, ImageBuffer, ImageError, ImageReader, Rgba};

//Custom types
type ImageMatrix = Vec<ImageRow>;
type ImageRow = Vec<Pixel>;
type Pixel = [u8; 4];
type Binary = Vec<u8>;

pub struct Image {
    pixel_matrix: ImageMatrix,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new() -> Self {
        Self {
            pixel_matrix: ImageMatrix::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn load_image(&mut self, target_file: &str) -> Result<(), ImageError> {
        let (image, width, height) = image_reader(target_file)?;

        self.pixel_matrix = image;
        self.width = width;
        self.height = height;

        #[cfg(debug_assertions)]
        debug_image(&self.pixel_matrix);

        Ok(())
    }

    pub fn insert_hidden_message(&mut self, payload: &Binary) -> Result<(), NotEnoughBits> {
        self.are_bits_enough(payload)?;
        let image = &mut self.pixel_matrix;
        let mut payload_copy = payload.clone();
        for row in image.iter_mut() {
            for pixel in row.iter_mut() {
                for channel in pixel.iter_mut().take(3) {
                    if payload_copy.is_empty() {
                        #[cfg(debug_assertions)]
                        debug_image(image);

                        return Ok(());
                    }
                    *channel = payload_copy.pop().unwrap();
                }
            }
        }
        Ok(())
    }

    fn are_bits_enough(&self, payload: &Binary) -> Result<(), NotEnoughBits> {
        if (self.height * self.width) * 3 < payload.len() as u32 {
            return Err(NotEnoughBits);
        }
        Ok(())
    }
}

fn image_reader(target_file: &str) -> Result<(ImageMatrix, u32, u32), ImageError> {
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
