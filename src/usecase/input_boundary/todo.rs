use crate::domain::todo::Todo;

pub trait TodoInputBoundary {
    fn create(&self, todo: Todo) -> Result<(), String>;
    fn update_title(&self, id: String, title: String) -> Result<(), String>;
    fn update_content(&self, id: String, content: String) -> Result<(), String>;
    fn delete(&self, id: String) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<Todo>, String>;
    fn get_by_user_id(&self, user_id: String) -> Result<Vec<Todo>, String>;
}
