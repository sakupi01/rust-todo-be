use crate::domain::todo::Todo;
use crate::interface_adapter::controller;
use crate::interface_adapter::controller::web_todo::WebTodoController;
use crate::interface_adapter::controller::web_user::WebUserController;
use crate::usecase::interactor::user::input_user;
use crate::{
    domain::user::User,
    usecase::data_access::{todo::TodoDataAccess, user::UserDataAccess},
};
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use serde::{Deserialize, Serialize};

//=======================================================================
struct FakeUserDataAccess {}
impl UserDataAccess for FakeUserDataAccess {
    fn create(&self, user: User) -> Result<(), String> {
        todo!()
    }

    fn update_name(&self, name: String) -> Result<(), String> {
        todo!()
    }

    fn delete(&self, id: String) -> Result<(), String> {
        todo!()
    }

    fn get_all(&self) -> Result<Vec<User>, String> {
        Ok(vec![User {
            id: "綾鷹".to_string(),
            created_at: Local::now(),
            updated_at: Local::now(),
            name: "綾鷹".to_string(),
        }])
    }

    fn get_by_id(&self, id: String) -> Result<User, String> {
        todo!()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserInputDto {
    name: String,
}

#[get("/users")]
async fn get_all_users() -> impl Responder {
    let controller = WebUserController {
        userInputBoundary: (input_user {
            userDataAccess: FakeUserDataAccess {},
        }),
    };
    let users = controller.get_all_user().unwrap();
    HttpResponse::Ok().body(format!("Welcome, users {:?}!", users[0]))
}

#[get("/users/{user_id}")]
async fn get_user_by_id(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    println!("user_id: {:?}", user_id);
    HttpResponse::Ok().body(format!("Welcome, user_id {}!", user_id))
}

#[post("/users")]
async fn create_user(user_input: web::Json<UserInputDto>) -> impl Responder {
    HttpResponse::Ok().body("Created Success!")
}

#[put("/users")]
async fn update_user(user_input: web::Json<UserInputDto>) -> impl Responder {
    HttpResponse::Ok().body("Updated Success!")
}

#[delete("/users")]
async fn delete_user(user_input: web::Json<UserInputDto>) -> impl Responder {
    HttpResponse::Ok().body("Deleted Success!")
}

//=======================================================================
struct FakeTodoDataAccess {}
impl TodoDataAccess for FakeTodoDataAccess {
    fn create(&self, todo: Todo) -> Result<(), String> {
        todo!()
    }

    fn update_title(&self, name: String) -> Result<(), String> {
        todo!()
    }

    fn update_content(&self, content: String) -> Result<(), String> {
        todo!()
    }

    fn delete(&self, id: String) -> Result<(), String> {
        todo!()
    }

    fn get_all(&self) -> Result<Vec<Todo>, String> {
        todo!()
    }

    fn get_by_user_id(&self, id: String) -> Result<Vec<Todo>, String> {
        todo!()
    }
}

#[get("/todo")]
async fn get_all_todo() -> impl Responder {
    let controller = WebTodoController {
        todoInputBoundary: (input_user {
            todoDataAccess: FakeTodoDataAccess {},
        }),
    };
    let users = controller.get_all_user().unwrap();
    HttpResponse::Ok().body(format!("Welcome, users {:?}!", users[0]))
}

//=======================================================================
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
