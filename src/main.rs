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

    println!("{:?}", target_file);

    let img: DynamicImage = match image_reader(target_file) {
        Ok(image) => {
            println!("Success!");
            image
        }
        Err(e) => {
            println!("Error loading image: {}", e);
            return;
        }
    };

    println!("{:?}", img);
}

fn image_reader(target_file: String) -> Result<DynamicImage, ImageError> {
    let img = ImageReader::open(target_file)?.decode()?;
    Ok(img)
}
