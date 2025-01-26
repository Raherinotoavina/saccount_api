use crate::utils::validator::validate_password;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct UserDTO {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateUserDTO {
    #[validate(email(message = "Adresse email invalide"))]
    pub email: String,
    pub firstname: String,

    #[validate(length(min = 1))]
    pub lastname: String,

    #[validate(custom = "validate_password")]
    pub password: Option<String>,
}
