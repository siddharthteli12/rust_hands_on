use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
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

fn main() {
    // Read cli args.
    let args = Cli::parse();

    // Read file.
    let file = File::open(&args.path).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    // Store file content.
    let mut contents = String::new();
    let _ = reader
        .read_to_string(&mut contents)
        .expect("Unable to read file");

    // Print line num & line.
    for (num, line) in contents.lines().enumerate() {
        println!("{:}, {:}", num, line);
    }
}
