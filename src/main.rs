use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
mod controller;
mod dto;
mod routes;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(web::scope("/user").configure(routes::user_route::user_route_config)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
