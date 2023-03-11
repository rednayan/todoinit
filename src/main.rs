use actix_web::{get,web,App,HttpServer};
use serde::{Deserialize,Serialize};
use std::sync::Mutex;

mod todolist;
use todolist::service;

#[derive(Serialize,Deserialize, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String
}

struct AppState {
    todolist_entries : Mutex<Vec<TodolistEntry>>
}

#[get("/")]
async fn index() ->  String {
    format!("This is a health check route")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries : Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(service::config)
    })
    .bind(("127.0.0.1",6969))?
    .run()
    .await
}   
