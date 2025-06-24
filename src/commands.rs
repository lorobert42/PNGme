use std::{
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::Write,
};

use anyhow::{Result, anyhow};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode(
    filename: &String,
    chunk_type: &ChunkType,
    message: &String,
    output: Option<&String>,
) -> Result<()> {
    let data: Vec<u8> = fs::read(filename)?;
    let mut png = Png::try_from(&data[..])?;
    let chunk = Chunk::new(chunk_type.clone(), message.as_bytes().to_vec());
    png.append_chunk(chunk);
    let mut ofile;
    if let Some(path) = output {
        ofile = File::create(path)?;
    } else {
        ofile = File::create("res".to_string() + filename)?;
    }
    ofile.write_all(&png.as_bytes())?;
    Ok(())
}

pub fn decode(filename: &String, chunk_type: &str) -> Result<()> {
    let data: Vec<u8> = fs::read(filename)?;
    let png = Png::try_from(&data[..])?;
    let chunk = png.chunk_by_type(chunk_type);
    match chunk {
        Some(chunk) => println!("{chunk}"),
        None => return Err(anyhow!("Chunk not found.")),
    }
    Ok(())
}

pub fn remove(filename: &String, chunk_type: &str) -> Result<()> {
    let data: Vec<u8> = fs::read(filename)?;
    let mut png = Png::try_from(&data[..])?;
    png.remove_first_chunk(chunk_type)?;
    let mut ofile = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)?;
    ofile.write_all(&png.as_bytes())?;
    Ok(())
}

pub fn print(filename: &String) -> Result<()> {
    let data: Vec<u8> = fs::read(filename)?;
    let png = Png::try_from(&data[..])?;
    png.chunks()
        .iter()
        .for_each(|c| println!("{}", c.chunk_type()));
    Ok(())
}
