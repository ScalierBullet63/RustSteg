mod image;
mod payload;

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

    image::process_image(target_file.clone());
    let mut payload = payload::Payload::new();
    payload.set_plain_text(hidden_message);

    dbg!(payload.plain_text());
    dbg!(payload.binary());

    println!("Selected {}", target_file);
}
