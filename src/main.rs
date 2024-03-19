#![feature(const_option)]

use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

mod command;
mod website;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Build { out_dir: PathBuf },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Build { out_dir } => crate::command::build::invoke(out_dir).unwrap(),
    }
}
