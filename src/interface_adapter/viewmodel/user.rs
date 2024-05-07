use std::option::Option;

pub struct ResultUserViewModel {
    pub error: Option<String>,
    pub success: Option<String>,
    pub error_message: Option<String>,
}

pub struct UserViewModel {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct ResultGetUserViewModel {
    pub error: Option<String>,
    pub success: Option<String>,
    pub error_message: Option<String>,
    pub users: Option<Vec<UserViewModel>>,
}
