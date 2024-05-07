use crate::{
    domain::todo::Todo,
    interface_adapter::{
        viewmodel::todo::TodoViewModel,
        viewmodel::todo::{ResultGetTodoViewModel, ResultTodoViewModel},
    },
};

fn create(res: Result<(), String>) -> ResultTodoViewModel {
    match res {
        Ok(_) => ResultTodoViewModel {
            success: Some("Success".to_string()),
            error: None,
            error_message: None,
        },
        Err(e) => ResultTodoViewModel {
            success: None,
            error: Some(e.to_string()),
            error_message: None,
        },
    }
}
fn update_title(res: Result<(), String>) -> ResultTodoViewModel {
    match res {
        Ok(_) => ResultTodoViewModel {
            success: Some("Success".to_string()),
            error: None,
            error_message: None,
        },
        Err(e) => ResultTodoViewModel {
            success: None,
            error: Some(e.to_string()),
            error_message: None,
        },
    }
}
fn update_content(res: Result<(), String>) -> ResultTodoViewModel {
    match res {
        Ok(_) => ResultTodoViewModel {
            success: Some("Success".to_string()),
            error: None,
            error_message: None,
        },
        Err(e) => ResultTodoViewModel {
            success: None,
            error: Some(e.to_string()),
            error_message: None,
        },
    }
}
fn delete(res: Result<(), String>) -> ResultTodoViewModel {
    match res {
        Ok(_) => ResultTodoViewModel {
            success: Some("Success".to_string()),
            error: None,
            error_message: None,
        },
        Err(e) => ResultTodoViewModel {
            success: None,
            error: Some(e.to_string()),
            error_message: None,
        },
    }
}

fn get_all(res: Result<Vec<Todo>, String>) -> ResultGetTodoViewModel {
    match res {
        Ok(todos) => {
            let todos = todos
                .iter()
                .map(|todo| TodoViewModel {
                    id: todo.id.clone(),
                    created_at: todo.created_at.to_string(),
                    updated_at: todo.updated_at.to_string(),
                    title: todo.title.clone(),
                    content: todo.content.clone(),
                    user_id: todo.user_id.clone(),
                })
                .collect();
            ResultGetTodoViewModel {
                success: None,
                error: None,
                error_message: None,
                todos: Some(todos),
            }
        }
        Err(e) => ResultGetTodoViewModel {
            success: None,
            error: Some(e.to_string()),
            error_message: None,
            todos: None,
        },
    }
}

fn get_by_user_id(res: Result<Vec<Todo>, String>) -> ResultGetTodoViewModel {
    match res {
        Ok(todos) => {
            let todos = todos
                .iter()
                .map(|todo| TodoViewModel {
                    id: todo.id.clone(),
                    created_at: todo.created_at.to_string(),
                    updated_at: todo.updated_at.to_string(),
                    title: todo.title.clone(),
                    content: todo.content.clone(),
                    user_id: todo.user_id.clone(),
                })
                .collect();
            ResultGetTodoViewModel {
                success: None,
                error: None,
                error_message: None,
                todos: Some(todos),
            }
        }
        Err(e) => ResultGetTodoViewModel {
            success: None,
            error: Some(e.to_string()),
            error_message: None,
            todos: None,
        },
    }
}
