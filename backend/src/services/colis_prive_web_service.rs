//! Servicio web mínimo para Colis Privé
//! 
//! Este módulo contiene el servicio mínimo para la API web de Colis Privé.

use anyhow::Result;
use reqwest::Client;

/// Servicio mínimo para la API Web de Colis Privé
pub struct ColisPriveWebService {
    client: Client,
}

impl ColisPriveWebService {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self { client })
    }
}
