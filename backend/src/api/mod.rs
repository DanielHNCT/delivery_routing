//! API del sistema
//! 
//! Este módulo contiene todos los handlers HTTP para la API REST,
//! organizados por entidad del negocio.

pub mod auth;
pub mod companies;
pub mod users;
pub mod vehicles;
pub mod tournees;
pub mod packages;
pub mod analytics;
pub mod routers;

pub use auth::*;
pub use companies::*;
pub use users::*;
pub use vehicles::*;
pub use tournees::*;
pub use packages::*;
pub use analytics::*;

use axum::Router;
use crate::state::AppState;

/// Crear el router principal de la API
pub fn create_api_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::create_auth_router())
        .nest("/companies", routers::create_companies_router())
        .nest("/users", routers::create_users_router())
        .nest("/vehicles", routers::create_vehicles_router())
        .nest("/tournees", routers::create_tournees_router())
        .nest("/packages", routers::create_packages_router())
        .nest("/analytics", routers::create_analytics_router())
}
