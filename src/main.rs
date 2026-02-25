use clap::Parser;
use image::{DynamicImage, ImageError, ImageReader};

//Custom types
type ImageMatrixType = Vec<Vec<[u8; 4]>>;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Target file
    #[arg(short, long)]
    target_file: Option<String>,

    /// Message to hide
    #[arg(short, long)]
    msg: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let target_file: String = match args.target_file {
        Some(name) => {
            println!("Target file: {}", name);
            name
        }
        None => {
            println!("No target file specified!");
            return;
        }
    };

    println!("Selected {}", target_file);

    let (img, _width, height): (ImageMatrixType, u32, u32) = match image_reader(target_file) {
        Ok((image, width, height)) => {
            println!("Success!");
            (image, width, height)
        }
        Err(e) => {
            println!("Error loading image: {}", e);
            return;
        }
    };

    // println!("{:?}", img);
    for y in img.iter().take(height as usize) {
        for pixel in y {
            print!("Red: {:?} ", pixel[0]);
            print!("Green: {:?} ", pixel[1]);
            println!("Blue: {:?}", pixel[2]);
        }
        println!();
        println!();
        println!();
    }
}

fn image_reader(target_file: String) -> Result<(ImageMatrixType, u32, u32), ImageError> {
    let img: DynamicImage = ImageReader::open(target_file)?.decode()?;

    let rgba_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
    let (width, height): (u32, u32) = rgba_image.dimensions();

    let mut rgba_image_matrix: ImageMatrixType = Vec::new();

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
