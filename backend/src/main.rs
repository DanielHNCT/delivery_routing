mod api;
mod config;
mod state;
mod database;
mod services;
mod utils;
mod client;
mod models;
mod external_models;

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
    response::Json,
};
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, error};
use dotenvy::dotenv;
use serde_json::json;

use api::*;
use config::*;
use state::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Cargar variables de entorno
    dotenv().ok();

    // Configurar logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🚚 Delivery Route Optimizer - API REST");
    info!("=====================================");

    // Inicializar base de datos
    let pool = match crate::database::connection::create_pool(None).await {
        Ok(pool) => {
            info!("✅ Base de datos conectada exitosamente");
            pool
        }
        Err(e) => {
            error!("❌ Error conectando a la base de datos: {}", e);
            return Err(anyhow::anyhow!("Error de base de datos: {}", e));
        }
    };

    // Crear router de la API
    let app_state = AppState {
        pool,
        config: EnvironmentConfig::default(),
    };
    
    let app = Router::new()
        .route("/test", get(test_endpoint))
        .route("/api/colis-prive/auth", post(authenticate_colis_prive))
        .route("/api/colis-prive/tournee", post(get_tournee_data))
        .route("/api/colis-prive/health", get(health_check))
        .with_state(app_state);

    // Puerto del servidor
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;

    info!("🌐 Servidor iniciando en http://{}", addr);
    info!("🔍 Endpoints disponibles:");
    info!("   GET  /test - Endpoint de prueba");
    info!("   POST /api/colis-prive/auth - Autenticación Colis Privé");
    info!("   POST /api/colis-prive/tournee - Tournée Colis Privé");
    info!("   GET  /api/colis-prive/health - Health check Colis Privé");

    // Iniciar servidor en background
    let server_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| {
                error!("❌ Error del servidor: {}", e);
                e
            })
    });

    // La API ahora es completamente stateless - no hay conexiones automáticas

    // Esperar a que el servidor termine
    if let Err(e) = server_handle.await? {
        error!("❌ Servidor terminó con error: {}", e);
    }

    info!("👋 Servidor terminado");
    Ok(())
}

/// Endpoint de prueba simple
async fn test_endpoint() -> Json<serde_json::Value> {
    Json(json!({
        "message": "¡API funcionando correctamente!",
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// La función run_colis_prive_demo() ha sido eliminada
// La API ahora es completamente stateless y solo responde a requests HTTP

/// Señal de apagado graceful
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("🛑 Señal Ctrl+C recibida, apagando servidor...");
        },
        _ = terminate => {
            info!("🛑 Señal de terminación recibida, apagando servidor...");
        },
    }
}
