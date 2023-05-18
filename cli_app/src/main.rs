use clap::Parser;
use log::{info, warn};
use std::{
    fs::File,
    io::{BufRead, BufReader},
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read cli args.
    env_logger::init();
    warn!("Make sure file exists");
    let args = Cli::parse();
    // Read file.
    info!("Reading file");
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    for (counter, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{:}, {:}", counter, line);
        }
    }
    Ok(())
}
