//! Servicios para Colis Privé (API Web)
//! 
//! Este módulo contiene los servicios mínimos necesarios para la API web de Colis Privé.

use serde::{Deserialize, Serialize};

/// Request de autenticación para Colis Privé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColisPriveAuthRequest {
    pub username: String,
    pub password: String,
    pub societe: String,
}

/// Response de autenticación de Colis Privé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColisPriveAuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub matricule: Option<String>,
}

/// Request para obtener tournée
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTourneeRequest {
    pub username: String,
    pub password: String,
    pub societe: String,
    pub matricule: String,
    pub date: Option<String>, // Campo opcional para fecha
}

/// Request para obtener paquetes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPackagesRequest {
    pub matricule: String,
    pub date: Option<String>, // Campo opcional para fecha
}

/// Datos de un paquete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageData {
    pub id: String,
    pub tracking_number: String,
    pub recipient_name: String,
    pub address: String,
    pub status: String,
    pub instructions: String,
    pub phone: String,
    pub priority: String,
}

/// Datos de error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
}

/// Response para obtener paquetes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPackagesResponse {
    pub success: bool,
    pub message: String,
    pub packages: Option<Vec<PackageData>>,
    pub error: Option<ErrorData>,
}
