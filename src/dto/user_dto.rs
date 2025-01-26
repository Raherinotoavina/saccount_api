use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateUserDTO {
    #[validate(email(message = "Adresse email invalide"))]
    pub email: String,
    pub firstname: String,

    #[validate(length(min = 1))]
    pub lastname: String,
}
