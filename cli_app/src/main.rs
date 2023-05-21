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
    match_pattern(&args.pattern, &content, &mut std::io::stdout())
}

fn match_pattern(
    pattern: &str,
    content: &str,
    mut output: impl std::io::Write,
) -> Result<(), std::io::Error> {
    for (counter, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            writeln!(&mut output, "{:},{:}", counter, line)?
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Write};

    #[test]
    fn test_with_non_existent_pattern() {
        let content = "Abc\n Aaa\n amc\n";
        let pattern = "azdzds";
        let writer_path = "output.txt";
        let mut output = File::create(writer_path).unwrap();
        assert!(match_pattern(pattern, content, &mut output).is_ok());
        let _ = output.flush();
        assert_eq!(fs::read_to_string(writer_path).unwrap(), "");
    }

    #[test]
    fn test_with_existent_pattern() {
        let content = "Abc\n Aaa\n amc\n";
        let pattern = "a";
        let writer_path = "output.txt";
        let mut output = File::create(writer_path).unwrap();
        assert!(match_pattern(pattern, content, &mut output).is_ok());
        let _ = output.flush();
        assert_eq!(fs::read_to_string(writer_path).unwrap(), "1, Aaa\n2, amc\n");
    }
}
