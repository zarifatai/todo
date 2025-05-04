use rusqlite::{Connection, Result};

use super::common_utils;
use crate::database;
use crate::models::Identifier;

pub fn run(connection: Connection, name: Option<String>, id: Option<i32>) -> Result<()> {
    match common_utils::resolve_identifier(name, id) {
        Some(Identifier::Id(task_id)) => {
            database::operations::complete_task_by_id(connection, task_id)?;
            println!("task completed!");
        }
        Some(Identifier::Name(task_name)) => {
            database::operations::complete_task_by_name(connection, task_name)?;
            println!("task completed!");
        }
        None => {
            eprintln!("Error: must provide either name or id to complete task");
        }
    }
    Ok(())
}
