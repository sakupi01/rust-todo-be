// fn main() {
//     println!("Hello, world!");

// }
use actix_web::{web, App, HttpServer};
use chrono::Local;
use rust_todo_be::{db::init_todo_db::get_todo_db, domain::todo::Todo, framework::api_server, usecase::data_access::todo::TodoDataAccess};
use std::{borrow::Borrow, ops::DerefMut, thread};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let t1 = thread::spawn(|| {
        let mut todo_db = get_todo_db().lock().unwrap();
        let todo_test = Todo{
            id: "todo_id".to_string(),
            created_at: Local::now(),
            updated_at: Local::now(),
            title: "todo dayo".to_string(),
            content: "content dayo".to_string(),
            user_id: "user_id dayo".to_string(),
        };
        todo_db.deref_mut().create(todo_test);
    });
    t1.join().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(api_server::hello)
            .service(api_server::get_all_users)
            .service(api_server::get_user_by_id)
            .service(api_server::create_user)
            .service(api_server::update_user)
            .service(api_server::delete_user)
            .service(api_server::get_all_todo)
            .service(api_server::update_title)
            .service(api_server::update_content)
            .service(api_server::delete_todo)
            .service(api_server::get_todo_by_user_id)
            .service(api_server::echo)
            .route("/hey", web::get().to(api_server::manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
