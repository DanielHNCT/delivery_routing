use serde::{Deserialize, Serialize};

/// Modelos para la API Web real de Colis Privé
/// Basados en las respuestas reales capturadas del tráfico

// ============================================================================
// LOGIN WEB API
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginRequest {
    pub login: String,
    pub password: String,
    pub societe: String,
    pub commun: WebLoginCommun,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginCommun {
    pub dureeTokenInHour: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebLoginResponse {
    pub success: bool,
    pub message: Option<String>,
    pub sso_hopps: Option<String>,
    pub session_id: Option<String>,
}

// ============================================================================
// TOURNÉE DASHBOARD
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct WebTourneeRequest {
    pub Societe: String,
    pub Matricule: String,
    pub DateDebut: String,
    pub Agence: Option<String>,
    pub Concentrateur: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebTourneeResponse {
    pub success: bool,
    pub data: Option<Vec<WebPackage>>,
    pub message: Option<String>,
    pub total_packages: Option<i32>,
    pub total_weight: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebPackage {
    pub id: String,
    pub reference: String,
    pub barcode: String,
    pub weight: f64,
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub status: String,
    pub coordinates: Option<WebCoordinates>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

// ============================================================================
// LETRA DE VOZ (LETTER VOITURE)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct WebLetterRequest {
    pub enumTypeLettreVoiture: String,
    pub beanParamsMatriculeDateDebut: WebLetterParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebLetterParams {
    pub Societe: String,
    pub Matricule: String,
    pub DateDebut: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebLetterResponse {
    pub success: bool,
    pub data: Option<String>, // Base64 encoded letter content
    pub message: Option<String>,
}

// ============================================================================
// HEADERS WEB API
// ============================================================================

#[derive(Debug, Clone)]
pub struct WebApiHeaders {
    pub sso_hopps: String,
    pub user_agent: String,
    pub accept: String,
    pub content_type: String,
    pub origin: String,
    pub referer: String,
}

impl Default for WebApiHeaders {
    fn default() -> Self {
        Self {
            sso_hopps: String::new(),
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36".to_string(),
            accept: "application/json, text/plain, */*".to_string(),
            content_type: "application/json".to_string(),
            origin: "https://gestiontournee.colisprive.com".to_string(),
            referer: "https://gestiontournee.colisprive.com/".to_string(),
        }
    }
}
