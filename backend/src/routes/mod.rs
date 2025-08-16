//! Routes module
//! 
//! Este módulo define las rutas HTTP de la aplicación usando Axum.
//! Organiza todos los endpoints y sus handlers correspondientes.

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::{
    api::{
        auth, users, companies, vehicles, tournees, packages, analytics,
    },
    state::AppState,
};

/// Crear todas las rutas de la aplicación
pub fn create_routes(app_state: AppState) -> Router {
    Router::new()
        // Rutas de autenticación (públicas)
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/refresh", post(auth::refresh_token))
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/me", get(auth::me))
        
        // Rutas de usuarios
        .route("/api/users", get(users::get_users))
        .route("/api/users", post(users::create_user))
        .route("/api/users/:id", get(users::get_user))
        .route("/api/users/:id", put(users::update_user))
        .route("/api/users/:id", delete(users::delete_user))
        
        // Rutas de empresas
        .route("/api/companies", get(companies::get_companies))
        .route("/api/companies", post(companies::create_company))
        .route("/api/companies/:id", get(companies::get_company))
        .route("/api/companies/:id", put(companies::update_company))
        .route("/api/companies/:id", delete(companies::delete_company))
        
        // Rutas de vehículos
        .route("/api/vehicles", get(vehicles::get_vehicles))
        .route("/api/vehicles", post(vehicles::create_vehicle))
        .route("/api/vehicles/:id", get(vehicles::get_vehicle))
        .route("/api/vehicles/:id", put(vehicles::update_vehicle))
        .route("/api/vehicles/:id", delete(vehicles::delete_vehicle))
        
        // Rutas de tournées
        .route("/api/tournees", get(tournees::get_tournees))
        .route("/api/tournees", post(tournees::create_tournee))
        .route("/api/tournees/:id", get(tournees::get_tournee))
        .route("/api/tournees/:id/start", post(tournees::start_tournee))
        .route("/api/tournees/:id/end", post(tournees::end_tournee))
        .route("/api/tournees/:id", delete(tournees::delete_tournee))
        
        // Rutas de paquetes
        .route("/api/packages", get(packages::get_packages))
        .route("/api/packages", post(packages::create_package))
        .route("/api/packages/:id", get(packages::get_package))
        .route("/api/packages/:id/delivered", post(packages::mark_delivered))
        .route("/api/packages/:id/failed", post(packages::mark_failed))
        .route("/api/packages/:id", delete(packages::delete_package))
        
        // Rutas de analytics
        .route("/api/analytics/dashboard", get(analytics::get_dashboard_summary))
        .route("/api/analytics/tournees", get(analytics::get_performance_by_tournee))
        .route("/api/analytics/drivers", get(analytics::get_driver_performance))
        .route("/api/analytics/vehicles", get(analytics::get_vehicle_performance))
        
        // Agregar estado compartido
        .with_state(app_state)
}