//! Modelo de Company
//! 
//! Este módulo contiene el struct Company y sus variantes para CRUD operations.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Company principal
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub subscription_plan: String,
    pub subscription_status: String,
    pub max_drivers: i32,
    pub max_vehicles: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Request para crear una nueva company
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCompany {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub name: String,
    
    #[validate(length(max = 500, message = "Dirección no puede exceder 500 caracteres"))]
    pub address: String,
    
    #[validate(length(max = 50, message = "Plan de suscripción no puede exceder 50 caracteres"))]
    pub subscription_plan: Option<String>,
    
    #[validate(length(max = 20, message = "Estado de suscripción no puede exceder 20 caracteres"))]
    pub subscription_status: Option<String>,
    
    #[validate(range(min = 1, max = 1000, message = "Máximo de conductores debe estar entre 1 y 1000"))]
    pub max_drivers: Option<i32>,
    
    #[validate(range(min = 1, max = 1000, message = "Máximo de vehículos debe estar entre 1 y 1000"))]
    pub max_vehicles: Option<i32>,
}

/// Request para actualizar una company
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCompany {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub name: Option<String>,
    
    #[validate(length(max = 500, message = "Dirección no puede exceder 500 caracteres"))]
    pub address: Option<String>,
    
    #[validate(length(max = 50, message = "Plan de suscripción no puede exceder 50 caracteres"))]
    pub subscription_plan: Option<String>,
    
    #[validate(length(max = 20, message = "Estado de suscripción no puede exceder 20 caracteres"))]
    pub subscription_status: Option<String>,
    
    #[validate(range(min = 1, max = 1000, message = "Máximo de conductores debe estar entre 1 y 1000"))]
    pub max_drivers: Option<i32>,
    
    #[validate(range(min = 1, max = 1000, message = "Máximo de vehículos debe estar entre 1 y 1000"))]
    pub max_vehicles: Option<i32>,
}
