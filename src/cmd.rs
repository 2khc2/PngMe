// use std::path::PathBuf;
use clap::Parser;

// use clap;

#[derive(Parser)]
#[command(name = "pngme")]
#[command(bin_name = "pngme")]
#[command(author = "Kelvin Chan <kh.kelvinchan@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Added a secret message into a PNG file", long_about=None)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct EncodeArgs {
    // Write me!
    // #[arg(long)]
    pub filepath: std::path::PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output: String,
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct DecodeArgs {
    pub filepath: std::path::PathBuf,
    pub chunk_type: String,
}
#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct RemoveArgs {
    pub filepath: std::path::PathBuf,
    pub chunk_type: String,
}
#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct PrintArgs {
    pub filepath: std::path::PathBuf,
}
