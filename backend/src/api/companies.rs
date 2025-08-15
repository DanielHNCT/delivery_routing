//! Endpoints de Companies
//! 
//! Este módulo contiene los endpoints para gestión de empresas.

use axum::{
    extract::{Path, State, Extension},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{Company, CreateCompany, UpdateCompany},
    utils::errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
};

/// Listar todas las empresas (solo super admin)
pub async fn list_companies(
    State(pool): State<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<Company>>> {
    // TODO: Verificar que el usuario sea super admin
    // Por ahora, solo permitir a usuarios autenticados
    
    let companies = sqlx::query_as!(
        Company,
        "SELECT * FROM companies WHERE deleted_at IS NULL ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(companies))
}

/// Crear nueva empresa
pub async fn create_company(
    State(pool): State<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(company_data): Json<CreateCompany>,
) -> AppResult<Json<Company>> {
    // Validar datos de entrada
    company_data.validate()
        .map_err(AppError::ValidationError)?;

    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el nombre no esté en uso
    let existing_company = sqlx::query!(
        "SELECT id FROM companies WHERE name = $1 AND deleted_at IS NULL",
        company_data.name
    )
    .fetch_optional(&pool)
    .await?;

    if existing_company.is_some() {
        return Err(AppError::Conflict("Ya existe una empresa con ese nombre".to_string()));
    }

    // Crear empresa
    let company_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let company = sqlx::query_as!(
        Company,
        r#"
        INSERT INTO companies (
            id, name, address, city, postal_code, country, phone, email, 
            website, siret, tva_number, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
        company_id,
        company_data.name,
        company_data.address,
        company_data.city,
        company_data.postal_code,
        company_data.country,
        company_data.phone,
        company_data.email,
        company_data.website,
        company_data.siret,
        company_data.tva_number,
        now,
        now
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(company))
}

/// Obtener empresa por ID
pub async fn get_company(
    State(pool): State<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(company_id): Path<String>,
) -> AppResult<Json<Company>> {
    let company_uuid = Uuid::parse_str(&company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el usuario pertenezca a la empresa
    if user.company_id != company_id {
        return Err(AppError::Forbidden("No tienes acceso a esta empresa".to_string()));
    }

    let company = sqlx::query_as!(
        Company,
        "SELECT * FROM companies WHERE id = $1 AND deleted_at IS NULL",
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(company))
}

/// Actualizar empresa
pub async fn update_company(
    State(pool): State<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(company_id): Path<String>,
    Json(update_data): Json<UpdateCompany>,
) -> AppResult<Json<Company>> {
    let company_uuid = Uuid::parse_str(&company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el usuario pertenezca a la empresa
    if user.company_id != company_id {
        return Err(AppError::Forbidden("No tienes acceso a esta empresa".to_string()));
    }

    // Verificar que la empresa existe
    let existing_company = sqlx::query_as!(
        Company,
        "SELECT * FROM companies WHERE id = $1 AND deleted_at IS NULL",
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    // Construir query de actualización dinámicamente
    let mut query_parts = Vec::new();
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut param_count = 1;

    if let Some(name) = update_data.name {
        query_parts.push(format!("name = ${}", param_count));
        params.push(Box::new(name));
        param_count += 1;
    }

    if let Some(address) = update_data.address {
        query_parts.push(format!("address = ${}", param_count));
        params.push(Box::new(address));
        param_count += 1;
    }

    if let Some(city) = update_data.city {
        query_parts.push(format!("city = ${}", param_count));
        params.push(Box::new(city));
        param_count += 1;
    }

    if let Some(postal_code) = update_data.postal_code {
        query_parts.push(format!("postal_code = ${}", param_count));
        params.push(Box::new(postal_code));
        param_count += 1;
    }

    if let Some(country) = update_data.country {
        query_parts.push(format!("country = ${}", param_count));
        params.push(Box::new(country));
        param_count += 1;
    }

    if let Some(phone) = update_data.phone {
        query_parts.push(format!("phone = ${}", param_count));
        params.push(Box::new(phone));
        param_count += 1;
    }

    if let Some(email) = update_data.email {
        query_parts.push(format!("email = ${}", param_count));
        params.push(Box::new(email));
        param_count += 1;
    }

    if let Some(website) = update_data.website {
        query_parts.push(format!("website = ${}", param_count));
        params.push(Box::new(website));
        param_count += 1;
    }

    if let Some(siret) = update_data.siret {
        query_parts.push(format!("siret = ${}", param_count));
        params.push(Box::new(siret));
        param_count += 1;
    }

    if let Some(tva_number) = update_data.tva_number {
        query_parts.push(format!("tva_number = ${}", param_count));
        params.push(Box::new(tva_number));
        param_count += 1;
    }

    // Agregar updated_at
    query_parts.push(format!("updated_at = ${}", param_count));
    params.push(Box::new(chrono::Utc::now()));

    if query_parts.is_empty() {
        return Ok(Json(existing_company));
    }

    // Construir query final
    let query = format!(
        "UPDATE companies SET {} WHERE id = ${} AND deleted_at IS NULL RETURNING *",
        query_parts.join(", "),
        param_count
    );

    // Ejecutar query
    let company = sqlx::query_as!(
        Company,
        &query,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(company))
}
