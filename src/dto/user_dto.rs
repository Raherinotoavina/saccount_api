use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateUserDTO {
    #[validate(email(message = "Adresse email invalide"))]
    pub username: String,

    #[validate(length(
        min = 8,
        message = "Le mot de passe doit contenir au moins 8 caractères"
    ))]
    pub password: String,
}
