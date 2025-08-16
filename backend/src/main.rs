mod api;
mod config;
mod models;
mod middleware;
mod services;
mod utils;
mod routes;
mod state;
mod client;
mod external_models;
mod database;

use anyhow::Result;
use axum::{
    Extension, Router,
    http::Method,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};
use dotenvy::dotenv;

use api::*;
use config::*;
use models::*;
use middleware::*;
use services::*;
use utils::*;
use routes::*;
use state::*;

use crate::client::ColisPriveClient;
use crate::utils::decode_base64;
use config::{COLIS_PRIVE_USERNAME, COLIS_PRIVE_PASSWORD, COLIS_PRIVE_SOCIETE};

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
        .merge(crate::api::create_api_router())
        .with_state(app_state);

    // Puerto del servidor
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;

    info!("🌐 Servidor iniciando en http://{}", addr);

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

    // Ejecutar funcionalidad existente de Colis Privé
    run_colis_prive_demo().await?;

    // Esperar a que el servidor termine
    if let Err(e) = server_handle.await? {
        error!("❌ Servidor terminó con error: {}", e);
    }

    info!("👋 Servidor terminado");
    Ok(())
}

/// Función para ejecutar la demo existente de Colis Privé
async fn run_colis_prive_demo() -> Result<()> {
    info!("🔐 Ejecutando demo de Colis Privé...");

    // Verificar credenciales
    if COLIS_PRIVE_USERNAME == "tu_usuario_aqui" ||
       COLIS_PRIVE_PASSWORD == "tu_password_aqui" ||
       COLIS_PRIVE_SOCIETE == "tu_societe_aqui" {
        info!("⚠️  Credenciales de Colis Privé no configuradas, saltando demo");
        return Ok(());
    }

    // Crear cliente
    let mut client = ColisPriveClient::new()?;

    info!("🔐 Intentando login con:");
    info!("   Login: {}", COLIS_PRIVE_USERNAME);
    info!("   Societe: {}", COLIS_PRIVE_SOCIETE);

    // Login
    let login_response = client.login(COLIS_PRIVE_USERNAME, COLIS_PRIVE_PASSWORD, COLIS_PRIVE_SOCIETE).await?;

    info!("✅ Login exitoso!");
    info!("   🔐 Authentifié: {}", login_response.isAuthentif);
    info!("   👤 Identity: {}", login_response.identity);
    info!("   📋 Matricule: {}", login_response.matricule);
    info!("   🏢 Societe: {}", login_response.societe);
    info!("   🔑 Token SsoHopps: {}...", &login_response.tokens.SsoHopps[..50.min(login_response.tokens.SsoHopps.len())]);

    // Pilot access
    let _pilot_response = client.get_pilot_access(
        &login_response.tokens.SsoHopps,
        &login_response.matricule,
        &login_response.societe
    ).await?;

    info!("✅ Pilot access exitoso!");

    // Dashboard info - PROBAR CON CURL PRIMERO
    info!("🔍 Probando Dashboard info con curl...");
    let _dashboard_response_curl = client.get_dashboard_info_curl(
        &login_response.tokens.SsoHopps,
        &login_response.societe,
        &login_response.matricule,
        "2025-08-14"  // FECHA DE HOY
    ).await?;
    
    info!("✅ Dashboard info con curl exitoso!");
    
    // Dashboard info - PROBAR CON REQWEST
    info!("🔍 Probando Dashboard info con reqwest...");
    let _dashboard_response = client.get_dashboard_info(
        &login_response.tokens.SsoHopps,
        &login_response.societe,
        &login_response.matricule,
        "2025-08-14"  // FECHA DE HOY
    ).await?;
    
    info!("✅ Dashboard info con reqwest exitoso!");

    // Obtener tournée con curl (que funciona)
    let date = "2025-08-14"; // FECHA DE HOY
    info!("📅 Obteniendo tournée para la fecha: {}", date);

    match client.get_tournee_curl(&login_response.tokens.SsoHopps, &login_response.societe, &login_response.matricule, date).await {
        Ok(tournee_data) => {
            info!("✅ Tournée obtenida exitosamente");
            info!("\n🔍 Decodificando datos Base64...");

            match decode_base64(&tournee_data) {
                Ok(decoded_str) => {
                    if decoded_str.contains("No hay tournées programadas") {
                        info!("ℹ️  {}", decoded_str);
                        info!("✅ Sistema funcionando correctamente - La API responde normalmente");
                    } else {
                        info!("✅ Datos decodificados correctamente");
                        info!("\n📊 Información de la tournée:");
                        info!("📋 Datos completos de la tournée:");
                        info!("{}", decoded_str);
                    }
                    info!("\n🎉 Demo de Colis Privé completado exitosamente!");
                }
                Err(e) => {
                    info!("❌ Error decodificando Base64: {}", e);
                    info!("📋 Datos crudos recibidos: {}", tournee_data);
                }
            }
        }
        Err(e) => {
            info!("❌ Error obteniendo tournée: {}", e);
        }
    }

    Ok(())
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
