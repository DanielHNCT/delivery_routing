use anyhow::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::client::ColisPriveClient;
use crate::external_models::{ColisPriveCredentials, MobileTourneeRequest};
use crate::state::AppState;

#[derive(Debug, Deserialize, Validate)]
pub struct ColisPriveAuthRequest {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 3))]
    pub password: String,
    #[validate(length(min = 3))]
    pub societe: String,
}

#[derive(Debug, Serialize)]
pub struct ColisPriveAuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub message: String,
    pub matricule: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetTourneeRequest {
    pub username: String,
    pub password: String,
    pub societe: String,
    pub date: String, // YYYY-MM-DD
    pub matricule: String,
}

pub struct ColisPriveService;

impl ColisPriveService {
    /// Crear DeviceInfo de prueba para el servicio
    fn create_test_device_info() -> crate::external_models::DeviceInfo {
        crate::external_models::DeviceInfo {
            model: "Service Test Device".to_string(),
            imei: "000000000000000".to_string(),
            serial_number: "service123".to_string(),
            android_version: "14".to_string(),
            install_id: "service-install-id".to_string(),
        }
    }
    
    /// Autenticar con Colis Privé usando credenciales dinámicas
    pub async fn authenticate_colis_prive(
        credentials: ColisPriveAuthRequest
    ) -> Result<ColisPriveAuthResponse> {
        // Validar credenciales
        credentials.validate()
            .map_err(|e| anyhow::anyhow!("Credenciales inválidas: {}", e))?;

        // Crear cliente temporal
        let mut client = ColisPriveClient::new(Self::create_test_device_info())?;

        // Intentar login
        match client.login(&credentials.username, &credentials.password, &credentials.societe).await {
            Ok(login_response) => {
                // Verificar si la autenticación fue exitosa
                if login_response.isAuthentif {
                    Ok(ColisPriveAuthResponse {
                        success: true,
                        token: Some(login_response.tokens.SsoHopps),
                        message: "Autenticación exitosa".to_string(),
                        matricule: Some(login_response.matricule),
                    })
                } else {
                    Ok(ColisPriveAuthResponse {
                        success: false,
                        token: None,
                        message: "Credenciales inválidas".to_string(),
                        matricule: None,
                    })
                }
            }
            Err(e) => {
                Ok(ColisPriveAuthResponse {
                    success: false,
                    token: None,
                    message: format!("Error de autenticación: {}", e),
                    matricule: None,
                })
            }
        }
    }

    /// Obtener datos de tournée usando credenciales dinámicas
    pub async fn get_tournee_data(
        credentials: &ColisPriveAuthRequest,
        date: &str,
        matricule: &str
    ) -> Result<String> {
        // Crear cliente temporal
        let mut client = ColisPriveClient::new(Self::create_test_device_info())?;

        // Autenticar primero
        let login_response = client.login(&credentials.username, &credentials.password, &credentials.societe).await?;
        
        if !login_response.isAuthentif {
            anyhow::bail!("Credenciales inválidas para obtener tournée");
        }

        // Obtener tournée usando el método curl que funciona
        let tournee_data = client.get_tournee_curl(
            &login_response.tokens.SsoHopps,
            &credentials.societe,
            matricule,
            date
        ).await?;

        Ok(tournee_data)
    }

    /// Obtener tournée usando el endpoint móvil real de Colis Privé
    pub async fn get_mobile_tournee(
        request: crate::external_models::MobileTourneeRequest,
    ) -> Result<crate::external_models::MobileTourneeResponse> {
        // Primero autenticar para obtener token
        let auth_request = ColisPriveAuthRequest {
            username: request.username.clone(),
            password: request.password.clone(),
            societe: request.societe.clone(),
        };
        
        let auth_result = Self::authenticate_colis_prive(auth_request).await?;
        
        if !auth_result.success {
            return Ok(crate::external_models::MobileTourneeResponse {
                success: false,
                data: None,
                message: "Autenticación falló".to_string(),
                endpoint_used: "mobile".to_string(),
                total_packages: 0,
            });
        }

        let token = auth_result.token.unwrap();
        
        // Crear cliente y llamar API móvil
        let client = ColisPriveClient::new(Self::create_test_device_info())?;
        
        // Crear credenciales para el cliente
        let credentials = ColisPriveCredentials {
            username: request.username.clone(),
            password: request.password.clone(),
            societe: request.societe.clone(),
        };

        // Llamar API móvil
        match client.get_mobile_tournee(&credentials, &request.date, &request.matricule, &token).await {
            Ok(mobile_data) => {
                Ok(crate::external_models::MobileTourneeResponse {
                    success: true,
                    data: Some(mobile_data.clone()),
                    message: "Datos de tournée móvil obtenidos exitosamente".to_string(),
                    endpoint_used: "mobile".to_string(),
                    total_packages: mobile_data.len(),
                })
            }
            Err(e) => {
                Ok(crate::external_models::MobileTourneeResponse {
                    success: false,
                    data: None,
                    message: format!("Error obteniendo tournée móvil: {}", e),
                    endpoint_used: "mobile".to_string(),
                    total_packages: 0,
                })
            }
        }
    }
}

