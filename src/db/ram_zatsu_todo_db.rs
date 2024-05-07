use std::collections::HashMap;

use crate::{domain::todo::Todo, usecase::data_access::todo::TodoDataAccess};

struct RamZatsuTodoDb {
    db: HashMap<String, Todo>,
}

impl TodoDataAccess for RamZatsuTodoDb {
    fn create(&self, todo: crate::domain::todo::Todo) -> Result<(), String> {
        if self.db.insert(todo.id, todo).is_none() {
            Ok(())
        } else {
            Err("fail to insert todo".to_string())
        }
    }
}
