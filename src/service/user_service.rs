use crate::dto::user_dto::CreateUserDTO;
use crate::entity::user_entity;
use actix_web::web;
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

pub async fn create_user_service(
    db: &DatabaseConnection,
    user: web::Json<CreateUserDTO>,
) -> Result<user_entity::Model, DbErr> {
    let new_user = user_entity::ActiveModel {
        email: Set(user.email.to_owned()),
        firstname: Set(user.firstname.to_owned()),
        lastname: Set(user.lastname.to_owned()),
        ..Default::default()
    };

    new_user.insert(db).await
}
