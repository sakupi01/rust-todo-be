// fn main() {
//     println!("Hello, world!");

// }
use actix_web::{web, App, HttpServer};
use rust_todo_be::{db::init_todo_db::get_todo_db, framework::api_server};
use std::thread;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // writer thread
    let t1 = thread::spawn(|| {
        let mut todo_db = get_todo_db().lock().unwrap();
        todo_db.insert("hello".to_owned(), "world".to_owned());
    });
    // reader thread
    let t2 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        let todo_db = get_todo_db().lock().unwrap();
        println!("{:?}", todo_db.get("hello"));
    });
    t1.join().unwrap();
    t2.join().unwrap();

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
