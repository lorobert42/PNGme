use clap::Parser;
use commands::{decode, encode, print, remove};
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    match &cli.command {
        Some(args::Commands::Encode {
            path,
            chunk_type,
            message,
            output,
        }) => encode(path, chunk_type, message, output.as_ref())?,
        Some(args::Commands::Decode { path, chunk_type }) => decode(path, chunk_type)?,
        Some(args::Commands::Remove { path, chunk_type }) => remove(path, chunk_type)?,
        Some(args::Commands::Print { path }) => print(path)?,
        _ => {}
    }

    Ok(())
}
