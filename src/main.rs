use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use dotenv::dotenv;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the index!")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
