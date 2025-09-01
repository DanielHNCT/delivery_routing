mod api;
mod config;
mod state;
mod database;
mod services;
mod utils;
mod client;
mod models;
mod external_models;
mod cache;
mod migration;

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

use config::*;
use state::*;

use cache::{RedisClient, CacheConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Cargar variables de entorno
    dotenv().ok();

    // Configurar logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🚚 Delivery Route Optimizer - API Web Colis Privé");
    info!("================================================");

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

    // Inicializar Redis y cache
    let cache_config = CacheConfig::default();
    let redis_client = match RedisClient::new(cache_config.clone()).await {
        Ok(client) => {
            info!("✅ Redis conectado exitosamente");
            client
        }
        Err(e) => {
            error!("❌ Error conectando a Redis: {}", e);
            return Err(anyhow::anyhow!("Error de Redis: {}", e));
        }
    };

    // Crear router de la API
    let app_state = AppState::new(pool, EnvironmentConfig::default(), redis_client);
    
    let app = Router::new()
        .route("/test", get(test_endpoint))
        .route("/api/colis-prive/health", get(api::colis_prive::health_check_colis_prive))
        .route("/api/colis-prive/auth", post(api::colis_prive::authenticate_colis_prive))
        .route("/api/colis-prive/packages", post(api::colis_prive::get_packages))
        .route("/api/colis-prive/tournee", post(api::colis_prive::get_tournee_data))
        .route("/api/migration/status", get(migration::api::get_migration_status))
        .route("/api/migration/strategy", post(migration::api::change_migration_strategy))
        .route("/api/migration/metrics", get(migration::api::get_migration_metrics))
        .route("/api/migration/progress", post(migration::api::force_migration_progress))
        .route("/api/migration/rollback", post(migration::api::force_migration_rollback))
        .route("/api/migration/health", get(migration::api::migration_health_check))
        .with_state(app_state);

    // Puerto del servidor
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;

    info!("🌐 Servidor iniciando en http://{}", addr);
    info!("🔍 Endpoints disponibles:");
    info!("   GET  /test - Endpoint de prueba");
    info!("   GET  /api/colis-prive/health - Health check Colis Privé");
    info!("   POST /api/colis-prive/auth - Autenticación Colis Privé");
    info!("   POST /api/colis-prive/packages - Obtener paquetes");
    info!("   POST /api/colis-prive/tournee - Tournée Colis Privé (API Web)");
    info!("   GET  /api/migration/status - Estado de migración");
    info!("   POST /api/migration/strategy - Cambiar estrategia");
    info!("   GET  /api/migration/metrics - Métricas de migración");
    info!("   POST /api/migration/progress - Forzar progresión");
    info!("   POST /api/migration/rollback - Forzar rollback");
    info!("   GET  /api/migration/health - Health check migración");

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
        "message": "¡API Web Colis Privé funcionando correctamente!",
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "api_type": "web_only"
    }))
}

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
