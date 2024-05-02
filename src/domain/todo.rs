use chrono::{Local, DateTime};

struct Todo {
    id: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    title: String,
    content: String,
    user_id: String
}
