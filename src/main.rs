use std::io;

use actix_web::{web, App, HttpServer};
use rust_todo_api::configs::Configs;
use rust_todo_api::todo::todo_http_handlers;
use rust_todo_api::todo::todo_repository::TodoRepository;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let configs_result = Configs::new();
    let configs = match configs_result {
        Ok(c) => c,
        Err(error) => panic!("Error while reading the configuration files: {}", error),
    };

    let server_address = format!("0.0.0.0:{}", configs.http.port);

    println!("Running debug mode: {}", configs.debug);
    println!("Listening to: {}", server_address);

    let repo = TodoRepository::new(configs.database.connection_string);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .service(todo_http_handlers::list)
            .service(todo_http_handlers::new_list)
            .service(todo_http_handlers::update_list)
            .service(todo_http_handlers::delete_list)
            .service(todo_http_handlers::get_list_by_id)
            .service(todo_http_handlers::get_todos_by_list_id)
            .service(todo_http_handlers::get_todo_item_by_id)
            .service(todo_http_handlers::new_todo_item)
            .service(todo_http_handlers::update_todo_item)
            .service(todo_http_handlers::delete_todo_item)
            .route("/healthz", web::get().to(|| async { "OK" }))
    })
    .bind(server_address)?
    .run()
    .await
}
