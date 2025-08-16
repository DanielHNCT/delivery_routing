//! Shared application state
//! 
//! Este módulo define el estado compartido de la aplicación que se pasa
//! a través del router de Axum.

use sqlx::PgPool;
use crate::config::EnvironmentConfig;

/// Estado compartido de la aplicación
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: EnvironmentConfig,
}

impl AppState {
    /// Crear un nuevo estado de aplicación
    pub fn new(pool: PgPool, config: EnvironmentConfig) -> Self {
        Self { pool, config }
    }
}