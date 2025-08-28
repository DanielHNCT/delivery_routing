//! Cliente HTTP para Colis Privé (API Web)
//! 
//! Este módulo contiene el cliente HTTP simplificado para la API web de Colis Privé.

use anyhow::Result;
use reqwest::Client;

/// Cliente HTTP simplificado para Colis Privé (API Web)
pub struct ColisPriveWebClient {
    pub client: Client,
    pub auth_base_url: String,
    pub tournee_base_url: String,
}

impl ColisPriveWebClient {
    /// Crear nuevo cliente HTTP para API web
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            auth_base_url: "https://wsauthentificationexterne.colisprive.com".to_string(),
            tournee_base_url: "https://wstournee-v2.colisprive.com".to_string(),
        })
    }
}
