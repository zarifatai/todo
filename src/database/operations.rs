use chrono::NaiveDateTime;
use rusqlite::{Connection, Result};

use crate::models::Task;

pub fn add_task(
    connection: Connection,
    name: String,
    description: Option<String>,
    create_date: NaiveDateTime,
    due_date: Option<NaiveDateTime>,
    label: Option<String>,
) -> Result<()> {
    connection.execute(
        "INSERT INTO task (name, description, create_date, due_date, label) VALUES (?1, ?2, ?3, ?4, ?5);",
        (name, description, create_date, due_date, label),
    )?;
    Ok(())
}

pub fn get_tasks(connection: Connection, all: bool) -> Result<Vec<Task>> {
    let mut stmt = String::from(
        "SELECT id, name, description, active, create_date, due_date, label FROM task",
    );
    if !all {
        stmt.push_str(" WHERE active=1;");
    }

    let mut stmt = connection.prepare(&stmt)?;
    let tasks_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get("id")?,
            name: row.get("name")?,
            description: row.get("description")?,
            active: row.get("active")?,
            create_date: row.get("create_date")?,
            due_date: row.get("due_date")?,
            label: row.get("label")?,
        })
    })?;

    let mut tasks = Vec::new();
    for task_result in tasks_iter {
        tasks.push(task_result?);
    }

    Ok(tasks)
}

pub fn complete_task_by_id(connection: Connection, task_id: i32) -> Result<()> {
    connection.execute(
        "UPDATE task SET active = 0 WHERE id = ?1 AND active = 1;",
        (task_id,),
    )?;
    Ok(())
}

pub fn complete_task_by_name(connection: Connection, task_name: String) -> Result<()> {
    connection.execute(
        "UPDATE task SET active = 0 WHERE name = ?1 AND active = 1;",
        (&task_name,),
    )?;
    Ok(())
}

pub fn remove_task_by_id(connection: Connection, task_id: i32) -> Result<()> {
    connection.execute("DELETE FROM task WHERE id = ?1;", (task_id,))?;
    Ok(())
}

pub fn remove_task_by_name(connection: Connection, task_name: String) -> Result<()> {
    connection.execute("DELETE FROM task WHERE name = ?1;", (task_name,))?;
    Ok(())
}

pub fn remove_all_tasks(connection: Connection) -> Result<()> {
    connection.execute("DELETE FROM task;", ())?;
    Ok(())
}
//
// pub fn modify_name(connection: Connection) -> Result<()> {
//     connection.execute("UPDATE task (name) VALUES (?1, ?2, ?3, ?4);")
// }
