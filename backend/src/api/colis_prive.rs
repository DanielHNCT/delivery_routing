use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::json;
use log;
use reqwest;
use crate::{
    state::AppState,
    services::colis_prive_service::{ColisPriveAuthRequest, GetTourneeRequest, ColisPriveAuthResponse},
};

/// POST /api/colis-prive/auth - Autenticar con Colis Privé (API Web)
pub async fn authenticate_colis_prive(
    State(_state): State<AppState>,
    Json(credentials): Json<ColisPriveAuthRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    log::info!("🔐 Iniciando autenticación Colis Privé (API Web)");
    
    // Clonar las credenciales para poder usarlas después
    let username = credentials.username.clone();
    let societe = credentials.societe.clone();
    
    match authenticate_colis_prive_web(&credentials).await {
        Ok(response) => {
            if response.success {
                let auth_response = json!({
                    "success": true,
                    "authentication": {
                        "token": response.token,
                        "matricule": response.matricule,
                        "message": response.message
                    },
                    "credentials_used": {
                        "username": username,
                        "societe": societe
                    },
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "api_type": "web"
                });
                Ok(Json(auth_response))
            } else {
                let error_response = json!({
                    "success": false,
                    "error": {
                        "message": response.message,
                        "code": "AUTH_FAILED"
                    },
                    "credentials_used": {
                        "username": username,
                        "societe": societe
                    },
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });
                Ok(Json(error_response))
            }
        }
        Err(e) => {
            log::error!("❌ Error en autenticación Colis Privé: {}", e);
            let error_response = json!({
                "success": false,
                "error": {
                    "message": format!("Error interno del servidor: {}", e),
                    "code": "INTERNAL_ERROR"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            Ok(Json(error_response))
        }
    }
}

/// 🔧 FUNCIÓN AUXILIAR: Autenticación web con headers exactos del navegador
async fn authenticate_colis_prive_web(
    credentials: &ColisPriveAuthRequest
) -> Result<ColisPriveAuthResponse, anyhow::Error> {
    log::info!("🔐 Autenticando con Colis Privé (API Web - Headers exactos)");
    
    // Validar credenciales básicas
    if credentials.username.is_empty() || credentials.password.is_empty() || credentials.societe.is_empty() {
        anyhow::bail!("Credenciales incompletas");
    }
    
    // 🔧 IMPLEMENTACIÓN WEB: Autenticación con headers exactos del navegador
    let login_field = format!("{}_{}", credentials.societe, credentials.username);
    
    let auth_url = "https://wsauthentificationexterne.colisprive.com/api/auth/login/Membership";
    let auth_payload = json!({
        "login": login_field,
        "password": credentials.password,
        "societe": credentials.societe,
        "commun": {
            "dureeTokenInHour": 24
        }
    });
    
    log::info!("📤 Enviando autenticación a: {}", auth_url);
    log::info!("🔑 Login field: {}", login_field);
    
    // 🎯 HEADERS EXACTOS DEL NAVEGADOR
    let auth_response = reqwest::Client::new()
        .post(auth_url)
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Encoding", "gzip, deflate, br, zstd")
        .header("Accept-Language", "fr-FR,fr;q=0.5")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Content-Type", "application/json")
        .header("Origin", "https://gestiontournee.colisprive.com")
        .header("Referer", "https://gestiontournee.colisprive.com/")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
        .json(&auth_payload)
        .send()
        .await
        .map_err(|e| {
            log::error!("❌ Error de conexión con Colis Privé: {}", e);
            anyhow::anyhow!("Error de conexión: {}", e)
        })?;
    
    let status = auth_response.status();
    if !status.is_success() {
        let error_text = auth_response.text().await.unwrap_or_default();
        log::error!("❌ Colis Privé respondió con error {}: {}", status, error_text);
        anyhow::bail!("Error de autenticación {}: {}", status, error_text);
    }
    
    let auth_text = auth_response.text().await.map_err(|e| {
        log::error!("❌ Error leyendo respuesta de autenticación: {}", e);
        anyhow::anyhow!("Error leyendo respuesta: {}", e)
    })?;
    
    log::info!("📥 Respuesta de autenticación recibida: {}", &auth_text[..auth_text.len().min(200)]);
    
    // Parsear la respuesta de Colis Privé
    let auth_data: serde_json::Value = serde_json::from_str(&auth_text).map_err(|e| {
        log::error!("❌ Error parseando respuesta de autenticación: {}", e);
        anyhow::anyhow!("Error parseando respuesta: {}", e)
    })?;
    
    // Extraer el token SsoHopps real
    let sso_hopps = auth_data.get("tokens")
        .and_then(|tokens| tokens.get("SsoHopps"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Token SsoHopps no encontrado en la respuesta"))?;
    
    log::info!("✅ Token SsoHopps obtenido exitosamente");
    
    let auth_response = ColisPriveAuthResponse {
        success: true,
        message: "Autenticación exitosa con Colis Privé (API Web)".to_string(),
        token: Some(sso_hopps.to_string()),
        matricule: Some(credentials.username.clone()),
    };
    
    Ok(auth_response)
}

/// POST /api/colis-prive/tournee - Obtener tournée (API Web)
pub async fn get_tournee_data(
    State(_state): State<AppState>,
    Json(request): Json<GetTourneeRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    log::info!("🔄 Obteniendo tournée para: {} (API Web)", request.matricule);
    
    // ✅ SOLO FUNCIONA CON API WEB - NO REQUIERE DEVICE_INFO
    
    // Crear credenciales para el servicio
    let credentials = ColisPriveAuthRequest {
        username: request.username.clone(),
        password: request.password.clone(),
        societe: request.societe.clone(),
    };
    
    // Autenticar primero para obtener el token
    let auth_result = authenticate_colis_prive_web(&credentials).await;
    match auth_result {
        Ok(auth_response) => {
            if let Some(token) = auth_response.token {
                // Ahora usar el token para obtener la lettre de voiture
                match get_lettre_de_voiture_web(&token, &request).await {
                    Ok(lettre_data) => {
                        let response = json!({
                            "success": true,
                            "tournee": {
                                "matricule": request.matricule,
                                "societe": request.societe,
                                "lettre_de_voiture": lettre_data
                            },
                            "timestamp": chrono::Utc::now().to_rfc3339(),
                            "api_type": "web"
                        });
                        Ok(Json(response))
                    }
                    Err(e) => {
                        log::error!("❌ Error obteniendo lettre de voiture: {}", e);
                        let error_response = json!({
                            "success": false,
                            "error": {
                                "message": format!("Error obteniendo lettre de voiture: {}", e),
                                "code": "LETTRE_ERROR"
                            },
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        });
                        Ok(Json(error_response))
                    }
                }
            } else {
                let error_response = json!({
                    "success": false,
                    "error": {
                        "message": "No se pudo obtener el token de autenticación",
                        "code": "TOKEN_ERROR"
                    },
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });
                Ok(Json(error_response))
            }
        }
        Err(e) => {
            log::error!("❌ Error de autenticación para tournée: {}", e);
            let error_response = json!({
                "success": false,
                "error": {
                    "message": format!("Error de autenticación: {}", e),
                    "code": "AUTH_ERROR"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            Ok(Json(error_response))
        }
    }
}

/// 🔧 FUNCIÓN AUXILIAR: Obtener lettre de voiture con headers web
async fn get_lettre_de_voiture_web(
    token: &str,
    request: &GetTourneeRequest
) -> Result<serde_json::Value, anyhow::Error> {
    log::info!("📄 Obteniendo lettre de voiture (API Web)");
    
    let lettre_url = "https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getLettreVoitureEco_POST";
    let lettre_payload = json!({
        "Societe": request.societe,
        "Matricule": request.matricule,
        "DateDebut": chrono::Utc::now().format("%Y-%m-%dT00:00:00.000Z").to_string(),
        "Agence": null,
        "Concentrateur": null
    });
    
    log::info!("📤 Enviando request a: {}", lettre_url);
    
    // 🎯 HEADERS EXACTOS DEL NAVEGADOR
    let response = reqwest::Client::new()
        .post(lettre_url)
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Encoding", "gzip, deflate, br, zstd")
        .header("Accept-Language", "fr-FR,fr;q=0.5")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Content-Type", "application/json")
        .header("Origin", "https://gestiontournee.colisprive.com")
        .header("Referer", "https://gestiontournee.colisprive.com/")
        .header("SsoHopps", token)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
        .json(&lettre_payload)
        .send()
        .await
        .map_err(|e| {
            log::error!("❌ Error de conexión con Colis Privé: {}", e);
            anyhow::anyhow!("Error de conexión: {}", e)
        })?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        log::error!("❌ Colis Privé respondió con error {}: {}", status, error_text);
        anyhow::bail!("Error obteniendo lettre de voiture {}: {}", status, error_text);
    }
    
    let response_text = response.text().await.map_err(|e| {
        log::error!("❌ Error leyendo respuesta: {}", e);
        anyhow::anyhow!("Error leyendo respuesta: {}", e)
    })?;
    
    log::info!("📥 Respuesta recibida: {}", &response_text[..response_text.len().min(200)]);
    
    // Parsear la respuesta
    let lettre_data: serde_json::Value = serde_json::from_str(&response_text).map_err(|e| {
        log::error!("❌ Error parseando respuesta: {}", e);
        anyhow::anyhow!("Error parseando respuesta: {}", e)
    })?;
    
    // Verificar si hay error de autorización
    if let Some(error_msg) = lettre_data.get("Message") {
        if error_msg.as_str() == Some("Authorization has been denied for this request.") {
            log::error!("❌ Error de autorización: {}", error_msg);
            anyhow::bail!("Error de autorización: {}", error_msg);
        }
    }
    
    log::info!("✅ Lettre de voiture obtenido exitosamente");
    Ok(lettre_data)
}

/// GET /api/colis-prive/health - Health check
pub async fn health_check_colis_prive() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "colis_prive_web_api",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "endpoints": {
            "auth": "POST /api/colis-prive/auth",
            "tournee": "POST /api/colis-prive/tournee",
            "health": "GET /api/colis-prive/health"
        }
    }))
}
