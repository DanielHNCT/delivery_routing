//! Modelo de User
//! 
//! Este módulo contiene el struct User, tipos de usuario y variantes para CRUD operations.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Tipos de usuario
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    Admin,
    Driver,
}

/// User principal
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub company_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub user_type: UserType,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Request para crear un nuevo usuario
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 50, message = "Username debe tener entre 3 y 50 caracteres"))]
    pub username: String,
    
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "Password debe tener al menos 6 caracteres"))]
    pub password: String,
    
    #[validate(length(max = 100, message = "Nombre no puede exceder 100 caracteres"))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100, message = "Apellido no puede exceder 100 caracteres"))]
    pub last_name: Option<String>,
    
    #[validate(length(max = 20, message = "Teléfono no puede exceder 20 caracteres"))]
    pub phone: Option<String>,
    
    pub user_type: UserType,
}

/// Request para actualizar un usuario
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 3, max = 50, message = "Username debe tener entre 3 y 50 caracteres"))]
    pub username: Option<String>,
    
    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
    
    #[validate(length(min = 6, message = "Password debe tener al menos 6 caracteres"))]
    pub password: Option<String>,
    
    #[validate(length(max = 100, message = "Nombre no puede exceder 100 caracteres"))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100, message = "Apellido no puede exceder 100 caracteres"))]
    pub last_name: Option<String>,
    
    #[validate(length(max = 20, message = "Teléfono no puede exceder 20 caracteres"))]
    pub phone: Option<String>,
    
    pub user_type: Option<UserType>,
    pub is_active: Option<bool>,
}

/// Request de login
#[derive(Debug, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password es requerido"))]
    pub password: String,
}

/// Usuario sin información sensible para respuestas
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub user_type: UserType,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
