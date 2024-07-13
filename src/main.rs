use args::PngMeArgs;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngMeArgs::parse();

    // println!("args is : {:#?}", args);
    // dbg!(args);

    match args {
        PngMeArgs::Encode(args) => {
            commands::encode(args);
        }
        PngMeArgs::Decode(args) => {
            commands::decode(args);
        }
        PngMeArgs::Print(args) => {
            commands::print_chunks(args);
        }
        PngMeArgs::Remove(args) => {
            commands::remove(args);
        }
    }

    Ok(())
}
