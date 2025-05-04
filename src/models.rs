use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub id: i32,
    pub description: Option<String>,
    pub active: bool,
    pub create_date: NaiveDateTime,
    pub due_date: Option<NaiveDateTime>,
    pub label: Option<String>,
}

pub enum Identifier {
    Id(i32),
    Name(String),
}
