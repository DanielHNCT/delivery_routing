use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
    pub societe: String,
    pub commun: Commun,
}

// NUEVO: Estructura EXACTA de la app oficial de Colis Privé
#[derive(Debug, Serialize, Deserialize)]
pub struct ColisPriveOfficialLoginRequest {
    pub audit: serde_json::Value,
    pub commun: ColisPriveCommun,
    pub login: String,  // Con espacio al final como en la app oficial
    pub password: String,
    pub societe: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColisPriveCommun {
    pub dureeTokenInHour: i32,  // Exacto como en la app oficial
}

// NUEVO: Sistema completo de versiones de la app
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionCheckRequest {
    pub username: String,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionCheckResponse {
    pub has_update: bool,
    pub version: String,
    pub download_url: Option<String>,
    pub binary_id: Option<String>,
    pub is_mandatory: bool,
    pub changelog: Option<String>,
    pub file_size: Option<u64>,
    pub checksum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditInstallRequest {
    pub version: String,
    pub device_info: DeviceInfo,
    pub install_result: String,
    pub binary_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditInstallResponse {
    pub success: bool,
    pub message: String,
    pub audit_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppVersion {
    pub version: String,
    pub binary_id: String,
    pub download_date: String,
    pub apk_path: String,
    pub file_size: u64,
    pub checksum: String,
    pub reverse_engineering_status: String,
    pub analysis_report: Option<serde_json::Value>,
}

// NUEVO: Device Audit - EXACTO como la app oficial
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceAuditRequest {
    pub device_cpu: String,
    pub device_disk: String,
    pub device_id_device: String,
    pub device_langue: String,
    pub device_os: String,
    pub device_ram: String,
    pub device_version: String,
    pub id_externe_application: String,
    pub is_install_ok: bool,
    pub matricule: String,
    pub num_application_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceAuditResponse {
    pub success: bool,
    pub message: String,
    pub audit_id: Option<String>,
}

// NUEVO: Version Check - EXACTO como la app oficial
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionCheckRealRequest {
    pub username: String,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionCheckRealResponse {
    pub has_update: bool,
    pub version: String,
    pub download_url: Option<String>,
    pub binary_id: Option<String>,
    pub is_mandatory: bool,
    pub changelog: Option<String>,
}

// NUEVO: Logging automático - EXACTO como la app oficial
#[derive(Debug, Serialize, Deserialize)]
pub struct LogMobiliteRequest {
    pub app_name: String,
    pub indiana_version: String,
    pub date_logged: String,
    pub dns_host_name: String,
    pub exception: String,
    pub ip_address: String,
    pub log_level: String,
    pub logger: String,
    pub memory: String,
    pub message: String,
    pub parameters: String,
    pub screen_name: String,
    pub session_id: String,
    pub thread: String,
    pub trace: String,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMobiliteResponse {
    pub success: bool,
    pub message: String,
    pub log_id: Option<String>,
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
    // Campos existentes
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
    #[serde(rename = "CoOrigineCreation")]
    pub co_origine_creation: String,

    // NUEVOS CAMPOS CAPTURADOS
    // Coordenadas GPS
    #[serde(rename = "coordXGPSCptRendu")]
    pub coord_x_gps_cpt_rendu: Option<f64>,
    #[serde(rename = "coordYGPSCptRendu")]
    pub coord_y_gps_cpt_rendu: Option<f64>,
    #[serde(rename = "gpsQualite")]
    pub gps_qualite: Option<String>,
    
    // Duración y orden
    #[serde(rename = "dureeSecondePrevueAction")]
    pub duree_seconde_prevue_action: Option<f64>,
    #[serde(rename = "numOrdreAction")]
    pub num_ordre_action: Option<u32>,
    #[serde(rename = "numOrdreCptRendu")]
    pub num_ordre_cpt_rendu: Option<u32>,
    
    // Timestamps
    #[serde(rename = "horodatageCptRendu")]
    pub horodatage_cpt_rendu: Option<String>,
    #[serde(rename = "valeurAttenduCptRendu")]
    pub valeur_attendu_cpt_rendu: Option<String>,
    
    // Estados de transmisión
    #[serde(rename = "VFTransmisSITiers")]
    pub vf_transmis_si_tiers: Option<bool>,
    #[serde(rename = "DateTransmisSiTiers")]
    pub date_transmis_si_tiers: Option<String>,
    
    // Campos adicionales de seguimiento
    #[serde(rename = "idCptRendu")]
    pub id_cpt_rendu: Option<String>,
    #[serde(rename = "codeCleCptRendu")]
    pub code_cle_cpt_rendu: Option<String>,
    #[serde(rename = "codeTypeCptRendu")]
    pub code_type_cpt_rendu: Option<String>,
    #[serde(rename = "valeurCptRendu")]
    pub valeur_cpt_rendu: Option<String>,
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

// Nuevas estructuras para flujo completo de autenticación
#[derive(Serialize)]
pub struct ColisLoginRequest {
    pub audit: AuditData,
    pub commun: CommunData,
    pub login: String,
    pub password: String,
    pub societe: String,
}

#[derive(Serialize)]
pub struct AuditData {
    #[serde(rename = "appName")]
    pub app_name: String,
    pub cle1: String,
    pub cle2: String,
    pub cle3: String,
    #[serde(rename = "deviceModelName")]
    pub device_model_name: String,
    pub iccid: String,
    pub imei: String,
    pub msisdn: String,
    #[serde(rename = "noSerie")]
    pub no_serie: String,
}

#[derive(Serialize)]
pub struct CommunData {
    #[serde(rename = "dureeTokenInHour")]
    pub duree_token_in_hour: u32,
}

// Estructuras duplicadas eliminadas - ya definidas arriba

// Estructura para respuesta de login actualizada
#[derive(Deserialize)]
pub struct ColisLoginResponse {
    #[serde(rename = "isAuthentif")]
    pub is_ok: bool,
    pub code: u32,
    pub duration: Option<u32>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    #[serde(rename = "errorBody")]
    pub error_body: Option<String>,
    pub data: Option<serde_json::Value>,
    #[serde(rename = "titreFromBean")]
    pub titre_from_bean: Option<String>,
    #[serde(rename = "errorMessageFromBean")]
    pub error_message_from_bean: Option<String>,
    pub exception: Option<String>,
}

// Estructuras para sistema de tokens con auto-refresh
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequestLegacy {
    #[serde(rename = "dureeTokenInHour")]
    pub duree_token_in_hour: u32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeRequestWithToken {
    pub username: String,
    pub password: String,
    pub societe: String,
    pub date: String,
    pub matricule: String,
    pub token: Option<String>, // Token opcional para retry
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColisAuthResponse {
    #[serde(rename = "infoConsolidee")]
    pub info_consolidee: Option<String>,
    #[serde(rename = "isAuthentif")]
    pub is_authentif: bool,
    #[serde(rename = "accountExpirationDate")]
    pub account_expiration_date: Option<String>,
    #[serde(rename = "roleSGBD")]
    pub role_sgbd: Option<Vec<String>>,
    #[serde(rename = "roleSI")]
    pub role_si: Option<String>,
    pub identity: Option<String>,
    #[serde(rename = "isAdminMetier")]
    pub is_admin_metier: bool,
    #[serde(rename = "isAdminIndiana")]
    pub is_admin_indiana: bool,
    pub matricule: Option<String>,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    #[serde(rename = "codeAnalytique")]
    pub code_analytique: Option<String>,
    pub domaine: Option<String>,
    pub tenant: Option<String>,
    pub societe: Option<String>,
    #[serde(rename = "libelleSociete")]
    pub libelle_societe: Option<String>,
    #[serde(rename = "typeClient")]
    pub type_client: Option<String>,
    #[serde(rename = "habilitationAD")]
    pub habilitation_ad: Option<serde_json::Value>,
    #[serde(rename = "habilitationInterprete")]
    pub habilitation_interprete: Option<serde_json::Value>,
    pub roles: Option<Vec<String>>,
    pub tokens: TokenData,
    #[serde(rename = "shortToken")]
    pub short_token: TokenData,
    #[serde(rename = "profilUtilisateur")]
    pub profil_utilisateur: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    #[serde(rename = "SsoHopps")]
    pub sso_hopps: String,
}

/// Información del dispositivo Android para headers dinámicos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Modelo del dispositivo (ej: "Samsung SM-S916B", "Google Pixel 7")
    pub model: String,
    /// IMEI del dispositivo (real o fake consistente)
    pub imei: String,
    /// Número de serie del dispositivo
    pub serial_number: String,
    /// Versión de Android (ej: "13", "14")
    pub android_version: String,
    /// ID único de instalación de la app
    pub install_id: String,
}

/// Request de autenticación con Colis Privé incluyendo device info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColisAuthRequest {
    pub username: String,
    pub password: String,
    pub societe: String,
    /// Información del dispositivo para headers dinámicos
    pub device_info: DeviceInfo,
}

/// Request de refresh token con device info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub token: String,
    /// Información del dispositivo para headers dinámicos
    pub device_info: DeviceInfo,
}

/// Request de tournée con auto-retry incluyendo device info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TourneeRequestWithRetry {
    pub username: String,
    pub password: String,
    pub societe: String,
    pub date: String,
    pub matricule: String,
    /// Token opcional (si no hay, se hace login automático)
    pub token: Option<String>,
    /// Información del dispositivo para headers dinámicos
    pub device_info: DeviceInfo,
    /// 🆕 NUEVO: Tipo de API a usar ("web" o "mobile")
    #[serde(rename = "api_choice")]
    pub api_choice: Option<String>,
}
