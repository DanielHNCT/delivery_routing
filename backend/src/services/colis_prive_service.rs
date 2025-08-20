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
    /// Autenticar con Colis Privé usando credenciales dinámicas
    pub async fn authenticate_colis_prive(
        credentials: ColisPriveAuthRequest
    ) -> Result<ColisPriveAuthResponse> {
        // Validar credenciales
        credentials.validate()
            .map_err(|e| anyhow::anyhow!("Credenciales inválidas: {}", e))?;

        // ❌ PROBLEMA: No podemos autenticar sin device_info real de Android
        // El device_info debe venir SIEMPRE del request de Android
        anyhow::bail!("Este método requiere device_info real de Android. Use authenticate_colis_prive_with_device() en su lugar.");

        // ❌ MÉTODO DEPRECADO: Use authenticate_colis_prive_with_device() en su lugar
        anyhow::bail!("Este método requiere device_info real de Android. Use authenticate_colis_prive_with_device() en su lugar.");
    }

    /// Obtener datos de tournée usando credenciales dinámicas
    pub async fn get_tournee_data(
        credentials: &ColisPriveAuthRequest,
        date: &str,
        matricule: &str
    ) -> Result<String> {
        // ❌ PROBLEMA: No podemos obtener tournée sin device_info real de Android
        // El device_info debe venir SIEMPRE del request de Android
        anyhow::bail!("Este método requiere device_info real de Android. Use get_mobile_tournee() en su lugar.");

        // ❌ MÉTODO DEPRECADO: Use get_mobile_tournee() en su lugar
        anyhow::bail!("Método deprecado. Use get_mobile_tournee() que incluye device_info real de Android.");
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
        
        // ❌ PROBLEMA: No podemos usar device info hardcodeado
        // El device_info debe venir SIEMPRE del request de Android
        anyhow::bail!("Este método requiere device_info real de Android. Use get_mobile_tournee_structured() en su lugar.");
        
        // ❌ MÉTODO DEPRECADO: Use get_mobile_tournee_structured() en su lugar
        anyhow::bail!("Este método requiere device_info real de Android. Use get_mobile_tournee_structured() en su lugar.");
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
    
    // ❌ PROBLEMA: No podemos usar device info hardcodeado
    // El device_info debe venir SIEMPRE del request de Android
    return Err(StatusCode::BAD_REQUEST);
    
    // ❌ MÉTODO DEPRECADO: Use authenticate_colis_prive_with_device() en su lugar
    return Err(StatusCode::BAD_REQUEST);
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
    
    // ❌ PROBLEMA: No podemos usar device info hardcodeado
    // El device_info debe venir SIEMPRE del request de Android
    return Err(StatusCode::BAD_REQUEST);
    
    // ❌ MÉTODO DEPRECADO: Use get_mobile_tournee_structured() en su lugar
    return Err(StatusCode::BAD_REQUEST);
}
