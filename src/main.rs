mod controller;
mod dto;
mod routes;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(web::scope("/user").configure(routes::user_route::user_route_config)),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
