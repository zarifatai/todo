#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub id: i32,
    pub description: Option<String>,
    pub active: bool,
}

pub enum Identifier {
    Id(i32),
    Name(String),
}
