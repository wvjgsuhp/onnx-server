#[macro_use]
extern crate log;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod neuralnetwork;
mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = utils::get_env("HOST");
    let port = utils::get_env("PORT");
    
    neuralnetwork::init_env();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .configure(neuralnetwork::init_routes)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
