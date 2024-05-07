// fn main() {
//     println!("Hello, world!");

// }
use actix_web::{web, App, HttpServer};
use rust_todo_be::framework::api_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api_server::hello)
            .service(api_server::echo)
            .route("/hey", web::get().to(api_server::manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
