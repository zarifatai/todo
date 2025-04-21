use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI to do application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    // Path to the SQLite database file
    #[arg(short = 'D', long, default_value = "./todo.db")]
    database: PathBuf,
}

#[derive(Debug, Subcommand)]
enum Command {
    // Adds a new todo item
    Add {
        #[arg(short = 'N', long)]
        name: String,

        #[arg(short = 'D', long)]
        description: Option<String>,
    },

    // Completes a todo item given an id or name
    Complete {
        #[arg(short = 'N', long)]
        name: Option<String>,

        #[arg(long)]
        id: Option<i32>,
    },

    // Removes a todo item given an id or name
    Remove {
        #[arg(short = 'N', long)]
        name: Option<String>,

        id: Option<i32>,
    },

    // Lists todo items
    List {
        #[arg(short = 'A', long)]
        all: bool,
    },
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    id: i32,
    description: Option<String>,
    active: bool,
}

enum Identifier {
    Id(i32),
    Name(String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let conn = Connection::open(&args.database)?;
    let _ = create_item_table(&conn);

    match args.command {
        Command::Add { name, description } => add_item(&conn, name, description)?,
        Command::Complete { name, id } => complete_item(&conn, name, id)?,
        Command::Remove { name, id } => remove_item(&conn, name, id)?,
        Command::List { all } => list_items(&conn, all)?,
    }
    Ok(())
}

fn create_item_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            description TEXT,
            active INTEGER NOT NULL DEFAULT 1)",
        (),
    )?;
    Ok(())
}

fn add_item(conn: &Connection, name: String, description: Option<String>) -> Result<()> {
    conn.execute(
        "INSERT INTO item (name, description) VALUES (?1, ?2);",
        (name, description),
    )?;
    println!("Item added!");
    Ok(())
}

fn resolve_identifier(name: Option<String>, id: Option<i32>) -> Option<Identifier> {
    match (id, name) {
        (Some(id), _) => Some(Identifier::Id(id)),
        (None, Some(name)) => Some(Identifier::Name(name)),
        _ => None,
    }
}

fn complete_item(conn: &Connection, name: Option<String>, id: Option<i32>) -> Result<()> {
    match resolve_identifier(name, id) {
        Some(Identifier::Id(item_id)) => {
            conn.execute("UPDATE item SET active = 0 WHERE id = ?1;", (item_id,))?;
            println!("Item completed!");
        }
        Some(Identifier::Name(item_name)) => {
            conn.execute("UPDATE item SET active = 0 WHERE name = ?1;", (&item_name,))?;
            println!("Item completed!");
        }
        None => {
            eprintln!("Error: must provide either name or id to complete item");
        }
    }
    Ok(())
}

fn remove_item(conn: &Connection, name: Option<String>, id: Option<i32>) -> Result<()> {
    match resolve_identifier(name, id) {
        Some(Identifier::Id(item_id)) => {
            conn.execute("DELETE FROM item WHERE id = ?1;", (item_id,))?;
            println!("Item removed!")
        }
        Some(Identifier::Name(item_name)) => {
            conn.execute("DELETE FROM item WHERE name = ?1;", (item_name,))?;
        }
        None => {
            eprintln!("Error: must provide either name or id to remove item");
        }
    }
    Ok(())
}

fn list_items(conn: &Connection, all: bool) -> Result<()> {
    let mut stmt = String::from("SELECT * FROM item");
    if !all {
        stmt.push_str(" WHERE active=1");
    }

    let mut stmt = conn.prepare(&stmt)?;
    let items = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            active: row.get(3)?,
        })
    })?;

    for item in items {
        if let Ok(x) = item {
            print_item(x)
        }
    }
    Ok(())
}

fn print_item(item: Item) -> () {
    let mut checked = "[ ]";
    if !item.active {
        checked = "[x]";
    }

    let name = item.name;
    let id = item.id;

    let description = item.description.unwrap_or_default();
    let description = if description.is_empty() {
        "".to_string()
    } else {
        format!(": {}", description)
    };

    println!("{} {}: {}{}", checked, id, name, description);
}
