use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Modelos para Colis Privé API v3.3.0.9 - Flujo Completo Verificado

// ============================================================================
// PASO 1: DEVICE AUDIT - Registro de dispositivo
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceAuditRequest {
    // ✅ REPRODUCCIÓN 100% APK OFICIAL - BeanWSRequestAuditDevice
    #[serde(rename = "deviceDisk")]
    pub device_disk: String,
    
    #[serde(rename = "deviceIdDevice")]
    pub device_id_device: String,
    
    #[serde(rename = "deviceRam")]
    pub device_ram: String,
    
    #[serde(rename = "idExterneApplication")]
    pub id_externe_application: String,
    
    #[serde(rename = "isInstallOK")]
    pub is_install_ok: bool,
    
    #[serde(rename = "numApplicationVersion")]
    pub num_application_version: String,
    
    #[serde(rename = "deviceCPU")]
    pub device_cpu: String,
    
    #[serde(rename = "deviceLangue")]
    pub device_langue: String,
    
    #[serde(rename = "deviceOs")]
    pub device_os: String,
    
    #[serde(rename = "deviceVersion")]
    pub device_version: String,
    
    #[serde(rename = "matricule")]
    pub matricule: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    pub imei: String,
    pub android_id: String,
    pub android_version: String,
    pub brand: String,
    pub device: String,
    pub hardware: String,
    pub install_id: String,
    pub manufacturer: String,
    pub model: String,
    pub product: String,
    pub serial_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppInfo {
    pub app_identifier: String,
    pub version_name: String,
    pub version_code: String,
    pub societe: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceAuditResponse {
    pub success: bool,
    pub message: Option<String>,
    pub sso_hopps: Option<String>,
    pub session_id: Option<String>,
}

// ============================================================================
// PASO 2: VERSION CHECK - Verificación de versión
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionCheckResponse {
    // ✅ REPRODUCCIÓN 100% APK OFICIAL - Respuesta real de Colis Privé
    pub ApplicationVersion_id: u32,         // 0
    pub IsObligatoire: bool,                // true
    pub Action: String,                     // "Remove"
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
    pub duree_token_in_hour: u32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenResponse {
    pub success: bool,
    pub message: Option<String>,
    pub auth_token: Option<String>,
    pub matricule: Option<String>,
    pub sso_hopps: Option<String>, // ✅ Campo para recibir SsoHopps del login
    pub user_info: Option<UserInfo>,
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
// PASO 4: LOGGING AUTOMÁTICO - Confirmación de sesión (REPRODUCCIÓN 100% APP OFICIAL)
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMobiliteRequest {
    // ✅ REPRODUCCIÓN EXACTA DE LA APP OFICIAL
    pub AppName: String,                    // "CP DISTRI V2"
    pub IndianaVersion: String,             // "3.3.0.9"
    pub DateLogged: String,                 // "/Date(1755716094136)/"
    pub DnsHostName: String,                // ""
    pub Exception: String,                  // ""
    pub IpAdress: String,                   // ""
    pub LogLevel: String,                   // "Info"
    pub Logger: String,                     // "AndroidLogger"
    pub Memory: String,                     // ""
    pub Message: String,                    // "[GPS] No lastKnownLocation received from Android API"
    pub Parameters: String,                 // "IdDevice: 3qtg83zdy95jmczkeiyx1rfa9\nDevice: Sony D5503\n"
    pub ScreenName: String,                 // "SplashScreenActivity"
    pub SessionId: String,                  // "e3a135c1-4435-4e97-a613-2fab30653dfc"
    pub Thread: String,                     // ""
    pub Trace: String,                      // ""
    pub UserName: String,                   // ""
}

impl Default for LogMobiliteRequest {
    fn default() -> Self {
        Self {
            AppName: "CP DISTRI V2".to_string(),
            IndianaVersion: "3.3.0.9".to_string(),
            DateLogged: String::new(),
            DnsHostName: String::new(),
            Exception: String::new(),
            IpAdress: String::new(),
            LogLevel: "Info".to_string(),
            Logger: "AndroidLogger".to_string(),
            Memory: String::new(),
            Message: String::new(),
            Parameters: String::new(),
            ScreenName: String::new(),
            SessionId: String::new(),
            Thread: String::new(),
            Trace: String::new(),
            UserName: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMobiliteResponse {
    pub success: bool,
    pub message: Option<String>,
    pub log_id: Option<String>,
    pub server_timestamp: Option<String>,
}

// ============================================================================
// TOURNÉES - Obtención de datos después del login
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeRequestV3 {
    // ✅ REPRODUCCIÓN 100% APP OFICIAL
    pub DateDebut: String,      // ✅ Campo correcto según APK
    pub Matricule: String,      // ✅ Campo correcto según APK
    pub Societe: String,        // ✅ Campo requerido según APK
    pub Agence: String,         // ✅ Campo requerido según APK
    pub Concentrateur: String,  // ✅ Campo requerido según APK
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeResponseV3 {
    pub success: bool,
    pub message: Option<String>,
    pub tournees: Option<Vec<TourneeV3>>,
    pub total_packages: Option<u32>,
    pub metadata: Option<TourneeMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TourneeV3 {
    pub tournee_id: String,
    pub date_tournee: String,
    pub statut: String,
    pub packages: Vec<PackageV3>,
    pub statistics: Option<TourneeStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageV3 {
    pub num_colis: String,
    pub destinataire: DestinataireV3,
    pub adresse: AdresseV3,
    pub statut: String,
    pub type_colis: String,
    pub instructions: Option<String>,
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
    pub matricule: String,
    pub device_info: crate::external_models::DeviceInfo, // 🆕 NUEVO: Usar ExternalDeviceInfo para compatibilidad
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Default for DeviceAuditRequest {
    fn default() -> Self {
        Self {
            // ✅ REPRODUCCIÓN 100% APK OFICIAL - BeanWSRequestAuditDevice
            device_disk: "8192".to_string(),              // Aproximación: 8GB
            device_id_device: "dev_install_001".to_string(), // ✅ HARDCODEADO para desarrollo
            device_ram: "3072".to_string(),               // Aproximación: 3GB (Sony Xperia Z1)
            id_externe_application: "com.danem.cpdistriv2".to_string(), // ✅ App ID oficial
            is_install_ok: true,                          // ✅ Instalación exitosa
            num_application_version: "3.3.0.9".to_string(), // ✅ Versión oficial
            device_cpu: "Qualcomm Snapdragon 800".to_string(), // Sony Xperia Z1
            device_langue: "es".to_string(),              // Español
            device_os: "Android 5.1.1, API 22".to_string(), // ✅ Sony Xperia Z1 real
            device_version: "Sony D5503".to_string(),     // ✅ Modelo real
            matricule: "PCP0010699_A187518".to_string(),  // ✅ Será actualizado en runtime
        }
    }
}