/// Autenticación con Colis Privé usando cache
pub async fn authenticate_colis_prive_cached(
    State(state): State<AppState>,
    credentials: ColisPriveCredentials,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verificar cache primero
    if let Ok(Some(cached_auth)) = state.auth_cache.get_auth(&credentials.username, &credentials.societe).await {
        tracing::info!("✅ Autenticación obtenida del cache para usuario: {}", credentials.username);
        return Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Autenticación exitosa (cache)",
            "data": cached_auth,
            "source": "cache"
        })));
    }

    // Si no está en cache, hacer request real
    tracing::info!("🔄 Autenticación no encontrada en cache, haciendo request real...");
    
    // Crear DeviceInfo de prueba para el servicio
    let test_device_info = crate::external_models::DeviceInfo {
        model: "Service Test Device".to_string(),
        imei: "000000000000000".to_string(),
        serial_number: "service123".to_string(),
        android_version: "14".to_string(),
        install_id: "service-install-id".to_string(),
    };
    
    let mut client = ColisPriveClient::new(test_device_info).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match client.login(&credentials.username, &credentials.password, &credentials.societe).await {
        Ok(auth_data) => {
            // Guardar en cache
            if let Err(e) = state.auth_cache.set_auth(&credentials.username, &credentials.societe, &auth_data.tokens.SsoHopps, &auth_data.matricule, 1800).await {
                tracing::warn!("⚠️ Error guardando en cache: {}", e);
            }
            tracing::info!("💾 Autenticación guardada en cache para usuario: {}", credentials.username);
            
            Ok(Json(serde_json::json!({
                "status": "success",
                "message": "Autenticación exitosa (nueva)",
                "data": auth_data,
                "source": "api"
            })))
        }
        Err(e) => {
            tracing::error!("❌ Error en autenticación: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Obtener datos de tournée con cache
pub async fn get_tournee_data_cached(
    State(state): State<AppState>,
    request: MobileTourneeRequest,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verificar cache primero
    if let Ok(Some(cached_tournee)) = state.tournee_cache.get_tournee(&request.societe, &request.matricule, &request.date).await {
        tracing::info!("✅ Tournée obtenida del cache para: {}:{}:{}", request.societe, request.matricule, request.date);
        return Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Tournée obtenida exitosamente (cache)",
            "data": cached_tournee,
            "source": "cache"
        })));
    }

    // Si no está en cache, hacer request real
    tracing::info!("🔄 Tournée no encontrada en cache, haciendo request real...");
    
    // Crear DeviceInfo de prueba para el servicio
    let test_device_info = crate::external_models::DeviceInfo {
        model: "Service Test Device".to_string(),
        imei: "000000000000000".to_string(),
        serial_number: "service123".to_string(),
        android_version: "14".to_string(),
        install_id: "service-install-id".to_string(),
    };
    
    let mut client = ColisPriveClient::new(test_device_info).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Primero autenticar
    let credentials = ColisPriveCredentials {
        username: request.username.clone(),
        password: request.password.clone(),
        societe: request.societe.clone(),
    };
    
    let auth_result = client.login(&credentials.username, &credentials.password, &credentials.societe).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Obtener tournée usando el endpoint móvil
    match client.get_mobile_tournee(&credentials, &request.date, &request.matricule, &auth_result.tokens.SsoHopps).await {
        Ok(tournee_data) => {
            // Guardar en cache
            if let Err(e) = state.tournee_cache.set_tournee(&request.societe, &request.matricule, &request.date, &tournee_data, 900).await {
                tracing::warn!("⚠️ Error guardando en cache: {}", e);
            }
            tracing::info!("💾 Tournée guardada en cache para: {}:{}:{}", request.societe, request.matricule, request.date);
            
            Ok(Json(serde_json::json!({
                "status": "success",
                "message": "Tournée obtenida exitosamente (nueva)",
                "data": tournee_data,
                "source": "api"
            })))
        }
        Err(e) => {
            tracing::error!("❌ Error obteniendo tournée: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
