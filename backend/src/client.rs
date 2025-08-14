use crate::models::{LoginRequest, LoginResponse, TourneeRequest, Commun};
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub struct ColisPriveClient {
    client: Client,
    auth_base_url: String,
    tournee_base_url: String,
    sso_token: Option<String>,
}

impl ColisPriveClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .http1_only()  // CRÍTICO: Forzar HTTP/1.1
            .no_gzip()     // Deshabilitar GZIP automático
            .build()?;

        Ok(Self {
            client,
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
        self.sso_token = Some(login_response.tokens.sso_hopps.clone());
        
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
            bean_params: crate::models::TourneeParams {
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
}
