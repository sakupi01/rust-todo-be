use crate::domain::todo::Todo;

pub trait TodoDataAccess {
    fn create(&self, todo: Todo) -> Result<(), String>;
    fn update_title(&self, title: String) -> Result<(), String>;
    fn update_content(&self, content: String) -> Result<(), String>;
    fn delete(&self, id: String) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<Todo>, String>;
    fn get_by_user_id(&self, user_id: String) -> Result<Vec<Todo>, String>;
}
