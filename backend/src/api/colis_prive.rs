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
