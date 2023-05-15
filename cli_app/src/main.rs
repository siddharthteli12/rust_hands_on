use std::{env, path::PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("struct args from cli - {:?}", args);
}
