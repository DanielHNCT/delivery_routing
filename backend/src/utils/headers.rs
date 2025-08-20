use reqwest::header::HeaderMap;
use uuid::Uuid;
use crate::external_models::DeviceInfo;
use tracing::{debug, info};

/// Generar headers exactos de la app oficial de Colis Privé usando device info dinámico
pub fn get_colis_headers(
    endpoint: &str,
    device_info: &DeviceInfo,
    username: Option<&str>,
    token: Option<&str>,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    
    // Generar ActivityId único por request
    let activity_id = Uuid::new_v4().to_string();
    
    // CORE HEADERS (todos los endpoints)
    headers.insert("Accept-Charset", "UTF-8".parse().unwrap());
    headers.insert("Content-Type", "application/json; charset=UTF-8".parse().unwrap());
    headers.insert("Connection", "Keep-Alive".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip".parse().unwrap());
    headers.insert("User-Agent", "okhttp/3.4.1".parse().unwrap());
    
    // APP IDENTIFICATION (usando device info dinámico)
    headers.insert("ActivityId", activity_id.parse().unwrap());
    headers.insert("AppName", "CP DISTRI V2".parse().unwrap());
    headers.insert("AppIdentifier", "com.danem.cpdistriv2".parse().unwrap());
    headers.insert("Device", device_info.model.parse().unwrap());
    headers.insert("VersionOS", device_info.android_version.parse().unwrap());
    headers.insert("VersionApplication", "3.3.0.9".parse().unwrap()); // CRÍTICO - fijo
    headers.insert("VersionCode", "1".parse().unwrap());
    headers.insert("Domaine", "Membership".parse().unwrap());
    
    // USER CONTEXT (solo en endpoints autenticados)
    if let Some(username) = username {
        // Extraer username sin prefijo societe (ej: "A187518" no "PCP0010699_A187518")
        let clean_username = username.split('_').last().unwrap_or(username);
        headers.insert("UserName", clean_username.parse().unwrap());
        headers.insert("Societe", "PCP0010699".parse().unwrap());
    }
    
    // TOKEN (solo en requests autenticados)
    if let Some(token) = token {
        headers.insert("SsoHopps", token.parse().unwrap());
    }
    
    // HEADERS ESPECÍFICOS POR ENDPOINT
    match endpoint {
        "auth" | "login" => {
            headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
            headers.insert("Accept-Language", "fr-FR,fr;q=0.5".parse().unwrap());
            headers.insert("Cache-Control", "no-cache".parse().unwrap());
            headers.insert("Pragma", "no-cache".parse().unwrap());
            headers.insert("Origin", "https://gestiontournee.colisprive.com".parse().unwrap());
            headers.insert("Referer", "https://gestiontournee.colisprive.com/".parse().unwrap());
        }
        "refresh" => {
            // Para refresh token, no agregar headers adicionales
            // Solo los core headers son necesarios
        }
        "tournee" => {
            // Para tournée, agregar headers específicos
            headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
            headers.insert("X-Device-Info", "Android".parse().unwrap());
        }
        _ => {
            // Headers por defecto para otros endpoints
        }
    }
    
    // Logging seguro de headers generados
    debug!(
        endpoint = %endpoint,
        activity_id = %activity_id,
        device_model = %device_info.model,
        android_version = %device_info.android_version,
        has_username = username.is_some(),
        has_token = token.is_some(),
        total_headers = headers.len(),
        "Headers generados para endpoint"
    );
    
    headers
}

/// Crear audit data usando device info real
pub fn create_audit_data(device_info: &DeviceInfo) -> serde_json::Value {
    let audit_data = serde_json::json!({
        "appName": "CP DISTRI V2",
        "deviceModelName": device_info.model,
        "imei": device_info.imei,
        "noSerie": device_info.serial_number,
        "iccid": "indisponible",
        "msisdn": "indisponible",
        "cle1": "",
        "cle2": "",
        "cle3": ""
    });
    
    info!(
        device_model = %device_info.model,
        imei_preview = %&device_info.imei[..8.min(device_info.imei.len())],
        serial_preview = %&device_info.serial_number[..8.min(device_info.serial_number.len())],
        "Audit data creado con device info real"
    );
    
    audit_data
}

/// Crear cliente HTTP con SSL bypass para Colis Privé
pub fn create_colis_client() -> Result<reqwest::Client, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)          // CRÍTICO para Colis Privé
        .danger_accept_invalid_hostnames(true)      // CRÍTICO para Colis Privé
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .http1_only() // Forzar HTTP/1.1
        .http1_title_case_headers() // Headers en formato correcto
        .cookie_store(true) // Mantener cookies de sesión
        .build()?;
    
    info!("Cliente HTTP con SSL bypass creado para Colis Privé");
    Ok(client)
}
