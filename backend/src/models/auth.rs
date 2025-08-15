//! Modelos de autenticación
//! 
//! Este módulo contiene los structs para login, registro y respuestas de autenticación.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request de login
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "Password debe tener al menos 6 caracteres"))]
    pub password: String,
}

/// Request de registro
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub company_name: String,
    
    #[validate(email(message = "Email inválido"))]
    pub admin_email: String,
    
    #[validate(length(min = 3, max = 50, message = "Username debe tener entre 3 y 50 caracteres"))]
    pub admin_username: String,
    
    #[validate(length(min = 6, message = "Password debe tener al menos 6 caracteres"))]
    pub admin_password: String,
}

/// Respuesta de autenticación exitosa
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

/// Información del usuario para respuesta de autenticación
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub username: String,
    pub user_type: String,
    pub company_id: String,
    pub company_name: String,
}
