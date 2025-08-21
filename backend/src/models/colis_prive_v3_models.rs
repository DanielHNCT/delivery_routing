use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Modelos para Colis Privé API v3.3.0.9 - Flujo Completo Verificado

// ============================================================================
// PASO 1: DEVICE AUDIT - Registro de dispositivo
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceAuditRequest {
    #[serde(rename = "DeviceInfo")]
    pub device_info: DeviceInfo,
    #[serde(rename = "AppInfo")]
    pub app_info: AppInfo,
    #[serde(rename = "InstallId")]
    pub install_id: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    #[serde(rename = "imei")]
    pub imei: String,
    #[serde(rename = "android_id")]
    pub android_id: String,
    #[serde(rename = "android_version")]
    pub android_version: String,
    #[serde(rename = "brand")]
    pub brand: String,
    #[serde(rename = "device")]
    pub device: String,
    #[serde(rename = "hardware")]
    pub hardware: String,
    #[serde(rename = "install_id")]
    pub install_id: String,
    #[serde(rename = "manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "product")]
    pub product: String,
    #[serde(rename = "serial_number")]
    pub serial_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppInfo {
    #[serde(rename = "AppIdentifier")]
    pub app_identifier: String,
    #[serde(rename = "VersionName")]
    pub version_name: String,
    #[serde(rename = "VersionCode")]
    pub version_code: String,
    #[serde(rename = "Societe")]
    pub societe: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceAuditResponse {
    pub success: bool,
    pub message: Option<String>,
    #[serde(rename = "SsoHopps")]
    pub sso_hopps: Option<String>,
    #[serde(rename = "SessionId")]
    pub session_id: Option<String>,
}

// ============================================================================
// PASO 2: VERSION CHECK - Verificación de versión
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionCheckResponse {
    pub success: bool,
    pub version_valid: bool,
    pub message: Option<String>,
    #[serde(rename = "SsoHopps")]
    pub sso_hopps: Option<String>,
    #[serde(rename = "Config")]
    pub config: Option<AppConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub endpoints: Option<EndpointConfig>,
    pub features: Option<FeatureConfig>,
    pub timeout_settings: Option<TimeoutConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub auth_url: Option<String>,
    pub tournee_url: Option<String>,
    pub log_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub offline_mode: Option<bool>,
    pub auto_sync: Option<bool>,
    pub photo_required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeoutConfig {
    pub auth_timeout: Option<u32>,
    pub data_timeout: Option<u32>,
    pub retry_attempts: Option<u32>,
}

// ============================================================================
// PASO 3: LOGIN PRINCIPAL - Autenticación con token SsoHopps
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenRequest {
    #[serde(rename = "dureeTokenInHour")]
    pub duree_token_in_hour: u32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenResponse {
    pub success: bool,
    pub message: Option<String>,
    #[serde(rename = "AuthToken")]
    pub auth_token: Option<String>,
    #[serde(rename = "Matricule")]
    pub matricule: Option<String>,
    #[serde(rename = "UserInfo")]
    pub user_info: Option<UserInfo>,
    #[serde(rename = "SessionInfo")]
    pub session_info: Option<SessionInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub matricule: String,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub societe: String,
    pub role: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub expires_at: Option<String>,
    pub refresh_token: Option<String>,
    pub last_activity: Option<String>,
}

// ============================================================================
// PASO 4: LOGGING AUTOMÁTICO - Confirmación de sesión
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMobiliteRequest {
    #[serde(rename = "Matricule")]
    pub matricule: String,
    #[serde(rename = "TypeLog")]
    pub type_log: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "DeviceInfo")]
    pub device_info: DeviceInfo,
    #[serde(rename = "SessionId")]
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMobiliteResponse {
    pub success: bool,
    pub message: Option<String>,
    #[serde(rename = "LogId")]
    pub log_id: Option<String>,
    #[serde(rename = "ServerTimestamp")]
    pub server_timestamp: Option<String>,
}

// ============================================================================
// TOURNÉES - Obtención de datos después del login
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeRequestV3 {
    #[serde(rename = "DateDebut")]
    pub date_debut: String,
    #[serde(rename = "Matricule")]
    pub matricule: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeResponseV3 {
    pub success: bool,
    pub message: Option<String>,
    #[serde(rename = "Tournees")]
    pub tournees: Option<Vec<TourneeV3>>,
    #[serde(rename = "TotalPackages")]
    pub total_packages: Option<u32>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<TourneeMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeV3 {
    #[serde(rename = "TourneeId")]
    pub tournee_id: String,
    #[serde(rename = "DateTournee")]
    pub date_tournee: String,
    #[serde(rename = "Statut")]
    pub statut: String,
    #[serde(rename = "Packages")]
    pub packages: Vec<PackageV3>,
    #[serde(rename = "Statistics")]
    pub statistics: Option<TourneeStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageV3 {
    #[serde(rename = "NumColis")]
    pub num_colis: String,
    #[serde(rename = "Destinataire")]
    pub destinataire: DestinataireV3,
    #[serde(rename = "Adresse")]
    pub adresse: AdresseV3,
    #[serde(rename = "Statut")]
    pub statut: String,
    #[serde(rename = "TypeColis")]
    pub type_colis: String,
    #[serde(rename = "Instructions")]
    pub instructions: Option<String>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<PackageMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinataireV3 {
    pub nom: String,
    pub prenom: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub instructions_livraison: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdresseV3 {
    pub rue: String,
    pub ville: String,
    pub code_postal: String,
    pub pays: Option<String>,
    pub coordonnees: Option<Coordonnees>,
    pub complement: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordonnees {
    pub latitude: f64,
    pub longitude: f64,
    pub precision: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeStats {
    pub total_colis: u32,
    pub colis_livres: u32,
    pub colis_en_attente: u32,
    pub colis_echecs: u32,
    pub distance_totale: Option<f32>,
    pub temps_estime: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeMetadata {
    pub last_update: String,
    pub sync_status: String,
    pub version: String,
    pub total_results: u32,
    pub page_size: Option<u32>,
    pub current_page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub priority: Option<String>,
    pub delivery_window: Option<DeliveryWindow>,
    pub special_instructions: Option<Vec<String>>,
    pub photos_required: Option<bool>,
    pub signature_required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryWindow {
    pub start_time: String,
    pub end_time: String,
    pub preferred_time: Option<String>,
}

// ============================================================================
// STRUCTS DE CONTROL DE FLUJO Y ESTADO
// ============================================================================

#[derive(Debug, Clone)]
pub struct FlowState {
    pub step: FlowStep,
    pub sso_hopps: Option<String>,
    pub session_id: Option<String>,
    pub auth_token: Option<String>,
    pub matricule: Option<String>,
    pub device_info: DeviceInfo,
    pub app_info: AppInfo,
    pub activity_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlowStep {
    NotStarted,
    DeviceAuditInProgress,
    DeviceAuditCompleted,
    VersionCheckInProgress,
    VersionCheckCompleted,
    LoginInProgress,
    LoginCompleted,
    LoggingInProgress,
    LoggingCompleted,
    Ready,
    Failed(String),
}

impl FlowState {
    pub fn new(device_info: DeviceInfo, app_info: AppInfo) -> Self {
        let now = Utc::now();
        Self {
            step: FlowStep::NotStarted,
            sso_hopps: None,
            session_id: None,
            auth_token: None,
            matricule: None,
            device_info,
            app_info,
            activity_id: Uuid::new_v4(),
            started_at: now,
            last_update: now,
        }
    }

    pub fn update_step(&mut self, step: FlowStep) {
        self.step = step;
        self.last_update = Utc::now();
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.step, FlowStep::Ready)
    }

    pub fn has_failed(&self) -> bool {
        matches!(self.step, FlowStep::Failed(_))
    }

    pub fn get_error(&self) -> Option<String> {
        match &self.step {
            FlowStep::Failed(error) => Some(error.clone()),
            _ => None,
        }
    }
}

// ============================================================================
// REQUEST/RESPONSE WRAPPERS PARA COMPATIBILIDAD
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteFlowRequest {
    pub username: String,
    pub password: String,
    pub societe: String,
    pub date: String,
    pub device_info: Option<DeviceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteFlowResponse {
    pub success: bool,
    pub message: String,
    pub flow_state: Option<FlowStep>,
    pub auth_data: Option<AuthData>,
    pub tournee_data: Option<TourneeResponseV3>,
    pub timing: FlowTiming,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub sso_hopps: String,
    pub auth_token: Option<String>,
    pub matricule: String,
    pub session_id: String,
    pub user_info: Option<UserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowTiming {
    pub total_duration_ms: u64,
    pub device_audit_ms: Option<u64>,
    pub version_check_ms: Option<u64>,
    pub login_ms: Option<u64>,
    pub logging_ms: Option<u64>,
    pub tournee_fetch_ms: Option<u64>,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            imei: String::new(),
            android_id: String::new(),
            android_version: String::new(),
            brand: String::new(),
            device: String::new(),
            hardware: String::new(),
            install_id: String::new(),
            manufacturer: String::new(),
            model: String::new(),
            product: String::new(),
            serial_number: String::new(),
        }
    }
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            app_identifier: "com.danem.cpdistriv2".to_string(),
            version_name: "3.3.0.9".to_string(),
            version_code: "1".to_string(),
            societe: String::new(),
        }
    }
}