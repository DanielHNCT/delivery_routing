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
}

impl ColisPriveClient {
    pub fn new() -> Result<Self> {
        // ureq no necesita builder, es más simple

        Ok(Self {
            client: reqwest::Client::new(),
            auth_base_url: "https://wsauthentificationexterne.colisprive.com".to_string(),
            tournee_base_url: "https://wstournee-v2.colisprive.com".to_string(),
            sso_token: None,
        })
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

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "fr-FR,fr;q=0.5")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Origin", "https://gestiontournee.colisprive.com")
            .header("Referer", "https://gestiontournee.colisprive.com/")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
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
        let activity_id = uuid::Uuid::new_v4().to_string();
        let basic_auth = format!("Basic {}", base64::engine::general_purpose::STANDARD.encode(format!("{}:null", matricule)));
        
        let body = serde_json::json!({
            "DateDebut": date,
            "Matricule": matricule
        });

        println!("🚀 Llamando endpoint móvil de Colis Privé...");
        println!("📱 URL: https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST");
        println!("🔑 Token: {}", token);
        println!("📅 Fecha: {}", date);
        println!("🆔 Matrícula: {}", matricule);

        let response = self.client
            .post("https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST")
            .header("Accept-Charset", "UTF-8")
            .header("ActivityId", &activity_id)
            .header("AppName", "CP DISTRI V2")
            .header("UserName", matricule.split('_').last().unwrap_or(matricule).to_string())
            .header("AppIdentifier", "com.delivery.optimizer")
            .header("Device", "AndroidApp")
            .header("VersionOS", "Android")
            .header("VersionApplication", "1.0.0")
            .header("VersionCode", "1")
            .header("Societe", &credentials.societe)
            .header("Domaine", "Membership")
            .header("SsoHopps", token)
            .header("Authorization", &basic_auth)
            .header("Content-Type", "application/json; charset=UTF-8")
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
        
        let response = self.client
            .post("https://wsauthentificationexterne.colisprive.com/api/auth/login-token")
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "fr-FR,fr;q=0.5")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Origin", "https://gestiontournee.colisprive.com")
            .header("Referer", "https://gestiontournee.colisprive.com/")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
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
        
        let response = self.client
            .post("https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST")
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("SsoHopps", token)
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "fr-FR,fr;q=0.5")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Origin", "https://gestiontournee.colisprive.com")
            .header("Referer", "https://gestiontournee.colisprive.com/")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
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
}
