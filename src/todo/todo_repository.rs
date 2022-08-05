use futures::future::BoxFuture;
use uuid::Uuid;

use super::{TodoItem, TodoList};

pub struct TodoRepository {
    conn_string: String,
}

pub trait IRepository {
    fn get_all(&self) -> BoxFuture<Result<Vec<TodoList>, ()>>;
    fn get_by_id(&self, id: Uuid) -> BoxFuture<Result<Option<TodoList>, ()>>;
    fn get_list_items(&self, list_id: Uuid) -> BoxFuture<Result<Vec<TodoItem>, ()>>;
    fn get_item(&self, id: Uuid) -> BoxFuture<Result<Option<TodoItem>, ()>>;
    fn create_list(&self, todo: TodoList) -> BoxFuture<Result<TodoList, ()>>;
    fn create_item(&self, todo: TodoItem) -> BoxFuture<Result<TodoItem, ()>>;
    fn update_list(&self, todo: TodoList) -> BoxFuture<Result<TodoList, ()>>;
    fn update_item(&self, todo: TodoItem) -> BoxFuture<Result<TodoItem, ()>>;
    fn delete_list(&self, id: Uuid) -> BoxFuture<Result<bool, ()>>;
    fn delete_item(&self, id: Uuid) -> BoxFuture<Result<bool, ()>>;
}

#[allow(unused)]
impl IRepository for TodoRepository {
    fn get_all(&self) -> BoxFuture<Result<Vec<TodoList>, ()>> {
        unimplemented!()
    }
    fn get_by_id(&self, id: Uuid) -> BoxFuture<Result<Option<TodoList>, ()>> {
        unimplemented!()
    }
    fn get_list_items(&self, list_id: Uuid) -> BoxFuture<Result<Vec<TodoItem>, ()>> {
        unimplemented!()
    }
    fn get_item(&self, id: Uuid) -> BoxFuture<Result<Option<TodoItem>, ()>> {
        unimplemented!()
    }
    fn create_list(&self, todo: TodoList) -> BoxFuture<Result<TodoList, ()>> {
        unimplemented!()
    }
    fn create_item(&self, todo: TodoItem) -> BoxFuture<Result<TodoItem, ()>> {
        unimplemented!()
    }
    fn update_list(&self, todo: TodoList) -> BoxFuture<Result<TodoList, ()>> {
        unimplemented!()
    }
    fn delete_list(&self, id: Uuid) -> BoxFuture<Result<bool, ()>> {
        unimplemented!()
    }
    fn update_item(&self, todo: TodoItem) -> BoxFuture<Result<TodoItem, ()>> {
        todo!()
    }
    fn delete_item(&self, id: Uuid) -> BoxFuture<Result<bool, ()>> {
        todo!()
    }
}

impl Clone for TodoRepository {
    fn clone(&self) -> Self {
        Self {
            conn_string: self.conn_string.clone(),
        }
    }
}

impl TodoRepository {
    pub fn new(conn_string: String) -> Self {
        Self { conn_string }
    }
}
