use chrono::{Local, NaiveDate, NaiveDateTime};
use rusqlite::{Connection, Result};

use crate::database;

pub fn run(
    connection: Connection,
    name: String,
    description: Option<String>,
    due_date: Option<String>,
    label: Option<String>,
) -> Result<()> {
    let create_date = Local::now().naive_local();
    let due_date = parse_datetime_str(due_date);
    database::operations::add_item(connection, name, description, create_date, due_date, label)?;
    println!("Item added!");
    Ok(())
}

fn parse_datetime_str(datetime_string: Option<String>) -> Option<NaiveDateTime> {
    let s = datetime_string?;

    if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M") {
        Some(dt)
    } else {
        match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            Ok(d) => d.and_hms_opt(0, 0, 0),
            Err(e) => {
                eprintln!("Failed to parse datetime {}: {}", s, e);
                None
            }
        }
    }
}
