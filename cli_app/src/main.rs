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

fn main() -> Result<(), std::io::Error> {
    // Read cli args.
    env_logger::init();
    warn!("Make sure file exists");
    let args = Cli::parse();
    // Read file.
    info!("Reading file");
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    match_pattern(&args.pattern, reader, std::io::stdout())
}

fn match_pattern(
    pattern: &str,
    content: impl BufRead,
    mut output: impl std::io::Write,
) -> Result<(), std::io::Error> {
    for (counter, line) in content.lines().enumerate() {
        let line = line?;
        if line.contains(&pattern) {
            writeln!(output, "{:}, {:}", counter, line)?
        }
    }
    Ok(())
}
