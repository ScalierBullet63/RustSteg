use image::{DynamicImage, ImageError, ImageReader};

//Custom types
type ImageMatrix = Vec<Vec<[u8; 4]>>;

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

        #[cfg(debug_assertions)]
        {
            println!("Success!");
            dbg!(&image);
        }

        self.pixel_matrix = image;
        self.width = width;
        self.height = height;

        #[cfg(debug_assertions)]
        debug_image(&self.pixel_matrix);

        Ok(())
    }
}

fn image_reader(target_file: &str) -> Result<(ImageMatrix, u32, u32), ImageError> {
    let img: DynamicImage = ImageReader::open(target_file)?.decode()?;

    let rgba_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
    let (width, height): (u32, u32) = rgba_image.dimensions();

    let mut rgba_image_matrix: ImageMatrix = Vec::new();

    for y in 0..height {
        let mut row: Vec<[u8; 4]> = Vec::new();
        for x in 0..width {
            let pixel: &image::Rgba<u8> = rgba_image.get_pixel(x, y);
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
