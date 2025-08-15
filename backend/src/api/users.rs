//! Endpoints de Users
//! 
//! Este módulo contiene los endpoints para gestión de usuarios.

use axum::{
    extract::{Path, Extension},
    Json,
};
use bcrypt::hash;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{User, CreateUser, UpdateUser, UserResponse, UserType},
    utils::errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
};

/// Listar usuarios de la empresa
pub async fn list_users(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<UserResponse>>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, company_id, username, email, password_hash, first_name, last_name, 
               phone, user_type as "user_type: UserType", is_active, last_login, 
               created_at, updated_at, deleted_at
        FROM users 
        WHERE company_id = $1 AND deleted_at IS NULL 
        ORDER BY username
        "#,
        company_uuid
    )
    .fetch_all(&pool)
    .await?;

    // Convertir a UserResponse (sin password_hash)
    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id,
            company_id: u.company_id,
            username: u.username,
            email: u.email,
            first_name: u.first_name,
            last_name: u.last_name,
            phone: u.phone,
            user_type: u.user_type,
            is_active: u.is_active,
            last_login: u.last_login,
            created_at: u.created_at,
            updated_at: u.updated_at,
        })
        .collect();

    Ok(Json(user_responses))
}

/// Crear nuevo usuario
pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(user_data): Json<CreateUser>,
) -> AppResult<Json<UserResponse>> {
    // Validar datos de entrada
    user_data.validate()
        .map_err(AppError::ValidationError)?;

    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el email no esté en uso en la empresa
    let existing_email = sqlx::query!(
        "SELECT id FROM users WHERE email = $1 AND company_id = $2 AND deleted_at IS NULL",
        user_data.email,
        company_uuid
    )
    .fetch_optional(&pool)
    .await?;

    if existing_email.is_some() {
        return Err(AppError::Conflict("El email ya está en uso en esta empresa".to_string()));
    }

    // Verificar que el username no esté en uso en la empresa
    let existing_username = sqlx::query!(
        "SELECT id FROM users WHERE username = $1 AND company_id = $2 AND deleted_at IS NULL",
        user_data.username,
        company_uuid
    )
    .fetch_optional(&pool)
    .await?;

    if existing_username.is_some() {
        return Err(AppError::Conflict("El username ya está en uso en esta empresa".to_string()));
    }

    // Hash password
    let password_hash = hash(user_data.password, bcrypt::DEFAULT_COST)?;

    // Crear usuario
    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let new_user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (
            id, company_id, username, email, password_hash, first_name, last_name, 
            phone, user_type, is_active, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
        user_id,
        company_uuid,
        user_data.username,
        user_data.email,
        password_hash,
        user_data.first_name,
        user_data.last_name,
        user_data.phone,
        user_data.user_type as UserType,
        true,
        now,
        now
    )
    .fetch_one(&pool)
    .await?;

    // Convertir a UserResponse
    let user_response = UserResponse {
        id: new_user.id,
        company_id: new_user.company_id,
        username: new_user.username,
        email: new_user.email,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        phone: new_user.phone,
        user_type: new_user.user_type,
        is_active: new_user.is_active,
        last_login: new_user.last_login,
        created_at: new_user.created_at,
        updated_at: new_user.updated_at,
    };

    Ok(Json(user_response))
}

/// Obtener usuario por ID
pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(user_id): Path<String>,
) -> AppResult<Json<UserResponse>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    let target_user = sqlx::query_as!(
        User,
        r#"
        SELECT id, company_id, username, email, password_hash, first_name, last_name, 
               phone, user_type as "user_type: UserType", is_active, last_login, 
               created_at, updated_at, deleted_at
        FROM users 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        target_user_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    // Convertir a UserResponse
    let user_response = UserResponse {
        id: target_user.id,
        company_id: target_user.company_id,
        username: target_user.username,
        email: target_user.email,
        first_name: target_user.first_name,
        last_name: target_user.last_name,
        phone: target_user.phone,
        user_type: target_user.user_type,
        is_active: target_user.is_active,
        last_login: target_user.last_login,
        created_at: target_user.created_at,
        updated_at: target_user.updated_at,
    };

    Ok(Json(user_response))
}

