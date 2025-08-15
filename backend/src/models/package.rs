//! Modelo de Package
//! 
//! Este módulo contiene el struct Package, estados del paquete y variantes para CRUD operations.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Estados del paquete
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "package_status", rename_all = "lowercase")]
pub enum PackageStatus {
    Pending,
    InTransit,
    Delivered,
    Failed,
    Returned,
}

/// Package principal
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Package {
    pub id: Uuid,
    pub company_id: Uuid,
    pub tournee_id: Option<Uuid>,
    pub tracking_number: String,
    pub recipient_name: String,
    pub recipient_address: String,
    pub recipient_city: String,
    pub recipient_postal_code: String,
    pub recipient_phone: Option<String>,
    pub recipient_email: Option<String>,
    pub package_type: String,
    pub weight: Option<f64>,
    pub dimensions: Option<String>,
    pub status: PackageStatus,
    pub priority: i32,
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub delivery_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Request para crear un nuevo paquete
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePackage {
    #[validate(length(min = 5, max = 50, message = "Número de seguimiento debe tener entre 5 y 50 caracteres"))]
    pub tracking_number: String,
    
    #[validate(length(min = 2, max = 100, message = "Nombre del destinatario debe tener entre 2 y 100 caracteres"))]
    pub recipient_name: String,
    
    #[validate(length(min = 10, max = 200, message = "Dirección debe tener entre 10 y 200 caracteres"))]
    pub recipient_address: String,
    
    #[validate(length(min = 2, max = 100, message = "Ciudad debe tener entre 2 y 100 caracteres"))]
    pub recipient_city: String,
    
    #[validate(length(min = 3, max = 20, message = "Código postal debe tener entre 3 y 20 caracteres"))]
    pub recipient_postal_code: String,
    
    #[validate(length(max = 20, message = "Teléfono no puede exceder 20 caracteres"))]
    pub recipient_phone: Option<String>,
    
    #[validate(email(message = "Email inválido"))]
    pub recipient_email: Option<String>,
    
    #[validate(length(min = 2, max = 50, message = "Tipo de paquete debe tener entre 2 y 50 caracteres"))]
    pub package_type: String,
    
    #[validate(range(min = 0.0, message = "Peso debe ser mayor a 0"))]
    pub weight: Option<f64>,
    
    #[validate(length(max = 100, message = "Dimensiones no pueden exceder 100 caracteres"))]
    pub dimensions: Option<String>,
    
    pub status: PackageStatus,
    
    #[validate(range(min = 1, max = 5, message = "Prioridad debe estar entre 1 y 5"))]
    pub priority: i32,
    
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub delivery_notes: Option<String>,
}

/// Request para actualizar un paquete
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePackage {
    #[validate(length(min = 2, max = 100, message = "Nombre del destinatario debe tener entre 2 y 100 caracteres"))]
    pub recipient_name: Option<String>,
    
    #[validate(length(min = 10, max = 200, message = "Dirección debe tener entre 10 y 200 caracteres"))]
    pub recipient_address: Option<String>,
    
    #[validate(length(min = 2, max = 100, message = "Ciudad debe tener entre 2 y 100 caracteres"))]
    pub recipient_city: Option<String>,
    
    #[validate(length(min = 3, max = 20, message = "Código postal debe tener entre 3 y 20 caracteres"))]
    pub recipient_postal_code: Option<String>,
    
    #[validate(length(max = 20, message = "Teléfono no puede exceder 20 caracteres"))]
    pub recipient_phone: Option<String>,
    
    #[validate(email(message = "Email inválido"))]
    pub recipient_email: Option<String>,
    
    #[validate(length(min = 2, max = 50, message = "Tipo de paquete debe tener entre 2 y 50 caracteres"))]
    pub package_type: Option<String>,
    
    #[validate(range(min = 0.0, message = "Peso debe ser mayor a 0"))]
    pub weight: Option<f64>,
    
    #[validate(length(max = 100, message = "Dimensiones no pueden exceder 100 caracteres"))]
    pub dimensions: Option<String>,
    
    pub status: Option<PackageStatus>,
    
    #[validate(range(min = 1, max = 5, message = "Prioridad debe estar entre 1 y 5"))]
    pub priority: Option<i32>,
    
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub delivery_notes: Option<String>,
}
