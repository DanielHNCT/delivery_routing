use reqwest::header::HeaderMap;
use uuid::Uuid;
use crate::models::colis_prive_v3_models::{DeviceInfo as DeviceInfoV3, AppInfo};
use crate::external_models::DeviceInfo;
use tracing::{debug, info, warn, error};
use anyhow::Result;

/// Generar headers exactos de la app oficial de Colis Privé usando device info dinámico
pub fn get_colis_headers(
    endpoint: &str,
    device_info: &DeviceInfo,
    username: Option<&str>,
    token: Option<&str>,
) -> HeaderMap {
    // Verificar consistencia de device info ANTES de generar headers
    verify_device_info_consistency(endpoint, device_info, username, token);
    
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
    // CORREGIDO: Usar solo la versión sin "Android" y "(API XX)"
    let clean_version = device_info.android_version
        .replace("Android ", "")
        .replace(" (API ", "")
        .replace(")", "");
    headers.insert("VersionOS", clean_version.parse().unwrap());
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
            // HEADERS CRÍTICOS PARA TOURNÉE - Basados en tráfico real capturado
            headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
            headers.insert("Accept-Language", "fr-FR,fr;q=0.5".parse().unwrap());
            headers.insert("Cache-Control", "no-cache".parse().unwrap());
            headers.insert("Pragma", "no-cache".parse().unwrap());
            headers.insert("Origin", "https://gestiontournee.colisprive.com".parse().unwrap());
            headers.insert("Referer", "https://gestiontournee.colisprive.com/".parse().unwrap());
            headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
            headers.insert("X-Device-Info", "Android".parse().unwrap());
            
            // HEADERS ESPECÍFICOS DE TOURNÉE - CRÍTICOS para evitar 401
            headers.insert("Host", "wstournee-v2.colisprive.com".parse().unwrap());
            headers.insert("Connection", "Keep-Alive".parse().unwrap());
            headers.insert("Accept-Encoding", "gzip, deflate".parse().unwrap());
            
            // VERIFICAR QUE USERNAME Y SOCIETE ESTÉN PRESENTES
            if username.is_none() {
                warn!("⚠️ TOURNÉE: Username faltante - puede causar 401");
            }
            if token.is_none() {
                warn!("⚠️ TOURNÉE: Token faltante - puede causar 401");
            }
        }
        _ => {
            // Headers por defecto para otros endpoints
        }
    }
    
    // Logging detallado de headers generados
    info!(
        endpoint = %endpoint,
        activity_id = %activity_id,
        device_model = %device_info.model,
        android_version = %device_info.android_version,
        has_username = username.is_some(),
        has_token = token.is_some(),
        total_headers = headers.len(),
        "Headers generados para endpoint"
    );
    
    // Logging detallado de headers críticos para tournée
    if endpoint == "tournee" {
        let username_header = headers.get("UserName").map(|h| h.to_str().unwrap_or("ERROR"));
        let societe_header = headers.get("Societe").map(|h| h.to_str().unwrap_or("ERROR"));
        let token_header = headers.get("SsoHopps").map(|h| {
            let token = h.to_str().unwrap_or("ERROR");
            if token.len() > 20 {
                format!("{}...", &token[..20])
            } else {
                token.to_string()
            }
        });
        
        info!(
            endpoint = "tournee",
            username_header = ?username_header,
            societe_header = ?societe_header,
            token_preview = ?token_header,
            activity_id = %activity_id,
            "Headers críticos para tournée verificados"
        );
        
        // Verificar headers obligatorios para tournée
        let required_headers = ["UserName", "Societe", "SsoHopps", "ActivityId", "Device"];
        for header_name in &required_headers {
            if !headers.contains_key(*header_name) {
                warn!("⚠️ TOURNÉE: Header faltante: {} - puede causar 401", header_name);
            }
        }
    }
    
    headers
}

/// Verificar consistencia de device info entre endpoints
pub fn verify_device_info_consistency(
    endpoint: &str,
    device_info: &DeviceInfo,
    username: Option<&str>,
    token: Option<&str>,
) {
    info!(
        endpoint = %endpoint,
        device_model = %device_info.model,
        android_version = %device_info.android_version,
        has_username = username.is_some(),
        has_token = token.is_some(),
        "Verificando consistencia de device info"
    );
    
    // Verificar que device info sea consistente
    if device_info.model.is_empty() || device_info.android_version.is_empty() {
        warn!("⚠️ {}: Device info incompleto - puede causar problemas", endpoint);
    }
    
    // Verificar que username esté presente en endpoints autenticados
    if endpoint == "tournee" && username.is_none() {
        error!("❌ TOURNÉE: Username faltante - causará 401");
    }
    
    // Verificar que token esté presente en endpoints autenticados
    if endpoint == "tournee" && token.is_none() {
        error!("❌ TOURNÉE: Token faltante - causará 401");
    }
}

