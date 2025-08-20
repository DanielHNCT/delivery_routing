use anyhow::Result;
use serde_json::json;
use tracing::{info, warn, error, debug, instrument};
use crate::external_models::{
    DeviceInfo, DeviceAuditRequest, DeviceAuditResponse, VersionCheckRealRequest, 
    VersionCheckRealResponse, LogMobiliteRequest, LogMobiliteResponse,
    ColisPriveOfficialLoginRequest, ColisPriveCommun
};
use crate::utils::headers::{get_colis_headers, create_audit_data, create_colis_client};
use reqwest::Client;
use uuid::Uuid;
use chrono::{Utc, DateTime};

pub struct ColisPriveFlowService {
    client: Client,
    store_base_url: String,
    log_base_url: String,
    auth_base_url: String,
}

impl ColisPriveFlowService {
    pub fn new() -> Result<Self> {
        let client = create_colis_client()?;
        
        Ok(Self {
            client,
            store_base_url: "https://store.colisprive.com".to_string(),
            log_base_url: "https://wslog.colisprive.com".to_string(),
            auth_base_url: "https://wsauthentificationexterne.colisprive.com".to_string(),
        })
    }

    /// FLUJO COMPLETO DE AUTENTICACIÓN - EXACTO como la app oficial
    #[instrument(skip(self, device_info, username, password, societe))]
    pub async fn complete_authentication_flow(
        &self,
        device_info: &DeviceInfo,
        username: &str,
        password: &str,
        societe: &str,
    ) -> Result<serde_json::Value> {
        info!(
            username = %username,
            societe = %societe,
            device_model = %device_info.model,
            "Iniciando flujo completo de autenticación Colis Privé"
        );

        let session_id = Uuid::new_v4().to_string();
        let activity_id = Uuid::new_v4().to_string();

        // PASO 1: DEVICE AUDIT (antes del login)
        info!("🔄 PASO 1: Device Audit");
        let device_audit_result = self.device_audit(
            device_info,
            username,
            societe,
            &session_id,
            &activity_id
        ).await?;

        // PASO 2: VERSION CHECK (antes del login)
        info!("🔄 PASO 2: Version Check");
        let version_check_result = self.version_check_real(
            username,
            &session_id,
            &activity_id
        ).await?;

        // PASO 3: LOGIN PRINCIPAL
        info!("🔄 PASO 3: Login Principal");
        let login_result = self.login_with_context(
            device_info,
            username,
            password,
            societe,
            &session_id,
            &activity_id
        ).await?;

        // PASO 4: LOGGING AUTOMÁTICO
        info!("🔄 PASO 4: Logging Automático");
        let logging_result = self.log_mobilite(
            device_info,
            username,
            societe,
            &session_id,
            &activity_id,
            "Login exitoso completado"
        ).await?;

        // RESPUESTA COMPLETA DEL FLUJO
        let complete_response = json!({
            "success": true,
            "flow_completed": true,
            "session_id": session_id,
            "activity_id": activity_id,
            "timestamp": Utc::now().to_rfc3339(),
            "steps": {
                "device_audit": device_audit_result,
                "version_check": version_check_result,
                "login": login_result,
                "logging": logging_result
            },
            "message": "Flujo completo de autenticación ejecutado exitosamente"
        });

        info!(
            username = %username,
            societe = %societe,
            session_id = %session_id,
            "Flujo completo de autenticación completado exitosamente"
        );

        Ok(complete_response)
    }

    /// PASO 1: Device Audit - EXACTO como la app oficial
    #[instrument(skip(self, device_info, username, societe, session_id, activity_id))]
    async fn device_audit(
        &self,
        device_info: &DeviceInfo,
        username: &str,
        societe: &str,
        session_id: &str,
        activity_id: &str,
    ) -> Result<serde_json::Value> {
        info!(
            username = %username,
            societe = %societe,
            "Ejecutando Device Audit"
        );

        let url = format!("{}/WebApi/STORE/API/ANDROID/application/AuditDeviceInstall", self.store_base_url);

        // Crear request de device audit - EXACTO como la app oficial
        let audit_request = json!({
            "deviceCPU": "ARMv7 Processor rev 0 (v7l)",
            "deviceDisk": "1738",
            "deviceIdDevice": "3qtg83zdy95jmczkeiyx1rfa9",
            "deviceLangue": "français",
            "deviceOs": format!("Android {}, API 22", device_info.android_version.replace("Android ", "").replace(" (API 22)", "")),
            "deviceRam": "1738",
            "deviceVersion": device_info.model,
            "idExterneApplication": "com.danem.cpdistriv2",
            "isInstallOK": false,
            "matricule": username.split('_').last().unwrap_or(username),
            "numApplicationVersion": "3.3.0.9"
        });

        // Headers para device audit
        let mut headers = get_colis_headers("device_audit", device_info, Some(username), None);
        headers.insert("ActivityId", activity_id.parse().unwrap());
        headers.insert("UserName", username.parse().unwrap());
        headers.insert("Societe", societe.parse().unwrap());
        headers.insert("SsoHopps", "".parse().unwrap()); // Vacío para device audit

        debug!(
            url = %url,
            request_body = ?audit_request,
            "Enviando Device Audit request"
        );

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&audit_request)
            .send()
            .await?;

