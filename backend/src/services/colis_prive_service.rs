use anyhow::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::client::ColisPriveClient;

#[derive(Debug, Deserialize, Validate)]
pub struct ColisPriveAuthRequest {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 3))]
    pub password: String,
    #[validate(length(min = 3))]
    pub societe: String,
}

#[derive(Debug, Serialize)]
pub struct ColisPriveAuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub message: String,
    pub matricule: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetTourneeRequest {
    pub username: String,
    pub password: String,
    pub societe: String,
    pub date: String, // YYYY-MM-DD
    pub matricule: String,
}

pub struct ColisPriveService;

impl ColisPriveService {
    /// Autenticar con Colis Privé usando credenciales dinámicas
    pub async fn authenticate_colis_prive(
        credentials: ColisPriveAuthRequest
    ) -> Result<ColisPriveAuthResponse> {
        // Validar credenciales
        credentials.validate()
            .map_err(|e| anyhow::anyhow!("Credenciales inválidas: {}", e))?;

        // Crear cliente temporal
        let mut client = ColisPriveClient::new()?;

        // Intentar login
        match client.login(&credentials.username, &credentials.password, &credentials.societe).await {
            Ok(login_response) => {
                // Verificar si la autenticación fue exitosa
                if login_response.isAuthentif {
                    Ok(ColisPriveAuthResponse {
                        success: true,
                        token: Some(login_response.tokens.SsoHopps),
                        message: "Autenticación exitosa".to_string(),
                        matricule: Some(login_response.matricule),
                    })
                } else {
                    Ok(ColisPriveAuthResponse {
                        success: false,
                        token: None,
                        message: "Credenciales inválidas".to_string(),
                        matricule: None,
                    })
                }
            }
            Err(e) => {
                Ok(ColisPriveAuthResponse {
                    success: false,
                    token: None,
                    message: format!("Error de autenticación: {}", e),
                    matricule: None,
                })
            }
        }
    }

    /// Obtener datos de tournée usando credenciales dinámicas
    pub async fn get_tournee_data(
        credentials: &ColisPriveAuthRequest,
        date: &str,
        matricule: &str
    ) -> Result<String> {
        // Crear cliente temporal
        let mut client = ColisPriveClient::new()?;

        // Autenticar primero
        let login_response = client.login(&credentials.username, &credentials.password, &credentials.societe).await?;
        
        if !login_response.isAuthentif {
            anyhow::bail!("Credenciales inválidas para obtener tournée");
        }

        // Obtener tournée usando el método curl que funciona
        let tournee_data = client.get_tournee_curl(
            &login_response.tokens.SsoHopps,
            &credentials.societe,
            matricule,
            date
        ).await?;

        Ok(tournee_data)
    }
}
