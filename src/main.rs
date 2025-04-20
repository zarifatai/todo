use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI to do application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    // Adds a new todo item
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: Option<String>,
    },

    // Completes a todo item given an id or name
    Complete {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(long)]
        id: Option<i32>,
    },

    // Removes a todo item given an id or name
    Remove {
        #[arg(short, long)]
        name: Option<String>,
        id: Option<i32>,
    },

    // Lists todo items
    List {
        #[arg(short = 'A', long)]
        all: bool,
    },
}

#[derive(Debug)]
struct Item {
    name: String,
    id: i32,
    description: Option<String>,
    active: bool,
}

fn main() -> Result<()> {
    let db_path = "./todo.db";
    let conn = Connection::open(db_path)?;
    let _ = create_item_table(&conn);

    let args = Cli::parse();
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
    Ok(())
}

fn complete_item(conn: &Connection, name: Option<String>, id: Option<i32>) -> Result<()> {
    let mut column_name = String::new();
    let mut param = String::new();

    if let Some(item_id) = id {
        column_name.push_str("id");
        param = item_id.to_string();
    } else if let Some(item_name) = name {
        column_name.push_str("name");
        param = format!("'{}'", item_name);
    }

    conn.execute(
        format!(
            "UPDATE item SET active = 0 WHERE {} = {};",
            &column_name, &param
        )
        .as_str(),
        (),
    )?;
    Ok(())
}

fn remove_item(conn: &Connection, name: Option<String>, id: Option<i32>) -> Result<()> {
    let mut column_name = String::new();
    let mut param = String::new();

    if let Some(item_id) = id {
        column_name.push_str("id");
        param = item_id.to_string();
    } else if let Some(item_name) = name {
        column_name.push_str("name");
        param = format!("'{}'", item_name);
    }

    conn.execute(
        format!("DELETE FROM item WHERE {} = {};", &column_name, &param).as_str(),
        (),
    )?;
    Ok(())
}

fn list_items(conn: &Connection, all: bool) -> Result<()> {
    let mut where_clause = String::new();
    if all {
        where_clause.push_str("1=1");
    } else {
        where_clause.push_str("active=1");
    }

    let mut stmt = conn.prepare(format!("SELECT * FROM item WHERE {}", &where_clause).as_str())?;
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
            let mut checked = "[ ]";
            if !x.active {
                checked = "[x]";
            }

            let name = x.name;
            let id = x.id;

            let mut description = String::new();
            if let Some(descr) = x.description {
                description.push_str(format!(": {}", &descr).as_str());
                println!("Yes!")
            }

            println!("{} {}: {}{}", checked, id, name, description);
        }
    }
    Ok(())
}
