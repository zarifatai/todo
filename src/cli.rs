use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI to do application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    // Adds a new todo item
    Add {
        #[arg(short = 'N', long)]
        name: String,

        #[arg(short = 'd', long)]
        description: Option<String>,

        #[arg(short = 'D', long)]
        due_date: Option<String>,
    },

    // Completes a todo item given an id or name
    Complete {
        #[arg(short = 'N', long)]
        name: Option<String>,

        #[arg(short = 'I', long)]
        id: Option<i32>,
    },

    // Removes a todo item given an id or name
    Remove {
        #[arg(short = 'N', long)]
        name: Option<String>,

        #[arg(short = 'I', long)]
        id: Option<i32>,
    },

    // Lists todo items
    List {
        #[arg(short = 'A', long)]
        all: bool,
    },
}
