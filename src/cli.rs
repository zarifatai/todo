use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Adds a new todo item
    Add {
        name: String,

        #[arg(short = 'd', long)]
        description: Option<String>,

        #[arg(short = 'D', long, value_name = "YYYY-MM-DD (HH:MM)")]
        due_date: Option<String>,
    },

    /// Completes a todo item given an id or name
    Complete {
        id: Option<i32>,

        #[arg(short = 'N', long)]
        name: Option<String>,
    },

    /// Removes a todo item given an id or name
    Remove {
        id: Option<i32>,

        #[arg(short = 'N', long)]
        name: Option<String>,

        #[arg(short = 'A', long)]
        all: bool,
    },

    /// Lists todo items
    List {
        #[arg(short = 'A', long)]
        all: bool,
    },
}
