//! Middleware para la API
//! 
//! Este módulo contiene middleware para autenticación, CORS y otras funcionalidades.

pub mod auth;
pub mod cors;

pub use auth::auth_middleware;
pub use cors::cors_middleware;
