//! Endpoints de Vehicles
//! 
//! Este módulo contiene los endpoints para gestión de vehículos.

use axum::{
    extract::{Path, Extension},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{Vehicle, CreateVehicle, UpdateVehicle, VehicleStatus},
    utils::errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
};

/// Listar vehículos de la empresa
pub async fn list_vehicles(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<Vehicle>>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    let vehicles = sqlx::query_as!(
        Vehicle,
        r#"
        SELECT id, company_id, name, license_plate, vehicle_type, capacity, capacity_unit, 
               fuel_type, fuel_consumption, driver_id, status as "status: VehicleStatus", 
               purchase_date, last_maintenance, notes, created_at, updated_at, deleted_at
        FROM vehicles 
        WHERE company_id = $1 AND deleted_at IS NULL 
        ORDER BY name
        "#,
        company_uuid
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(vehicles))
}

/// Crear nuevo vehículo
pub async fn create_vehicle(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(vehicle_data): Json<CreateVehicle>,
) -> AppResult<Json<Vehicle>> {
    // Validar datos de entrada
    vehicle_data.validate()
        .map_err(AppError::ValidationError)?;

    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que la matrícula no esté en uso en la empresa
    if let Some(license_plate) = &vehicle_data.license_plate {
        let existing_vehicle = sqlx::query!(
            "SELECT id FROM vehicles WHERE license_plate = $1 AND company_id = $2 AND deleted_at IS NULL",
            license_plate,
            company_uuid
        )
        .fetch_optional(&pool)
        .await?;

        if existing_vehicle.is_some() {
            return Err(AppError::Conflict("Ya existe un vehículo con esa matrícula".to_string()));
        }
    }

    // Crear vehículo
    let vehicle_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let vehicle = sqlx::query_as!(
        Vehicle,
        r#"
        INSERT INTO vehicles (
            id, company_id, name, license_plate, vehicle_type, capacity, capacity_unit, 
            fuel_type, fuel_consumption, driver_id, status, purchase_date, notes, 
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING *
        "#,
        vehicle_id,
        company_uuid,
        vehicle_data.name,
        vehicle_data.license_plate,
        vehicle_data.vehicle_type,
        vehicle_data.capacity,
        vehicle_data.capacity_unit,
        vehicle_data.fuel_type,
        vehicle_data.fuel_consumption,
        vehicle_data.driver_id,
        vehicle_data.status as VehicleStatus,
        vehicle_data.purchase_date,
        vehicle_data.notes,
        now,
        now
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(vehicle))
}

/// Obtener vehículo por ID
pub async fn get_vehicle(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(vehicle_id): Path<String>,
) -> AppResult<Json<Vehicle>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_vehicle_uuid = Uuid::parse_str(&vehicle_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    let vehicle = sqlx::query_as!(
        Vehicle,
        r#"
        SELECT id, company_id, name, license_plate, vehicle_type, capacity, capacity_unit, 
               fuel_type, fuel_consumption, driver_id, status as "status: VehicleStatus", 
               purchase_date, last_maintenance, notes, created_at, updated_at, deleted_at
        FROM vehicles 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        target_vehicle_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(vehicle))
}

/// Actualizar vehículo
pub async fn update_vehicle(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(vehicle_id): Path<String>,
    Json(update_data): Json<UpdateVehicle>,
) -> AppResult<Json<Vehicle>> {
    let company_uuid = Uuid::parse_str(&user.company_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;
    
    let target_vehicle_uuid = Uuid::parse_str(&vehicle_id)
        .map_err(|_| AppError::ValidationError(validator::ValidationErrors::new()))?;

    // Verificar que el vehículo existe y pertenece a la empresa
    let existing_vehicle = sqlx::query_as!(
        Vehicle,
        r#"
        SELECT id, company_id, name, license_plate, vehicle_type, capacity, capacity_unit, 
               fuel_type, fuel_consumption, driver_id, status as "status: VehicleStatus", 
               purchase_date, last_maintenance, notes, created_at, updated_at, deleted_at
        FROM vehicles 
        WHERE id = $1 AND company_id = $2 AND deleted_at IS NULL
        "#,
        target_vehicle_uuid,
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

    if let Some(license_plate) = update_data.license_plate {
        query_parts.push(format!("license_plate = ${}", param_count));
        params.push(Box::new(license_plate));
        param_count += 1;
    }

    if let Some(vehicle_type) = update_data.vehicle_type {
        query_parts.push(format!("vehicle_type = ${}", param_count));
        params.push(Box::new(vehicle_type));
        param_count += 1;
    }

    if let Some(capacity) = update_data.capacity {
        query_parts.push(format!("capacity = ${}", param_count));
        params.push(Box::new(capacity));
        param_count += 1;
    }

    if let Some(capacity_unit) = update_data.capacity_unit {
        query_parts.push(format!("capacity_unit = ${}", param_count));
        params.push(Box::new(capacity_unit));
        param_count += 1;
    }

    if let Some(fuel_type) = update_data.fuel_type {
        query_parts.push(format!("fuel_type = ${}", param_count));
        params.push(Box::new(fuel_type));
        param_count += 1;
    }

    if let Some(fuel_consumption) = update_data.fuel_consumption {
        query_parts.push(format!("fuel_consumption = ${}", param_count));
        params.push(Box::new(fuel_consumption));
        param_count += 1;
    }

    if let Some(driver_id) = update_data.driver_id {
        query_parts.push(format!("driver_id = ${}", param_count));
        params.push(Box::new(driver_id));
        param_count += 1;
    }

    if let Some(status) = update_data.status {
        query_parts.push(format!("status = ${}", param_count));
        params.push(Box::new(status as VehicleStatus));
        param_count += 1;
    }

    if let Some(purchase_date) = update_data.purchase_date {
        query_parts.push(format!("purchase_date = ${}", param_count));
        params.push(Box::new(purchase_date));
        param_count += 1;
    }

    if let Some(last_maintenance) = update_data.last_maintenance {
        query_parts.push(format!("last_maintenance = ${}", param_count));
        params.push(Box::new(last_maintenance));
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
        return Ok(Json(existing_vehicle));
    }

    // Construir query final
    let query = format!(
        "UPDATE vehicles SET {} WHERE id = ${} AND company_id = ${} AND deleted_at IS NULL RETURNING *",
        query_parts.join(", "),
        param_count,
        param_count + 1
    );

    // Ejecutar query
    let vehicle = sqlx::query_as!(
        Vehicle,
        &query,
        target_vehicle_uuid,
        company_uuid
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(vehicle))
}
