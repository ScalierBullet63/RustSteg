mod errors;
mod image;
mod payload;

use crate::image::Image;
use crate::payload::{Flags, Payload};

use clap::Parser;

/// Advanced CLI steganography tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Target file
    #[arg(short, long)]
    target_file: String,

    /// Message to hide
    #[arg(short, long)]
    msg: String,

    /// Encrypt the message
    #[arg(long)]
    encrypt: bool,
}

fn main() {
    let args: Args = Args::parse();

    let target_file: String = args.target_file;
    let hidden_message: String = args.msg;

    println!("Selected {}", target_file);

    //Load image
    let mut image = Image::new();
    match image.load_image(&target_file) {
        Ok(()) => println!("Image loaded successfully"),
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    }

    //Process flags
    let mut flags = Flags::NONE;
    if args.encrypt {
        flags.insert(Flags::ENCRYPTED);
    }

    //Process payload
    let mut payload = Payload::new(flags);
    match payload.set_hidden_message(hidden_message) {
        Ok(output_message) => println!("{output_message}"),
        Err(e) => println!("Error: {e}"),
    }

    //Process image
    match image.insert_hidden_message(payload) {
        Ok(()) => println!("Message hidden successfully"),
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    }

    match image.save_image() {
        Ok(()) => println!("Image saved successfully"),
        Err(e) => {
            println!("Error: {e}");
        }
    }
}
