mod decode;
mod encode;
mod utils;

use crate::errors::StegError;
use image::{DynamicImage, ImageBuffer, ImageReader, Rgba};
use std::path::{Path, PathBuf};

//Custom types
type ImageMatrix = Vec<ImageRow>;
type ImageRow = Vec<Pixel>;
type Pixel = [u8; 4];
type Bytes = Vec<Byte>;
type Byte = u8;

pub struct Image {
    pixel_matrix: ImageMatrix,
    source_path: PathBuf,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new() -> Self {
        Self {
            pixel_matrix: ImageMatrix::new(),
            source_path: PathBuf::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn load_image(&mut self, target_file: &Path) -> Result<(), StegError> {
        let (image, width, height) = image_reader(target_file)?;

        self.pixel_matrix = image;
        self.source_path = target_file.to_owned();
        self.width = width;
        self.height = height;

        Ok(())
    }

    pub fn save_image(&self, output_path: &Path) -> Result<(), StegError> {
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

        image_buffer.save(output_path)?;
        Ok(())
    }
}

fn image_reader(target_file: &Path) -> Result<(ImageMatrix, u32, u32), StegError> {
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
