use std::error::Error;

use clap::{Parser, Subcommand};
mod client;
mod server;
use client::handle_client;
use server::handle_server;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.run_type {
        RunType::SERVER { port } => {
            handle_server()?;
            Ok(())
        }
        RunType::CLIENT { url } => {
            handle_client()?;
            Ok(())
        }
    }
}
