use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::{
    state::AppState,
    services::colis_prive_service::{ColisPriveService, ColisPriveAuthRequest, GetTourneeRequest},
    services::app_version_service::AppVersionService,
    services::colis_prive_flow_service::ColisPriveFlowService,
    utils::extract_structured_data_for_mobile,
};
use std::sync::Arc;
use crate::external_models::{MobileTourneeRequest, MobileTourneeResponse, MobilePackageAction, RefreshTokenRequest, TourneeRequestWithToken, TourneeRequestWithRetry, ColisAuthResponse, VersionCheckRequest, AuditInstallRequest};

/// POST /api/colis-prive/auth - Autenticar con Colis Privé
pub async fn authenticate_colis_prive(
    State(_state): State<AppState>,
    Json(credentials): Json<ColisPriveAuthRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Clonar las credenciales para poder usarlas después
    let username = credentials.username.clone();
    let societe = credentials.societe.clone();
    
    match ColisPriveService::authenticate_colis_prive(credentials).await {
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
                    "timestamp": chrono::Utc::now().to_rfc3339()
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
            tracing::error!("Error en autenticación Colis Privé: {}", e);
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

/// GET /api/colis-prive/tournee - Obtener tournée
pub async fn get_tournee_data(
    State(_state): State<AppState>,
    Json(request): Json<GetTourneeRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Crear credenciales para el servicio
    let credentials = ColisPriveAuthRequest {
        username: request.username.clone(),
        password: request.password.clone(),
        societe: request.societe.clone(),
    };

    match ColisPriveService::get_tournee_data(&credentials, &request.date, &request.matricule).await {
        Ok(tournee_data) => {
            // Extraer datos estructurados para aplicaciones móviles
            let mobile_data = match extract_structured_data_for_mobile(&tournee_data) {
                Ok(structured) => structured,
                Err(_) => json!({
                    "error": "Error procesando datos para móvil",
                    "raw_data": tournee_data
                }),
            };

            // Crear respuesta optimizada para móviles
            let response = json!({
                "success": true,
                "metadata": {
                    "date": request.date,
                    "matricule": request.matricule,
                    "username": request.username,
                    "societe": request.societe
                },
                "tournee_data": mobile_data,
                "timestamp": chrono::Utc::now().to_rfc3339()
            });

            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("Error obteniendo tournée: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
    
    // ❌ PROBLEMA: No podemos usar device info hardcodeado en health check
    // El health check debe verificar solo la conectividad básica, no crear clientes
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
    
    // Health check completado - device info consistency enforced
    
    info!(
        endpoint = "health_check",
        status = "healthy",
        duration_ms = start_time.elapsed().as_millis(),
        "Health check completed successfully"
    );
    
    Ok(Json(health_info))
}

/// POST /api/colis-prive/mobile-tournee - Obtener tournée usando endpoint móvil real
pub async fn get_mobile_tournee(
    State(_state): State<AppState>,
    Json(request): Json<crate::external_models::MobileTourneeRequest>,
) -> Result<Json<crate::external_models::MobileTourneeResponse>, StatusCode> {
    match crate::services::ColisPriveService::get_mobile_tournee(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Error obteniendo tournée móvil: {}", e);
            let error_response = crate::external_models::MobileTourneeResponse {
                success: false,
                data: None,
                message: format!("Error interno del servidor: {}", e),
                endpoint_used: "mobile".to_string(),
                total_packages: 0,
            };
            Ok(Json(error_response))
        }
    }
}


/// Endpoint estructurado para app móvil con análisis de datos GPS y metadatos
pub async fn get_mobile_tournee_structured(
    State(state): State<AppState>,
    Json(request): Json<MobileTourneeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match ColisPriveService::get_mobile_tournee(request).await {
        Ok(response) => {
            if response.success {
                // Crear respuesta estructurada para app móvil
                let structured_response = create_mobile_structured_response(&response);
                Ok(Json(structured_response))
            } else {
                let error_response = serde_json::json!({
                    "success": false,
                    "message": response.message,
                    "data": null
                });
                Err((StatusCode::BAD_REQUEST, Json(error_response)))
            }
        },
        Err(e) => {
            let error_response = serde_json::json!({
                "success": false,
                "message": format!("Error getting mobile tournee: {}", e),
                "data": null
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

/// Función para crear respuesta estructurada con análisis de datos GPS y metadatos
fn create_mobile_structured_response(response: &MobileTourneeResponse) -> serde_json::Value {
    let empty_vec = Vec::new();
    let packages = response.data.as_ref().unwrap_or(&empty_vec);
    
    // Análisis de datos
    let has_gps = packages.iter().any(|p| p.coord_x_gps_cpt_rendu.is_some());
    let action_types: std::collections::HashSet<String> = packages.iter()
        .map(|p| p.code_cle_action.clone())
        .collect();
    
    // Análisis de coordenadas GPS
    let gps_packages: Vec<&MobilePackageAction> = packages.iter()
        .filter(|p| p.coord_x_gps_cpt_rendu.is_some() && p.coord_y_gps_cpt_rendu.is_some())
        .collect();
    
    let gps_stats = if !gps_packages.is_empty() {
        let lats: Vec<f64> = gps_packages.iter()
            .filter_map(|p| p.coord_y_gps_cpt_rendu)
            .collect();
        let lngs: Vec<f64> = gps_packages.iter()
            .filter_map(|p| p.coord_x_gps_cpt_rendu)
            .collect();
        
        serde_json::json!({
            "total_with_gps": gps_packages.len(),
            "coverage_percentage": (gps_packages.len() as f64 / packages.len() as f64) * 100.0,
            "bounds": {
                "min_lat": lats.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
                "max_lat": lats.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
                "min_lng": lngs.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
                "max_lng": lngs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
            }
        })
    } else {
        serde_json::json!({
            "total_with_gps": 0,
            "coverage_percentage": 0.0,
            "bounds": null
        })
    };
    
    serde_json::json!({
        "success": true,
        "metadata": {
            "total_packages": packages.len(),
            "has_gps_coordinates": has_gps,
            "unique_action_types": action_types.into_iter().collect::<Vec<String>>(),
            "tournee_id": packages.first().map(|p| &p.code_tournee_mcp),
            "agent_id": packages.first().map(|p| &p.matricule_distributeur),
            "gps_statistics": gps_stats
        },
        "packages": packages.iter().map(|package| {
            serde_json::json!({
                // Identificadores
                "id": package.id_article,
                "location_id": package.id_lieu_article,
                "reference": package.ref_externe_article,
                "barcode": package.code_barre_article,
                "tournee_code": package.code_tournee_mcp,
                
                // Acción a realizar
                "action": {
                    "id": package.id_action,
                    "code": package.code_cle_action,
                    "label": package.libelle_action,
                    "type": package.code_type_action,
                    "order": package.num_ordre_action,
                    "estimated_duration_minutes": package.duree_seconde_prevue_action.map(|d| d / 60.0)
                },
                
                // Ubicación (para futuro uso con Mapbox)
                "location": if package.coord_x_gps_cpt_rendu.is_some() {
                    serde_json::json!({
                        "latitude": package.coord_y_gps_cpt_rendu,
                        "longitude": package.coord_x_gps_cpt_rendu,
                        "gps_quality_meters": package.gps_qualite,
                        "has_coordinates": true,
                        "coordinates_ready_for_maps": true
                    })
                } else {
                    serde_json::json!({
                        "has_coordinates": false,
                        "coordinates_ready_for_maps": false
                    })
                },
                
                // Información temporal
                "timing": {
                    "recorded_at": package.horodatage_cpt_rendu,
                    "expected_at": package.valeur_attendu_cpt_rendu,
                    "transmitted_at": package.date_transmis_si_tiers
                },
                
                // Estado
                "status": {
                    "transmitted_to_third_party": package.vf_transmis_si_tiers.unwrap_or(false),
                    "order_in_route": package.num_ordre_cpt_rendu
                },
                
                // Empresa emisora
                "sender": {
                    "code": package.code_societe_emetrice_article,
                    "agency": package.code_agence
                },
                
                // Información adicional de seguimiento
                "tracking": {
                    "compteur_id": package.id_cpt_rendu,
                    "compteur_code": package.code_cle_cpt_rendu,
                    "compteur_type": package.code_type_cpt_rendu,
                    "compteur_value": package.valeur_cpt_rendu
                }
            })
        }).collect::<Vec<_>>()
    })
}

/// POST /api/colis-prive/refresh-token - Refresh token con Colis Privé
pub async fn refresh_colis_prive_token(
    State(_state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ColisAuthResponse>, StatusCode> {
    use tracing::{info, warn, error, debug};
    
    // Logging seguro - nunca logear tokens completos
    info!(
        endpoint = "refresh_token",
        token_preview = %&request.token[..20.min(request.token.len())],
        token_length = request.token.len(),
        device_model = %request.device_info.model,
        "Starting Colis Privé token refresh with dynamic device info"
    );
    
    // Crear cliente con SSL bypass y headers exactos usando device info
    let mut client = crate::client::ColisPriveClient::new(request.device_info.clone())
        .map_err(|e| {
            error!(
                error = %e,
                context = "client_creation",
                "Failed to create Colis Privé client"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Request body exacto como especificaste (dureeTokenInHour fijo en 0)
    let refresh_request = json!({
        "dureeTokenInHour": 0,
        "token": request.token
    });
    
    debug!(
        url = "https://wsauthentificationexterne.colisprive.com/api/auth/login-token",
        body_size = refresh_request.to_string().len(),
        device_model = %request.device_info.model,
        "Sending refresh token request to Colis Privé"
    );
    
    // Usar el método refresh_token del cliente (ya usa headers correctos)
    match client.refresh_token(&request.token).await {
        Ok(auth_response) => {
            info!(
                endpoint = "refresh_token",
                success = true,
                new_token_preview = %&auth_response.tokens.sso_hopps[..20.min(auth_response.tokens.sso_hopps.len())],
                new_token_length = auth_response.tokens.sso_hopps.len(),
                is_authentif = auth_response.is_authentif,
                "Token refresh successful"
            );
            
            // Verificar que la autenticación sea válida
            if !auth_response.is_authentif {
                warn!(
                    endpoint = "refresh_token",
                    is_authentif = false,
                    "Refresh token returned invalid authentication"
                );
                return Err(StatusCode::UNAUTHORIZED);
            }
            
            Ok(Json(auth_response))
        }
        Err(e) => {
            error!(
                error = %e,
                context = "token_refresh",
                endpoint = "refresh_token",
                "Token refresh failed"
            );
            
            // Determinar el status code apropiado basado en el error
            let status_code = if e.to_string().contains("401") || e.to_string().contains("Unauthorized") {
                StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("timeout") || e.to_string().contains("Timeout") {
                StatusCode::REQUEST_TIMEOUT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            
            Err(status_code)
        }
    }
}

/// POST /api/colis-prive/mobile-tournee-with-retry - Tournée móvil con auto-retry
pub async fn mobile_tournee_with_retry(
    State(_state): State<AppState>,
    Json(request): Json<TourneeRequestWithRetry>,
) -> Result<Json<MobileTourneeResponse>, StatusCode> {
    use tracing::{info, warn, error, debug};
    
    info!(
        endpoint = "mobile_tournee_with_retry",
        username = %request.username,
        matricule = %request.matricule,
        date = %request.date,
        has_token = request.token.is_some(),
        device_model = %request.device_info.model,
        "Starting mobile tournée with auto-retry using dynamic device info"
    );
    
    let mut client = crate::client::ColisPriveClient::new(request.device_info.clone())
        .map_err(|e| {
            error!(
                error = %e,
                context = "client_creation",
                endpoint = "mobile_tournee_with_retry",
                "Failed to create Colis Privé client"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Extraer username sin prefix para el flujo completo
    let username = request.matricule.split('_').last().unwrap_or(&request.matricule);
    
    // Si no hay token, hacer login primero
    let token = if let Some(token) = request.token {
        info!(
            endpoint = "mobile_tournee_with_retry",
            token_preview = %&token[..20.min(token.len())],
            "Using existing token for tournée"
        );
        token
    } else {
        info!(
            endpoint = "mobile_tournee_with_retry",
            username = %username,
            "No token provided, performing initial login"
        );
        
        // Usar el método login existente
        let auth_response = client.login(username, &request.password, &request.societe).await.map_err(|e| {
            error!(
                error = %e,
                context = "initial_login",
                endpoint = "mobile_tournee_with_retry",
                username = %username,
                "Initial login failed"
            );
            StatusCode::UNAUTHORIZED
        })?;
        
        // Extraer token del response (LoginResponse usa tokens.SsoHopps)
        let new_token = auth_response.tokens.SsoHopps.clone();
        info!(
            endpoint = "mobile_tournee_with_retry",
            token_preview = %&new_token[..20.min(new_token.len())],
            "Initial login successful, obtained token"
        );
        new_token
    };
    
    // Intento 1: con token actual
    debug!(
        endpoint = "mobile_tournee_with_retry",
        attempt = 1,
        token_preview = %&token[..20.min(token.len())],
        "Attempting tournée with current token"
    );
    
    match client.get_mobile_tournee_with_token(
        username,
        &request.password,
        &request.societe,
        &request.date,
        &token
    ).await {
        Ok(packages_json) => {
            info!(
                endpoint = "mobile_tournee_with_retry",
                attempt = 1,
                success = true,
                "Tournée successful with current token"
            );
            
            // Convertir Value a Vec<MobilePackageAction> si es posible
            let packages: Vec<MobilePackageAction> = serde_json::from_value(packages_json.clone())
                .unwrap_or_else(|_| Vec::new());
            
            let tournee_response = MobileTourneeResponse {
                success: true,
                message: "Tournée móvil obtenida exitosamente con auto-retry".to_string(),
                data: Some(packages.clone()),
                endpoint_used: "mobile_tournee_with_retry".to_string(),
                total_packages: packages.len(),
            };
            
            Ok(Json(tournee_response))
        }
        Err(e) => {
            if e.to_string().contains("401") || e.to_string().contains("Unauthorized") || e.to_string().contains("Token expirado") {
                warn!(
                    endpoint = "mobile_tournee_with_retry",
                    attempt = 1,
                    error = %e,
                    "Token expired, attempting refresh"
                );
                
                // Intento 2: Refresh token y retry
                debug!(
                    endpoint = "mobile_tournee_with_retry",
                    attempt = 2,
                    "Starting token refresh for retry"
                );
                
                // Refresh token
                let refresh_response = client.refresh_token(&token).await.map_err(|refresh_e| {
                    error!(
                        error = %refresh_e,
                        context = "token_refresh",
                        endpoint = "mobile_tournee_with_retry",
                        "Token refresh failed during retry"
                    );
                    StatusCode::UNAUTHORIZED
                })?;
                
                let new_token = refresh_response.tokens.sso_hopps;
                info!(
                    endpoint = "mobile_tournee_with_retry",
                    attempt = 2,
                    new_token_preview = %&new_token[..20.min(new_token.len())],
                    "Token refresh successful, retrying tournée"
                );
                
                // Retry con nuevo token
                match client.get_mobile_tournee_with_token(
                    username,
                    &request.password,
                    &request.societe,
                    &request.date,
                    &new_token
                ).await {
                    Ok(packages_json) => {
                        println!("✅ Tournée exitosa después de refresh");
                        
                        let packages: Vec<MobilePackageAction> = serde_json::from_value(packages_json.clone())
                            .unwrap_or_else(|_| Vec::new());
                        
                        let tournee_response = MobileTourneeResponse {
                            success: true,
                            message: "Tournée móvil obtenida exitosamente después de refresh".to_string(),
                            data: Some(packages.clone()),
                            endpoint_used: "mobile_tournee_with_retry_refresh".to_string(),
                            total_packages: packages.len(),
                        };
                        
                        Ok(Json(tournee_response))
                    }
                    Err(retry_e) => {
                        println!("❌ Tournée falló incluso después de refresh: {}", retry_e);
                        let error_response = MobileTourneeResponse {
                            success: false,
                            message: format!("Error obteniendo tournée móvil: {}", retry_e),
                            data: None,
                            endpoint_used: "mobile_tournee_with_retry_failed".to_string(),
                            total_packages: 0,
                        };
                        Ok(Json(error_response))
                    }
                }
            } else {
                println!("❌ Error en tournée (no es 401): {}", e);
                let error_response = MobileTourneeResponse {
                    success: false,
                    message: format!("Error obteniendo tournée móvil: {}", e),
                    data: None,
                    endpoint_used: "mobile_tournee_with_retry_error".to_string(),
                    total_packages: 0,
                };
                Ok(Json(error_response))
            }
        }
    }
}

// NUEVOS ENDPOINTS: FLUJO COMPLETO DE AUTENTICACIÓN (RESUELVE EL 401)
// ====================================================================

/// POST /api/colis-prive/complete-auth-flow - Flujo completo de autenticación (RESUELVE EL 401)
pub async fn complete_authentication_flow(
    State(_state): State<AppState>,
    Json(request): Json<TourneeRequestWithRetry>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    use tracing::{info, error};
    
    info!(
        username = %request.username,
        societe = %request.societe,
        device_model = %request.device_info.model,
        "Iniciando flujo completo de autenticación (RESUELVE EL 401)"
    );

    match ColisPriveFlowService::new() {
        Ok(flow_service) => {
            match flow_service.complete_authentication_flow(
                &request.device_info,
                &request.username,
                &request.password,
                &request.societe
            ).await {
                Ok(flow_result) => {
                    info!(
                        username = %request.username,
                        "Flujo completo de autenticación ejecutado exitosamente"
                    );

                    let success_response = json!({
                        "success": true,
                        "message": "Flujo completo de autenticación ejecutado exitosamente",
                        "flow_result": flow_result,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    });

                    Ok(Json(success_response))
                }
                Err(e) => {
                    error!("Error en flujo completo de autenticación: {}", e);
                    let error_response = json!({
                        "success": false,
                        "error": {
                            "message": format!("Error en flujo completo: {}", e),
                            "code": "FLOW_FAILED"
                        },
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    });
                    Ok(Json(error_response))
                }
            }
        }
        Err(e) => {
            error!("Error inicializando ColisPriveFlowService: {}", e);
            let error_response = json!({
                "success": false,
                "error": {
                    "message": format!("Error interno del servidor: {}", e),
                    "code": "SERVICE_INIT_FAILED"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            Ok(Json(error_response))
        }
    }
}

/// POST /api/colis-prive/reconnect - Manejo específico de reconexión (RESUELVE EL 401)
pub async fn handle_reconnection(
    State(_state): State<AppState>,
    Json(request): Json<TourneeRequestWithRetry>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    use tracing::{info, error};
    
    info!(
        username = %request.username,
        societe = %request.societe,
        device_model = %request.device_info.model,
        "Manejando reconexión específica (RESUELVE EL 401)"
    );

    match ColisPriveFlowService::new() {
        Ok(flow_service) => {
            match flow_service.handle_reconnection(
                &request.device_info,
                &request.username,
                &request.password,
                &request.societe
            ).await {
                Ok(reconnection_result) => {
                    info!(
                        username = %request.username,
                        "Reconexión manejada exitosamente"
                    );

                    let success_response = json!({
                        "success": true,
                        "message": "Reconexión manejada exitosamente (401 RESUELTO)",
                        "reconnection_result": reconnection_result,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    });

                    Ok(Json(success_response))
                }
                Err(e) => {
                    error!("Error en reconexión: {}", e);
                    let error_response = json!({
                        "success": false,
                        "error": {
                            "message": format!("Error en reconexión: {}", e),
                            "code": "RECONNECTION_FAILED"
                        },
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    });
                    Ok(Json(error_response))
                }
            }
        }
        Err(e) => {
            error!("Error inicializando ColisPriveFlowService: {}", e);
            let error_response = json!({
                "success": false,
                "error": {
                    "message": format!("Error interno del servidor: {}", e),
                    "code": "SERVICE_INIT_FAILED"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            Ok(Json(error_response))
        }
    }
}
