use serde::{Deserialize, Serialize};

// Struct para el campo commun del login
#[derive(Serialize, Debug)]
pub struct Commun {
    #[serde(rename = "dureeTokenInHour")]
    pub duree_token_in_hour: i32,
}

// Struct para login request - CORREGIDO con campos correctos
#[derive(Serialize, Debug)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
    pub societe: String,
    pub commun: Commun,
}

// Struct para tokens en la respuesta de login
#[derive(Deserialize)]
pub struct Tokens {
    #[serde(rename = "SsoHopps")]
    pub sso_hopps: String,
}

// Struct para login response (solo campos importantes)
#[derive(Deserialize)]
pub struct LoginResponse {
    pub tokens: Tokens,
    pub matricule: String,
    pub societe: String,
}

// Struct para parámetros de tournée
#[derive(Serialize, Debug)]
pub struct TourneeParams {
    #[serde(rename = "Societe")]
    pub societe: String,
    #[serde(rename = "Matricule")]
    pub matricule: String,
    #[serde(rename = "DateDebut")]
    pub date_debut: String,
}

// Struct para tournée request
#[derive(Serialize, Debug)]
pub struct TourneeRequest {
    #[serde(rename = "enumTypeLettreVoiture")]
    pub enum_type_lettre_voiture: String,
    #[serde(rename = "beanParamsMatriculeDateDebut")]
    pub bean_params: TourneeParams,
}

// Struct para representar un paquete de delivery (placeholder para futuras implementaciones)
#[derive(Debug)]
pub struct Delivery {
    pub tracking_number: String,
    pub address: String,
    pub weight: f64,
    pub status: String,
}
