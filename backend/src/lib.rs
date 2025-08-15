//! Delivery Route Optimizer API
//! 
//! API REST completa para optimización de rutas de entrega con soporte multi-tenant
//! y integración con PostgreSQL + PostGIS.

pub mod api;
pub mod middleware;
pub mod models;
pub mod database;
pub mod utils;
pub mod external_models;
pub mod external_utils;

// Re-export common types for convenience
pub use database::connection::DatabasePool;
pub use utils::errors::AppError;
