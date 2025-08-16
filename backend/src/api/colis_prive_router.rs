use axum::{
    Router,
    routing::{post, get},
};
use crate::api::colis_prive::*;
use crate::state::AppState;

/// Crear el router para endpoints de Colis Privé
pub fn create_colis_prive_router() -> Router<AppState> {
    Router::new()
        .route("/auth", post(authenticate_colis_prive))
        .route("/tournee", post(get_tournee_data))
        .route("/health", get(health_check))
}
