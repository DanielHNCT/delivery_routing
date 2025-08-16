use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
    pub societe: String,
    pub commun: Commun,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commun {
    pub duree_token_in_hour: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub infoConsolidee: String,
    pub isAuthentif: bool,
    pub accountExpirationDate: Option<String>,
    pub roleSGBD: Vec<String>,
    pub roleSI: Option<Vec<String>>,
    pub identity: String,
    pub isAdminMetier: bool,
    pub isAdminIndiana: bool,
    pub matricule: String,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub codeAnalytique: Option<String>,
    pub domaine: Option<String>,
    pub tenant: String,
    pub societe: String,
    pub libelleSociete: String,
    pub typeClient: Option<String>,
    pub habilitationAD: HabilitationAD,
    pub habilitationInterprete: serde_json::Value,
    pub roles: Vec<String>,
    pub tokens: Tokens,
    pub shortToken: ShortToken,
    pub profilUtilisateur: Vec<ProfilUtilisateur>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HabilitationAD {
    pub SsoHopps: Vec<SsoHoppsInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SsoHoppsInfo {
    pub applicatif: String,
    pub role: String,
    pub cle: String,
    pub valeur: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub SsoHopps: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortToken {
    pub SsoHopps: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfilUtilisateur {
    pub matricule: String,
    pub cle: String,
    pub valeur: String,
    pub regroupement: String,
    pub dateDebut: String,
    pub dateFin: String,
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
    pub beanAlerte: Option<String>,
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
    pub alerteTourneePreparation: Option<Vec<Alerte>>,
    pub alerteTourneeDistribution: Option<Vec<Alerte>>,
    pub codeCentre: String,
    pub codePointConcentration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alerte {
    pub codeAlerte: String,
    pub dateAlerte: String,
    pub libelleAlerte: String,
    pub detailAlerte: Option<String>,
}

// Struct para representar un paquete de delivery (placeholder para futuras implementaciones)
#[derive(Debug)]
pub struct Delivery {
    pub tracking_number: String,
    pub address: String,
    pub weight: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeRequest {
    pub enum_type_lettre_voiture: String,
    pub bean_params: TourneeParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeParams {
    pub societe: String,
    pub matricule: String,
    pub date_debut: String,
}
