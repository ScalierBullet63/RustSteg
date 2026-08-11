mod image;
mod payload;

use clap::Parser;

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

    image::process_image(target_file.clone());
    payload::convert_binary();

    println!("Selected {}", target_file);
}
