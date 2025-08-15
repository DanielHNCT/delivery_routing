//! Modelo de Tournee
//! 
//! Este módulo contiene el struct Tournee, estados de la tournée y variantes para CRUD operations.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Estados de la tournée
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tournee_status", rename_all = "lowercase")]
pub enum TourneeStatus {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

/// Tournee principal
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tournee {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub driver_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub planned_date: DateTime<Utc>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: TourneeStatus,
    pub total_distance: Option<f64>,
    pub total_duration: Option<f64>,
    pub estimated_cost: Option<f64>,
    pub actual_cost: Option<f64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Request para crear una nueva tournée
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTournee {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub name: String,
    
    #[validate(length(max = 500, message = "Descripción no puede exceder 500 caracteres"))]
    pub description: Option<String>,
    
    pub driver_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    
    pub planned_date: DateTime<Utc>,
    
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: TourneeStatus,
    
    #[validate(range(min = 0.0, message = "Distancia total debe ser mayor a 0"))]
    pub total_distance: Option<f64>,
    
    #[validate(range(min = 0.0, message = "Duración total debe ser mayor a 0"))]
    pub total_duration: Option<f64>,
    
    #[validate(range(min = 0.0, message = "Costo estimado debe ser mayor a 0"))]
    pub estimated_cost: Option<f64>,
    
    #[validate(length(max = 1000, message = "Notas no pueden exceder 1000 caracteres"))]
    pub notes: Option<String>,
}

/// Request para actualizar una tournée
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTournee {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub name: Option<String>,
    
    #[validate(length(max = 500, message = "Descripción no puede exceder 500 caracteres"))]
    pub description: Option<String>,
    
    pub driver_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub planned_date: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: Option<TourneeStatus>,
    
    #[validate(range(min = 0.0, message = "Distancia total debe ser mayor a 0"))]
    pub total_distance: Option<f64>,
    
    #[validate(range(min = 0.0, message = "Duración total debe ser mayor a 0"))]
    pub total_duration: Option<f64>,
    
    #[validate(range(min = 0.0, message = "Costo estimado debe ser mayor a 0"))]
    pub estimated_cost: Option<f64>,
    
    #[validate(range(min = 0.0, message = "Costo actual debe ser mayor a 0"))]
    pub actual_cost: Option<f64>,
    
    #[validate(length(max = 1000, message = "Notas no pueden exceder 1000 caracteres"))]
    pub notes: Option<String>,
}
