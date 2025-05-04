use chrono::Local;
use colored::Colorize;
use rusqlite::{Connection, Result};

use crate::database;
use crate::models::Item;

pub fn run(connection: Connection, all: bool, create_date: bool) -> Result<()> {
    let items = database::operations::get_items(connection, all)?;
    for item in items {
        print_item(item, create_date);
    }
    Ok(())
}

fn print_item(item: Item, create_date: bool) {
    let mut output = String::new();
    if item.active {
        output.push_str("[ ]")
    } else {
        output.push_str("[x]")
    }
    output.push_str(format!(" {}:", item.id).as_str());

    if let Some(x) = item.label {
        output.push_str(format!(" [{}]", x).as_str());
    }

    output.push_str(format!(" {}", item.name).as_str());

    if let Some(x) = item.description {
        output.push_str(format!(": {}", x).as_str());
    }

    if let Some(x) = item.due_date {
        let mut due_date = x.to_string().as_str().clear();

        let today = Local::now().naive_local();

        if today > x {
            due_date = due_date.red();
        }
        output.push_str(format!(" ({})", due_date).as_str());
    }

    if create_date {
        output.push_str(
            format!(
                " ({} created)",
                item.create_date.format("%Y-%m-%d %H:%M:%S")
            )
            .as_str(),
        );
    }

    println!("{}", output);
}
