use rusqlite::{Connection, Result};

use super::common_utils;
use crate::database;
use crate::models::Identifier;

pub fn run(connection: Connection, name: Option<String>, id: Option<i32>, all: bool) -> Result<()> {
    if all {
        database::operations::remove_all_tasks(connection)?;
        println!("All tasks removed!");
        return Ok(());
    }

    match common_utils::resolve_identifier(name, id) {
        Some(Identifier::Id(task_id)) => {
            database::operations::remove_task_by_id(connection, task_id)?;
            println!("task removed!")
        }
        Some(Identifier::Name(task_name)) => {
            database::operations::remove_task_by_name(connection, task_name)?;
            println!("task removed!")
        }
        None => {
            eprintln!("Error: must provide either name or id to remove task");
        }
    }
    Ok(())
}
