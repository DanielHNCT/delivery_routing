//! API endpoints para Delivery Route Optimizer
//! 
//! Este módulo contiene todos los endpoints de la API REST organizados por funcionalidad.

pub mod auth;
pub mod companies;
pub mod users;
pub mod vehicles;
pub mod tournees;
pub mod packages;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::middleware::cors::cors_middleware;

/// Configura todas las rutas de la API
pub fn create_api_router() -> Router {
    Router::new()
        // Rutas públicas (sin autenticación)
        .route("/health", get(health_check))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/register", post(auth::register))
        
        // Rutas protegidas (con autenticación) - Comentadas temporalmente
        // .route("/api/auth/me", get(auth::me))
        // .route("/api/companies", get(companies::list_companies))
        // .route("/api/companies", post(companies::create_company))
        // .route("/api/companies/:id", get(companies::get_company))
        // .route("/api/companies/:id", put(companies::update_company))
        // .route("/api/users", get(users::list_users))
        // .route("/api/users", post(users::create_user))
        // .route("/api/users/:id", get(users::get_user))
        // .route("/api/users/:id", put(users::update_user))
        // .route("/api/users/:id", delete(users::delete_user))
        // .route("/api/vehicles", get(vehicles::list_vehicles))
        // .route("/api/vehicles", post(vehicles::create_vehicle))
        // .route("/api/vehicles/:id", get(vehicles::get_vehicle))
        // .route("/api/vehicles/:id", put(vehicles::update_vehicle))
        // .route("/api/tournees", get(tournees::list_tournees))
        // .route("/api/tournees", post(tournees::create_tournee))
        // .route("/api/tournees/:id", get(tournees::get_tournee))
        // .route("/api/tournees/:id", put(tournees::update_tournee))
        // .route("/api/tournees/:id/packages", get(tournees::get_tournee_packages))
        // .route("/api/packages", get(packages::list_packages))
        // .route("/api/packages", post(packages::create_package))
        // .route("/api/packages/:id", get(packages::get_package))
        // .route("/api/packages/:id", put(packages::update_package))
        .layer(cors_middleware())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "🚚 Delivery Route Optimizer API - OK"
}