/// Convertir DeviceInfo v1 a v3
pub fn convert_device_info_to_v3(old_device: &DeviceInfo) -> DeviceInfoV3 {
    DeviceInfoV3 {
        imei: old_device.imei.clone(),
        android_id: old_device.install_id.clone(),
        android_version: old_device.android_version.clone(),
        brand: "Sony".to_string(), // Default desde APK
        device: old_device.model.clone(),
        hardware: "qcom".to_string(), // Default desde APK
        install_id: old_device.install_id.clone(),
        manufacturer: "Sony".to_string(), // Default desde APK
        model: old_device.model.clone(),
        product: old_device.model.clone(),
        serial_number: old_device.serial_number.clone(),
    }
}

/// Crear audit data usando device info real - FORMATO EXACTO de la app oficial
pub fn create_audit_data(device_info: &DeviceInfoV3) -> serde_json::Value {
    let audit_data = serde_json::json!({
        "appName": "CP DISTRI V2",
        "cle1": "",
        "cle2": "",
        "cle3": "",
        "deviceModelName": device_info.model,
        "iccid": "indisponible",
        "imei": device_info.imei,
        "msisdn": "indisponible",
        "noSerie": "3qtg83zdy95jmczkeiyx1rfa9"  // CORREGIDO: Usar el valor exacto de la app oficial
    });
    
    info!(
        device_model = %device_info.model,
        imei_preview = %&device_info.imei[..8.min(device_info.imei.len())],
        serial_preview = %&device_info.serial_number[..8.min(device_info.serial_number.len())],
        "Audit data creado con device info real - FORMATO EXACTO de la app oficial"
    );
    
    audit_data
}

/// Generar headers para API v3.3.0.9 con estructuras nuevas
pub fn get_v3_headers(
    device_info: &DeviceInfoV3,
    app_info: &AppInfo,
    activity_id: Uuid,
    sso_hopps: Option<String>,
) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    
    // CORE HEADERS exactos de la versión 3.3.0.9
    headers.insert("Accept-Charset", "UTF-8".parse()?);
    headers.insert("Content-Type", "application/json; charset=UTF-8".parse()?);
    headers.insert("Connection", "Keep-Alive".parse()?);
    headers.insert("Accept-Encoding", "gzip".parse()?);
    headers.insert("User-Agent", "okhttp/3.4.1".parse()?);
    
    // APP IDENTIFICATION v3.3.0.9
    headers.insert("ActivityId", activity_id.to_string().parse()?);
    headers.insert("AppName", "CP DISTRI V2".parse()?);
    headers.insert("AppIdentifier", app_info.app_identifier.parse()?);
    headers.insert("Device", device_info.model.parse()?);
    headers.insert("VersionOS", device_info.android_version.parse()?);
    headers.insert("VersionApplication", app_info.version_name.parse()?);
    headers.insert("VersionCode", app_info.version_code.parse()?);
    headers.insert("Societe", app_info.societe.parse()?);
    headers.insert("Domaine", "Membership".parse()?);
    
    // TOKEN (si está disponible)
    let has_token = sso_hopps.is_some();
    if let Some(token) = sso_hopps {
        headers.insert("SsoHopps", token.parse()?);
    }
    
    info!(
        activity_id = %activity_id,
        device_model = %device_info.model,
        version = %app_info.version_name,
        has_token = has_token,
        "Headers v3.3.0.9 generados"
    );
    
    Ok(headers)
}

/// Generar headers para la API Web real de Colis Privé
/// Basados en el tráfico capturado del navegador
pub fn get_web_headers() -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    
    // HEADERS EXACTOS DEL TRÁFICO CAPTURADO
    headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
    headers.insert("Accept-Language", "fr-FR,fr;q=0.5".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Origin", "https://gestiontournee.colisprive.com".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Referer", "https://gestiontournee.colisprive.com/".parse().unwrap());
    headers.insert("Sec-GPC", "1".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36".parse().unwrap());
    
    // HEADERS SECURITY (opcionales pero recomendados)
    headers.insert("sec-ch-ua", "\"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    
    debug!("🌐 Headers Web API generados: {:?}", headers);
    Ok(headers)
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
