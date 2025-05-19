mod cli;
mod commands;
mod tasks;
mod storage;

use cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
