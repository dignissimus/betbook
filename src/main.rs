mod data;
mod database;

use data::*;
use database::DatabaseConnection;

use std::sync::Mutex;

use actix_files::{Files, NamedFile};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

type LockableDatabase = Mutex<DatabaseConnection>;

const DEFAULT_BALANCE: u64 = 500;

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("frontend/index.html")?)
}

#[get("/events")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize)]
struct CreateEvent {
    title: String,
}

#[post("/event/create")]
async fn create_event(
    request: web::Json<CreateEvent>,
    database: web::Data<LockableDatabase>,
) -> impl Responder {
    let database = database.lock().unwrap();
    database
        .create_event(&request.title, &String::from("dummy-identifier"))
        .expect("Unable to write to the database");
    HttpResponse::Ok().body("ok")
}

#[get("/events")]
async fn list_events(database: web::Data<LockableDatabase>) -> impl Responder {
    let events = database.lock().unwrap().get_events();
    HttpResponse::Ok().json(events)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let database = DatabaseConnection::new();
        let locked = Mutex::new(database);
        let web_data = web::Data::new(locked);
        App::new()
            .app_data(web_data)
            .service(create_event)
            .service(list_events)
            .service(index)
            .service(Files::new("/static", "frontend/static"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
