use crate::domain::todo::Todo;
struct WebTodoController{
    todoInputBoundary: TodoInputBoundary,
}
impl WebTodoController{
    fn create_todo(todo: &Todo)->Result<(), String>{
        todoInputBoundary.create(todo)
    }
    fn update_todo_title(todo: &Todo)->Result<(), String>{
        todoInputBoundary.update_title(todo.id, todo.title)
    }
    fn update_todo_content(todo: &Todo)->Result<(), String>{
        todoInputBoundary.update_content(todo.id, todo.content)
    }
    fn delete_todo(todo: &Todo)->Result<(), String>{
        todoInputBoundary.delete(todo.id)
    }
    fn get_all_todo()->Result<Vec<Todo>, String>{
        todoInputBoundary.get_all()
    }
    fn get_todo_by_user_id(user_id: String)->Result<Vec<Todo>, String>{
        todoInputBoundary.get_by_user_id(user_id)
    }
}

