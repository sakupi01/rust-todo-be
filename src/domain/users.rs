use chrono::{Local, DateTime};

struct Users {
    id: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    name: String,
}
