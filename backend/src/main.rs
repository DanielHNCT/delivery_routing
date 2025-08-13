mod models;
mod client;
mod utils;
mod config;

use anyhow::Result;
use client::ColisPriveClient;
use utils::{decode_base64, extract_basic_info};
use config::{COLIS_PRIVE_USERNAME, COLIS_PRIVE_PASSWORD, TOURNEE_DATE};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚚 Delivery Route Optimizer - MVP");
    println!("=====================================");
    
    // Crear cliente API
    let mut client = ColisPriveClient::new();
    
    // Usar credenciales de configuración
    let login = COLIS_PRIVE_USERNAME;
    let password = COLIS_PRIVE_PASSWORD;
    let societe = "PCP0010699"; // Societe fija basada en el análisis de Claude
    
    // Verificar que las credenciales no sean placeholder
    if login == "tu_usuario_aqui" || password == "tu_password_aqui" {
        println!("❌ Error: Credenciales no configuradas");
        println!("💡 Edita src/config.rs y reemplaza las credenciales placeholder");
        println!("   con tus credenciales reales de Colis Privé");
        return Err(anyhow::anyhow!("Credenciales no configuradas"));
    }
    
    println!("🔐 Intentando login con:");
    println!("   Login: {}", login);
    println!("   Societe: {}", societe);
    
    // Login con los 3 parámetros correctos
    let login_response = match client.login(login, password, societe).await {
        Ok(response) => {
            println!("✅ Login exitoso!");
            println!("   📋 Matricule: {}", response.matricule);
            println!("   🏢 Societe: {}", response.societe);
            response
        }
        Err(e) => {
            println!("❌ Error en login: {}", e);
            println!("💡 Verifica que las credenciales sean correctas");
            return Err(e);
        }
    };
    
    // Verificar autenticación
    if !client.is_authenticated() {
        anyhow::bail!("No se pudo autenticar con la API");
    }
    
    // Agregar delay para que el token se active completamente
    println!("\n⏳ Esperando 5 segundos para que el token se active...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    println!("\n📅 Obteniendo tournée para la fecha: {}", TOURNEE_DATE);
    
    // Obtener tournée
    let tournee_data = match client.get_tournee(
        &login_response.societe,
        &login_response.matricule,
        TOURNEE_DATE
    ).await {
        Ok(data) => {
            println!("✅ Tournée obtenida exitosamente");
            data
        }
        Err(e) => {
            println!("❌ Error obteniendo tournée: {}", e);
            return Err(e);
        }
    };
    
    println!("\n🔍 Decodificando datos Base64...");
    
    // Decodificar Base64
    let decoded = match decode_base64(&tournee_data) {
        Ok(data) => {
            println!("✅ Datos decodificados correctamente");
            data
        }
        Err(e) => {
            println!("❌ Error decodificando Base64: {}", e);
            return Err(e);
        }
    };
    
    // Extraer información básica
    println!("\n📊 Información de la tournée:");
    extract_basic_info(&decoded)?;
    
    // Mostrar datos completos (opcional, puede ser muy largo)
    println!("\n📋 Datos completos de la tournée:");
    println!("{}", decoded);
    
    println!("\n🎉 MVP completado exitosamente!");
    println!("💡 Próximos pasos:");
    println!("   - Implementar parser estructurado para extraer direcciones");
    println!("   - Agregar base de datos PostgreSQL");
    println!("   - Implementar algoritmos de optimización de rutas");
    println!("   - Crear API REST para apps móviles");
    
    Ok(())
}
