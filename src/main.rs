mod encode;
mod errors;
mod image;
mod payload;

use clap::{Parser, Subcommand};

use crate::errors::StegError;

/// Advanced CLI steganography tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ///Hide a message inside an image
    Encode {
        /// Target file
        #[arg(short, long)]
        target_file: String,

        /// Message to hide
        #[arg(short, long)]
        msg: String,

        /// Encrypt the message
        #[arg(long)]
        encrypt: bool,
    },
}

fn main() -> Result<(), StegError> {
    let args: Args = Args::parse();

    match args.commands {
        Commands::Encode {
            target_file,
            msg,
            encrypt,
        } => encode::encode(target_file, msg, encrypt)?,
    }

    Ok(())
}
