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

// ===== NUEVOS MODELOS MÓVILES =====

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MobilePackageAction {
    #[serde(rename = "nomDistributeur")]
    pub nom_distributeur: Option<String>,
    #[serde(rename = "matriculeDistributeur")]
    pub matricule_distributeur: String,
    #[serde(rename = "idSocieteDistributrice")]
    pub id_societe_distributrice: u32,
    #[serde(rename = "codeSocieteDistributrice")]
    pub code_societe_distributrice: String,
    #[serde(rename = "codeAgence")]
    pub code_agence: String,
    #[serde(rename = "idLieuArticle")]
    pub id_lieu_article: String,
    #[serde(rename = "codeTourneeMCP")]
    pub code_tournee_mcp: String,
    #[serde(rename = "idArticle")]
    pub id_article: String,
    #[serde(rename = "refExterneArticle")]
    pub ref_externe_article: String,
    #[serde(rename = "codeBarreArticle")]
    pub code_barre_article: String,
    #[serde(rename = "codeSocieteEmetriceArticle")]
    pub code_societe_emetrice_article: String,
    #[serde(rename = "codeSocietePriseEnCharge")]
    pub code_societe_prise_en_charge: String,
    #[serde(rename = "idAction")]
    pub id_action: String,
    #[serde(rename = "codeCleAction")]
    pub code_cle_action: String,
    #[serde(rename = "libelleAction")]
    pub libelle_action: String,
    #[serde(rename = "codeTypeAction")]
    pub code_type_action: String,
    #[serde(rename = "codeAction")]
    pub code_action: String,
    #[serde(rename = "numOrdreAction")]
    pub num_ordre_action: u32,
    #[serde(rename = "CoOrigineCreation")]
    pub co_origine_creation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileTourneeRequest {
    pub username: String,
    pub password: String,
    pub societe: String,
    pub date: String,
    pub matricule: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileTourneeResponse {
    pub success: bool,
    pub data: Option<Vec<MobilePackageAction>>,
    pub message: String,
    pub endpoint_used: String,
    pub total_packages: usize,
}

// Estructura auxiliar para credenciales
#[derive(Debug, Clone)]
pub struct ColisPriveCredentials {
    pub username: String,
    pub password: String,
    pub societe: String,
}
