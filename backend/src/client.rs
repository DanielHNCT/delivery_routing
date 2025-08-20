use crate::external_models::{
    MobilePackageAction, ColisPriveCredentials, LoginRequest, 
    LoginResponse, RefreshTokenRequest, ColisAuthResponse, Commun, TourneeRequest
};
use anyhow::Result;
use base64::Engine;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;
use tracing::{info, error};

pub struct ColisPriveClient {
    client: Client,
    auth_base_url: String,
    tournee_base_url: String,
    sso_token: Option<String>,
    activity_id: String, // UUID único por sesión
}

impl ColisPriveClient {
    pub fn new() -> Result<Self> {
        // Configurar cliente con SSL bypass y headers específicos
        let client = reqwest::Client::builder()
            .http1_only() // Forzar HTTP/1.1
            .http1_title_case_headers() // Headers en formato correcto
            .cookie_store(true) // Mantener cookies de sesión
            .danger_accept_invalid_certs(true) // SSL bypass
            .danger_accept_invalid_hostnames(true) // Hostnames inválidos
            .timeout(Duration::from_secs(30)) // Timeout de 30 segundos
            .build()?;

        Ok(Self {
            client,
            auth_base_url: "https://wsauthentificationexterne.colisprive.com".to_string(),
            tournee_base_url: "https://wstournee-v2.colisprive.com".to_string(),
            sso_token: None,
            activity_id: Uuid::new_v4().to_string(), // UUID único por sesión
        })
    }

    /// Obtener headers comunes para todas las requests
    fn get_common_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        
        headers.insert("Accept-Charset", "UTF-8".parse().unwrap());
        headers.insert("ActivityId", self.activity_id.parse().unwrap());
        headers.insert("AppName", "CP DISTRI V2".parse().unwrap());
        headers.insert("AppIdentifier", "com.danem.cpdistriv2".parse().unwrap());
        headers.insert("Device", "Sony D5503".parse().unwrap());
        headers.insert("VersionOS", "5.1.1".parse().unwrap());
        headers.insert("VersionApplication", "3.3.0.9".parse().unwrap());
        headers.insert("VersionCode", "1".parse().unwrap());
        headers.insert("Societe", "PCP0010699".parse().unwrap());
        headers.insert("Domaine", "Membership".parse().unwrap());
        headers.insert("Content-Type", "application/json; charset=UTF-8".parse().unwrap());
        headers.insert("Connection", "Keep-Alive".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip".parse().unwrap());
        headers.insert("User-Agent", "okhttp/3.4.1".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        headers.insert("X-Device-Info", "Android".parse().unwrap());
        headers.insert("X-App-Build", "1".parse().unwrap());
        headers.insert("X-Network-Type", "WIFI".parse().unwrap());
        
        headers
    }

    pub async fn login(&mut self, login: &str, password: &str, societe: &str) -> Result<LoginResponse> {
        let url = format!("{}/api/auth/login/Membership", self.auth_base_url);
        
        let login_req = LoginRequest {
            login: login.to_string(),
            password: password.to_string(),
            societe: societe.to_string(),
            commun: Commun {
                duree_token_in_hour: 24,
            },
        };

        println!("🔐 URL de login: {}", url);
        println!("📤 Enviando request: {:?}", login_req);

        let mut headers = self.get_common_headers();
        // Agregar headers específicos del login
        headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
        headers.insert("Accept-Language", "fr-FR,fr;q=0.5".parse().unwrap());
        headers.insert("Cache-Control", "no-cache".parse().unwrap());
        headers.insert("Pragma", "no-cache".parse().unwrap());
        headers.insert("Origin", "https://gestiontournee.colisprive.com".parse().unwrap());
        headers.insert("Referer", "https://gestiontournee.colisprive.com/".parse().unwrap());

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&login_req)
            .send()
            .await?;

        let status = response.status();
        println!("📥 Status de respuesta: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            anyhow::bail!(
                "Login falló con status: {} - Body: {}",
                status,
                error_body
            );
        }

        let login_response: LoginResponse = response.json().await?;
        // Ahora usamos el token real de la respuesta
        self.sso_token = Some(login_response.tokens.SsoHopps.clone());
        
