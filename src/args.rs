use clap::{Parser, Subcommand};

use crate::chunk_type::ChunkType;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode {
        path: String,
        chunk_type: ChunkType,
        message: String,
        output: Option<String>,
    },
    Decode {
        path: String,
        chunk_type: String,
    },
    Remove {
        path: String,
        chunk_type: String,
    },
    Print {
        path: String,
    },
}
