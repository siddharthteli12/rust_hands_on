use clap::Parser;
use std::path::PathBuf;

/// Search a file for a given pattern & display line.
#[derive(Parser, Debug)]
struct Cli {
    /// Pattern to be searched in the file.
    #[arg(long = "pattern")]
    pattern: String,
    /// Path of the file to be searched.
    #[arg(long = "path")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("struct args from cli - {:?}", args);
}
