use crate::dto::user_dto::CreateUserDTO;
use crate::entity::user_entity;
use actix_web::web;
use bcrypt::hash;
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

pub async fn create_user_service(
    db: &DatabaseConnection,
    user: web::Json<CreateUserDTO>,
) -> Result<user_entity::Model, DbErr> {
    let mut hashed_password = None;

    if let Some(password) = user.password.clone() {
        let pwd = hash(password, 7);
        if let Ok(pwd) = pwd {
            hashed_password = Some(pwd.to_string());
        }
    }

    let new_user = user_entity::ActiveModel {
        email: Set(user.email.to_owned()),
        firstname: Set(user.firstname.to_owned()),
        lastname: Set(user.lastname.to_owned()),
        password: Set(hashed_password),
        ..Default::default()
    };

    new_user.insert(db).await
}
