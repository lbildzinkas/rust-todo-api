use std::str::FromStr;

use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse,
};
use uuid::Uuid;

use crate::{
    todo::{TodoItem, TodoList},
    APPLICATION_JSON,
};

#[get("/todo-list")]
pub async fn list() -> HttpResponse {
    let lists: Vec<TodoList> = Vec::new();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(lists)
}

#[get("/todo-list/{id}")]
pub async fn get_list_by_id(path: Path<(String,)>) -> HttpResponse {
    let id = Uuid::from_str(path.0.as_str());

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(id.unwrap())
}

#[get("/todo-list/{id}/todo")]
pub async fn get_todos_by_list_id(path: Path<(String,)>) -> HttpResponse {
    let list_id = Uuid::from_str(path.0.as_str());
    let todos: Vec<TodoItem> = Vec::new();
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(todos)
}

#[post("/todo-list")]
pub async fn new_list(todo_list: Json<TodoList>) -> HttpResponse {
    let todo_list_id = Uuid::new_v4();
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(todo_list_id)
}

#[get("/todo/{id}")]
pub async fn get_todo_by_id(path: Path<(String,)>) -> HttpResponse {
    let id = Uuid::from_str(path.0.as_str());
    let todos: Vec<TodoItem> = Vec::new();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(todos)
}

#[post("/todo")]
pub async fn new_todo(todo: Json<TodoItem>) -> HttpResponse {
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(todo)
}
