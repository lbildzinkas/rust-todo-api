use std::io;

use actix_web::{App, HttpServer};
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
    HttpServer::new(|| App::new())
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
