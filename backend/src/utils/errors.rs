//! Manejo de errores para la API
//! 
//! Este módulo define todos los tipos de errores que pueden ocurrir en la API.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

/// Errores principales de la aplicación
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de validación: {0}")]
    ValidationError(#[from] ValidationErrors),
    
    #[error("Error de base de datos: {0}")]
    DatabaseError(String),
    
    #[error("Error de autenticación: {0}")]
    Unauthorized(String),
    
    #[error("Recurso no encontrado: {0}")]
    NotFound(String),
    
    #[error("Error interno del servidor: {0}")]
    InternalError(String),
    
    #[error("Error de serialización: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Error de JWT: {0}")]
    JwtError(String),
    
    #[error("Error de hash de password: {0}")]
    PasswordHashError(String),
    
    #[error("Error de permisos: {0}")]
    Forbidden(String),
    
    #[error("Error de conflicto: {0}")]
    Conflict(String),
}

impl AppError {
    /// Obtener el código de estado HTTP correspondiente
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SerializationError(_) => StatusCode::BAD_REQUEST,
            AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
            AppError::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Conflict(_) => StatusCode::CONFLICT,
        }
    }
    
    /// Obtener el mensaje de error para el usuario
    pub fn user_message(&self) -> String {
        match self {
            AppError::ValidationError(errors) => {
                let mut messages = Vec::new();
                for (field, errors) in errors.field_errors() {
                    for error in errors {
                        if let Some(message) = &error.message {
                            messages.push(format!("{}: {}", field, message));
                        }
                    }
                }
                if messages.is_empty() {
                    "Datos de entrada inválidos".to_string()
                } else {
                    messages.join(", ")
                }
            }
            AppError::DatabaseError(_) => "Error interno de la base de datos".to_string(),
            AppError::Unauthorized(msg) => msg.clone(),
            AppError::NotFound(msg) => msg.clone(),
            AppError::InternalError(_) => "Error interno del servidor".to_string(),
            AppError::SerializationError(_) => "Error en el formato de datos".to_string(),
            AppError::JwtError(_) => "Token de autenticación inválido".to_string(),
            AppError::PasswordHashError(_) => "Error interno del servidor".to_string(),
            AppError::Forbidden(msg) => msg.clone(),
            AppError::Conflict(msg) => msg.clone(),
        }
    }
    
    /// Obtener el código de error interno
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::InternalError(_) => "INTERNAL_ERROR",
            AppError::SerializationError(_) => "SERIALIZATION_ERROR",
            AppError::JwtError(_) => "JWT_ERROR",
            AppError::PasswordHashError(_) => "PASSWORD_HASH_ERROR",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::Conflict(_) => "CONFLICT",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let error_response = json!({
            "error": {
                "code": self.error_code(),
                "message": self.user_message(),
                "status": status_code.as_u16(),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });
        
        (status_code, Json(error_response)).into_response()
    }
}

/// Result type personalizado para la aplicación
pub type AppResult<T> = Result<T, AppError>;

/// Helper para convertir errores de SQLx
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Recurso no encontrado".to_string()),
            sqlx::Error::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    match code.as_ref() {
                        "23505" => AppError::Conflict("Recurso ya existe".to_string()),
                        "23503" => AppError::Conflict("Referencia inválida".to_string()),
                        _ => AppError::DatabaseError(db_err.message().to_string()),
                    }
                } else {
                    AppError::DatabaseError(db_err.message().to_string())
                }
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

/// Helper para convertir errores de JWT
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError(err.to_string())
    }
}

/// Helper para convertir errores de bcrypt
impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::PasswordHashError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::ValidationError;

    #[test]
    fn test_app_error_status_codes() {
        assert_eq!(AppError::ValidationError(ValidationErrors::new()).status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(AppError::Unauthorized("test".to_string()).status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(AppError::NotFound("test".to_string()).status_code(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_user_messages() {
        assert_eq!(
            AppError::Unauthorized("Acceso denegado".to_string()).user_message(),
            "Acceso denegado"
        );
        assert_eq!(
            AppError::NotFound("Usuario no encontrado".to_string()).user_message(),
            "Usuario no encontrado"
        );
    }
}
