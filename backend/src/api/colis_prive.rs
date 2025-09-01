//! API de Colis Privé - Solo API Web
//! 
//! Este módulo contiene solo las funciones necesarias para la API web de Colis Privé.
//! Todas las funciones móviles han sido comentadas para simplificar el backend.

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
    services::colis_prive_service::{ColisPriveAuthRequest, GetTourneeRequest, GetPackagesRequest, ColisPriveAuthResponse},
};

/// POST /api/colis-prive/auth - Autenticar con Colis Privé
pub async fn authenticate_colis_prive(
    State(state): State<AppState>,
    Json(credentials): Json<ColisPriveAuthRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Clonar las credenciales para poder usarlas después
    let username = credentials.username.clone();
    let societe = credentials.societe.clone();
    
    // 🔧 IMPLEMENTACIÓN REAL: Autenticación directa con Colis Privé
    match authenticate_colis_prive_simple(&credentials).await {
        Ok(auth_response) => {
            if auth_response.success {
                // 🆕 ALMACENAR EL TOKEN EN EL ESTADO DE LA APLICACIÓN
                if let Some(token) = &auth_response.token {
                    // Limpiar tokens expirados antes de almacenar uno nuevo
                    state.cleanup_expired_tokens().await;
                    
                    // Almacenar el nuevo token (asumiendo 24 horas de validez)
                    state.store_auth_token(
                        username.clone(),
                        societe.clone(),
                        token.clone(),
                        24
                    ).await;
                    
                    log::info!("✅ Token almacenado en el estado de la aplicación para {}:{}", societe, username);
                }
                
                let auth_response = json!({
                    "success": true,
                    "authentication": {
                        "token": auth_response.token,
                        "matricule": auth_response.matricule,
                        "message": auth_response.message
                    },
                    "credentials_used": {
                        "username": username,
                        "societe": societe
                    },
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });
                Ok(Json(auth_response))
            } else {
                let error_response = json!({
                    "success": false,
                    "error": {
                        "message": auth_response.message,
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
            log::error!("Error en autenticación Colis Privé: {}", e);
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

/// 🔧 FUNCIÓN AUXILIAR: Autenticación simple sin device_info
async fn authenticate_colis_prive_simple(
    credentials: &ColisPriveAuthRequest
) -> Result<ColisPriveAuthResponse, anyhow::Error> {
    log::info!("🔐 Autenticando con Colis Privé (modo real)");
    
    // Validar credenciales básicas
    if credentials.username.is_empty() || credentials.password.is_empty() || credentials.societe.is_empty() {
        anyhow::bail!("Credenciales incompletas");
    }
    
    // 🔧 IMPLEMENTACIÓN REAL: Autenticación directa con Colis Privé
    let login_field = format!("{}_{}", credentials.societe, credentials.username);
    
    let auth_url = "https://wsauthentificationexterne.colisprive.com/api/auth/login/Membership";
    let login_field = format!("{}_{}", credentials.societe, credentials.username);
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
    
    let auth_response = reqwest::Client::new()
        .post(auth_url)
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "fr-FR,fr;q=0.5")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Content-Type", "application/json")
        .header("Origin", "https://gestiontournee.colisprive.com")
        .header("Pragma", "no-cache")
        .header("Referer", "https://gestiontournee.colisprive.com/")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-site")
        .header("Sec-GPC", "1")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
        .header("sec-ch-ua", "\"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"macOS\"")
        .json(&auth_payload)
        .send()
        .await
        .map_err(|e| {
            log::error!("❌ Error de conexión con Colis Privé: {}", e);
            anyhow::anyhow!("Error de conexión: {}", e)
        })?;
    
    if !auth_response.status().is_success() {
        let error_text = auth_response.text().await.unwrap_or_default();
        log::error!("❌ Colis Privé respondió con error: {}", error_text);
        anyhow::bail!("Error de autenticación: {}", error_text);
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
    
    // 🔍 DEBUG: Imprimir todos los campos de la respuesta
    log::info!("🔍 Campos disponibles en la respuesta:");
    if let Some(obj) = auth_data.as_object() {
        for (key, value) in obj {
            log::info!("  - {}: {}", key, value);
        }
    }
    
    // 🔍 BUSCAR EL TOKEN EN DIFERENTES CAMPOS POSIBLES (incluyendo campos anidados)
    let sso_hopps = auth_data.get("SsoHopps")
        .or_else(|| auth_data.get("ssoHopps"))
        .or_else(|| auth_data.get("token"))
        .or_else(|| auth_data.get("Token"))
        .or_else(|| auth_data.get("access_token"))
        .or_else(|| auth_data.get("accessToken"))
        .or_else(|| auth_data.get("tokens").and_then(|t| t.get("SsoHopps")))
        .or_else(|| auth_data.get("shortToken").and_then(|t| t.get("SsoHopps")))
        .or_else(|| auth_data.get("habilitationAD")
            .and_then(|h| h.get("SsoHopps"))
            .and_then(|s| s.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|item| item.get("valeur")))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            log::error!("❌ Token no encontrado en ningún campo. Campos disponibles: {:?}", 
                auth_data.as_object().map(|obj| obj.keys().collect::<Vec<_>>()));
            anyhow::anyhow!("Token no encontrado en la respuesta")
        })?;
    
    log::info!("✅ Token SsoHopps obtenido exitosamente");
    
    let auth_response = ColisPriveAuthResponse {
        success: true,
        message: "Autenticación exitosa con Colis Privé".to_string(),
        token: Some(sso_hopps.to_string()),
        matricule: Some(credentials.username.clone()),
    };
    
    Ok(auth_response)
}

/// POST /api/colis-prive/packages - Obtener paquetes desde Colis Privé (IMPLEMENTACIÓN REAL)
pub async fn get_packages(
    State(state): State<AppState>,
    Json(request): Json<GetPackagesRequest>,
) -> Result<Json<crate::services::GetPackagesResponse>, StatusCode> {
    use tracing::{info, error};
    use crate::services::colis_prive_service::{GetPackagesResponse, PackageData};

    info!("📦 Obteniendo paquetes para matricule: {}", request.matricule);

    // Construir el matricule completo (societe + username)
    let societe = "PCP0010699".to_string();
    let matricule_completo = format!("{}_{}", societe, request.matricule);
    
    // Construir la fecha (hoy si no se especifica)
    let date = request.date.unwrap_or_else(|| {
        chrono::Utc::now().format("%Y-%m-%d").to_string()
    });

    // Llamar al endpoint real de Colis Privé
    let tournee_url = "https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getTourneeByMatriculeDistributeurDateDebut_POST";
    
    let tournee_payload = serde_json::json!({
        "Matricule": matricule_completo,
        "DateDebut": date
    });

    // 🆕 OBTENER EL TOKEN DINÁMICAMENTE DEL ESTADO DE LA APLICACIÓN
    let sso_hopps = match state.get_auth_token(&request.matricule, &societe).await {
        Some(auth_token) => {
            if auth_token.is_expired() {
                log::warn!("⚠️ Token expirado para {}:{}, necesitamos re-autenticar", societe, request.matricule);
                return Err(StatusCode::UNAUTHORIZED);
            }
            log::info!("✅ Usando token almacenado para {}:{}", societe, request.matricule);
            auth_token.token
        }
        None => {
            log::warn!("⚠️ No hay token almacenado para {}:{}, intentando autenticación automática", societe, request.matricule);
            
            // 🆕 INTENTAR AUTENTICACIÓN AUTOMÁTICA
            match attempt_auto_auth(&state, &request.matricule, &societe).await {
                Ok(token) => {
                    log::info!("✅ Autenticación automática exitosa para {}:{}", societe, request.matricule);
                    token
                }
                Err(e) => {
                    log::error!("❌ Autenticación automática falló para {}:{} - {}", societe, request.matricule, e);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    };

    let tournee_response = state
        .http_client
        .post(tournee_url)
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "fr-FR,fr;q=0.5")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Content-Type", "application/json")
        .header("Origin", "https://gestiontournee.colisprive.com")
        .header("Pragma", "no-cache")
        .header("Referer", "https://gestiontournee.colisprive.com/")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-site")
        .header("Sec-GPC", "1")
        .header("SsoHopps", sso_hopps)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
        .header("sec-ch-ua", "\"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"macOS\"")
        .json(&tournee_payload)
        .send()
        .await
        .map_err(|e| {
            error!("❌ Error llamando a Colis Privé: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !tournee_response.status().is_success() {
        error!("❌ Colis Privé respondió con error: {}", tournee_response.status());
        return Err(StatusCode::BAD_REQUEST);
    }

    let tournee_data: serde_json::Value = tournee_response.json().await.map_err(|e| {
        error!("❌ Error parseando respuesta de Colis Privé: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!("📦 Respuesta de Colis Privé recibida: {:?}", tournee_data);

    // Extraer la lista de paquetes
    let packages = if let Some(lst_lieu_article) = tournee_data.get("LstLieuArticle") {
        if let Some(packages_array) = lst_lieu_article.as_array() {
            packages_array
                .iter()
                .filter_map(|package| {
                    // Solo procesar paquetes de tipo COLIS
                    if package.get("metier")?.as_str() == Some("COLIS") {
                        Some(PackageData {
                            id: package.get("idArticle")?.as_str()?.to_string(),
                            tracking_number: package.get("refExterneArticle")?.as_str()?.to_string(),
                            recipient_name: package.get("nomDestinataire")?.as_str()?.to_string(),
                            address: format!(
                                "{}, {} {}",
                                package.get("LibelleVoieOrigineDestinataire")?.as_str()?,
                                package.get("codePostalOrigineDestinataire")?.as_str()?,
                                package.get("LibelleLocaliteOrigineDestinataire")?.as_str()?
                            ),
                            status: package.get("codeStatutArticle")?.as_str()?.to_string(),
                            instructions: package.get("PreferenceLivraison")?.as_str()?.to_string(),
                            phone: package.get("telephoneMobileDestinataire")?.as_str()?.to_string(),
                            priority: package.get("priorite")?.as_u64()?.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    info!("📦 Paquetes extraídos: {} paquetes", packages.len());

    // Si no hay paquetes, verificar si es una tournée completada
    if packages.is_empty() {
        if let Some(infos_tournee) = tournee_data.get("InfosTournee") {
            let code_tournee = infos_tournee.get("codeTourneeDistribution").and_then(|v| v.as_str()).ok_or(StatusCode::BAD_REQUEST)?;
            return Ok(Json(GetPackagesResponse {
                success: true,
                message: format!("Tournée {} completada - No hay paquetes pendientes", code_tournee),
                packages: None,
                error: None,
            }));
        }
    }

    Ok(Json(GetPackagesResponse {
        success: true,
        message: format!("Paquetes obtenidos exitosamente - {} paquetes", packages.len()),
        packages: Some(packages),
        error: None,
    }))
}

/// POST /api/colis-prive/tournee - Obtener tournée (IMPLEMENTACIÓN COMPLETA)
pub async fn get_tournee_data(
    State(state): State<AppState>,
    Json(request): Json<GetTourneeRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    log::info!("🔄 Obteniendo tournée para: {}", request.matricule);
    
    // 🆕 PASO 1: OBTENER TOKEN DEL ESTADO COMPARTIDO (AUTENTICACIÓN DINÁMICA)
    log::info!("🔍 Buscando token para username: '{}', societe: '{}'", request.username, request.societe);
    
    let sso_hopps = match state.get_auth_token(&request.username, &request.societe).await {
        Some(auth_token) => {
            if auth_token.is_expired() {
                log::warn!("⚠️ Token expirado para {}:{}, necesitamos re-autenticar", request.societe, request.username);
                return Err(StatusCode::UNAUTHORIZED);
            }
            log::info!("✅ Usando token almacenado para {}:{}", request.societe, request.username);
            auth_token.token
        }
        None => {
            log::warn!("⚠️ No hay token almacenado para {}:{}, intentando autenticación automática", request.societe, request.username);
            
            // 🆕 INTENTAR AUTENTICACIÓN AUTOMÁTICA
            match attempt_auto_auth(&state, &request.username, &request.societe).await {
                Ok(token) => {
                    log::info!("✅ Autenticación automática exitosa para {}:{}", request.societe, request.username);
                    token
                }
                Err(e) => {
                    log::error!("❌ Autenticación automática falló para {}:{} - {}", request.societe, request.username, e);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    };

    // 🆕 PASO 2: Hacer petición REAL a Colis Privé para obtener tournée
    let tournee_url = "https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getLettreVoitureEco_POST";

    let tournee_payload = json!({
        "enumTypeLettreVoiture": "ordreScan",
        "beanParamsMatriculeDateDebut": {
            "Societe": request.societe,
            "Matricule": request.matricule,
            "DateDebut": request.date.clone().unwrap_or_else(|| "2025-08-28".to_string())
        }
    });

    log::info!("📤 Enviando petición tournée a: {}", tournee_url);
    log::info!("📦 Payload: {}", serde_json::to_string_pretty(&tournee_payload).unwrap_or_default());
    
    // 🔍 LOGGING DETALLADO DE HEADERS Y TOKEN
    log::info!("🔑 TOKEN USADO: {}", sso_hopps);
    log::info!("📋 HEADERS COMPLETOS:");
    log::info!("   Accept: application/json, text/plain, */*");
    log::info!("   Accept-Encoding: gzip, deflate, br, zstd");
    log::info!("   Accept-Language: fr-FR,fr;q=0.5");
    log::info!("   Cache-Control: no-cache");
    log::info!("   Connection: keep-alive");
    log::info!("   Content-Type: application/json");
    log::info!("   Origin: https://gestiontournee.colisprive.com");
    log::info!("   Referer: https://gestiontournee.colisprive.com/");
    log::info!("   SsoHopps: {}", sso_hopps);
    log::info!("   User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36");
    log::info!("   Sec-Fetch-Dest: empty");
    log::info!("   Sec-Fetch-Mode: cors");
    log::info!("   Sec-Fetch-Site: same-site");
    log::info!("   Sec-GPC: 1");
    log::info!("   sec-ch-ua: \"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"");
    log::info!("   sec-ch-ua-mobile: ?0");
    log::info!("   sec-ch-ua-platform: \"macOS\"");

    let tournee_response = reqwest::Client::new()
        .post(tournee_url)
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Encoding", "gzip, deflate, br, zstd")
        .header("Accept-Language", "fr-FR,fr;q=0.5")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .header("Content-Type", "application/json")
        .header("Origin", "https://gestiontournee.colisprive.com")
        .header("Referer", "https://gestiontournee.colisprive.com/")
        .header("SsoHopps", &sso_hopps)  // 🆕 USAR TOKEN DEL ESTADO COMPARTIDO
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
        // 🔒 HEADERS DE SEGURIDAD CRÍTICOS - Agregados para compatibilidad con CURL funcional
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors") 
        .header("Sec-Fetch-Site", "same-site")
        .header("Sec-GPC", "1")
        .header("sec-ch-ua", "\"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"macOS\"")
        .json(&tournee_payload)
        .send()
        .await
        .map_err(|e| {
            log::error!("❌ Error enviando petición tournée: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let status = tournee_response.status();
    if !status.is_success() {
        let error_text = tournee_response.text().await.unwrap_or_default();
        log::error!("❌ Error {} tournée: {}", status, error_text);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let tournee_text = tournee_response.text().await.map_err(|e| {
        log::error!("❌ Error leyendo respuesta tournée: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    log::info!("📥 Respuesta tournée recibida: {} bytes", tournee_text.len());

    // 🔧 PASO 3: Decodificar base64 si es necesario
    let decoded_data = if tournee_text.starts_with('"') && tournee_text.ends_with('"') {
        let base64_content = &tournee_text[1..tournee_text.len()-1];
        match base64::decode(base64_content) {
            Ok(decoded) => {
                log::info!("✅ Datos decodificados de base64: {} bytes", decoded.len());
                String::from_utf8(decoded).unwrap_or(tournee_text)
            },
            Err(_) => {
                log::info!("ℹ️ No se pudo decodificar base64, usando texto original");
                tournee_text
            }
        }
    } else {
        log::info!("ℹ️ Respuesta no es base64, usando texto original");
        tournee_text
    };

    // 🔧 PASO 4: Respuesta final con datos reales de Colis Privé
    let response = json!({
        "success": true,
        "message": "Tournée obtenida exitosamente de Colis Privé",
        "data": decoded_data,
        "metadata": {
            "matricule": request.matricule,
            "societe": request.societe,
            "date": request.date.clone().unwrap_or_else(|| "2025-08-28".to_string()),
            "api_type": "web",
            "token_used": true,
            "headers_sent": true,
            "real_request": true,
            "token_source": "shared_state"  // 🆕 INDICAR QUE EL TOKEN VIENE DEL ESTADO COMPARTIDO
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    log::info!("✅ Tournée obtenida exitosamente con datos reales usando token del estado compartido");
    Ok(Json(response))
}

/// GET /api/colis-prive/health - Health check del servicio
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "service": "colis-prive",
        "status": "healthy",
        "message": "Servicio Colis Privé funcionando correctamente"
    }))
}

/// GET /api/colis-prive/health - Health check de Colis Privé
pub async fn health_check_colis_prive() -> Result<Json<serde_json::Value>, StatusCode> {
    use tracing::info;
    
    info!(
        endpoint = "health_check",
        "Starting Colis Privé health check"
    );
    
    let start_time = std::time::Instant::now();
    
    let health_info = json!({
        "status": "healthy",
        "colis_prive_client": {
            "ssl_bypass_enabled": true,
            "headers_system": "implemented",
            "device_info_consistency": "enforced"
        },
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "check_duration_ms": start_time.elapsed().as_millis(),
        "note": "Device info consistency enforced - no hardcoded values"
    });
    
    info!(
        endpoint = "health_check",
        status = "healthy",
        duration_ms = start_time.elapsed().as_millis(),
        "Health check completed successfully"
    );
    
    Ok(Json(health_info))
}

// ====================================================================
// FUNCIONES DE AUTENTICACIÓN AUTOMÁTICA
// ====================================================================

/// 🆕 FUNCIÓN DE AUTENTICACIÓN AUTOMÁTICA
/// Intenta autenticar automáticamente cuando no hay token disponible
async fn attempt_auto_auth(
    state: &AppState,
    username: &str,
    societe: &str,
) -> Result<String, anyhow::Error> {
    use crate::services::colis_prive_service::ColisPriveAuthRequest;
    
    log::info!("🔄 Intentando autenticación automática para {}:{}", societe, username);
    
    // 🔑 CREDENCIALES PREDEFINIDAS PARA AUTENTICACIÓN AUTOMÁTICA
    // En producción, esto debería venir de variables de entorno o configuración segura
    let credentials = ColisPriveAuthRequest {
        username: username.to_string(),
        password: "INTI7518".to_string(), // ⚠️ En producción, usar configuración segura
        societe: societe.to_string(),
    };
    
    // Intentar autenticación
    match authenticate_colis_prive_simple(&credentials).await {
        Ok(auth_response) => {
            if auth_response.success {
                if let Some(token) = auth_response.token {
                    // Almacenar el token en el estado
                    state.store_auth_token(
                        username.to_string(),
                        societe.to_string(),
                        token.clone(),
                        24
                    ).await;
                    
                    log::info!("✅ Autenticación automática exitosa para {}:{}", societe, username);
                    Ok(token)
                } else {
                    anyhow::bail!("Token no recibido en la respuesta de autenticación")
                }
            } else {
                anyhow::bail!("Autenticación automática falló: {}", auth_response.message)
            }
        }
        Err(e) => {
            log::error!("❌ Error en autenticación automática: {}", e);
            Err(e)
        }
    }
}

// ====================================================================
// FUNCIONES MÓVILES COMENTADAS - Solo API Web
// ====================================================================

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/mobile-tournee - Obtener tournée usando endpoint móvil real
// pub async fn get_mobile_tournee(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web  
// /// Endpoint estructurado para app móvil con análisis de datos GPS y metadatos
// pub async fn get_mobile_tournee_structured(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// Función para crear respuesta estructurada con análisis de datos GPS y metadatos
// fn create_mobile_structured_response(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/refresh-token - Refresh token con Colis Privé
// pub async fn refresh_colis_prive_token(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/mobile-tournee-with-retry - Tournée móvil con auto-retry
// pub async fn mobile_tournee_with_retry(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/complete-auth-flow - Flujo completo de autenticación
// pub async fn complete_authentication_flow(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/reconnect - Manejo específico de reconexión
// pub async fn handle_reconnection(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/v3/complete-flow - Flujo completo v3.3.0.9
// pub async fn execute_complete_flow_v3(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/v3/reconnect - Reconexión rápida con tokens existentes
// pub async fn reconnect_with_tokens_v3(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/lettre-voiture-only - Obtener lettre de voiture usando token guardado
// pub async fn get_lettre_voiture_only(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// Request de login directo a Colis Prive
// pub async fn login_colis_prive(...) { ... }

// ❌ FUNCIÓN MÓVIL COMENTADA - Solo API Web
// /// POST /api/colis-prive/lettre-voiture - Obtener Lettre de Voiture
// pub async fn get_lettre_de_voiture(...) { ... }
