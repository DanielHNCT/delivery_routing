//! Middleware de autenticación JWT
//! 
//! Este middleware extrae y valida tokens JWT de las requests HTTP.

use axum::{
    extract::{Request, Extension},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::utils::errors::AppError;

/// Claims del token JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // user_id
    pub company_id: String, // company_id
    pub exp: usize,         // expiration
    pub iat: usize,         // issued at
}

/// Información del usuario autenticado
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub company_id: String,
}

/// Middleware de autenticación
pub async fn auth_middleware(
    Extension(pool): Extension<PgPool>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extraer token del header Authorization
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "));

    let token = auth_header.ok_or_else(|| {
        AppError::Unauthorized("Token de autenticación requerido".to_string())
    })?;

    // Validar token JWT
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::InternalError("JWT_SECRET no configurado".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Token inválido".to_string()))?;

    let claims = token_data.claims;

    // Verificar que el usuario existe en la base de datos
    let user_exists = sqlx::query!(
        "SELECT id FROM users WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL",
        claims.sub,
        claims.company_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .is_some();

    if !user_exists {
        return Err(AppError::Unauthorized("Usuario no encontrado".to_string()));
    }

    // Crear usuario autenticado
    let authenticated_user = AuthenticatedUser {
        user_id: claims.sub,
        company_id: claims.company_id,
    };

    // Inyectar usuario autenticado en la request
    request.extensions_mut().insert(authenticated_user);

    Ok(next.run(request).await)
}
