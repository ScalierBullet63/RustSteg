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
    hidden_message: String,
}

fn main() {
    let args = Args::parse();

    match args.target_file {
        Some(name) => println!("Target file: {}", name),
        None => println!("No target file specified!"),
    }
}
