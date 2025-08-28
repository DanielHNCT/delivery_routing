use serde::{Deserialize, Serialize};

// 🎯 MODELOS WEB ESENCIALES PARA COLIS PRIVÉ
// Solo los modelos necesarios para la API web

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
    pub societe: String,
    pub commun: Commun,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commun {
    pub dureeTokenInHour: i32,
}

// Modelo para autenticación web
#[derive(Debug, Serialize, Deserialize)]
pub struct ColisPriveWebAuthRequest {
    pub login: String,
    pub password: String,
    pub societe: String,
    pub commun: Commun,
}

// Modelo para respuesta de autenticación web
#[derive(Debug, Serialize, Deserialize)]
pub struct ColisPriveWebAuthResponse {
    pub isAuthentif: bool,
    pub identity: String,
    pub matricule: String,
    pub societe: String,
    pub tokens: ColisPriveTokens,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColisPriveTokens {
    pub SsoHopps: String,
}

// Modelo para request de tournée web
#[derive(Debug, Serialize, Deserialize)]
pub struct ColisPriveWebTourneeRequest {
    pub Societe: String,
    pub Matricule: String,
    pub DateDebut: String,
    pub Agence: Option<String>,
    pub Concentrateur: Option<String>,
}

// Modelo para respuesta de tournée web
#[derive(Debug, Serialize, Deserialize)]
pub struct ColisPriveWebTourneeResponse {
    pub statut: String,
    pub date: String,
    pub beanToday: BeanToday,
    pub listBeanDistributeur: Vec<BeanDistributeur>,
    pub listBeanLocalite: Vec<BeanLocalite>,
    pub listBeanTournee: Vec<BeanTournee>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeanToday {
    pub date: String,
    pub nbColis: i32,
    pub nbColisCollecte: i32,
    pub nbColisPremium: i32,
    pub nbNonAttribue: i32,
    pub nbCollecteNonAttribue: i32,
    pub nbDistribue: i32,
    pub nbNonAttribuePremium: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeanDistributeur {
    pub matriculeDistributeur: String,
    pub nomDistributeur: String,
    pub isColisAffecte: bool,
    pub dureeJourneeInMinute: i32,
    pub nbPauseEnMinutes: i32,
    pub DateDebutTournee: String,
    pub DateDebutPause: String,
    pub DateFinPause: String,
    pub nbColisMaxByDay: i32,
    pub beanAlerte: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeanLocalite {
    pub codePostal: String,
    pub libelleLocalite: String,
    pub nbColis: i32,
    pub isHasColis: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeanTournee {
    pub codeTournee: String,
    pub codeTourneeMCP: String,
    pub statutTournee: String,
    pub listBeanLocalite: Vec<BeanLocalite>,
    pub nbColis: i32,
    pub nbColisACollecter: i32,
    pub nbColisCollecte: i32,
    pub nbColisPremium: i32,
    pub nbColisRestantPremiumADistribue: i32,
    pub beanDistributeur: BeanDistributeur,
    pub nbColisDistribue: i32,
    pub nbColisRestantADistribue: i32,
    pub nbColisTraite: i32,
    pub nbColisTraitePremium: i32,
    pub dureeTourneePrevuInMinute: i32,
    pub dureeTourneeRealiseInMinute: i32,
    pub dureeTourneeRestanteMinutes: i32,
    pub nbColisRelais: i32,
    pub nbColisRelaisPremium: i32,
    pub nbColisCasier: i32,
    pub nbColisCasierPremium: i32,
    pub alerteTourneePreparation: Option<serde_json::Value>,
    pub alerteTourneeDistribution: Option<serde_json::Value>,
    pub codeCentre: String,
    pub codePointConcentration: String,
}
