use crate::dto::response_dto::ResponseDTO;
use crate::dto::user_dto::CreateUserDTO;
use crate::service::user_service::create_user_service;
use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use std::string::ToString;
use validator::Validate;

#[get("/")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("All users")
}

#[post("/create_user")]
pub async fn create_user(
    req_body: web::Json<CreateUserDTO>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    if let Err(errors) = req_body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let res = create_user_service(db.get_ref(), req_body).await;

    match res {
        Ok(user) => HttpResponse::Created().json(ResponseDTO {
            status: "Ok".to_string(),
        }),
        Err(error) => HttpResponse::BadRequest().json(ResponseDTO {
            status: "Error".to_string(),
        }),
    }
}
