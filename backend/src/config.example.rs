// Archivo de configuración de ejemplo
// Copia este archivo a config.rs y configura tus credenciales reales

pub const COLIS_PRIVE_USERNAME: &str = "tu_usuario_aqui";
pub const COLIS_PRIVE_PASSWORD: &str = "tu_password_aqui";
pub const COLIS_PRIVE_SOCIETE: &str = "tu_societe_aqui";

// URLs de la API (no cambiar)
pub const AUTH_BASE_URL: &str = "https://wsauthentificationexterne.colisprive.com";
pub const TOURNEE_BASE_URL: &str = "https://wstournee-v2.colisprive.com";

// Configuración adicional
pub const TOKEN_DURATION_HOURS: i32 = 24;
pub const REQUEST_DELAY_SECONDS: u64 = 5;
