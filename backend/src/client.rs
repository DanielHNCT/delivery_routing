use crate::external_models::*;
use crate::external_models::ColisPriveCredentials;
use anyhow::Result;
use base64::Engine;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use lazy_static::lazy_static;
use tracing::{info, error};

// Estructura para manejar sesiones persistentes por usuario
#[derive(Clone, Debug)]
struct UserSession {
    activity_id: String,
    sso_hopps: Option<String>,
    initial_token: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    last_used: chrono::DateTime<chrono::Utc>,
}

impl UserSession {
    fn new() -> Self {
        Self {
            activity_id: uuid::Uuid::new_v4().to_string(),
            sso_hopps: None,
            initial_token: None,
            created_at: chrono::Utc::now(),
            last_used: chrono::Utc::now(),
        }
    }

    fn is_expired(&self) -> bool {
        let now = chrono::Utc::now();
        let age = now.signed_duration_since(self.last_used);
        age.num_hours() > 24 // Expirar sesiones después de 24h
    }

    fn touch(&mut self) {
        self.last_used = chrono::Utc::now();
    }
}

// Cache global de sesiones por usuario
lazy_static! {
    static ref USER_SESSIONS: Arc<RwLock<HashMap<String, UserSession>>> = 
        Arc::new(RwLock::new(HashMap::new()));
}

// Obtener o crear sesión para un usuario
async fn get_or_create_session(matricule: &str) -> UserSession {
    let mut sessions = USER_SESSIONS.write().await;
    
    // Limpiar sesiones expiradas
    sessions.retain(|_, session| !session.is_expired());
    
    // Obtener sesión existente o crear nueva
    let session = sessions.entry(matricule.to_string())
        .or_insert_with(UserSession::new);
    
    session.touch();
    session.clone()
}

// Actualizar sesión con nuevos tokens
async fn update_session(matricule: &str, initial_token: Option<String>, sso_hopps: Option<String>) {
    let mut sessions = USER_SESSIONS.write().await;
    
    if let Some(session) = sessions.get_mut(matricule) {
        if let Some(token) = initial_token {
            session.initial_token = Some(token);
        }
        if let Some(hopps) = sso_hopps {
            session.sso_hopps = Some(hopps);
        }
        session.touch();
    }
}

pub struct ColisPriveClient {
    client: Client,
    auth_base_url: String,
    tournee_base_url: String,
    sso_token: Option<String>,
}

