use std::collections::HashMap;

use crate::{domain::todo::Todo, usecase::data_access::todo::TodoDataAccess};

#[derive(Default)]
pub struct RamZatsuTodoDb {
    db: HashMap<String, Todo>,
}

impl RamZatsuTodoDb {
    pub fn new() -> RamZatsuTodoDb {
        RamZatsuTodoDb{db:HashMap::new()}
    }
}

impl TodoDataAccess for RamZatsuTodoDb {
    fn create(&mut self, todo: crate::domain::todo::Todo) -> Result<(), String> {
        if self.db.insert(todo.id.clone(), todo).is_none() {
            Ok(())
        } else {
            Err("fail to insert todo".to_string())
        }
    }
    fn update_title(&mut self, id: String, title: String) -> Result<(), String> {
        if let Some(x) = self.db.get_mut(&id) {
            (*x).title = title;
            Ok(())
        } else {
            Err("fail to update title".to_string())
        }
    }
    fn update_content(&mut self, id: String, content: String) -> Result<(), String> {
        if let Some(x) = self.db.get_mut(&id) {
            (*x).content = content;
            Ok(())
        } else {
            Err("fail to update title".to_string())
        }
    }
    fn delete(&mut self, id: String) -> Result<(), String> {
        if self.db.remove(&id).is_some() {
            Ok(())
        } else {
            Err("fail to delete todo".to_string())
        }
    }
    fn get_all(&self) -> Result<Vec<Todo>, String> {
        Ok(self.db.values().cloned().collect())
    }
    fn get_by_user_id(&self, user_id: String) -> Result<Vec<Todo>, String> {
        let mut todos = vec![];
        for todo in self.db.values() {
            if todo.user_id == user_id {
                todos.push(todo.clone());
            }
        }
        Ok(todos)
    }
}
