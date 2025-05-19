use clap::{Parser, Subcommand};

use crate::commands::{add::handle_add, list::handle_list};

#[derive(Parser)]
#[command(name = "tasker", version = "0.1", author = "Ты")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::Add { description } => handle_add(description),
            Commands::List => handle_list(),
        }
    }
}