        let status = response.status();
        let response_body = response.text().await?;

        info!(
            status = %status,
            response_length = response_body.len(),
            "Device Audit response recibida"
        );

        if status.is_success() {
            Ok(json!({
                "success": true,
                "status": status.as_u16(),
                "message": "Device Audit exitoso",
                "response": response_body
            }))
        } else {
            warn!(
                status = %status,
                response_body = %response_body,
                "Device Audit falló, pero continuando con el flujo"
            );
            Ok(json!({
                "success": false,
                "status": status.as_u16(),
                "message": "Device Audit falló, pero continuando",
                "response": response_body
            }))
        }
    }

    /// PASO 2: Version Check Real - EXACTO como la app oficial
    #[instrument(skip(self, username, session_id, activity_id))]
    async fn version_check_real(
        &self,
        username: &str,
        session_id: &str,
        activity_id: &str,
    ) -> Result<serde_json::Value> {
        info!(
            username = %username,
            "Ejecutando Version Check Real"
        );

        let url = format!("{}/WebApi/STORE/api/android/Application/com.danem.cpdistriv2/CheckVersionForUser/{}/Version/2/3/0/8/0", 
            self.store_base_url, username.split('_').last().unwrap_or(username));

        // Headers para version check
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("ActivityId", activity_id.parse().unwrap());
        headers.insert("AppName", "CP DISTRI V2".parse().unwrap());
        headers.insert("UserName", username.parse().unwrap());
        headers.insert("AppIdentifier", "com.danem.cpdistriv2".parse().unwrap());
        headers.insert("Device", "Sony D5503".parse().unwrap());
        headers.insert("VersionOS", "5.1.1".parse().unwrap());
        headers.insert("VersionApplication", "3.3.0.9".parse().unwrap());
        headers.insert("VersionCode", "1".parse().unwrap());
        headers.insert("Societe", "PCP0010699".parse().unwrap());
        headers.insert("Domaine", "".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());

        debug!(
            url = %url,
            "Enviando Version Check request"
        );

        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = response.status();
        let response_body = response.text().await?;

        info!(
            status = %status,
            response_length = response_body.len(),
            "Version Check response recibida"
        );

        if status.is_success() {
            Ok(json!({
                "success": true,
                "status": status.as_u16(),
                "message": "Version Check exitoso",
                "response": response_body
            }))
        } else {
            warn!(
                status = %status,
                response_body = %response_body,
                "Version Check falló, pero continuando con el flujo"
            );
            Ok(json!({
                "success": false,
                "status": status.as_u16(),
                "message": "Version Check falló, pero continuando",
                "response": response_body
            }))
        }
    }

    /// PASO 3: Login con Contexto Completo
    #[instrument(skip(self, device_info, username, password, societe, session_id, activity_id))]
    async fn login_with_context(
        &self,
        device_info: &DeviceInfo,
        username: &str,
        password: &str,
        societe: &str,
        session_id: &str,
        activity_id: &str,
    ) -> Result<serde_json::Value> {
        info!(
            username = %username,
            societe = %societe,
            "Ejecutando Login con Contexto Completo"
        );

        let url = format!("{}/api/auth/login/Membership", self.auth_base_url);

        // Crear audit data usando device info real
        let audit_data = create_audit_data(device_info);

        // Login request - EXACTO como la app oficial
        let login_req = ColisPriveOfficialLoginRequest {
            audit: audit_data,
            commun: ColisPriveCommun {
                dureeTokenInHour: 0,
            },
            login: format!("{} ", username), // Con espacio al final como en la app oficial
            password: password.to_string(),
            societe: societe.to_string(),
        };

        // Headers para login con contexto completo
        let mut headers = get_colis_headers("login", device_info, Some(username), None);
        headers.insert("ActivityId", activity_id.parse().unwrap());
        headers.insert("UserName", username.parse().unwrap());
        headers.insert("Societe", societe.parse().unwrap());

        debug!(
            url = %url,
            request_body = ?login_req,
            "Enviando Login request con contexto completo"
        );

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&login_req)
            .send()
            .await?;

        let status = response.status();
        let response_body = response.text().await?;

        info!(
            status = %status,
            response_length = response_body.len(),
            "Login response recibida"
        );

        if status.is_success() {
            Ok(json!({
                "success": true,
                "status": status.as_u16(),
                "message": "Login exitoso con contexto completo",
                "response": response_body
            }))
        } else {
            error!(
                status = %status,
                response_body = %response_body,
                "Login falló con contexto completo"
            );
            anyhow::bail!(
                "Login falló con status: {} - Body: {}",
                status,
                response_body
            );
        }
    }

    /// PASO 4: Logging Automático - EXACTO como la app oficial
    #[instrument(skip(self, device_info, username, societe, session_id, activity_id, message))]
    async fn log_mobilite(
        &self,
        device_info: &DeviceInfo,
        username: &str,
        societe: &str,
        session_id: &str,
        activity_id: &str,
        message: &str,
    ) -> Result<serde_json::Value> {
        info!(
            username = %username,
            message = %message,
            "Ejecutando Logging Automático"
        );

        let url = format!("{}/WS_Commun/ServiceWCFLogSpir.svc/REST/LogMobilite", self.log_base_url);

        // Crear request de logging - EXACTO como la app oficial
        let log_request = json!({
            "AppName": "CP DISTRI V2",
            "IndianaVersion": "3.3.0.9",
            "DateLogged": format!("/Date({})/", Utc::now().timestamp_millis()),
            "DnsHostName": "",
            "Exception": "",
            "IpAdress": "",
            "LogLevel": "Info",
            "Logger": "AndroidLogger",
            "Memory": "",
            "Message": message,
            "Parameters": format!("IdDevice: {}\nDevice: {}", 
                "3qtg83zdy95jmczkeiyx1rfa9", device_info.model),
            "ScreenName": "LoginActivity",
            "SessionId": session_id,
            "Thread": "",
            "Trace": "",
            "UserName": username
        });

        // Headers para logging
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept-Charset", "UTF-8".parse().unwrap());
        headers.insert("Authorization", "".parse().unwrap());
        headers.insert("ActivityId", activity_id.parse().unwrap());
        headers.insert("AppName", "CP DISTRI V2".parse().unwrap());
        headers.insert("UserName", "".parse().unwrap());
        headers.insert("AppIdentifier", "com.danem.cpdistriv2".parse().unwrap());
        headers.insert("Device", device_info.model.parse().unwrap());
        headers.insert("VersionOS", device_info.android_version.replace("Android ", "").replace(" (API 22)", "").parse().unwrap());
        headers.insert("VersionApplication", "3.3.0.9".parse().unwrap());
        headers.insert("VersionCode", "1".parse().unwrap());
        headers.insert("Societe", "".parse().unwrap());
        headers.insert("Domaine", "".parse().unwrap());
        headers.insert("Content-Type", "application/json; charset=UTF-8".parse().unwrap());

        debug!(
            url = %url,
            request_body = ?log_request,
            "Enviando Logging request"
        );

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&log_request)
            .send()
            .await?;

        let status = response.status();
        let response_body = response.text().await?;

        info!(
            status = %status,
            response_length = response_body.len(),
            "Logging response recibida"
        );

        if status.is_success() {
            Ok(json!({
                "success": true,
                "status": status.as_u16(),
                "message": "Logging automático exitoso",
                "response": response_body
            }))
        } else {
            warn!(
                status = %status,
                response_body = %response_body,
                "Logging falló, pero no es crítico"
            );
            Ok(json!({
                "success": false,
                "status": status.as_u16(),
                "message": "Logging falló, pero no es crítico",
                "response": response_body
            }))
        }
    }

    /// MANEJO DE RECONEXIÓN - Resuelve el problema del 401
    #[instrument(skip(self, device_info, username, password, societe))]
    pub async fn handle_reconnection(
        &self,
        device_info: &DeviceInfo,
        username: &str,
        password: &str,
        societe: &str,
    ) -> Result<serde_json::Value> {
        info!(
            username = %username,
            societe = %societe,
            "Manejando reconexión (resolviendo 401)"
        );

        // Para reconexión, SIEMPRE ejecutar el flujo completo
        // Esto resuelve el problema del 401 porque recrea todo el contexto
        let result = self.complete_authentication_flow(
            device_info,
            username,
            password,
            societe
        ).await?;

        info!(
            username = %username,
            "Reconexión manejada exitosamente"
        );

        Ok(result)
    }
}
