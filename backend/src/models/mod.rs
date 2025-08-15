//! Modelos de datos para la API
//! 
//! Este módulo contiene todos los structs que representan las entidades de la base de datos.

pub mod auth;
pub mod company;
pub mod user;
pub mod vehicle;
pub mod tournee;
pub mod package;

pub use auth::*;
pub use company::*;
pub use user::*;
pub use vehicle::*;
pub use tournee::*;
pub use package::*;
