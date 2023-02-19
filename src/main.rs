pub mod chunk;
pub mod chunk_type;
mod cmd;
mod commands;
pub mod png;
use ::std::fs;
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use cmd::PngMeArgs;
use color_eyre::eyre::Result;
use png::Png;
use std::str::FromStr;
pub type Error = Box<dyn std::error::Error>;

fn read_png(fp: std::path::PathBuf) -> Result<Png> {
    Ok(Png::try_from(fs::read(fp)?.as_ref())?)
}

fn main() -> Result<()> {
    // let PngMeArgs(args) = PngMeArgs::parse() ;

    match PngMeArgs::parse() {
        PngMeArgs::Encode(args) => {
            let mut png = read_png(args.filepath)?;
            let coded_chunk = Chunk::new(
                ChunkType::from_str(&args.chunk_type)?,
                args.message.as_bytes().to_vec(),
            );

            png.append_chunk(coded_chunk);

            fs::write(args.output, png.as_bytes())?;
        }
        PngMeArgs::Decode(args) => {
            let png = read_png(args.filepath)?;

            match png.chunk_by_type(&args.chunk_type) {
                Some(a) => println!("Encoded Message: {}", a.data_as_string()?),
                None => println!(
                    "There was no Encoded Message found with ChunkType: {}",
                    args.chunk_type
                ),
            }
        }
        PngMeArgs::Remove(args) => {
            let mut png = read_png(args.filepath)?;
            png.remove_chunk(&args.chunk_type)?;
            println!(
                "ChunkType: {} has successfully been removed!",
                args.chunk_type
            );
        }
        PngMeArgs::Print(args) => {
            let png = read_png(args.filepath)?;
            println!("{:?}", png.as_bytes())
        }
    }
    Ok(())
}
