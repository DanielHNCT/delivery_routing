//! Endpoints de autenticación
//! 
//! Este módulo contiene los endpoints para login, registro y verificación de usuario.

use axum::{
    extract::State,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, Duration};
use validator::Validate;

use crate::{
    models::{LoginRequest, RegisterRequest, AuthResponse, UserInfo, CreateUser, UserType},
    utils::errors::{AppError, AppResult},
    middleware::auth::{Claims, AuthenticatedUser},
};

/// Login de usuario
pub async fn login(
    State(pool): State<PgPool>,
    Json(login_data): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Validar datos de entrada
    login_data.validate()
        .map_err(AppError::ValidationError)?;

    // Buscar usuario por email
    let user = sqlx::query_as!(
        crate::models::User,
        r#"
        SELECT id, company_id, username, email, password_hash, first_name, last_name, 
               phone, user_type as "user_type: UserType", is_active, last_login, 
               created_at, updated_at, deleted_at
        FROM users 
        WHERE email = $1 AND deleted_at IS NULL
        "#,
        login_data.email
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Credenciales inválidas".to_string()))?;

    // Verificar que el usuario esté activo
    if !user.is_active {
        return Err(AppError::Unauthorized("Usuario inactivo".to_string()));
    }

    // Verificar password
    if !verify(&login_data.password, &user.password_hash)? {
        return Err(AppError::Unauthorized("Credenciales inválidas".to_string()));
    }

    // Obtener información de la empresa
    let company = sqlx::query_as!(
        crate::models::Company,
        "SELECT * FROM companies WHERE id = $1 AND deleted_at IS NULL",
        user.company_id
    )
    .fetch_one(&pool)
    .await?;

    // Generar JWT token
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::InternalError("JWT_SECRET no configurado".to_string()))?;

    let expiration = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user.id.to_string(),
        company_id: user.company_id.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    // Actualizar último login
    sqlx::query!(
        "UPDATE users SET last_login = $1 WHERE id = $2",
        Utc::now(),
        user.id
    )
    .execute(&pool)
    .await?;

    // Crear respuesta
    let user_info = UserInfo {
        id: user.id.to_string(),
        email: user.email,
        username: user.username,
        user_type: format!("{:?}", user.user_type),
        company_id: user.company_id.to_string(),
        company_name: company.name,
    };

    let response = AuthResponse {
        token,
        user: user_info,
    };

    Ok(Json(response))
}

/// Registro de nueva empresa y usuario admin
pub async fn register(
    State(pool): State<PgPool>,
    Json(register_data): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    // Validar datos de entrada
    register_data.validate()
        .map_err(AppError::ValidationError)?;

    // Verificar que el email no esté en uso
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1 AND deleted_at IS NULL",
        register_data.admin_email
    )
    .fetch_optional(&pool)
    .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("El email ya está en uso".to_string()));
    }

    // Verificar que el username no esté en uso
    let existing_username = sqlx::query!(
        "SELECT id FROM users WHERE username = $1 AND deleted_at IS NULL",
        register_data.admin_username
    )
    .fetch_optional(&pool)
    .await?;

    if existing_username.is_some() {
        return Err(AppError::Conflict("El username ya está en uso".to_string()));
    }

    // Crear empresa
    let company_id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO companies (id, name, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        "#,
        company_id,
        register_data.company_name,
        now,
        now
    )
    .execute(&pool)
    .await?;

    // Hash password
    let password_hash = hash(register_data.admin_password, DEFAULT_COST)?;

    // Crear usuario admin
    let user_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO users (id, company_id, username, email, password_hash, user_type, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        user_id,
        company_id,
        register_data.admin_username,
        register_data.admin_email,
        password_hash,
        UserType::Admin as UserType,
        true,
        now,
        now
    )
    .execute(&pool)
    .await?;

    // Generar JWT token
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::InternalError("JWT_SECRET no configurado".to_string()))?;

    let expiration = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        company_id: company_id.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    // Crear respuesta
    let user_info = UserInfo {
        id: user_id.to_string(),
        email: register_data.admin_email,
        username: register_data.admin_username,
        user_type: "Admin".to_string(),
        company_id: company_id.to_string(),
        company_name: register_data.company_name,
    };

    let response = AuthResponse {
        token,
        user: user_info,
    };

    Ok(Json(response))
}

/// Obtener información del usuario autenticado
pub async fn me(
    State(pool): State<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<UserInfo>> {
    // Obtener usuario completo
    let user_data = sqlx::query_as!(
        crate::models::User,
        r#"
        SELECT id, company_id, username, email, password_hash, first_name, last_name, 
               phone, user_type as "user_type: UserType", is_active, last_login, 
               created_at, updated_at, deleted_at
        FROM users 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        Uuid::parse_str(&user.user_id)?,
        Uuid::parse_str(&user.company_id)?
    )
    .fetch_one(&pool)
    .await?;

    // Obtener información de la empresa
    let company = sqlx::query_as!(
        crate::models::Company,
        "SELECT * FROM companies WHERE id = $1 AND deleted_at IS NULL",
        user_data.company_id
    )
    .fetch_one(&pool)
    .await?;

    // Crear respuesta
    let user_info = UserInfo {
        id: user_data.id.to_string(),
        email: user_data.email,
        username: user_data.username,
        user_type: format!("{:?}", user_data.user_type),
        company_id: user_data.company_id.to_string(),
        company_name: company.name,
    };

    Ok(Json(user_info))
}
