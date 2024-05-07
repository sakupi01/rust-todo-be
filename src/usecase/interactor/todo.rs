use crate::domain::todo::Todo;
use crate::usecase::data_access::todo::TodoDataAccess;
use crate::usecase::input_boundary::todo::TodoInputBoundary;

pub struct InputTodo<T> {
    pub todo_data_access: T,
}

impl<T> TodoInputBoundary for InputTodo<T>
where
    T: TodoDataAccess,
{
    fn create(&self, todo: Todo) -> Result<(), String> {
        self.todo_data_access.create(todo)
    }

    fn update_title(&self, id: String, title: String) -> Result<(), String> {
        self.todo_data_access.update_title(title)
    }

    fn update_content(&self, id: String, content: String) -> Result<(), String> {
        self.todo_data_access.update_content(content)
    }

    fn delete(&self, id: String) -> Result<(), String> {
        self.todo_data_access.delete(id)
    }

    fn get_all(&self) -> Result<Vec<Todo>, String> {
        self.todo_data_access.get_all()
    }

    fn get_by_user_id(&self, user_id: String) -> Result<Vec<Todo>, String> {
        self.todo_data_access.get_by_user_id(user_id)
    }
}
