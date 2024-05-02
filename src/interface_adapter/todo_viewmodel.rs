use std::option;
pub struct ResultTodoViewModel {
    pub error:option::Option<String>,
    pub success:option::Option<String>,
    pub error_message:option::Option<String>,
}

pub struct TodoViewModel {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub title: String,
    pub content: String,
    pub user_id: String,
}

pub struct ResultGetTodoViewModel {
    pub error:option::Option<String>,
    pub success:option::Option<String>,
    pub error_message:option::Option<String>,
    pub todos:option::Option<Vec<TodoViewModel>>,
}
