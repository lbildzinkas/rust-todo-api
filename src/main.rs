use std::io;

use actix_web::{App, HttpServer};
use rust_todo_api::todo::{self, todo_http_handlers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    connection: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            connection: Default::default(),
        }
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let cfg: Config = confy::load("rust-todo-api").unwrap();
    dbg!(cfg);

    HttpServer::new(|| {
        App::new()
            .service(todo_http_handlers::list)
            .service(todo_http_handlers::new_list)
            .service(todo_http_handlers::get_list_by_id)
            .service(todo_http_handlers::get_todos_by_list_id)
            .service(todo_http_handlers::get_todo_by_id)
            .service(todo_http_handlers::new_todo)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
