use chrono::Local;
use rusqlite::{Connection, Result};

use crate::database;
use crate::utils;

pub fn run(
    connection: Connection,
    name: String,
    description: Option<String>,
    due_date: Option<String>,
) -> Result<()> {
    let create_date = Local::now().naive_local();
    let due_date = utils::parse_datetime_str(due_date);
    database::operations::add_item(connection, name, description, create_date, due_date)?;
    println!("Item added!");
    Ok(())
}
