use reqwest::Client;
use serde_json::Value;
use anyhow::Result;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:3000".to_string(),
        }
    }
    
    /// Test Colis Privé authentication
    pub async fn test_colis_prive_auth(
        &self,
        username: &str,
        password: &str,
        societe: &str
    ) -> Result<Value> {
        let url = format!("{}/api/colis-prive/auth", self.base_url);
        
        let payload = serde_json::json!({
            "username": username,
            "password": password,
            "societe": societe
        });
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
            
        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, response.text().await?);
        }
        
        let response_data: Value = response.json().await?;
        Ok(response_data)
    }
    
    /// Get tournée data
    pub async fn get_tournee_data(
        &self,
        username: &str,
        password: &str,
        societe: &str,
        date: &str,
        matricule: &str
    ) -> Result<Value> {
        let url = format!("{}/api/colis-prive/tournee", self.base_url);
        
        let payload = serde_json::json!({
            "username": username,
            "password": password,
            "societe": societe,
            "date": date,
            "matricule": matricule
        });
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
            
        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, response.text().await?);
        }
        
        let response_data: Value = response.json().await?;
        Ok(response_data)
    }
    
    /// Test health endpoint
    pub async fn test_health_endpoint(&self) -> Result<Value> {
        let url = format!("{}/api/colis-prive/health", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, response.text().await?);
        }
        
        let response_data: Value = response.json().await?;
        Ok(response_data)
    }
}
