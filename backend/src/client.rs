use crate::external_models::{
    MobilePackageAction, ColisPriveCredentials, LoginRequest, 
    LoginResponse, RefreshTokenRequest, ColisAuthResponse, Commun, TourneeRequest,
    DeviceInfo, ColisPriveOfficialLoginRequest, ColisPriveCommun
};
use anyhow::Result;
use base64::Engine;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;
use tracing::{info, warn, error, debug, instrument};
use crate::utils::headers::{get_colis_headers, create_audit_data, create_colis_client, convert_device_info_to_v3};

pub struct ColisPriveClient {
    pub client: Client,
    pub auth_base_url: String,
    pub tournee_base_url: String,
    sso_token: Option<String>,
    device_info: DeviceInfo, // Device info dinámico
}

impl ColisPriveClient {
    pub fn new(device_info: DeviceInfo) -> Result<Self> {
        // Usar la función de headers para crear cliente con SSL bypass
        let client = create_colis_client()?;

        Ok(Self {
            client,
            auth_base_url: "https://wsauthentificationexterne.colisprive.com".to_string(),
            tournee_base_url: "https://wstournee-v2.colisprive.com".to_string(),
            sso_token: None,
            device_info,
        })
    }

    /// Obtener headers exactos de la app oficial de Colis Privé usando device info dinámico
    fn get_colis_headers(&self, endpoint: &str, username: Option<&str>, token: Option<&str>) -> reqwest::header::HeaderMap {
        get_colis_headers(endpoint, &self.device_info, username, token)
    }

    /// Obtener headers comunes para todas las requests (método legacy - mantener compatibilidad)
    fn get_common_headers(&self) -> reqwest::header::HeaderMap {
        self.get_colis_headers("default", None, None)
    }

    #[instrument(skip(self, login, password, societe), fields(username = %login, societe = %societe))]
    pub async fn login(&mut self, login: &str, password: &str, societe: &str) -> Result<LoginResponse> {
        let start_time = std::time::Instant::now();
        
        info!(
            endpoint = "login",
            username = %login,
            societe = %societe,
            device_model = %self.device_info.model,
            "Starting Colis Privé authentication with dynamic device info"
        );
        
        let url = format!("{}/api/auth/login/Membership", self.auth_base_url);
        
        // Crear audit data usando device info real
        let device_v3 = convert_device_info_to_v3(&self.device_info);
        let audit_data = create_audit_data(&device_v3);
        
        // CORREGIDO: Usar estructura EXACTA de la app oficial
        let login_req = ColisPriveOfficialLoginRequest {
            audit: audit_data.clone(),  // CORREGIDO: Clonar para poder usar en debug
            commun: ColisPriveCommun {
                dureeTokenInHour: 0,  // Exacto como en la app oficial
            },
            login: format!("{} ", login),  // CORREGIDO: Agregar espacio al final como en la app oficial
            password: password.to_string(),
            societe: societe.to_string(),
        };

        debug!(
            endpoint = "login",
            url = %url,
            request_body = ?login_req,
            audit_data = ?audit_data,
            "Sending authentication request with dynamic audit data"
        );

        let headers = self.get_colis_headers("login", Some(login), None);
        
        debug!(
            endpoint = "login",
            headers_count = headers.len(),
            has_activity_id = headers.contains_key("ActivityId"),
            has_app_name = headers.contains_key("AppName"),
            device_model = %self.device_info.model,
            "Using authentication headers with dynamic device info"
        );

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&login_req)
            .send()
            .await?;

        let status = response.status();
        let duration = start_time.elapsed();
        
        info!(
            endpoint = "login",
            status = %status,
            duration_ms = duration.as_millis(),
            success = status.is_success(),
            device_model = %self.device_info.model,
            "Authentication response received"
        );

        if !status.is_success() {
            let error_body = response.text().await?;
            error!(
                endpoint = "login",
                status = %status,
                error_body = %error_body,
                duration_ms = duration.as_millis(),
                device_model = %self.device_info.model,
                "Authentication failed"
            );
            anyhow::bail!(
                "Login falló con status: {} - Body: {}",
                status,
                error_body
            );
        }

