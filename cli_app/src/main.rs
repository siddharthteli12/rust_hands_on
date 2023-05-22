use clap::Parser;
use log::{info, warn};
use std::{fs, path::PathBuf};
/// Search a file for a given pattern, & display line.
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
    let content = fs::read_to_string(args.path)?;
    cli_app::match_pattern(&args.pattern, &content, &mut std::io::stdout())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_with_non_existent_pattern() -> Result<(), Box<dyn std::error::Error>> {
        let content = "Abc\n Aaa\n amc\n";
        let pattern = "azdzds";
        let mut test_vec = Vec::new();
        assert!(cli_app::match_pattern(pattern, content, &mut test_vec).is_ok());
        assert!(test_vec.is_empty());
        Ok(())
    }

    #[test]
    fn test_with_existent_pattern() {
        let content = "Abc\n Aaa\n amc\n";
        let pattern = "a";
        let mut test_vec = Vec::new();
        assert!(cli_app::match_pattern(pattern, content, &mut test_vec).is_ok());
        assert_eq!(test_vec, b"1, Aaa\n2, amc\n");
    }
}
