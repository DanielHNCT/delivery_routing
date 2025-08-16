//! API del sistema
//! 
//! Este módulo contiene todos los handlers HTTP para la API REST,
//! organizados por entidad del negocio.

pub mod colis_prive;
pub mod colis_prive_router;

pub use colis_prive::*;
pub use colis_prive_router::*;

use axum::Router;
use crate::state::AppState;

/// Crear el router principal de la API
pub fn create_api_router() -> Router<AppState> {
    Router::new()
        .nest("/colis-prive", create_colis_prive_router())
}
