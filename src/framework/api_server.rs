use crate::{
    domain::user::User,
    interface_adapter::controller::web_user::WebUserController,
    usecase::{
        data_access::user::UserDataAccess, input_boundary::user::UserInputBoundary,
        interactor::user::input_user,
    },
};
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

    fn get_all(&self) -> Result<Vec<crate::domain::user::User>, String> {
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

#[get("/user")]
async fn get_all_user() -> impl Responder {
    let foo = FakeDataAccess {};
    let bar = input_user {
        userDataAccess: foo,
    };

    let controller = WebUserController {
        userInputBoundary: bar,
    };
    let users = get_all_user();
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
