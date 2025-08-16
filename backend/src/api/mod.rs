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

pub use auth::*;
pub use companies::*;
pub use users::*;
pub use vehicles::*;
pub use tournees::*;
pub use packages::*;
pub use analytics::*;
