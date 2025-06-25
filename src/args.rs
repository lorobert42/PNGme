use clap::{Parser, Subcommand};

use crate::chunk_type::ChunkType;

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encode a message into a PNG file
    Encode {
        /// Path to the PNG file to hide the message into
        path: String,
        /// Type of the chunk that will contain the message
        chunk_type: ChunkType,
        /// Message to encode
        message: String,
        /// Output file, (`res_[input.png]` if not specified)
        output: Option<String>,
    },
    /// Decode a message stored into a PNG file
    Decode {
        /// Path to the PNG file containing the hidden message
        path: String,
        /// Type of the chunk containing the message
        chunk_type: String,
    },
    /// Remove a message from a PNG file
    Remove {
        /// Path to the PNG file containing the hidden message
        path: String,
        /// Type of the chunk containing the message
        chunk_type: String,
    },
    /// Print a list of PNG chunks that can be searched for messages
    Print {
        /// Path to the PNG file to analyse
        path: String,
    },
}
