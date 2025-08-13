use crate::models::{LoginRequest, LoginResponse, TourneeRequest, Commun};
use anyhow::Result;
use reqwest::Client;

pub struct ColisPriveClient {
    client: Client,
    auth_base_url: String,
    tournee_base_url: String,
    sso_token: Option<String>,
}

impl ColisPriveClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            auth_base_url: "https://wsauthentificationexterne.colisprive.com".to_string(),
            tournee_base_url: "https://wstournee-v2.colisprive.com".to_string(),
            sso_token: None,
        }
    }
    
    pub async fn login(&mut self, login: &str, password: &str, societe: &str) -> Result<LoginResponse> {
        let login_url = format!("{}/api/auth/login/Membership", self.auth_base_url);
        
        let login_request = LoginRequest {
            login: login.to_string(),
            password: password.to_string(),
            societe: societe.to_string(),
            commun: Commun {
                duree_token_in_hour: 24,
            },
        };
        
        println!("🔍 URL de login: {}", login_url);
        println!("📤 Enviando request: {:?}", login_request);
        
        let response = self.client
            .post(&login_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "DeliveryOptimizer/1.0")
            .header("Accept", "application/json")
            .header("Origin", "https://gestiontournee.colisprive.com")
            .header("Referer", "https://gestiontournee.colisprive.com/")
            .json(&login_request)
            .send()
            .await?;
            
        let status = response.status();
        println!("📥 Status de respuesta: {}", status);
        println!("📋 Headers de respuesta: {:?}", response.headers());
        
        if !status.is_success() {
            let error_body = response.text().await?;
            anyhow::bail!(
                "Login falló con status: {} - Body: {}",
                status,
                error_body
            );
        }
        
        let login_response: LoginResponse = response.json().await?;
        
        // Guardar el token para futuras requests
        self.sso_token = Some(login_response.tokens.sso_hopps.clone());
        
        Ok(login_response)
    }
    
    pub async fn get_tournee(&self, societe: &str, matricule: &str, date: &str) -> Result<String> {
        let tournee_url = format!("{}/WS-TourneeColis/api/getLettreVoitureEco_POST", self.tournee_base_url);
        
        let sso_token = self.sso_token.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No hay token de autenticación. Haz login primero."))?;
        
        // El token corto puede ser necesario para las requests intermedias
        // Por ahora usamos el token principal, pero podríamos necesitar extraer el shortToken
        let token_for_intermediate = sso_token;
        
        println!("🔍 Activando sesión con requests intermedias...");
        
        // 1. Request intermedia: Pilot access (como hace el navegador)
        let pilot_url = format!("https://ws-gestiontournee-inter.colisprive.com/WS_PilotManagement/api/Pilot/access/{}/{}/FRONT_MOP", matricule, societe);
        println!("📤 Request 1: Pilot access - {}", pilot_url);
        
        let pilot_response = self.client
            .get(&pilot_url)
            .header("SsoHopps", token_for_intermediate)  // ¡Con mayúscula como en el navegador!
            .header("origin", "https://gestiontournee.colisprive.com")
            .header("referer", "https://gestiontournee.colisprive.com/")
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
            .send()
            .await?;
            
        println!("📥 Pilot access status: {}", pilot_response.status());
        
        // 2. Request intermedia: Dashboard info (como hace el navegador)
        let dashboard_url = format!("{}/WS-TourneeColis/api/getBeanInfoDashBoardBySocieteMatriculev2/", self.tournee_base_url);
        println!("📤 Request 2: Dashboard info - {}", dashboard_url);
        
        let dashboard_request = serde_json::json!({
            "Societe": societe,
            "Matricule": matricule,
            "DateDebut": format!("{}T00:00:00.000Z", date),  // Formato exacto del navegador
            "Agence": null,
            "Concentrateur": null
        });
        
        println!("📋 Dashboard request body: {}", serde_json::to_string_pretty(&dashboard_request).unwrap());
        println!("🔑 Token para Dashboard: {}", token_for_intermediate);
        
        let dashboard_response = self.client
            .post(&dashboard_url)
            .header("SsoHopps", token_for_intermediate)  // ¡Con mayúscula como en el navegador!
            .header("Content-Type", "application/json")  // ¡Con mayúscula como en el navegador!
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "fr-FR,fr;q=0.5")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("origin", "https://gestiontournee.colisprive.com")
            .header("referer", "https://gestiontournee.colisprive.com/")
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
            .header("sec-ch-ua", "\"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"")
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", "\"macOS\"")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-site")
            .header("sec-gpc", "1")
            .json(&dashboard_request)
            .send()
            .await?;
            
        let dashboard_status = dashboard_response.status();
        println!("📥 Dashboard info status: {}", dashboard_status);
        
        // Si falla, mostrar más detalles
        if !dashboard_status.is_success() {
            let error_body = dashboard_response.text().await?;
            println!("🚨 Dashboard info falló con status: {} - Body: {}", dashboard_status, error_body);
            println!("🔍 Esto sugiere que hay algo más que necesitamos para las requests intermedias");
        }
        
        // 3. Ahora sí, la request final de tournée
        println!("🚀 Activando request final de tournée...");
        
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
        
        // SOLO enviar los headers que el servidor acepta según el preflight
        println!("📋 Headers que se envían (solo los permitidos):");
        println!("   Content-Type: application/json");
        println!("   SsoHopps: {}", sso_token);
        println!("   origin: https://gestiontournee.colisprive.com");
        println!("   user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36");
        
        let response = self.client
            .post(&tournee_url)
            .header("Content-Type", "application/json")  // ¡Con mayúscula como en el navegador!
            .header("SsoHopps", sso_token)  // ¡Con mayúscula como en el navegador!
            .header("origin", "https://gestiontournee.colisprive.com")  // Agregar origin
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
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
