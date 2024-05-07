use chrono::{DateTime, Local};

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub name: String,
}