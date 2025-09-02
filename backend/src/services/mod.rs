//! Services module
//! 
//! Este módulo contiene la lógica de negocio y servicios de la aplicación.
//! Los servicios encapsulan operaciones complejas que pueden involucrar 
//! múltiples modelos o integraciones externas.

pub mod colis_prive_service;
// pub mod app_version_service; // Comentado temporalmente por errores de compilación
// pub mod colis_prive_flow_service; // Comentado temporalmente por errores de compilación
// pub mod colis_prive_complete_flow_service; // Comentado temporalmente por errores de compilación
pub mod colis_prive_web_service;
pub mod geocoding_service;

pub use colis_prive_service::*;
// pub use app_version_service::*; // Comentado temporalmente
// pub use colis_prive_flow_service::*; // Comentado temporalmente
// pub use colis_prive_complete_flow_service::*; // Comentado temporalmente
pub use colis_prive_web_service::*;
pub use geocoding_service::*;