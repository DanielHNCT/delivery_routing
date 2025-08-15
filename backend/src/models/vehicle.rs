//! Modelo de Vehicle
//! 
//! Este módulo contiene el struct Vehicle, estados del vehículo y variantes para CRUD operations.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Estados del vehículo
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "vehicle_status", rename_all = "lowercase")]
pub enum VehicleStatus {
    Active,
    Maintenance,
    OutOfService,
    Retired,
}

/// Vehicle principal
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vehicle {
    pub id: Uuid,
    pub company_id: Uuid,
    pub license_plate: String,
    pub brand: String,
    pub model: String,
    pub year: Option<i32>,
    pub color: Option<String>,
    pub vehicle_status: VehicleStatus,
    pub current_mileage: rust_decimal::Decimal,
    pub fuel_type: String,
    pub fuel_capacity: Option<rust_decimal::Decimal>,
    pub weekly_fuel_allocation: Option<rust_decimal::Decimal>,
    pub total_damage_cost: rust_decimal::Decimal,
    pub damage_incidents_count: i32,
    pub vin: Option<String>,
    pub engine_size: Option<String>,
    pub transmission: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Request para crear un nuevo vehículo
#[derive(Debug, Deserialize, Validate)]
pub struct CreateVehicle {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub name: String,
    
    #[validate(length(max = 20, message = "Matrícula no puede exceder 20 caracteres"))]
    pub license_plate: Option<String>,
    
    #[validate(length(min = 2, max = 50, message = "Tipo de vehículo debe tener entre 2 y 50 caracteres"))]
    pub vehicle_type: String,
    
    #[validate(range(min = 0.0, message = "Capacidad debe ser mayor a 0"))]
    pub capacity: Option<f64>,
    
    #[validate(length(max = 20, message = "Unidad de capacidad no puede exceder 20 caracteres"))]
    pub capacity_unit: Option<String>,
    
    #[validate(length(max = 30, message = "Tipo de combustible no puede exceder 30 caracteres"))]
    pub fuel_type: Option<String>,
    
    #[validate(range(min = 0.0, message = "Consumo de combustible debe ser mayor a 0"))]
    pub fuel_consumption: Option<f64>,
    
    pub driver_id: Option<Uuid>,
    pub status: VehicleStatus,
    pub purchase_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

/// Request para actualizar un vehículo
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateVehicle {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub name: Option<String>,
    
    #[validate(length(max = 20, message = "Matrícula no puede exceder 20 caracteres"))]
    pub license_plate: Option<String>,
    
    #[validate(length(min = 2, max = 50, message = "Tipo de vehículo debe tener entre 2 y 50 caracteres"))]
    pub vehicle_type: Option<String>,
    
    #[validate(range(min = 0.0, message = "Capacidad debe ser mayor a 0"))]
    pub capacity: Option<f64>,
    
    #[validate(length(max = 20, message = "Unidad de capacidad no puede exceder 20 caracteres"))]
    pub capacity_unit: Option<String>,
    
    #[validate(length(max = 30, message = "Tipo de combustible no puede exceder 30 caracteres"))]
    pub fuel_type: Option<String>,
    
    #[validate(range(min = 0.0, message = "Consumo de combustible debe ser mayor a 0"))]
    pub fuel_consumption: Option<f64>,
    
    pub driver_id: Option<Uuid>,
    pub status: Option<VehicleStatus>,
    pub purchase_date: Option<DateTime<Utc>>,
    pub last_maintenance: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}
