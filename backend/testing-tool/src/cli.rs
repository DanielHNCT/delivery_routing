use dialoguer::{Input, Select, Password, Confirm};
use colored::*;
use anyhow::Result;
use crate::api_client::ApiClient;

pub async fn run_interactive_cli() -> Result<()> {
    println!("{}", "🚀 Delivery Optimizer API Testing Tool".bright_green().bold());
    println!("{}", "=====================================".bright_blue());
    
    let api_client = ApiClient::new();
    
    loop {
        let options = vec![
            "🔐 Test Colis Privé Authentication",
            "📦 Get Tournée Data", 
            "🧪 Test All Endpoints",
            "❌ Exit"
        ];
        
        let selection = Select::new()
            .with_prompt("Select an option")
            .items(&options)
            .interact()?;
            
        match selection {
            0 => test_auth(&api_client).await?,
            1 => test_tournee(&api_client).await?,
            2 => test_all_endpoints(&api_client).await?,
            3 => break,
            _ => unreachable!(),
        }
        
        println!();
    }
    
    println!("{}", "👋 ¡Hasta luego!".bright_yellow());
    Ok(())
}

async fn test_auth(api_client: &ApiClient) -> Result<()> {
    println!("{}", "\n🔐 Testing Colis Privé Authentication".bright_cyan().bold());
    println!("{}", "=====================================".bright_blue());
    
    let username: String = Input::new()
        .with_prompt("Username")
        .interact_text()?;
        
    let password = Password::new()
        .with_prompt("Password")
        .interact()?;
        
    let societe: String = Input::new()
        .with_prompt("Societe")
        .interact_text()?;
    
    println!("{}", "\n🔄 Testing authentication...".yellow());
    
    match api_client.test_colis_prive_auth(&username, &password, &societe).await {
        Ok(response) => {
            if let Some(success) = response.get("success").and_then(|v| v.as_bool()) {
                if success {
                    println!("{}", "✅ Authentication successful!".bright_green());
                    if let Some(token) = response.get("token").and_then(|v| v.as_str()) {
                        println!("🔑 Token: {}...", &token[..token.len().min(50)]);
                    }
                    if let Some(matricule) = response.get("matricule").and_then(|v| v.as_str()) {
                        println!("📋 Matricule: {}", matricule);
                    }
                } else {
                    println!("{}", "❌ Authentication failed!".bright_red());
                }
                if let Some(message) = response.get("message").and_then(|v| v.as_str()) {
                    println!("💬 Message: {}", message);
                }
            } else {
                println!("{}", "⚠️  Unexpected response format".yellow());
                println!("📄 Response: {}", serde_json::to_string_pretty(&response)?);
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Error: {}", e).bright_red());
        }
    }
    
    Ok(())
}

async fn test_tournee(api_client: &ApiClient) -> Result<()> {
    println!("{}", "\n📦 Testing Tournée Data Retrieval".bright_cyan().bold());
    println!("{}", "=================================".bright_blue());
    
    let username: String = Input::new()
        .with_prompt("Username")
        .interact_text()?;
        
    let password = Password::new()
        .with_prompt("Password")
        .interact()?;
        
    let societe: String = Input::new()
        .with_prompt("Societe")
        .interact_text()?;
        
    let date: String = Input::new()
        .with_prompt("Date (YYYY-MM-DD)")
        .with_initial_text("2025-01-20")
        .interact_text()?;
        
    let matricule: String = Input::new()
        .with_prompt("Matricule")
        .interact_text()?;
    
    println!("{}", "\n🔄 Retrieving tournée data...".yellow());
    
    match api_client.get_tournee_data(&username, &password, &societe, &date, &matricule).await {
        Ok(response) => {
            if let Some(success) = response.get("success").and_then(|v| v.as_bool()) {
                if success {
                    println!("{}", "✅ Tournée data retrieved successfully!".bright_green());
                    println!("📅 Date: {}", date);
                    println!("📋 Matricule: {}", matricule);
                    
                    if let Some(decoded_data) = response.get("decoded_data").and_then(|v| v.as_str()) {
                        println!("📊 Decoded Data:");
                        println!("{}", decoded_data);
                    }
                } else {
                    println!("{}", "❌ Failed to retrieve tournée data!".bright_red());
                }
            } else {
                println!("{}", "⚠️  Unexpected response format".yellow());
                println!("📄 Response: {}", serde_json::to_string_pretty(&response)?);
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Error: {}", e).bright_red());
        }
    }
    
    Ok(())
}

async fn test_all_endpoints(api_client: &ApiClient) -> Result<()> {
    println!("{}", "\n🧪 Testing All Endpoints".bright_cyan().bold());
    println!("{}", "=======================".bright_blue());
    
    // Test health endpoint
    println!("{}", "\n🔍 Testing Health Endpoint...".yellow());
    match api_client.test_health_endpoint().await {
        Ok(response) => {
            println!("{}", "✅ Health endpoint working".bright_green());
            if let Some(message) = response.get("message").and_then(|v| v.as_str()) {
                println!("💬 Message: {}", message);
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Health endpoint error: {}", e).bright_red());
        }
    }
    
    // Test with sample credentials
    println!("{}", "\n🔍 Testing Authentication with Sample Data...".yellow());
    let sample_username = "test_user";
    let sample_password = "test_password";
    let sample_societe = "test_societe";
    
    match api_client.test_colis_prive_auth(sample_username, sample_password, sample_societe).await {
        Ok(response) => {
            if let Some(success) = response.get("success").and_then(|v| v.as_bool()) {
                if success {
                    println!("{}", "✅ Sample authentication successful (unexpected!)".bright_green());
                } else {
                    println!("{}", "✅ Sample authentication failed (expected)".bright_green());
                }
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Sample authentication error: {}", e).bright_red());
        }
    }
    
    println!("{}", "\n🎉 All endpoint tests completed!".bright_green());
    
    Ok(())
}
