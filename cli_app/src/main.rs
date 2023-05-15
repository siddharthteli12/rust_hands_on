use clap::Parser;
use std::{
    error::Error,
    fmt::Error,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    process::exit,
};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read cli args.
    let args = Cli::parse();

    // Read file.
    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);
    // Store file content.
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents)?;

    // Print line num & line.
    for (num, line) in contents.lines().enumerate() {
        println!("{:}, {:}", num, line);
    }
    Ok(())
}
