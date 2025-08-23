use axum::{
    Router,
    routing::{post, get},
};
use crate::api::colis_prive::*;
use crate::state::AppState;

/// Crear el router para endpoints de Colis Privé
pub fn create_colis_prive_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_colis_prive))           // 🆕 NUEVO: Login directo
        .route("/auth", post(authenticate_colis_prive))     // 🔄 MANTENER: Para compatibilidad
        .route("/tournee", post(get_tournee_data))
        .route("/health", get(health_check))
}
