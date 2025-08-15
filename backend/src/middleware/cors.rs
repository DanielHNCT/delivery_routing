//! Middleware de CORS
//! 
//! Este middleware configura CORS para permitir requests desde diferentes orígenes.

use tower_http::cors::{Any, CorsLayer};

/// Configura el middleware de CORS
pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(true)
}
