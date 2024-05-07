use crate::interface_adapter::controller;
use crate::interface_adapter::controller::web_user::WebUserController;
use crate::usecase::interactor::user::input_user;
use crate::{domain::user::User, usecase::data_access::user::UserDataAccess};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;

struct FakeDataAccess {}
impl UserDataAccess for FakeDataAccess {
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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users")]
async fn get_all_users() -> impl Responder {
    let controller = WebUserController {
        userInputBoundary: (input_user {
            userDataAccess: FakeDataAccess {},
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
async fn create_user(user_info: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("Created {:?}!",user_info.0))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
