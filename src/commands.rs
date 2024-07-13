use std::convert::TryFrom;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};
use crate::{Error, Result};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png_content = read_png_file(&args.file_path)?;
    let mut new_png = Png::try_from(png_content.as_ref())?;

    let chunk_type = args.chunk_type;
    let new_chunk_type = ChunkType::from_str(&chunk_type)?;
    let content = args.content;

    let new_chunk = Chunk::new(new_chunk_type, content.into());
    new_png.append_chunk(new_chunk);

    new_png.gen_png_file(args.output_path)?;

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let mut png_bytes = read_png_file(&args.file_path)?;
    let mut new_png = Png::try_from(png_bytes.as_ref())?;

    for (i, c) in new_png.chunks().iter().enumerate() {
        // println!("chunk:  {}", c.to_string());

        if c.chunk_type().to_string() == args.chunk_type {
            println!("Target Chunk: {}", c.to_string());
        }
    }

    // over write png file:
    // new_png.gen_png_file(args.file_path)?;

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png_bytes = read_png_file(&args.file_path)?;
    let mut new_png = Png::try_from(png_bytes.as_ref())?;

    for (i, c) in new_png.chunks().iter().enumerate() {
        // println!("chunk:  {}", c.to_string());

        if c.chunk_type().to_string() == args.chunk_type {
            new_png.remove_first_chunk(&args.chunk_type);
        }
    }

    // over write png file:
    new_png.gen_png_file(args.file_path)?;

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let mut png_bytes = read_png_file(&args.file_path)?;
    let png = Png::try_from(png_bytes.as_ref())?;

    for c in png.chunks() {
        println!("chunk:  {}", c.to_string());
    }

    Ok(())
}

fn read_png_file(file_path: &PathBuf) -> Result<Vec<u8>> {
    let mut content = vec![];

    let file = File::open(file_path)?;
    let mut buf = BufReader::new(file);
    buf.read_to_end(&mut content)?;

    Ok(content)
}