/// Actualizar usuario
pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(user_id): Path<String>,
    Json(update_data): Json<UpdateUser>,
) -> AppResult<Json<UserResponse>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el usuario existe y pertenece a la empresa
    let existing_user = sqlx::query_as!(
        User,
        r#"
        SELECT id, company_id, username, email, password_hash, first_name, last_name, 
               phone, user_type as "user_type: UserType", is_active, last_login, 
               created_at, updated_at, deleted_at
        FROM users 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        target_user_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    // Construir query de actualización
    let mut query_parts = Vec::new();
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut param_count = 1;

    if let Some(username) = update_data.username {
        query_parts.push(format!("username = ${}", param_count));
        params.push(Box::new(username));
        param_count += 1;
    }

    if let Some(email) = update_data.email {
        query_parts.push(format!("email = ${}", param_count));
        params.push(Box::new(email));
        param_count += 1;
    }

    if let Some(password) = update_data.password {
        let password_hash = hash(password, bcrypt::DEFAULT_COST)?;
        query_parts.push(format!("password_hash = ${}", param_count));
        params.push(Box::new(password_hash));
        param_count += 1;
    }

    if let Some(first_name) = update_data.first_name {
        query_parts.push(format!("first_name = ${}", param_count));
        params.push(Box::new(first_name));
        param_count += 1;
    }

    if let Some(last_name) = update_data.last_name {
        query_parts.push(format!("last_name = ${}", param_count));
        params.push(Box::new(last_name));
        param_count += 1;
    }

    if let Some(phone) = update_data.phone {
        query_parts.push(format!("phone = ${}", param_count));
        params.push(Box::new(phone));
        param_count += 1;
    }

    if let Some(user_type) = update_data.user_type {
        query_parts.push(format!("user_type = ${}", param_count));
        params.push(Box::new(user_type as UserType));
        param_count += 1;
    }

    if let Some(is_active) = update_data.is_active {
        query_parts.push(format!("is_active = ${}", param_count));
        params.push(Box::new(is_active));
        param_count += 1;
    }

    // Agregar updated_at
    query_parts.push(format!("updated_at = ${}", param_count));
    params.push(Box::new(chrono::Utc::now()));

    if query_parts.is_empty() {
        // Convertir existing_user a UserResponse
        let user_response = UserResponse {
            id: existing_user.id,
            company_id: existing_user.company_id,
            username: existing_user.username,
            email: existing_user.email,
            first_name: existing_user.first_name,
            last_name: existing_user.last_name,
            phone: existing_user.phone,
            user_type: existing_user.user_type,
            is_active: existing_user.is_active,
            last_login: existing_user.last_login,
            created_at: existing_user.created_at,
            updated_at: existing_user.updated_at,
        };
        return Ok(Json(user_response));
    }

    // Construir query final
    let query = format!(
        "UPDATE users SET {} WHERE id = ${} AND company_id = ${} AND deleted_at IS NULL RETURNING *",
        query_parts.join(", "),
        param_count,
        param_count + 1
    );

    // Ejecutar query
    let updated_user = sqlx::query_as!(
        User,
        &query,
        target_user_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    // Convertir a UserResponse
    let user_response = UserResponse {
        id: updated_user.id,
        company_id: updated_user.company_id,
        username: updated_user.username,
        email: updated_user.email,
        first_name: updated_user.first_name,
        last_name: updated_user.last_name,
        phone: updated_user.phone,
        user_type: updated_user.user_type,
        is_active: updated_user.is_active,
        last_login: updated_user.last_login,
        created_at: updated_user.created_at,
        updated_at: updated_user.updated_at,
    };

    Ok(Json(user_response))
}

/// Eliminar usuario (soft delete)
pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(user_id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el usuario existe y pertenece a la empresa
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL",
        target_user_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    // Soft delete
    sqlx::query!(
        "UPDATE users SET deleted_at = $1 WHERE id = $2",
        chrono::Utc::now(),
        target_user_uuid
    )
    .execute(&pool)
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Usuario eliminado exitosamente",
        "user_id": user_id
    })))
}
