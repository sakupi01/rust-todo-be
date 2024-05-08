use crate::db::init_todo_db::get_todo_db;
use crate::db::init_user_db::get_user_db;
use crate::db::ram_zatsu_todo_db::RamZatsuTodoDb;
use crate::domain::todo::Todo;
use crate::domain::user;
use crate::interface_adapter::controller;
use crate::interface_adapter::controller::web_todo::WebTodoController;
use crate::interface_adapter::controller::web_user::WebUserController;
use crate::usecase::interactor::todo::InputTodo;
use crate::usecase::interactor::user::input_user;
use crate::{
    domain::user::User,
    usecase::data_access::{todo::TodoDataAccess, user::UserDataAccess},
};
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f32::INFINITY;
use std::ops::{Deref, DerefMut};

//=======================================================================
struct FakeUserDataAccess {}
impl UserDataAccess for FakeUserDataAccess {
    fn create(&mut self, user: User) -> Result<(), String> {
        Ok(())
    }

    fn update_name(&mut self, id: String, name: String) -> Result<(), String> {
        Ok(())
    }

    fn delete(&mut self, id: String) -> Result<(), String> {
        Ok(())
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UpdateUserDto {
    id: String,
    name: String,
}

#[get("/users")]
async fn get_all_users() -> impl Responder {
    let mut user_db = get_user_db().lock().unwrap();
    let user_data_access = user_db.deref_mut();
    let controller = WebUserController {
        userInputBoundary: input_user {
            userDataAccess: user_data_access,
        },
    };
    let users = controller.get_all_user();
    HttpResponse::Ok().body(format!("Welcome, users {:?}!", users))
}

#[get("/users/{user_id}")]
async fn get_user_by_id(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();

    let mut user_db = get_user_db().lock().unwrap();
    let user_data_access = user_db.deref_mut();
    let controller = WebUserController {
        userInputBoundary: input_user {
            userDataAccess: user_data_access,
        },
    };

    match controller.get_user_by_id(user_id) {
        Ok(user) => HttpResponse::Ok().body(format!("Welcome, user_id {:?}!", user)),
        Err(_) => HttpResponse::BadRequest().body("err"),
    }
}

#[post("/users")]
async fn create_user(user_input: web::Json<UserInputDto>) -> impl Responder {
    let UserInputDto { name } = user_input.deref().clone();

    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(0..INFINITY as i32);

    let user = User {
        id: random_int.to_string(),
        created_at: Local::now(),
        updated_at: Local::now(),
        name: name,
    };

    let mut user_db = get_user_db().lock().unwrap();
    let userDataAccess = user_db.deref_mut();
    let mut controller = WebUserController {
        userInputBoundary: input_user { userDataAccess },
    };
    let _ = controller.create_user(&user);

    HttpResponse::Ok().body(format!("Created Success: {:?}", user))
}

#[put("/users")]
async fn update_user(update_user_dto: web::Json<UpdateUserDto>) -> impl Responder {
    let UpdateUserDto { id, name } = update_user_dto.deref().clone();
    
    let mut users_db = get_user_db().lock().unwrap();
    let userDataAccess = users_db.deref_mut();
    let mut controller = WebUserController { 
        userInputBoundary: input_user { userDataAccess },
    };
    let result = controller.get_user_by_id(id);

    if let Ok(u) = result { 
        let updated_user = User {
            name: name,
            ..u
        };
        let _ = controller.update_user_name(&updated_user);
        HttpResponse::Ok().body(format!("Updated Success: {:?}", update_user_dto))
    } else {
        HttpResponse::BadRequest().body(format!("Updated Failed: {:?}", result))
    }
}

#[delete("/users")]
async fn delete_user(user_input: web::Json<UserInputDto>) -> impl Responder {
    HttpResponse::Ok().body("Deleted Success!")
}

//=======================================================================
struct FakeTodoDataAccess {}
impl TodoDataAccess for FakeTodoDataAccess {
    fn create(&mut self, todo: Todo) -> Result<(), String> {
        Ok(())
    }

    fn update_title(&mut self, id: String, name: String) -> Result<(), String> {
        todo!()
    }

    fn update_content(&mut self, id: String, content: String) -> Result<(), String> {
        todo!()
    }

    fn delete(&mut self, id: String) -> Result<(), String> {
        todo!()
    }

    fn get_all(&self) -> Result<Vec<Todo>, String> {
        Ok(vec![])
    }

    fn get_by_user_id(&self, id: String) -> Result<Vec<Todo>, String> {
        todo!()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CreateTodoDto {
    title: String,
    content: String,
    user_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UpdateTitleDto {
    id: String,
    title: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UpdateContentDto {
    id: String,
    content: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeleteTodoDto {
    id: String,
}

#[get("/todo")]
async fn get_all_todo() -> impl Responder {
    let mut todo_db = get_todo_db().lock().unwrap();
    let todo_data_access = todo_db.deref_mut();
    let controller = WebTodoController {
        todo_input_boundary: InputTodo { todo_data_access },
    };
    let todo = controller.get_all_todo();

    HttpResponse::Ok().body(format!("Welcome, users {:?}!", todo))
}

#[post("/todo")]
async fn create_todo(create_todo_dto: web::Json<CreateTodoDto>) -> impl Responder {
    let CreateTodoDto {
        title,
        content,
        user_id,
    } = create_todo_dto.deref().clone();
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(0..INFINITY as i32);
    let todo = Todo {
        id: random_int.to_string(),
        created_at: Local::now(),
        updated_at: Local::now(),
        title: title,
        content: content,
        user_id: user_id,
    };

    let mut todo_db = get_todo_db().lock().unwrap();
    let todo_data_access = todo_db.deref_mut();
    let mut controller = WebTodoController {
        todo_input_boundary: InputTodo { todo_data_access },
    };
    let _ = controller.create_todo(&todo);

    HttpResponse::Ok().body(format!("Created Success: {:?}", todo))
}

#[put("/todo/title")]
async fn update_title(update_title_dto: web::Json<UpdateTitleDto>) -> impl Responder {
    let UpdateTitleDto { id, title } = update_title_dto.deref().clone();

    let mut todo_db = get_todo_db().lock().unwrap();
    let todo_data_access = todo_db.deref_mut();
    let mut controller = WebTodoController {
        todo_input_boundary: InputTodo { todo_data_access },
    };
    let todo = controller.get_todo_by_id(id);
    if let Ok(t) = todo {
        let updatedTodo = Todo {
            updated_at: Local::now(),
            title: title,
            ..t
        };
        let _ = controller.update_todo_title(&updatedTodo);
        HttpResponse::Ok().body(format!("Updated Success: {:?}", update_title_dto))
    } else {
        HttpResponse::BadRequest().body(format!("Updated Failed: {:?}", update_title_dto))
    }
}

#[put("/todo/content")]
async fn update_content(update_content_dto: web::Json<UpdateContentDto>) -> impl Responder {
    let UpdateContentDto { id, content } = update_content_dto.deref().clone();

    let mut todo_db = get_todo_db().lock().unwrap();
    let todo_data_access = todo_db.deref_mut();
    let mut controller = WebTodoController {
        todo_input_boundary: InputTodo { todo_data_access },
    };
    let todoResult = controller.get_todo_by_id(id);

    if let Ok(t) = todoResult {
        let updatedTodo = Todo {
            updated_at: Local::now(),
            content: content,
            ..t
        };
        let _ = controller.update_todo_content(&updatedTodo);
        HttpResponse::Ok().body(format!("Updated Success: {:?}", update_content_dto));
    } else if let Err(e) = todoResult {
        return HttpResponse::BadRequest().body(format!("Updated Failed: {:?}", e));
    }

    HttpResponse::Ok().body(format!("Updated Success: {:?}", update_content_dto))
}

#[delete("/todo")]
async fn delete_todo(delete_todo_dto: web::Json<DeleteTodoDto>) -> impl Responder {
    let DeleteTodoDto { id } = delete_todo_dto.deref().clone();
    let mut todo_db = get_todo_db().lock().unwrap();
    let todo_data_access = todo_db.deref_mut();
    let mut controller = WebTodoController {
        todo_input_boundary: InputTodo { todo_data_access },
    };
    let todoResult = controller.get_todo_by_id(id);

    if let Ok(t) = todoResult {
        let _ = controller.delete_todo(&t);
        HttpResponse::Ok().body(format!("Deleted Success: {:?}", delete_todo_dto));
    } else if let Err(e) = todoResult {
        return HttpResponse::BadRequest().body(format!("Deleted Failed: {:?}", e));
    }

    HttpResponse::Ok().body(format!("Deleted Success: {:?}", delete_todo_dto))
}

#[get("/users/{user_id}/todo")]
async fn get_todo_by_user_id(user_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Success: {:?}", user_id))
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