        Ok(login_response)
    }

    pub async fn get_pilot_access(&self, token: &str, matricule: &str, societe: &str) -> Result<()> {
        let url = format!("https://ws-gestiontournee-inter.colisprive.com/WS_PilotManagement/api/Pilot/access/{}/{}/FRONT_MOP", matricule, societe);
        
        println!("📤 Request 1: Pilot access - {}", url);

        let response = self.client
            .get(&url)
            .header("SsoHopps", token) // Intentar con mayúscula
            .header("Origin", "https://gestiontournee.colisprive.com")
            .header("Referer", "https://gestiontournee.colisprive.com/")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
            .send()
            .await?;

        let status = response.status();
        println!("📥 Pilot access status: {}", status);

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

        println!("🔍 Dashboard URL (curl): {}", url);
        println!("🔍 Dashboard Token (curl): {}...", &token[..50.min(token.len())]);

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

        println!("🔍 Comando curl: {}", curl_cmd);

        // Ejecutar curl
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&curl_cmd)
            .output()?;

        if output.status.success() {
            let response_text = String::from_utf8_lossy(&output.stdout);
            println!("✅ Curl Success! Response: {}", response_text);
            
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
            println!("❌ Curl Error: {}", error_text);
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

        println!("🔍 Tournée URL (curl): {}", url);
        println!("🔍 Tournée Token (curl): {}...", &token[..50.min(token.len())]);

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

        println!("🔍 Comando curl tournée: {}", curl_cmd);

        // Ejecutar curl
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&curl_cmd)
            .output()?;

        if output.status.success() {
            let response_text = String::from_utf8_lossy(&output.stdout);
            println!("✅ Curl Tournée Success! Response length: {}", response_text.len());
            Ok(response_text.to_string())
        } else {
            let error_text = String::from_utf8_lossy(&output.stderr);
            println!("❌ Curl Tournée Error: {}", error_text);
            Err(anyhow::anyhow!("Curl tournée request failed: {}", error_text))
        }
    }

    pub async fn get_tournee(&self, societe: &str, matricule: &str, date: &str) -> Result<String> {
        let sso_token = self.sso_token.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No hay token de autenticación. Haz login primero."))?;

        println!("🔍 Activando sesión con requests intermedias...");

        // 1. Request intermedia: Pilot access
        self.get_pilot_access(sso_token, matricule, societe).await?;
        println!("✅ Pilot access exitoso!");

        // 2. Request intermedia: Dashboard info
        let _dashboard_response = self.get_dashboard_info(sso_token, societe, matricule, date).await?;
        println!("✅ Dashboard info exitoso!");

        // 3. Ahora sí, la request final de tournée
        println!("🚀 Activando request final de tournée...");

        let tournee_url = format!("{}/WS-TourneeColis/api/getLettreVoitureEco_POST", self.tournee_base_url);

        let tournee_request = TourneeRequest {
            enum_type_lettre_voiture: "ordreScan".to_string(),
            bean_params: crate::external_models::TourneeParams {
                societe: societe.to_string(),
                matricule: matricule.to_string(),
                date_debut: date.to_string(),
            },
        };

        println!("🔍 URL de tournée: {}", tournee_url);
        println!("📤 Enviando request: {:?}", tournee_request);
        println!("🔑 Token de autorización: {}", sso_token);

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
        println!("📥 Status de respuesta: {}", status);

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

        println!("🚀 Llamando endpoint móvil de Colis Privé...");
        println!("📱 URL: https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST");
        println!("🔑 Token: {}...", &token[..50.min(token.len())]);
        println!("📅 Fecha: {}", date);
        println!("🆔 Matrícula: {}", matricule);

        // Usar headers correctos de la app oficial
        let mut headers = self.get_common_headers();
        headers.insert("SsoHopps", token.parse().unwrap());
        // Remover headers que no son necesarios para este endpoint
        headers.remove("Content-Type"); // Se establece automáticamente con .json()
        
        let response = self.client
            .post("https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        println!("📥 Status de respuesta móvil: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Endpoint móvil falló con status: {} - Body: {}", status, error_body)
            )));
        }

        let mobile_data: Vec<crate::external_models::MobilePackageAction> = response.json().await?;
        println!("✅ Datos móviles obtenidos exitosamente: {} paquetes", mobile_data.len());
        
        Ok(mobile_data)
    }

    /// Refresh token usando el endpoint /api/auth/login-token
    pub async fn refresh_token(&mut self, old_token: &str) -> Result<ColisAuthResponse> {
        println!("🔄 REFRESH TOKEN - Token anterior: {}...", &old_token[..50.min(old_token.len())]);
        
        let refresh_request = json!({
            "dureeTokenInHour": 0,
            "token": old_token
        });
        
        let url = format!("{}/api/auth/login-token", self.auth_base_url);
        let headers = self.get_common_headers();
        
        println!("🔐 URL de refresh: {}", url);
        println!("📤 Enviando refresh request: {:?}", refresh_request);
        
        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&refresh_request)
            .send()
            .await?;
        
        let status = response.status();
        println!("📥 Refresh Status: {}", status);
        
        if !status.is_success() {
            println!("❌ Refresh token falló con status: {}", status);
            return Err(anyhow::anyhow!("Refresh token falló con status: {}", status));
        }
        
        let refresh_response: ColisAuthResponse = response.json().await?;
        
        if !refresh_response.is_authentif {
            println!("❌ Refresh token falló - isAuthentif: false");
            return Err(anyhow::anyhow!("Refresh token falló - autenticación inválida"));
        }
        
        // Actualizar el token en el cliente
        self.sso_token = Some(refresh_response.tokens.sso_hopps.clone());
        
        println!("✅ Token refresh exitoso");
        println!("🔑 Nuevo token: {}...", &refresh_response.tokens.sso_hopps[..50.min(refresh_response.tokens.sso_hopps.len())]);
        
        Ok(refresh_response)
    }
    
    /// Obtener tournée móvil usando un token específico
    pub async fn get_mobile_tournee_with_token(
        &mut self,
        username: &str,
        password: &str,
        societe: &str,
        date: &str,
        token: &str,
    ) -> Result<serde_json::Value> {
        println!("📱 TOURNÉE CON TOKEN ESPECÍFICO - Username: {}", username);
        
        let body = json!({
            "DateDebut": date,
            "Matricule": format!("{}_{}", societe, username)
        });
        
        let url = "https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST";
        let mut headers = self.get_common_headers();
        // Agregar el token SsoHopps
        headers.insert("SsoHopps", token.parse().unwrap());
        
        println!("📱 URL de tournée: {}", url);
        println!("🔑 Token usado: {}...", &token[..50.min(token.len())]);
        
        let response = self.client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;
        
        let status = response.status();
        println!("📥 Status de respuesta tournée con token: {}", status);
        
        if status == reqwest::StatusCode::UNAUTHORIZED {
            println!("❌ 401 Unauthorized - Token expirado o inválido");
            return Err(anyhow::anyhow!("Token expirado o inválido"));
        }
        
        if !status.is_success() {
            println!("❌ Error en endpoint tournée con token: {}", status);
            return Err(anyhow::anyhow!("Endpoint tournée falló con status: {}", status));
        }
        
        let tournee_data = response.json().await?;
        println!("✅ Tournée obtenida exitosamente con token específico");
        Ok(tournee_data)
    }
    
    /// Obtener tournée móvil con auto-retry y refresh token automático
    pub async fn get_mobile_tournee_with_retry(
        &mut self,
        username: &str,
        _password: &str,
        societe: &str,
        date: &str,
        token: Option<&str>,
    ) -> Result<serde_json::Value> {
        println!("📱 TOURNÉE CON AUTO-RETRY - Username: {}", username);
        
        // Si no hay token, hacer login inicial
        if token.is_none() {
            println!("🔐 No hay token, haciendo login inicial...");
            self.login(username, _password, societe).await?;
        }
        
        // Obtener el token actual (clonar para evitar borrowing issues)
        let current_token = if let Some(t) = token {
            t.to_string()
        } else {
            self.sso_token.as_ref()
                .expect("Token debe existir después del login")
                .clone()
        };
        
        // Intentar obtener tournée
        match self.get_mobile_tournee_with_token(username, _password, societe, date, &current_token).await {
            Ok(tournee_data) => {
                println!("✅ Tournée obtenida exitosamente");
                Ok(tournee_data)
            }
            Err(e) if e.to_string().contains("401") || e.to_string().contains("Token expirado") => {
                println!("🔄 Token expirado, intentando refresh...");
                
                // Hacer refresh del token
                let refresh_response = self.refresh_token(&current_token).await?;
                let new_token = refresh_response.tokens.sso_hopps.clone();
                
                println!("🔄 Retry con nuevo token...");
                
                // Retry con el nuevo token
                self.get_mobile_tournee_with_token(username, _password, societe, date, &new_token).await
            }
            Err(e) => {
                println!("❌ Error no recuperable: {}", e);
                Err(e)
            }
        }
    }
}
