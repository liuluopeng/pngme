// use clap::{Args, Parser};
use clap::{Args, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

// pngme encode ./dice.png ruSt "This is a secret message!

#[derive(Args, Debug)]
pub struct EncodeArgs {
    // Write me!
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub content: String,
    pub output_path: PathBuf,
    // pub output_
}
// pngme decode ./dice.png ruSt
#[derive(Args, Debug)]
pub struct DecodeArgs {
    // Write me!
    pub file_path: PathBuf,
    pub chunk_type: String,
}
// pngme remove ./dice.png ruSt

#[derive(Args, Debug)]
pub struct RemoveArgs {
    // Write me!
    pub file_path: PathBuf,
    pub chunk_type: String,
}

// pngme print ./dice.png

#[derive(Args, Debug)]
pub struct PrintArgs {
    // Write me!
    pub file_path: PathBuf,
}
