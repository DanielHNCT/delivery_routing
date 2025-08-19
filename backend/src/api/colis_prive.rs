use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::{
    state::AppState,
    services::colis_prive_service::{ColisPriveService, ColisPriveAuthRequest, GetTourneeRequest},
    utils::extract_structured_data_for_mobile,
};
use std::sync::Arc;
use crate::external_models::{MobileTourneeRequest, MobileTourneeResponse, MobilePackageAction};
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

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

/// Request para tournée actualizada con datos GPS en tiempo real
#[derive(Debug, Deserialize, Validate)]
pub struct TourneeUpdateRequest {
    #[serde(rename = "DateDebut")]
    #[validate(length(min = 1))]
    pub date_debut: String,
    
    #[serde(rename = "Matricule")]
    #[validate(length(min = 1))]
    pub matricule: String,
    
    #[serde(rename = "Password")]
    #[validate(length(min = 3))]
    pub password: String,
    
    #[serde(rename = "Societe")]
    #[validate(length(min = 1))]
    pub societe: String,
}

/// Response para tournée actualizada
#[derive(Debug, Serialize)]
pub struct TourneeResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub gps_coordinates: Option<Vec<GpsCoordinates>>,
    pub timestamp: String,
}

/// Estructura para coordenadas GPS
#[derive(Debug, Serialize, Deserialize)]
pub struct GpsCoordinates {
    #[serde(rename = "coordXGPSCptRendu")]
    pub longitude: Option<f64>,
    #[serde(rename = "coordYGPSCptRendu")]
    pub latitude: Option<f64>,
    #[serde(rename = "gpsQualite")]
    pub quality_meters: Option<String>,
    #[serde(rename = "horodatageCptRendu")]
    pub timestamp: Option<String>,
    pub package_id: Option<String>,
    pub tournee_code: Option<String>,
}

/// Validar coordenadas GPS para Francia
pub fn validate_gps_coordinates(coords: &GpsCoordinates) -> bool {
    if let (Some(lat), Some(lng)) = (coords.latitude, coords.longitude) {
        // Validar coordenadas para Francia
        lat >= 41.0 && lat <= 51.5 && lng >= -5.0 && lng <= 10.0
    } else {
        false
    }
}

/// Extraer calidad GPS en metros
pub fn extract_gps_quality(quality_str: &Option<String>) -> Option<f64> {
    quality_str.as_ref()
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|&q| q < 50.0) // Solo coordenadas con buena precisión
}

/// Transformar coordenadas de Colis Privé a formato estándar
pub fn transform_colis_prive_coordinates(
    coord_x: Option<f64>,
    coord_y: Option<f64>,
    quality: Option<String>,
) -> Option<GpsCoordinates> {
    if let (Some(lng), Some(lat)) = (coord_x, coord_y) {
        // Validar que las coordenadas estén en rango de Francia
        if lat >= 41.0 && lat <= 51.5 && lng >= -5.0 && lng <= 10.0 {
            Some(GpsCoordinates {
                longitude: Some(lng),
                latitude: Some(lat),
                quality_meters: quality,
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
                package_id: None,
                tournee_code: None,
            })
        } else {
            None // Coordenadas fuera de rango
        }
    } else {
        None // Coordenadas faltantes
    }
}

/// Extraer username completo del matricule (formato: PCP0010699_A187518 -> PCP0010699_A187518)
fn extract_username_from_matricule(matricule: &str) -> String {
    // Para Colis Privé, necesitamos el username COMPLETO, no solo la parte del tournée
    // El matricule ya es el username completo: PCP0010699_A187518
    matricule.to_string()
}

