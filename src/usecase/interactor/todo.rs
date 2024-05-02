use crate::domain::todo::Todo;
use crate::usecase::data_access::todo::TodoDataAccess;
use crate::usecase::input_boundary::todo::TodoInputBoundary;

struct input_todo {
    todoDataAccess: dyn TodoDataAccess,
}

impl TodoInputBoundary for input_todo {
    fn create(&self, todo: Todo) -> Result<(), String> {
        self.todoDataAccess.create(todo)
    }

    fn update_title(&self, id: String, title: String) -> Result<(), String> {
        self.todoDataAccess.update_title(title)
    }

    fn update_content(&self, id: String, content: String) -> Result<(), String> {
        self.todoDataAccess.update_content(content)
    }

    fn delete(&self, id: String) -> Result<(), String> {
        self.todoDataAccess.delete(id)
    }

    fn get_all(&self) -> Result<Vec<Todo>, String> {
        self.todoDataAccess.get_all()
    }

    fn get_by_user_id(&self, user_id: String) -> Result<Vec<Todo>, String> {
        self.todoDataAccess.get_by_user_id(user_id)
    }
}
