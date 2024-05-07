use crate::domain::todo::Todo;
use crate::usecase::input_boundary::todo::TodoInputBoundary;

pub struct WebTodoController<T> {
    pub todo_input_boundary: T,
}
impl<T> WebTodoController<T>
where
    T: TodoInputBoundary,
{
    pub fn create_todo(&mut self, todo: &Todo) -> Result<(), String> {
        self.todo_input_boundary.create(todo.clone())
    }
    pub fn update_todo_title(&mut self, todo: &Todo) -> Result<(), String> {
        self.todo_input_boundary
            .update_title(todo.id.clone(), todo.title.clone())
    }
    pub fn update_todo_content(&mut self, todo: &Todo) -> Result<(), String> {
        self.todo_input_boundary
            .update_content(todo.id.clone(), todo.content.clone())
    }
    pub fn delete_todo(&mut self, todo: &Todo) -> Result<(), String> {
        self.todo_input_boundary.delete(todo.id.clone())
    }
    pub fn get_all_todo(&self) -> Result<Vec<Todo>, String> {
        self.todo_input_boundary.get_all()
    }
    pub fn get_todo_by_user_id(&self, user_id: String) -> Result<Vec<Todo>, String> {
        self.todo_input_boundary.get_by_user_id(user_id)
    }
}
