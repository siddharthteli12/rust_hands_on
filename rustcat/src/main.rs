use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    run_type: RunType,
}

#[derive(Subcommand, Debug)]
enum RunType {
    SERVER { port: String },
    CLIENT { url: String },
}

fn main() {
    let args = Args::parse();
    println!("struct value - {:?}", args);
}
