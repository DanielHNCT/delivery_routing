//! Endpoints de Packages
//! 
//! Este módulo contiene los endpoints para gestión de paquetes.

use axum::{
    extract::{Path, Extension},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{Package, CreatePackage, UpdatePackage, PackageStatus},
    utils::errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
};

/// Listar paquetes de la empresa
pub async fn list_packages(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<Package>>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    let packages = sqlx::query_as!(
        Package,
        r#"
        SELECT id, company_id, tournee_id, tracking_number, recipient_name, recipient_address, 
               recipient_city, recipient_postal_code, recipient_phone, recipient_email, 
               package_type, weight, dimensions, status as "status: PackageStatus", 
               priority, estimated_delivery, actual_delivery, delivery_notes, 
               created_at, updated_at, deleted_at
        FROM packages 
        WHERE company_id = $1 AND deleted_at IS NULL 
        ORDER BY priority DESC, created_at
        "#,
        company_uuid
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(packages))
}

/// Crear nuevo paquete
pub async fn create_package(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(package_data): Json<CreatePackage>,
) -> AppResult<Json<Package>> {
    // Validar datos de entrada
    package_data.validate()
        .map_err(AppError::ValidationError)?;

    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el número de seguimiento no esté en uso en la empresa
    let existing_package = sqlx::query!(
        "SELECT id FROM packages WHERE tracking_number = $1 AND company_id = $2 AND deleted_at IS NULL",
        package_data.tracking_number,
        company_uuid
    )
    .fetch_optional(&pool)
    .await?;

    if existing_package.is_some() {
        return Err(AppError::Conflict("Ya existe un paquete con ese número de seguimiento".to_string()));
    }

    // Crear paquete
    let package_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let package = sqlx::query_as!(
        Package,
        r#"
        INSERT INTO packages (
            id, company_id, tournee_id, tracking_number, recipient_name, recipient_address, 
            recipient_city, recipient_postal_code, recipient_phone, recipient_email, 
            package_type, weight, dimensions, status, priority, estimated_delivery, 
            delivery_notes, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
        RETURNING *
        "#,
        package_id,
        company_uuid,
        package_data.tournee_id,
        package_data.tracking_number,
        package_data.recipient_name,
        package_data.recipient_address,
        package_data.recipient_city,
        package_data.recipient_postal_code,
        package_data.recipient_phone,
        package_data.recipient_email,
        package_data.package_type,
        package_data.weight,
        package_data.dimensions,
        package_data.status as PackageStatus,
        package_data.priority,
        package_data.estimated_delivery,
        package_data.delivery_notes,
        now,
        now
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(package))
}

/// Obtener paquete por ID
pub async fn get_package(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(package_id): Path<String>,
) -> AppResult<Json<Package>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_package_uuid = Uuid::parse_str(&package_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    let package = sqlx::query_as!(
        Package,
        r#"
        SELECT id, company_id, tournee_id, tracking_number, recipient_name, recipient_address, 
               recipient_city, recipient_postal_code, recipient_phone, recipient_email, 
               package_type, weight, dimensions, status as "status: PackageStatus", 
               priority, estimated_delivery, actual_delivery, delivery_notes, 
               created_at, updated_at, deleted_at
        FROM packages 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        target_package_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(package))
}

/// Actualizar paquete
pub async fn update_package(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(package_id): Path<String>,
    Json(update_data): Json<UpdatePackage>,
) -> AppResult<Json<Package>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_package_uuid = Uuid::parse_str(&package_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el paquete existe y pertenece a la empresa
    let existing_package = sqlx::query_as!(
        Package,
        r#"
        SELECT id, company_id, tournee_id, tracking_number, recipient_name, recipient_address, 
               recipient_city, recipient_postal_code, recipient_phone, recipient_email, 
               package_type, weight, dimensions, status as "status: PackageStatus", 
               priority, estimated_delivery, actual_delivery, delivery_notes, 
               created_at, updated_at, deleted_at
        FROM packages 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        target_package_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    // Construir query de actualización
    let mut query_parts = Vec::new();
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut param_count = 1;

    if let Some(recipient_name) = update_data.recipient_name {
        query_parts.push(format!("recipient_name = ${}", param_count));
        params.push(Box::new(recipient_name));
        param_count += 1;
    }

    if let Some(recipient_address) = update_data.recipient_address {
        query_parts.push(format!("recipient_address = ${}", param_count));
        params.push(Box::new(recipient_address));
        param_count += 1;
    }

    if let Some(recipient_city) = update_data.recipient_city {
        query_parts.push(format!("recipient_city = ${}", param_count));
        params.push(Box::new(recipient_city));
        param_count += 1;
    }

    if let Some(recipient_postal_code) = update_data.recipient_postal_code {
        query_parts.push(format!("recipient_postal_code = ${}", param_count));
        params.push(Box::new(recipient_postal_code));
        param_count += 1;
    }

    if let Some(recipient_phone) = update_data.recipient_phone {
        query_parts.push(format!("recipient_phone = ${}", param_count));
        params.push(Box::new(recipient_phone));
        param_count += 1;
    }

    if let Some(recipient_email) = update_data.recipient_email {
        query_parts.push(format!("recipient_email = ${}", param_count));
        params.push(Box::new(recipient_email));
        param_count += 1;
    }

    if let Some(package_type) = update_data.package_type {
        query_parts.push(format!("package_type = ${}", param_count));
        params.push(Box::new(package_type));
        param_count += 1;
    }

    if let Some(weight) = update_data.weight {
        query_parts.push(format!("weight = ${}", param_count));
        params.push(Box::new(weight));
        param_count += 1;
    }

    if let Some(dimensions) = update_data.dimensions {
        query_parts.push(format!("dimensions = ${}", param_count));
        params.push(Box::new(dimensions));
        param_count += 1;
    }

    if let Some(status) = update_data.status {
        query_parts.push(format!("status = ${}", param_count));
        params.push(Box::new(status as PackageStatus));
        param_count += 1;
    }

    if let Some(priority) = update_data.priority {
        query_parts.push(format!("priority = ${}", param_count));
        params.push(Box::new(priority));
        param_count += 1;
    }

    if let Some(estimated_delivery) = update_data.estimated_delivery {
        query_parts.push(format!("estimated_delivery = ${}", param_count));
        params.push(Box::new(estimated_delivery));
        param_count += 1;
    }

    if let Some(actual_delivery) = update_data.actual_delivery {
        query_parts.push(format!("actual_delivery = ${}", param_count));
        params.push(Box::new(actual_delivery));
        param_count += 1;
    }

    if let Some(delivery_notes) = update_data.delivery_notes {
        query_parts.push(format!("delivery_notes = ${}", param_count));
        params.push(Box::new(delivery_notes));
        param_count += 1;
    }

    // Agregar updated_at
    query_parts.push(format!("updated_at = ${}", param_count));
    params.push(Box::new(chrono::Utc::now()));

    if query_parts.is_empty() {
        return Ok(Json(existing_package));
    }

    // Construir query final
    let query = format!(
        "UPDATE packages SET {} WHERE id = ${} AND company_id = ${} AND deleted_at IS NULL RETURNING *",
        query_parts.join(", "),
        param_count,
        param_count + 1
    );

    // Ejecutar query
    let package = sqlx::query_as!(
        Package,
        &query,
        target_package_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(package))
}
