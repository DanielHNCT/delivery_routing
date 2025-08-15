//! Endpoints de Tournees
//! 
//! Este módulo contiene los endpoints para gestión de tournees.

use axum::{
    extract::{Path, Extension},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{Tournee, CreateTournee, UpdateTournee, TourneeStatus},
    utils::errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
};

/// Listar tournees de la empresa
pub async fn list_tournees(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<Tournee>>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    let tournees = sqlx::query_as!(
        Tournee,
        r#"
        SELECT id, company_id, name, description, driver_id, vehicle_id, planned_date, 
               start_time, end_time, status as "status: TourneeStatus", total_distance, 
               total_duration, estimated_cost, actual_cost, notes, created_at, updated_at, deleted_at
        FROM tournees 
        WHERE company_id = $1 AND deleted_at IS NULL 
        ORDER BY planned_date DESC
        "#,
        company_uuid
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(tournees))
}

/// Crear nueva tournée
pub async fn create_tournee(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(tournee_data): Json<CreateTournee>,
) -> AppResult<Json<Tournee>> {
    // Validar datos de entrada
    tournee_data.validate()
        .map_err(AppError::ValidationError)?;

    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Crear tournée
    let tournee_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let tournee = sqlx::query_as!(
        Tournee,
        r#"
        INSERT INTO tournees (
            id, company_id, name, description, driver_id, vehicle_id, planned_date, 
            start_time, end_time, status, total_distance, total_duration, 
            estimated_cost, notes, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING *
        "#,
        tournee_id,
        company_uuid,
        tournee_data.name,
        tournee_data.description,
        tournee_data.driver_id,
        tournee_data.vehicle_id,
        tournee_data.planned_date,
        tournee_data.start_time,
        tournee_data.end_time,
        tournee_data.status as TourneeStatus,
        tournee_data.total_distance,
        tournee_data.total_duration,
        tournee_data.estimated_cost,
        tournee_data.notes,
        now,
        now
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(tournee))
}

/// Obtener tournée por ID
pub async fn get_tournee(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(tournee_id): Path<String>,
) -> AppResult<Json<Tournee>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_tournee_uuid = Uuid::parse_str(&tournee_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    let tournee = sqlx::query_as!(
        Tournee,
        r#"
        SELECT id, company_id, name, description, driver_id, vehicle_id, planned_date, 
               start_time, end_time, status as "status: TourneeStatus", total_distance, 
               total_duration, estimated_cost, actual_cost, notes, created_at, updated_at, deleted_at
        FROM tournees 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        target_tournee_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(tournee))
}

/// Actualizar tournée
pub async fn update_tournee(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(tournee_id): Path<String>,
    Json(update_data): Json<UpdateTournee>,
) -> AppResult<Json<Tournee>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_tournee_uuid = Uuid::parse_str(&tournee_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que la tournée existe y pertenece a la empresa
    let existing_tournee = sqlx::query_as!(
        Tournee,
        r#"
        SELECT id, company_id, name, description, driver_id, vehicle_id, planned_date, 
               start_time, end_time, status as "status: TourneeStatus", total_distance, 
               total_duration, estimated_cost, actual_cost, notes, created_at, updated_at, deleted_at
        FROM tournees 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        target_tournee_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    // Construir query de actualización
    let mut query_parts = Vec::new();
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut param_count = 1;

    if let Some(name) = update_data.name {
        query_parts.push(format!("name = ${}", param_count));
        params.push(Box::new(name));
        param_count += 1;
    }

    if let Some(description) = update_data.description {
        query_parts.push(format!("description = ${}", param_count));
        params.push(Box::new(description));
        param_count += 1;
    }

    if let Some(driver_id) = update_data.driver_id {
        query_parts.push(format!("driver_id = ${}", param_count));
        params.push(Box::new(driver_id));
        param_count += 1;
    }

    if let Some(vehicle_id) = update_data.vehicle_id {
        query_parts.push(format!("vehicle_id = ${}", param_count));
        params.push(Box::new(vehicle_id));
        param_count += 1;
    }

    if let Some(planned_date) = update_data.planned_date {
        query_parts.push(format!("planned_date = ${}", param_count));
        params.push(Box::new(planned_date));
        param_count += 1;
    }

    if let Some(start_time) = update_data.start_time {
        query_parts.push(format!("start_time = ${}", param_count));
        params.push(Box::new(start_time));
        param_count += 1;
    }

    if let Some(end_time) = update_data.end_time {
        query_parts.push(format!("end_time = ${}", param_count));
        params.push(Box::new(end_time));
        param_count += 1;
    }

    if let Some(status) = update_data.status {
        query_parts.push(format!("status = ${}", param_count));
        params.push(Box::new(status as TourneeStatus));
        param_count += 1;
    }

    if let Some(total_distance) = update_data.total_distance {
        query_parts.push(format!("total_distance = ${}", param_count));
        params.push(Box::new(total_distance));
        param_count += 1;
    }

    if let Some(total_duration) = update_data.total_duration {
        query_parts.push(format!("total_duration = ${}", param_count));
        params.push(Box::new(total_duration));
        param_count += 1;
    }

    if let Some(estimated_cost) = update_data.estimated_cost {
        query_parts.push(format!("estimated_cost = ${}", param_count));
        params.push(Box::new(estimated_cost));
        param_count += 1;
    }

    if let Some(actual_cost) = update_data.actual_cost {
        query_parts.push(format!("actual_cost = ${}", param_count));
        params.push(Box::new(actual_cost));
        param_count += 1;
    }

    if let Some(notes) = update_data.notes {
        query_parts.push(format!("notes = ${}", param_count));
        params.push(Box::new(notes));
        param_count += 1;
    }

    // Agregar updated_at
    query_parts.push(format!("updated_at = ${}", param_count));
    params.push(Box::new(chrono::Utc::now()));

    if query_parts.is_empty() {
        return Ok(Json(existing_tournee));
    }

    // Construir query final
    let query = format!(
        "UPDATE tournees SET {} WHERE id = ${} AND company_id = ${} AND deleted_at IS NULL RETURNING *",
        query_parts.join(", "),
        param_count,
        param_count + 1
    );

    // Ejecutar query
    let tournee = sqlx::query_as!(
        Tournee,
        &query,
        target_tournee_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(tournee))
}

/// Obtener paquetes de una tournée
pub async fn get_tournee_packages(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(tournee_id): Path<String>,
) -> AppResult<Json<Vec<crate::models::Package>>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_tournee_uuid = Uuid::parse_str(&tournee_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que la tournée pertenece a la empresa
    let tournee_exists = sqlx::query!(
        "SELECT id FROM tournees WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL",
        target_tournee_uuid,
        company_uuid
    )
    .fetch_optional(&pool)
    .await?;

    if tournee_exists.is_none() {
        return Err(AppError::NotFound("Tournée no encontrada".to_string()));
    }

    // Obtener paquetes de la tournée
    let packages = sqlx::query_as!(
        crate::models::Package,
        r#"
        SELECT id, company_id, tournee_id, tracking_number, recipient_name, recipient_address, 
               recipient_city, recipient_postal_code, recipient_phone, recipient_email, 
               package_type, weight, dimensions, status as "status: crate::models::PackageStatus", 
               priority, estimated_delivery, actual_delivery, delivery_notes, 
               created_at, updated_at, deleted_at
        FROM packages 
        WHERE tournee_id = $1 AND company_id = $2 AND deleted_at IS NULL 
        ORDER BY priority DESC, created_at
        "#,
        target_tournee_uuid,
        company_uuid
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(packages))
}
