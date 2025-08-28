//! Headers mínimos para Colis Privé
//! 
//! Este módulo contiene los headers mínimos necesarios para la API web.

use reqwest::header::HeaderMap;

/// Crear headers mínimos para Colis Privé
pub fn create_minimal_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers
}
