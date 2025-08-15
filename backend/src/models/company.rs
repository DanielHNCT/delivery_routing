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
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub siret: Option<String>,
    pub tva_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Request para crear una nueva company
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCompany {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub name: String,
    
    #[validate(length(max = 200, message = "Dirección no puede exceder 200 caracteres"))]
    pub address: Option<String>,
    
    #[validate(length(max = 100, message = "Ciudad no puede exceder 100 caracteres"))]
    pub city: Option<String>,
    
    #[validate(length(max = 20, message = "Código postal no puede exceder 20 caracteres"))]
    pub postal_code: Option<String>,
    
    #[validate(length(max = 100, message = "País no puede exceder 100 caracteres"))]
    pub country: Option<String>,
    
    #[validate(length(max = 20, message = "Teléfono no puede exceder 20 caracteres"))]
    pub phone: Option<String>,
    
    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
    
    #[validate(url(message = "Website inválido"))]
    pub website: Option<String>,
    
    #[validate(length(max = 50, message = "SIRET no puede exceder 50 caracteres"))]
    pub siret: Option<String>,
    
    #[validate(length(max = 50, message = "Número de TVA no puede exceder 50 caracteres"))]
    pub tva_number: Option<String>,
}

/// Request para actualizar una company
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCompany {
    #[validate(length(min = 2, max = 100, message = "Nombre debe tener entre 2 y 100 caracteres"))]
    pub name: Option<String>,
    
    #[validate(length(max = 200, message = "Dirección no puede exceder 200 caracteres"))]
    pub address: Option<String>,
    
    #[validate(length(max = 100, message = "Ciudad no puede exceder 100 caracteres"))]
    pub city: Option<String>,
    
    #[validate(length(max = 20, message = "Código postal no puede exceder 20 caracteres"))]
    pub postal_code: Option<String>,
    
    #[validate(length(max = 100, message = "País no puede exceder 100 caracteres"))]
    pub country: Option<String>,
    
    #[validate(length(max = 20, message = "Teléfono no puede exceder 20 caracteres"))]
    pub phone: Option<String>,
    
    #[validate(email(message = "Email inválido"))]
    pub email: Option<String>,
    
    #[validate(url(message = "Website inválido"))]
    pub website: Option<String>,
    
    #[validate(length(max = 50, message = "SIRET no puede exceder 50 caracteres"))]
    pub siret: Option<String>,
    
    #[validate(length(max = 50, message = "Número de TVA no puede exceder 50 caracteres"))]
    pub tva_number: Option<String>,
}
