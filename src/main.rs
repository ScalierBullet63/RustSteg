use clap::Parser;
use image::{DynamicImage, ImageError, ImageReader};

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
    let args = Args::parse();

    let target_file = match args.target_file {
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

    let (img, width, height) = match image_reader(target_file) {
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
    for y in 0usize..height as usize {
        for x in 0usize..width as usize {
            print!("{:?}", img[y][x]);
        }
        println!();
        println!();
        println!();
    }
}

fn image_reader(target_file: String) -> Result<(Vec<Vec<[u8; 4]>>, u32, u32), ImageError> {
    let img: DynamicImage = ImageReader::open(target_file)?.decode()?;

    let rgba_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
    let (width, height) = rgba_image.dimensions();

    let mut rgba_image_matrix: Vec<Vec<[u8; 4]>> = Vec::new();

    for y in 0..height {
        let mut row: Vec<[u8; 4]> = Vec::new();
        for x in 0..width {
            let pixel = rgba_image.get_pixel(x, y);
            row.push(pixel.0);
        }
        rgba_image_matrix.push(row);
    }

    Ok((rgba_image_matrix, width, height))
}