        let login_response: LoginResponse = response.json().await?;
        
        // Logging seguro del token
        let token_preview = &login_response.tokens.SsoHopps[..20.min(login_response.tokens.SsoHopps.len())];
        
        info!(
            endpoint = "login",
            success = true,
            token_preview = %token_preview,
            token_length = login_response.tokens.SsoHopps.len(),
            duration_ms = duration.as_millis(),
            device_model = %self.device_info.model,
            "Authentication successful with dynamic device info"
        );
        
        // Ahora usamos el token real de la respuesta
        self.sso_token = Some(login_response.tokens.SsoHopps.clone());
        
        Ok(login_response)
    }

    pub async fn get_pilot_access(&self, token: &str, matricule: &str, societe: &str) -> Result<()> {
        let url = format!("https://ws-gestiontournee-inter.colisprive.com/WS_PilotManagement/api/Pilot/access/{}/{}/FRONT_MOP", matricule, societe);
        
        debug!("📤 Request 1: Pilot access - {}", url);

        let response = self.client
            .get(&url)
            .header("SsoHopps", token) // Intentar con mayúscula
            .header("Origin", "https://gestiontournee.colisprive.com")
            .header("Referer", "https://gestiontournee.colisprive.com/")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
            .send()
            .await?;

        let status = response.status();
        debug!("📥 Pilot access status: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            anyhow::bail!("Pilot access falló con status: {} - Body: {}", status, error_body);
        }

        Ok(())
    }

    pub async fn get_dashboard_info(&self, token: &str, societe: &str, matricule: &str, date: &str) -> Result<serde_json::Value> {
        // Usar la función curl que funciona perfectamente
        self.get_dashboard_info_curl(token, societe, matricule, date).await
    }

    // Función alternativa usando curl para comparar
    pub async fn get_dashboard_info_curl(&self, token: &str, societe: &str, matricule: &str, date: &str) -> Result<serde_json::Value> {
        let url = format!("{}/WS-TourneeColis/api/getBeanInfoDashBoardBySocieteMatriculev2/", self.tournee_base_url);
        
        let dashboard_req = json!({
            "Societe": societe,
            "Matricule": matricule,
            "DateDebut": format!("{}T00:00:00.000Z", date),
            "Agence": null,
            "Concentrateur": null
        });

        debug!("🔍 Dashboard URL (curl): {}", url);
        debug!("🔍 Dashboard Token (curl): {}...", &token[..50.min(token.len())]);

        // Construir comando curl
        let curl_cmd = format!(
            "curl -X POST '{}' \
            -H 'Accept: application/json, text/plain, */*' \
            -H 'Accept-Language: fr-FR,fr;q=0.5' \
            -H 'Cache-Control: no-cache' \
            -H 'Connection: keep-alive' \
            -H 'Content-Type: application/json' \
            -H 'Origin: https://gestiontournee.colisprive.com' \
            -H 'Pragma: no-cache' \
            -H 'Referer: https://gestiontournee.colisprive.com/' \
            -H 'Sec-Fetch-Dest: empty' \
            -H 'Sec-Fetch-Mode: cors' \
            -H 'Sec-Fetch-Site: same-site' \
            -H 'Sec-GPC: 1' \
            -H 'SsoHopps: {}' \
            -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36' \
            -H 'sec-ch-ua: \"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"' \
            -H 'sec-ch-ua-mobile: ?0' \
            -H 'sec-ch-ua-platform: \"macOS\"' \
            -d '{}'",
            url, token, serde_json::to_string(&dashboard_req)?
        );

        debug!("🔍 Comando curl: {}", curl_cmd);

        // Ejecutar curl
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&curl_cmd)
            .output()?;

        if output.status.success() {
            let response_text = String::from_utf8_lossy(&output.stdout);
            debug!("✅ Curl Success! Response: {}", response_text);
            
            // Intentar parsear como JSON
            match serde_json::from_str::<serde_json::Value>(&response_text) {
                Ok(json_response) => Ok(json_response),
                Err(_) => {
                    // Si no es JSON válido, crear un objeto con el texto
                    Ok(json!({"raw_response": response_text}))
                }
            }
        } else {
            let error_text = String::from_utf8_lossy(&output.stderr);
            debug!("❌ Curl Error: {}", error_text);
            Err(anyhow::anyhow!("Curl request failed: {}", error_text))
        }
    }

    // Función alternativa usando curl para obtener tournée
    pub async fn get_tournee_curl(&self, token: &str, societe: &str, matricule: &str, date: &str) -> Result<String> {
        let url = format!("{}/WS-TourneeColis/api/getLettreVoitureEco_POST", self.tournee_base_url);
        
        let tournee_req = json!({
            "enumTypeLettreVoiture": "ordreScan",
            "beanParamsMatriculeDateDebut": {
                "Societe": societe,
                "Matricule": matricule,
                "DateDebut": date
            }
        });

        debug!("🔍 Tournée URL (curl): {}", url);
        debug!("🔍 Tournée Token (curl): {}...", &token[..50.min(token.len())]);

        // Construir comando curl
        let curl_cmd = format!(
            "curl -X POST '{}' \
            -H 'Accept: application/json, text/plain, */*' \
            -H 'Accept-Language: fr-FR,fr;q=0.5' \
            -H 'Cache-Control: no-cache' \
            -H 'Connection: keep-alive' \
            -H 'Content-Type: application/json' \
            -H 'Origin: https://gestiontournee.colisprive.com' \
            -H 'Pragma: no-cache' \
            -H 'Referer: https://gestiontournee.colisprive.com/' \
            -H 'Sec-Fetch-Dest: empty' \
            -H 'Sec-Fetch-Mode: cors' \
            -H 'Sec-Fetch-Site: same-site' \
            -H 'Sec-GPC: 1' \
            -H 'SsoHopps: {}' \
            -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36' \
            -H 'sec-ch-ua: \"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"' \
            -H 'sec-ch-ua-mobile: ?0' \
            -H 'sec-ch-ua-platform: \"macOS\"' \
            -d '{}'",
            url, token, serde_json::to_string(&tournee_req)?
        );

        debug!("🔍 Comando curl tournée: {}", curl_cmd);

        // Ejecutar curl
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&curl_cmd)
            .output()?;

        if output.status.success() {
            let response_text = String::from_utf8_lossy(&output.stdout);
            debug!("✅ Curl Tournée Success! Response length: {}", response_text.len());
            Ok(response_text.to_string())
        } else {
            let error_text = String::from_utf8_lossy(&output.stderr);
            debug!("❌ Curl Tournée Error: {}", error_text);
            Err(anyhow::anyhow!("Curl tournée request failed: {}", error_text))
        }
    }

    pub async fn get_tournee(&self, societe: &str, matricule: &str, date: &str) -> Result<String> {
        let sso_token = self.sso_token.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No hay token de autenticación. Haz login primero."))?;

        debug!("🔍 Activando sesión con requests intermedias...");

        // 1. Request intermedia: Pilot access
        self.get_pilot_access(sso_token, matricule, societe).await?;
        debug!("✅ Pilot access exitoso!");

        // 2. Request intermedia: Dashboard info
        let _dashboard_response = self.get_dashboard_info(sso_token, societe, matricule, date).await?;
        debug!("✅ Dashboard info exitoso!");

        // 3. Ahora sí, la request final de tournée
        debug!("🚀 Activando request final de tournée...");

        let tournee_url = format!("{}/WS-TourneeColis/api/getLettreVoitureEco_POST", self.tournee_base_url);

        let tournee_request = TourneeRequest {
            enum_type_lettre_voiture: "ordreScan".to_string(),
            bean_params: crate::external_models::TourneeParams {
                societe: societe.to_string(),
                matricule: matricule.to_string(),
                date_debut: date.to_string(),
            },
        };

        debug!("🔍 URL de tournée: {}", tournee_url);
        debug!("📤 Enviando request: {:?}", tournee_request);
        debug!("🔑 Token de autorización: {}", sso_token);

        let response = self.client
            .post(&tournee_url)
            .header("Content-Type", "application/json")
            .header("SsoHopps", sso_token) // Intentar con mayúscula
            .header("Origin", "https://gestiontournee.colisprive.com")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
            .json(&tournee_request)
            .send()
            .await?;

        let status = response.status();
        debug!("📥 Status de respuesta: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            anyhow::bail!(
                "Obtener tournée falló con status: {} - Body: {}",
                status,
                error_body
            );
        }

        let tournee_data: String = response.text().await?;

        Ok(tournee_data)
    }

    pub fn is_authenticated(&self) -> bool {
        self.sso_token.is_some()
    }

    /// Obtener tournée usando el endpoint móvil real de Colis Privé
    pub async fn get_mobile_tournee(
        &self,
        credentials: &ColisPriveCredentials,
        date: &str,
        matricule: &str,
        token: &str,
    ) -> Result<Vec<crate::external_models::MobilePackageAction>, Box<dyn std::error::Error>> {
        let body = serde_json::json!({
            "DateDebut": date,
            "Matricule": matricule
        });

        debug!("🚀 Llamando endpoint móvil de Colis Privé...");
        debug!("📱 URL: https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST");
        debug!("🔑 Token: {}...", &token[..50.min(token.len())]);
        debug!("�� Fecha: {}", date);
        debug!("🆔 Matrícula: {}", matricule);

        // Usar headers exactos de la app oficial
        let username = credentials.username.split('_').last().unwrap_or(&credentials.username);
        let headers = self.get_colis_headers("tournee", Some(username), Some(token));
        
        let response = self.client
            .post("https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        debug!("📥 Status de respuesta móvil: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Endpoint móvil falló con status: {} - Body: {}", status, error_body)
            )));
        }

        let mobile_data: Vec<crate::external_models::MobilePackageAction> = response.json().await?;
        debug!("✅ Datos móviles obtenidos exitosamente: {} paquetes", mobile_data.len());
        
        Ok(mobile_data)
    }

    /// Refresh token usando el endpoint /api/auth/login-token
    #[instrument(skip(self, old_token), fields(token_preview = %&old_token[..20.min(old_token.len())]))]
    pub async fn refresh_token(&mut self, old_token: &str) -> Result<ColisAuthResponse> {
        let start_time = std::time::Instant::now();
        
        info!(
            endpoint = "refresh_token",
            token_preview = %&old_token[..20.min(old_token.len())],
            token_length = old_token.len(),
            "Starting token refresh"
        );
        
        let refresh_request = json!({
            "dureeTokenInHour": 0,
            "token": old_token
        });
        
        let url = format!("{}/api/auth/login-token", self.auth_base_url);
        let headers = self.get_colis_headers("refresh", None, None);
        
        debug!(
            endpoint = "refresh_token",
            url = %url,
            request_body = ?refresh_request,
            headers_count = headers.len(),
            "Sending refresh token request"
        );
        
        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&refresh_request)
            .send()
            .await?;
        
        let status = response.status();
        let duration = start_time.elapsed();
        
        info!(
            endpoint = "refresh_token",
            status = %status,
            duration_ms = duration.as_millis(),
            success = status.is_success(),
            "Refresh token response received"
        );
        
        if !status.is_success() {
            let error_body = response.text().await?;
            error!(
                endpoint = "refresh_token",
                status = %status,
                error_body = %error_body,
                duration_ms = duration.as_millis(),
                "Token refresh failed"
            );
            return Err(anyhow::anyhow!("Refresh token falló con status: {}", status));
        }
        
        let refresh_response: ColisAuthResponse = response.json().await?;
        
        if !refresh_response.is_authentif {
            warn!(
                endpoint = "refresh_token",
                is_authentif = false,
                duration_ms = duration.as_millis(),
                "Refresh token returned invalid authentication"
            );
            return Err(anyhow::anyhow!("Refresh token falló - autenticación inválida"));
        }
        
        // Actualizar el token en el cliente
        self.sso_token = Some(refresh_response.tokens.sso_hopps.clone());
        
        // Logging seguro del nuevo token
        let new_token_preview = &refresh_response.tokens.sso_hopps[..20.min(refresh_response.tokens.sso_hopps.len())];
        
        info!(
            endpoint = "refresh_token",
            success = true,
            new_token_preview = %new_token_preview,
            new_token_length = refresh_response.tokens.sso_hopps.len(),
            is_authentif = refresh_response.is_authentif,
            duration_ms = duration.as_millis(),
            "Token refresh successful"
        );
        
        Ok(refresh_response)
    }
    
    /// Obtener tournée móvil usando un token específico
    #[instrument(skip(self, username, password, societe, date, token), fields(username = %username, date = %date, token_preview = %&token[..20.min(token.len())]))]
    pub async fn get_mobile_tournee_with_token(
        &mut self,
        username: &str,
        password: &str,
        societe: &str,
        date: &str,
        token: &str,
    ) -> Result<serde_json::Value> {
        let start_time = std::time::Instant::now();
        
        info!(
            endpoint = "mobile_tournee_with_token",
            username = %username,
            date = %date,
            societe = %societe,
            token_preview = %&token[..20.min(token.len())],
            "Starting mobile tournée request with specific token"
        );
        
        let body = json!({
            "DateDebut": date,
            "Matricule": username // ✅ CORREGIDO: Usar username directamente sin duplicar societe
        });
        
        let url = "https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST";
        let headers = self.get_colis_headers("tournee", Some(username), Some(token));
        
        // Logging detallado de headers para tournée
        let username_header = headers.get("UserName").map(|h| h.to_str().unwrap_or("ERROR"));
        let societe_header = headers.get("Societe").map(|h| h.to_str().unwrap_or("ERROR"));
        let token_header = headers.get("SsoHopps").map(|h| h.to_str().unwrap_or("ERROR"));
        let activity_id_header = headers.get("ActivityId").map(|h| h.to_str().unwrap_or("ERROR"));
        let device_header = headers.get("Device").map(|h| h.to_str().unwrap_or("ERROR"));
        
        info!(
            endpoint = "mobile_tournee_with_token",
            url = %url,
            request_body = ?body,
            headers_count = headers.len(),
            username_header = ?username_header,
            societe_header = ?societe_header,
            token_preview = ?token_header.map(|t| &t[..20.min(t.len())]),
            activity_id = ?activity_id_header,
            device_header = ?device_header,
            has_sso_hopps = headers.contains_key("SsoHopps"),
            "Headers detallados para tournée"
        );
        
        let response = self.client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;
        
        let status = response.status();
        let duration = start_time.elapsed();
        
        info!(
            endpoint = "mobile_tournee_with_token",
            status = %status,
            duration_ms = duration.as_millis(),
            success = status.is_success(),
            "Tournée response received"
        );
        
        if status == reqwest::StatusCode::UNAUTHORIZED {
            warn!(
                endpoint = "mobile_tournee_with_token",
                status = %status,
                duration_ms = duration.as_millis(),
                "401 Unauthorized - Token expired or invalid"
            );
            return Err(anyhow::anyhow!("Token expirado o inválido"));
        }
        
        if !status.is_success() {
            let error_body = response.text().await?;
            error!(
                endpoint = "mobile_tournee_with_token",
                status = %status,
                error_body = %error_body,
                duration_ms = duration.as_millis(),
                "Tournée request failed"
            );
            return Err(anyhow::anyhow!("Endpoint tournée falló con status: {}", status));
        }
        
        let tournee_data = response.json().await?;
        
        info!(
            endpoint = "mobile_tournee_with_token",
            success = true,
            duration_ms = duration.as_millis(),
            "Tournée data successfully retrieved"
        );
        
        Ok(tournee_data)
    }
    
    /// Obtener tournée móvil con auto-retry y refresh token automático
    #[instrument(skip(self, username, _password, societe, date, token), fields(username = %username, date = %date, has_token = token.is_some()))]
    pub async fn get_mobile_tournee_with_retry(
        &mut self,
        username: &str,
        _password: &str,
        societe: &str,
        date: &str,
        token: Option<&str>,
    ) -> Result<serde_json::Value> {
        let start_time = std::time::Instant::now();
        
        info!(
            endpoint = "mobile_tournee_with_retry",
            username = %username,
            date = %date,
            societe = %societe,
            has_token = token.is_some(),
            "Starting mobile tournée with auto-retry"
        );
        
        // Si no hay token, hacer login inicial
        let token = if let Some(token) = token {
            info!(
                endpoint = "mobile_tournee_with_retry",
                token_preview = %&token[..20.min(token.len())],
                "Using existing token for tournée"
            );
            token.to_string()
        } else {
            info!(
                endpoint = "mobile_tournee_with_retry",
                username = %username,
                "No token provided, performing initial login"
            );
            
            // Usar el método login existente
            let auth_response = self.login(username, _password, societe).await?;
            
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
        
        match self.get_mobile_tournee_with_token(username, _password, societe, date, &token).await {
            Ok(tournee_data) => {
                let duration = start_time.elapsed();
                info!(
                    endpoint = "mobile_tournee_with_retry",
                    attempt = 1,
                    success = true,
                    duration_ms = duration.as_millis(),
                    "Tournée successful with current token"
                );
                Ok(tournee_data)
            }
            Err(e) if e.to_string().contains("401") || e.to_string().contains("Token expirado") => {
                let attempt1_duration = start_time.elapsed();
                warn!(
                    endpoint = "mobile_tournee_with_retry",
                    attempt = 1,
                    error = %e,
                    duration_ms = attempt1_duration.as_millis(),
                    "Token expired, attempting refresh"
                );
                
                // Intento 2: Refresh token y retry
                debug!(
                    endpoint = "mobile_tournee_with_retry",
                    attempt = 2,
                    "Starting token refresh for retry"
                );
                
                let refresh_start = std::time::Instant::now();
                
                // Hacer refresh del token
                let refresh_response = self.refresh_token(&token).await?;
                let new_token = refresh_response.tokens.sso_hopps.clone();
                
                let refresh_duration = refresh_start.elapsed();
                info!(
                    endpoint = "mobile_tournee_with_retry",
                    attempt = 2,
                    new_token_preview = %&new_token[..20.min(new_token.len())],
                    refresh_duration_ms = refresh_duration.as_millis(),
                    "Token refresh successful, retrying tournée"
                );
                
                // Retry con el nuevo token
                let retry_start = std::time::Instant::now();
                let result = self.get_mobile_tournee_with_token(username, _password, societe, date, &new_token).await;
                
                let total_duration = start_time.elapsed();
                let retry_duration = retry_start.elapsed();
                
                match &result {
                    Ok(_) => {
                        info!(
                            endpoint = "mobile_tournee_with_retry",
                            attempt = 2,
                            success = true,
                            total_duration_ms = total_duration.as_millis(),
                            retry_duration_ms = retry_duration.as_millis(),
                            "Tournée successful after token refresh and retry"
                        );
                    }
                    Err(e) => {
                        error!(
                            endpoint = "mobile_tournee_with_retry",
                            attempt = 2,
                            error = %e,
                            total_duration_ms = total_duration.as_millis(),
                            retry_duration_ms = retry_duration.as_millis(),
                            "Tournée failed after token refresh and retry"
                        );
                    }
                }
                
                result
            }
            Err(e) => {
                let duration = start_time.elapsed();
                error!(
                    endpoint = "mobile_tournee_with_retry",
                    attempt = 1,
                    error = %e,
                    duration_ms = duration.as_millis(),
                    "Non-recoverable error in tournée request"
                );
                Err(e)
            }
        }
    }
}
