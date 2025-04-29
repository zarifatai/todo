pub mod add;
pub mod complete;
pub mod list;
pub mod remove;

mod common_utils {
    use crate::models::Identifier;

    pub fn resolve_identifier(name: Option<String>, id: Option<i32>) -> Option<Identifier> {
        match (id, name) {
            (Some(id), _) => Some(Identifier::Id(id)),
            (None, Some(name)) => Some(Identifier::Name(name)),
            _ => None,
        }
    }
}
