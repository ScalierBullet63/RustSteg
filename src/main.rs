mod image;
mod payload;

use crate::image::Image;
use crate::payload::Payload;

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
}

fn main() {
    let args: Args = Args::parse();

    let target_file: String = args.target_file;
    let hidden_message: String = args.msg;

    println!("Selected {}", target_file);

    //Process image
    let mut image = Image::new();
    match image.load_image(&target_file) {
        Ok(()) => println!("Image loaded successfully"),
        Err(e) => println!("Error: {e}"),
    }

    //Process payload
    let mut payload = Payload::new();
    payload.set_plain_text(hidden_message);

    image.insert_hidden_message(payload.binary());

    #[cfg(debug_assertions)]
    {
        dbg!(payload.plain_text());
        dbg!(payload.binary());
    }
}
