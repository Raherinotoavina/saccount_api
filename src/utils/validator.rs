use validator::ValidationError;

// Fonction de validation personnalisée
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    // Vérifie la longueur minimale
    if password.len() == 0 {
        return Ok(());
    } else if password.len() > 0 && password.len() < 8 {
        return Err(ValidationError::new(
            "La longueur du mot de passe doit être d'au moins 8 caractères",
        ));
    }

    // Vérifie s'il contient au moins un chiffre
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new(
            "Le mot de passe doit contenir au moins un chiffre",
        ));
    }

    // Vérifie s'il contient au moins un caractère spécial
    if !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        return Err(ValidationError::new(
            "Le mot de passe doit contenir au moins un caractère spécial",
        ));
    }

    // Vérifie s'il contient au moins une lettre majuscule
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new(
            "Le mot de passe doit contenir au moins une lettre majuscule",
        ));
    }

    Ok(())
}
