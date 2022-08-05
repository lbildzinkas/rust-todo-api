use std::str::FromStr;

use crate::{
    todo::{TodoItem, TodoList},
    APPLICATION_JSON,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Json, Path},
    HttpResponse, Responder,
};

use uuid::Uuid;

use super::todo_repository::IRepository;

#[get("/todo-list")]
pub async fn list(repo: web::Data<dyn IRepository>) -> impl Responder {
    let lists = repo.get_all().await;

    match lists {
        Ok(lists) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(lists),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/todo-list/{id}")]
pub async fn get_list_by_id(
    repo: web::Data<dyn IRepository>,
    path: Path<(String,)>,
) -> impl Responder {
    let id = Uuid::from_str(path.0.as_str());

    match id {
        Ok(id) => {
            let todo_list = repo.get_by_id(id).await;

            match todo_list {
                Ok(Some(todo_list)) => HttpResponse::Ok()
                    .content_type(APPLICATION_JSON)
                    .json(todo_list),
                Ok(None) => HttpResponse::NotFound().finish(),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[put("/todo-list/{id}")]
pub async fn update_list(
    repo: web::Data<dyn IRepository>,

    todo_list: Json<TodoList>,
) -> impl Responder {
    let todo_list = repo.update_list(todo_list.into_inner()).await;
    match todo_list {
        Ok(todo_list) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(todo_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/todo-list/{id}")]
pub async fn delete_list(
    repo: web::Data<dyn IRepository>,
    path: Path<(String,)>,
) -> impl Responder {
    let id = Uuid::from_str(path.0.as_str());
    match id {
        Ok(id) => {
            let deleted = repo.delete_list(id).await;
            match deleted {
                Ok(deleted) => HttpResponse::Ok()
                    .content_type(APPLICATION_JSON)
                    .json(deleted),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("/todo-list/{id}/todo")]
pub async fn get_todos_by_list_id(
    repo: web::Data<dyn IRepository>,
    path: Path<(String,)>,
) -> impl Responder {
    let list_id = Uuid::from_str(path.0.as_str());
    match list_id {
        Ok(list_id) => {
            let todos_result = repo.get_list_items(list_id).await;
            match todos {
                Ok(todos) => HttpResponse::Ok()
                    .content_type(APPLICATION_JSON)
                    .json(todos),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/todo-list")]
pub async fn new_list(
    repo: web::Data<dyn IRepository>,
    todo_list: Json<TodoList>,
) -> impl Responder {
    let todo_list = repo.create_list(todo_list.into_inner()).await;

    match todo_list {
        Ok(todo_list) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(todo_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/todo_item/{id}")]
pub async fn get_todo_item_by_id(
    repo: web::Data<dyn IRepository>,
    path: Path<(String,)>,
) -> impl Responder {
    let id = Uuid::from_str(path.0.as_str());
    match id {
        Ok(id) => {
            let todo_item = repo.get_item(id).await;

            match todo_item {
                Ok(Some(todo_item)) => HttpResponse::Ok()
                    .content_type(APPLICATION_JSON)
                    .json(todo_item),
                Ok(None) => HttpResponse::NotFound().finish(),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/todo_item")]
pub async fn new_todo_item(
    repo: web::Data<dyn IRepository>,
    todo: Json<TodoItem>,
) -> impl Responder {
    let todo_item = repo.create_item(todo.into_inner()).await;
    match todo_item {
        Ok(todo_item) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(todo_item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/todo_item/{id}")]
pub async fn update_todo_item(
    repo: web::Data<dyn IRepository>,
    todo: Json<TodoItem>,
) -> impl Responder {
    let todo_item = repo.update_item(todo.into_inner()).await;
    match todo_item {
        Ok(todo_item) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(todo_item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/todo_item/{id}")]
pub async fn delete_todo_item(
    repo: web::Data<dyn IRepository>,
    path: Path<(String,)>,
) -> impl Responder {
    let id = Uuid::from_str(path.0.as_str());
    match id {
        Ok(id) => {
            let deleted = repo.delete_item(id).await;
            match deleted {
                Ok(deleted) => HttpResponse::Ok()
                    .content_type(APPLICATION_JSON)
                    .json(deleted),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
