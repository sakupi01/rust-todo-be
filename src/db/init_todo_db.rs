use std::sync::{Mutex, OnceLock};

use super::ram_zatsu_todo_db::RamZatsuTodoDb;

// global mutable variable
static TODO_DB: OnceLock<Mutex<RamZatsuTodoDb>> = OnceLock::new();

pub fn get_todo_db() -> &'static Mutex<RamZatsuTodoDb> {
    TODO_DB.get_or_init(|| Mutex::new(RamZatsuTodoDb::new()))
}
