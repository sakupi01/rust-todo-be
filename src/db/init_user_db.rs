// use std::collections::HashMap;
// use std::sync::{Mutex, OnceLock};
// use std::thread;
// use std::time::Duration;

// // global mutable variable
// static TODO_DB: OnceLock<Mutex<RamZatsuTodoDb>> = OnceLock::new();

// pub fn get_todo_db() -> &'static Mutex<HashMap<String, String>> {
//     TODO_DB.get_or_init(|| Mutex::new(HashMap::new()))
// }
