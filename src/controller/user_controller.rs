use crate::dto::response_dto::{ResponseDTO, ResponseStatus};
use crate::dto::user_dto::{CreateUserDTO, UserDTO};
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
        return HttpResponse::UnprocessableEntity().json(ResponseDTO {
            status: ResponseStatus::Error,
            data: errors,
        });
    }

    match create_user_service(db.get_ref(), req_body).await {
        Ok(user) => HttpResponse::Created().json(ResponseDTO {
            status: ResponseStatus::Success,
            data: UserDTO {
                id: user.id,
                lastname: user.lastname,
                email: user.email,
                firstname: user.firstname,
            },
        }),
        Err(error) => HttpResponse::BadRequest().json(ResponseDTO {
            status: ResponseStatus::Error,
            data: error.to_string(),
        }),
    }
}