impl ColisPriveClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder()
                .http1_only()
                .http1_title_case_headers()
                .cookie_store(true)
                .build()?,
            auth_base_url: "https://wsauthentificationexterne.colisprive.com".to_string(),
            tournee_base_url: "https://wstournee-v2.colisprive.com".to_string(),
            sso_token: None,
        })
    }

    // Autenticación con sesión persistente
    pub async fn authenticate_with_persistent_session(
        &mut self,
        matricule: &str,
        password: &str,
        societe: &str
    ) -> Result<UserSession, anyhow::Error> {
        
        println!("🔐 INICIO: authenticate_with_persistent_session");
        println!("   Matricule: {}", matricule);
        println!("   Societe: {}", societe);
        println!("   Password length: {}", password.len());
        
        // Obtener sesión persistente
        let mut session = get_or_create_session(matricule).await;
        
        println!("🔑 Usando ActivityId persistente: {}", session.activity_id);
        
        // PASO 1: Login inicial (usar MISMO ActivityId)
        println!("🔐 PASO 1: Login inicial con Colis Privé...");
        let login_request = json!({
            "login": matricule,
            "password": password,
            "societe": societe,
            "commun": { "dureeTokenInHour": 24 }
        });

        println!("📤 Login request body: {}", serde_json::to_string_pretty(&login_request).unwrap());

        let response = self.client.post("https://wsauthentificationexterne.colisprive.com/api/auth/login/Membership")
            .header("Content-Type", "application/json")
            .header("ActivityId", &session.activity_id)  // MISMO ActivityId
            .header("AppName", "CP DISTRI V2")
            .header("UserName", "")
            .header("AppIdentifier", "com.danem.cpdistriv2")
            .header("Device", "Sony D5503")
            .header("VersionOS", "5.1.1")
            .header("VersionApplication", "3.3.0.9")
            .header("VersionCode", "1")
            .header("Societe", "")
            .header("Domaine", "")
            .json(&login_request)
            .send().await?;

        println!("📥 Login response status: {}", response.status());
        
        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await?;
            println!("❌ Login falló con status: {} - Body: {}", status, error_body);
            anyhow::bail!("Login falló con status: {} - Body: {}", status, error_body);
        }

        // Parsear respuesta tipada y extraer token inicial de forma segura
        println!("🔍 Parseando respuesta de login...");
        let login_parsed: LoginResponse = response.json().await?;
        println!("✅ LoginResponse parseado exitosamente");
        println!("   isAuthentif: {}", login_parsed.isAuthentif);
        println!("   shortToken.SsoHopps length: {}", login_parsed.shortToken.SsoHopps.len());
        println!("   tokens.SsoHopps length: {}", login_parsed.tokens.SsoHopps.len());
        
        let initial_token = if !login_parsed.shortToken.SsoHopps.is_empty() {
            login_parsed.shortToken.SsoHopps.clone()
        } else {
            login_parsed.tokens.SsoHopps.clone()
        };
        
        println!("🔑 Token inicial extraído, length: {}", initial_token.len());
        
        // PASO 2: Intercambio por SsoHopps (MISMO ActivityId)
        println!("🔐 PASO 2: Intercambio por SsoHopps...");
        let token_exchange = json!({
            "dureeTokenInHour": 0,
            "token": initial_token
        });

        println!("📤 Token exchange body: {}", serde_json::to_string_pretty(&token_exchange).unwrap());

        let sso_response = self.client.post("https://wsauthentificationexterne.colisprive.com/api/auth/login-token")
            .header("Content-Type", "application/json")
            .header("ActivityId", &session.activity_id)  // CRÍTICO: MISMO ActivityId
            .header("AppName", "CP DISTRI V2")
            .header("UserName", "")  // Vacío en intercambio
            .header("AppIdentifier", "com.danem.cpdistriv2")
            .header("Device", "Sony D5503")
            .header("VersionOS", "5.1.1")
            .header("VersionApplication", "3.3.0.9")
            .header("VersionCode", "1")
            .header("Societe", "")  // Vacío en intercambio
            .header("Domaine", "")
            .json(&token_exchange)
            .send().await?;

        println!("📥 SsoHopps response status: {}", sso_response.status());
        
        if !sso_response.status().is_success() {
            let status = sso_response.status();
            let error_body = sso_response.text().await?;
            println!("❌ SsoHopps exchange falló con status: {} - Body: {}", status, error_body);
            anyhow::bail!("SsoHopps exchange falló con status: {} - Body: {}", status, error_body);
        }

        let sso_data: serde_json::Value = sso_response.json().await?;
        println!("🔍 SsoHopps response data: {}", serde_json::to_string_pretty(&sso_data).unwrap());
        
        // ✅ CORRECCIÓN CLAUDE: USAR shortToken en lugar de tokens (según logs exitosos)
        let sso_hopps = match sso_data["shortToken"]["SsoHopps"]
            .as_str()
            .or_else(|| sso_data["tokens"]["SsoHopps"].as_str()) {
            Some(s) => s.to_string(),
            None => anyhow::bail!("No SsoHopps found in response"),
        };

        println!("🔑 SsoHopps extraído, length: {}", sso_hopps.len());

        // Actualizar sesión con nuevos tokens
        update_session(matricule, Some(initial_token), Some(sso_hopps.clone())).await;
        
        session.sso_hopps = Some(sso_hopps);
        
        println!("✅ Sesión persistente actualizada para {}", matricule);
        Ok(session)
    }

    pub async fn login(&mut self, login: &str, password: &str, societe: &str) -> Result<LoginResponse> {
        // Usar autenticación con sesión persistente
        let session = self.authenticate_with_persistent_session(login, password, societe).await?;
        
        // Crear respuesta de login compatible
        let login_response = LoginResponse {
            infoConsolidee: "Session authenticated".to_string(),
            isAuthentif: true,
            accountExpirationDate: None,
            roleSGBD: vec![],
            roleSI: None,
            identity: login.to_string(),
            isAdminMetier: false,
            isAdminIndiana: false,
            matricule: login.to_string(),
            nom: None,
            prenom: None,
            codeAnalytique: None,
            domaine: None,
            tenant: societe.to_string(),
            societe: societe.to_string(),
            libelleSociete: societe.to_string(),
            typeClient: None,
            habilitationAD: HabilitationAD { SsoHopps: vec![] },
            habilitationInterprete: serde_json::Value::Null,
            roles: vec![],
            tokens: Tokens { SsoHopps: session.sso_hopps.unwrap_or_default() },
            shortToken: ShortToken { SsoHopps: session.initial_token.unwrap_or_default() },
            profilUtilisateur: vec![],
        };
        
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

    /// Obtener tournée usando el endpoint móvil real de Colis Privé con sesión persistente
    pub async fn get_mobile_tournee(
        &self,
        credentials: &ColisPriveCredentials,
        date: &str,
        matricule: &str,
        token: &str,
    ) -> Result<Vec<crate::external_models::MobilePackageAction>, Box<dyn std::error::Error>> {
        // Obtener sesión persistente para este usuario
        let session = get_or_create_session(matricule).await;
        
        if session.sso_hopps.is_none() {
            return Err("Session not authenticated - need to call authenticate_with_persistent_session first".into());
        }
        // Usar SIEMPRE el token de la sesión para que coincida con el ActivityId persistente
        let token_to_use = session.sso_hopps.as_deref().unwrap();
        
        let username_tournee = matricule.split('_').last().unwrap_or(matricule);
        
        let body = serde_json::json!({
            "DateDebut": date,
            "Matricule": matricule
        });

        println!("=== DEBUG ENDPOINT MÓVIL CON SESIÓN PERSISTENTE ===");
        println!("🚀 Llamando endpoint móvil de Colis Privé...");
        println!("📱 URL: https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST");
        println!("🔑 Token SsoHopps: {}", token_to_use);
        println!("🔑 Token length: {}", token_to_use.len());
        println!("🆔 ActivityId persistente: {}", session.activity_id);
        println!("📅 Fecha: {}", date);
        println!("🆔 Matrícula: {}", matricule);
        println!("👤 Username (tournee): {}", username_tournee);
        println!("🏢 Societe: {}", credentials.societe);
        
        // IMPORTANTE: Esperar un momento para que el token se "active" en el servidor
        println!("⏳ Esperando 2 segundos para activación del token...");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("✅ Delay completado, procediendo con request...");

        // ✅ CORRECCIÓN CLAUDE: Headers EXACTOS como la app oficial exitosa
        info!("🔧 CORRIGIENDO HEADERS SEGÚN LOGS EXITOSOS");
        info!("   UserName: {} (solo tournée)", username_tournee);
        info!("   Societe: PCP0010699 (NO vacío)");
        info!("   Domaine: Membership (NO vacío)");
        
        let request_builder = self.client
            .post("https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST")
            .header("Accept-Charset", "UTF-8")
            .header("ActivityId", &session.activity_id)  // MISMO ActivityId SIEMPRE
            .header("AppName", "CP DISTRI V2")
            .header("UserName", username_tournee)  // ✅ SOLO "A187518" 
            .header("AppIdentifier", "com.danem.cpdistriv2")
            .header("Device", "Sony D5503")
            .header("VersionOS", "5.1.1")
            .header("VersionApplication", "3.3.0.9")
            .header("VersionCode", "1")
            .header("Societe", "PCP0010699")  // ✅ NO vacío
            .header("Domaine", "Membership")  // ✅ "Membership", NO vacío
            .header("SsoHopps", token_to_use)  // ÚNICO token de autenticación para tournée
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("Content-Length", &serde_json::to_string(&body).unwrap().len().to_string())
            .header("Host", "wstournee-v2.colisprive.com")
            .header("Connection", "Keep-Alive")
            .header("Accept-Encoding", "gzip")
            .header("User-Agent", "okhttp/3.4.1")
            .json(&body);
        
        // ✅ Logging detallado con headers corregidos
        info!("📤 Headers enviados:");
        info!("   Accept-Charset: UTF-8");
        info!("   ActivityId: {} (PERSISTENTE)", session.activity_id);
        info!("   AppName: CP DISTRI V2");
        info!("   UserName: {} (solo tournée)", username_tournee);
        info!("   AppIdentifier: com.danem.cpdistriv2");
        info!("   Device: Sony D5503");
        info!("   VersionOS: 5.1.1");
        info!("   VersionApplication: 3.3.0.9");
        info!("   VersionCode: 1");
        info!("   Societe: PCP0010699 (NO vacío)");
        info!("   Domaine: Membership (NO vacío)");
        info!("   SsoHopps: {} (length: {})", token_to_use, token_to_use.len());
        info!("   ❌ Authorization: Basic NO enviado (solo para logs/mobilité)");
        info!("   Content-Type: application/json; charset=UTF-8");
        info!("   Content-Length: {}", serde_json::to_string(&body).unwrap().len());
        info!("   Host: wstournee-v2.colisprive.com");
        info!("   Connection: Keep-Alive");
        info!("   Accept-Encoding: gzip");
        info!("   User-Agent: okhttp/3.4.1");
        println!("📦 Body JSON: {}", serde_json::to_string_pretty(&body).unwrap());
        println!("================================");

        let response = request_builder.send().await?;

        let status = response.status();
        println!("📥 Status de respuesta móvil: {}", status);
        println!("📥 Headers de respuesta:");
        
        // Logging de headers de respuesta
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                println!("   {}: {}", name, value_str);
            }
        }
        println!("================================");

        if !status.is_success() {
            let error_body = response.text().await?;
            info!("❌ Error en endpoint móvil:");
            info!("   Status: {}", status);
            info!("   Body: {}", error_body);
            
            if status == 401 {
                error!("❌ 401 - Headers o token inválidos");
                // ✅ Log headers enviados para debug según Claude
                info!("Headers enviados:");
                info!("  UserName: {} (solo tournée)", username_tournee);
                info!("  Societe: PCP0010699");
                info!("  Domaine: Membership");
                info!("  ActivityId: {}", session.activity_id);
                info!("  SsoHopps: {}...", &token_to_use[..50]);
                
                let mut sessions = USER_SESSIONS.write().await;
                sessions.remove(matricule);
                return Err("Authentication failed with corrected headers".into());
            }
            
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Endpoint móvil falló con status: {} - Body: {}", status, error_body)
            )));
        }

        let mobile_data: Vec<crate::external_models::MobilePackageAction> = response.json().await?;
        println!("✅ Datos móviles obtenidos exitosamente: {} paquetes", mobile_data.len());
        
        Ok(mobile_data)
    }
}
