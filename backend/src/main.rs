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

use api::*;
use config::*;
use state::*;
use migration::*;
use cache::{RedisClient, AuthCache, TourneeCache, CacheConfig};
use migration::services::MigrationService;
use services::colis_prive_service::{authenticate_colis_prive_cached, get_tournee_data_cached};

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
        .route("/api/colis-prive/auth", post(authenticate_colis_prive))
        // .route("/api/colis-prive/auth/cached", post(authenticate_colis_prive_cached))
        .route("/api/colis-prive/tournee", post(get_tournee_data))
        // .route("/api/colis-prive/tournee/cached", post(get_tournee_data_cached))
        .route("/api/colis-prive/mobile-tournee", post(get_mobile_tournee))
        .route("/api/colis-prive/mobile-tournee-structured", post(colis_prive::get_mobile_tournee_structured))
        .route("/api/colis-prive/mobile-tournee-updated", post(api::colis_prive::get_mobile_tournee_structured))
        .route("/api/colis-prive/refresh-token", post(api::colis_prive::refresh_colis_prive_token))
        .route("/api/colis-prive/mobile-tournee-with-retry", post(api::colis_prive::mobile_tournee_with_retry))
        .route("/api/colis-prive/health", get(api::colis_prive::health_check_colis_prive))
        // NUEVOS ENDPOINTS PARA SISTEMA DE VERSIONES
        // Sistema de versiones y reverse engineering (TEMPORALMENTE COMENTADO)
        // .route("/api/colis-prive/check-version", post(api::colis_prive::check_app_version))
        // .route("/api/colis-prive/download-version/:binary_id", get(api::colis_prive::download_app_version))
        // .route("/api/colis-prive/audit-install", post(api::colis_prive::audit_app_install))
        // .route("/api/colis_prive/versions", get(api::colis_prive::list_app_versions))
        // .route("/api/colis-prive/version-stats", get(api::colis_prive::get_version_stats))
        // .route("/api/colis-prive/start-reverse-engineering/:binary_id", post(api::colis_prive::start_reverse_engineering))
        // NUEVOS ENDPOINTS PARA FLUJO COMPLETO (RESUELVE EL 401)
        .route("/api/colis-prive/complete-auth-flow", post(api::colis_prive::complete_authentication_flow))
        .route("/api/colis-prive/reconnect", post(api::colis_prive::handle_reconnection))
        // NUEVOS ENDPOINTS v3.3.0.9 - FLUJO EXACTO DE LA APP OFICIAL
        .route("/api/colis-prive/v3/complete-flow", post(api::colis_prive::execute_complete_flow_v3))
        .route("/api/colis-prive/v3/reconnect", post(api::colis_prive::reconnect_with_tokens_v3))
        // 🆕 NUEVO: Endpoint para lettre de voiture solo (sin login completo)
        .route("/api/colis-prive/lettre-voiture-only", post(api::colis_prive::get_lettre_voiture_only))
        .route("/api/migration/status", get(get_migration_status))
        .route("/api/migration/strategy", post(change_migration_strategy))
        .route("/api/migration/metrics", get(get_migration_metrics))
        .route("/api/migration/progress", post(force_migration_progress))
        .route("/api/migration/rollback", post(force_migration_rollback))
        .route("/api/migration/health", get(migration_health_check))
        .with_state(app_state);

    // Puerto del servidor
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;

    info!("🌐 Servidor iniciando en http://{}", addr);
    info!("🔍 Endpoints disponibles:");
    info!("   GET  /test - Endpoint de prueba");
    info!("   POST /api/colis-prive/auth - Autenticación Colis Privé");
    // info!("   POST /api/colis-prive/auth/cached - Autenticación Colis Privé con cache");
    info!("   POST /api/colis-prive/tournee - Tournée Colis Privé");
    // info!("   POST /api/colis-prive/tournee/cached - Tournée Colis Privé con cache");
    info!("   POST /api/colis-prive/mobile-tournee - Tournée Móvil Colis Privé");
    info!("   POST /api/colis-prive/mobile-tournee-structured - Tournée Móvil Colis Privé Estructurada");
    info!("   POST /api/colis-prive/mobile-tournee-updated - Tournée Móvil Colis Privé Actualizada");
    info!("   POST /api/colis-prive/refresh-token - Refrescar token de Colis Privé");
    info!("   POST /api/colis-prive/mobile-tournee-with-retry - Tournée Móvil Colis Privé con retry");
    info!("   GET  /api/colis-prive/health - Health check Colis Privé");
    // NUEVOS ENDPOINTS PARA SISTEMA DE VERSIONES (TEMPORALMENTE COMENTADO)
    // info!("   POST /api/colis-prive/check-version - Verificar versión de la app");
    // info!("   GET  /api/colis-prive/download-version/:binary_id - Descargar versión de la app");
    // info!("   POST /api/colis-prive/audit-install - Registrar auditoría de instalación");
    // info!("   GET  /api/colis-prive/versions - Listar versiones disponibles");
    // info!("   GET  /api/colis-prive/version-stats - Obtener estadísticas de versiones");
    // info!("   POST /api/colis-prive/start-reverse-engineering/:binary_id - Iniciar reverse engineering");
    // NUEVOS ENDPOINTS PARA FLUJO COMPLETO (RESUELVE EL 401)
    info!("   POST /api/colis-prive/complete-auth-flow - Flujo completo de autenticación (RESUELVE EL 401)");
    info!("   POST /api/colis-prive/reconnect - Manejo específico de reconexión (RESUELVE EL 401)");
    // 🆕 NUEVO: Endpoint para lettre de voiture solo
    info!("   POST /api/colis-prive/lettre-voiture-only - Lettre de voiture solo (sin login completo)");
    // NUEVOS ENDPOINTS v3.3.0.9 - FLUJO EXACTO DE LA APP OFICIAL
    info!("   POST /api/colis-prive/v3/complete-flow - Flujo completo v3.3.0.9");
    info!("   POST /api/colis-prive/v3/reconnect - Reconexión v3.3.0.9");
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
