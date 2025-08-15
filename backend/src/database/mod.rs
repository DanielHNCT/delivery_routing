//! Configuración de base de datos
//! 
//! Este módulo contiene la configuración y conexión a PostgreSQL con PostGIS.

pub mod connection;

pub use connection::DatabasePool;
