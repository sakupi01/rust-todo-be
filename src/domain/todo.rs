use chrono::{DateTime, Local};

#[derive(Clone, Debug, Default)]
pub struct Todo {
    pub id: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub title: String,
    pub content: String,
    pub user_id: String,
}
