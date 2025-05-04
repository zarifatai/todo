use chrono::Local;
use colored::Colorize;
use rusqlite::{Connection, Result};

use crate::database;
use crate::models::Task;

pub fn run(connection: Connection, all: bool, create_date: bool) -> Result<()> {
    let tasks = database::operations::get_tasks(connection, all)?;
    for task in tasks {
        print_task(task, create_date);
    }
    Ok(())
}

fn print_task(task: Task, include_create_date: bool) {
    let mut output = String::new();
    if task.active {
        output.push_str("[ ]")
    } else {
        output.push_str("[x]")
    }
    output.push_str(format!(" {}:", task.id).as_str());

    if let Some(x) = task.label {
        output.push_str(format!(" [{}]", x).as_str());
    }

    output.push_str(format!(" {}", task.name).as_str());

    if let Some(x) = task.description {
        output.push_str(format!(": {}", x).as_str());
    }

    if let Some(x) = task.due_date {
        let mut due_date = x.to_string().as_str().clear();

        let today = Local::now().naive_local();

        if today > x {
            due_date = due_date.red();
        }
        output.push_str(format!(" (due: {})", due_date).as_str());
    }

    if include_create_date {
        output.push_str(
            format!(
                " (created: {})",
                task.create_date.format("%Y-%m-%d %H:%M:%S")
            )
            .as_str(),
        );
    }

    println!("{}", output);
}
