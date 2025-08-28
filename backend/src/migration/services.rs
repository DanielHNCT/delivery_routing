//! Servicios de migración mínimos
//! 
//! Este módulo contiene los servicios mínimos de migración.

use serde::{Deserialize, Serialize};

/// Configuración de migración
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    pub enabled: bool,
    pub strategy: String,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: "none".to_string(),
        }
    }
}
