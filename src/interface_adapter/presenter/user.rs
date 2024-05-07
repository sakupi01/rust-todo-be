use crate::{
    interface_adapter::{
        viewmodel::todo::ResultTodoViewModel, viewmodel::user::ResultUserViewModel,
    },
    usecase::output_data::user::User,
};

fn create_user(res: Result<(), String>) -> ResultUserViewModel {
    match res {
        Ok(_) => ResultUserViewModel {
            success: Some("Success".to_string()),
            error: None,
            error_message: None,
        },
        Err(e) => ResultUserViewModel {
            success: None,
            error: Some(e.to_string()),
            error_message: None,
        },
    }
}

fn update_name(res: Result<(), String>) -> ResultUserViewModel {
    match res {
        Ok(_) => ResultUserViewModel {
            success: Some("Success".to_string()),
            error: None,
            error_message: None,
        },
        Err(e) => ResultUserViewModel {
            success: None,
            error: Some(e.to_string()),
            error_message: None,
        },
    }
}