/// Endpoint para obtener tournée actualizada con datos GPS en tiempo real
pub async fn mobile_tournee_updated(
    State(_state): State<AppState>,
    Json(request): Json<TourneeUpdateRequest>,
) -> Result<Json<TourneeResponse>, StatusCode> {
    // Logging detallado para debug
    tracing::info!("=== REQUEST RECIBIDO ===");
    tracing::info!("Request completo: {:?}", request);
    tracing::info!("DateDebut: '{}'", request.date_debut);
    tracing::info!("Matricule: '{}'", request.matricule);
    tracing::info!("Password: '{}' (length: {})", request.password, request.password.len());
    tracing::info!("Societe: '{}'", request.societe);
    tracing::info!("=========================");
    
    // Validar el request
    if let Err(validation_errors) = request.validate() {
        tracing::error!("Error de validación: {:?}", validation_errors);
        let error_response = TourneeResponse {
            success: false,
            message: format!("Error de validación: {:?}", validation_errors),
            data: None,
            gps_coordinates: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        return Ok(Json(error_response));
    }
    
    // Crear credenciales usando los datos del request
    let username_completo = extract_username_from_matricule(&request.matricule);
    let credentials = crate::external_models::ColisPriveCredentials {
        username: username_completo.clone(),
        password: request.password.clone(),
        societe: request.societe.clone(),
    };
    
    tracing::info!("=== CREDENCIALES COLIS PRIVÉ ===");
    tracing::info!("Username completo: '{}'", username_completo);
    tracing::info!("Societe: '{}'", credentials.societe);
    tracing::info!("Password length: {}", credentials.password.len());
    tracing::info!("================================");

    // Crear request para el servicio móvil
    let mobile_request = crate::external_models::MobileTourneeRequest {
        username: credentials.username.clone(),
        password: credentials.password.clone(),
        societe: credentials.societe.clone(),
        date: request.date_debut.clone(),
        matricule: request.matricule.clone(),
    };
    
    tracing::info!("=== REQUEST MÓVIL COLIS PRIVÉ ===");
    tracing::info!("Username completo: '{}'", mobile_request.username);
    tracing::info!("Societe: '{}'", mobile_request.societe);
    tracing::info!("Date: '{}'", mobile_request.date);
    tracing::info!("Matricule: '{}'", mobile_request.matricule);
    tracing::info!("===================================");

    tracing::info!("Llamando al servicio Colis Privé...");
    match crate::services::ColisPriveService::get_mobile_tournee(mobile_request).await {
        Ok(response) => {
            tracing::info!("Respuesta del servicio: success={}, message='{}', total_packages={}", 
                           response.success, response.message, response.total_packages);
            
            if response.success {
                // Extraer coordenadas GPS de los paquetes con validación
                let gps_coordinates = response.data.as_ref()
                    .map(|packages| {
                        packages.iter()
                            .filter_map(|package| {
                                // Usar función de transformación con validación
                                transform_colis_prive_coordinates(
                                    package.coord_x_gps_cpt_rendu,
                                    package.coord_y_gps_cpt_rendu,
                                    package.gps_qualite.clone(),
                                ).map(|mut coords| {
                                    // Agregar información adicional del paquete
                                    coords.package_id = Some(package.id_article.clone());
                                    coords.tournee_code = Some(package.code_tournee_mcp.clone());
                                    coords.timestamp = package.horodatage_cpt_rendu.clone();
                                    coords
                                })
                            })
                            .filter(|coords| validate_gps_coordinates(coords)) // Solo coordenadas válidas
                            .collect()
                    });

                let tournee_response = TourneeResponse {
                    success: true,
                    message: "Tournée actualizada obtenida exitosamente".to_string(),
                    data: Some(serde_json::to_value(&response).unwrap_or_default()),
                    gps_coordinates,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                Ok(Json(tournee_response))
            } else {
                let error_response = TourneeResponse {
                    success: false,
                    message: response.message,
                    data: None,
                    gps_coordinates: None,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                Ok(Json(error_response))
            }
        }
        Err(e) => {
            tracing::error!("Error obteniendo tournée actualizada: {}", e);
            let error_response = TourneeResponse {
                success: false,
                message: format!("Error interno del servidor: {}", e),
                data: None,
                gps_coordinates: None,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            Ok(Json(error_response))
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
