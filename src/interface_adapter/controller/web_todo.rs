use crate::domain::todo::Todo;
use crate::usecase::input_boundary::todo::TodoInputBoundary;

struct WebTodoController {
    todoInputBoundary: dyn TodoInputBoundary,
}
impl WebTodoController {
    fn create_todo(&self, todo: &Todo) -> Result<(), String> {
        self.todoInputBoundary.create(todo.clone())
    }
    fn update_todo_title(&self, todo: &Todo) -> Result<(), String> {
        self.todoInputBoundary
            .update_title(todo.id.clone(), todo.title.clone())
    }
    fn update_todo_content(&self, todo: &Todo) -> Result<(), String> {
        self.todoInputBoundary
            .update_content(todo.id.clone(), todo.content.clone())
    }
    fn delete_todo(&self, todo: &Todo) -> Result<(), String> {
        self.todoInputBoundary.delete(todo.id.clone())
    }
    fn get_all_todo(&self) -> Result<Vec<Todo>, String> {
        self.todoInputBoundary.get_all()
    }
    fn get_todo_by_user_id(&self, user_id: String) -> Result<Vec<Todo>, String> {
        self.todoInputBoundary.get_by_user_id(user_id)
    }
}
