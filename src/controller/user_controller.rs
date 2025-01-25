use crate::dto::user_dto::CreateUserDTO;
use actix_web::{get, post, web, HttpResponse, Responder};
use validator::Validate;

#[get("/")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("All users")
}

#[post("/create_user")]
pub async fn create_user(req_body: web::Json<CreateUserDTO>) -> impl Responder {
    if let Err(errors) = req_body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    HttpResponse::Created().json(req_body)
}
