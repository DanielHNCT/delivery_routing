//! Biblioteca principal para el sistema de optimización de rutas de entrega
//! 
//! Esta biblioteca proporciona una API REST completa para gestionar
//! empresas, usuarios, vehículos, tournées y paquetes con optimización
//! de rutas y integración con APIs externas.

pub mod api;
pub mod config;
pub mod models;
pub mod middleware;
pub mod services;
pub mod utils;
pub mod routes;
pub mod state;

pub use api::*;
pub use config::*;
pub use models::*;
pub use middleware::*;
pub use services::*;
pub use utils::*;
pub use routes::*;
pub use state::*;
