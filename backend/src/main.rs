mod models;
mod client;
mod utils;
mod config;

use anyhow::Result;
use crate::client::ColisPriveClient;
use crate::utils::decode_base64;
use config::{COLIS_PRIVE_USERNAME, COLIS_PRIVE_PASSWORD, COLIS_PRIVE_SOCIETE};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚚 Delivery Route Optimizer - MVP");
    println!("=====================================");

    // Verificar credenciales
    if COLIS_PRIVE_USERNAME == "tu_usuario_aqui" ||
       COLIS_PRIVE_PASSWORD == "tu_password_aqui" ||
       COLIS_PRIVE_SOCIETE == "tu_societe_aqui" {
        anyhow::bail!("❌ Configura las credenciales en src/config.rs antes de ejecutar");
    }

    // Crear cliente
    let mut client = ColisPriveClient::new()?;

    println!("🔐 Intentando login con:");
    println!("   Login: {}", COLIS_PRIVE_USERNAME);
    println!("   Societe: {}", COLIS_PRIVE_SOCIETE);

    // Login
    let login_response = client.login(COLIS_PRIVE_USERNAME, COLIS_PRIVE_PASSWORD, COLIS_PRIVE_SOCIETE).await?;

    println!("✅ Login exitoso!");
    println!("   📋 Matricule: {}", login_response.matricule);
    println!("   🏢 Societe: {}", login_response.societe);
    println!("   🔑 Token: {}...", &login_response.tokens.sso_hopps[..50.min(login_response.tokens.sso_hopps.len())]);

    // Pilot access
    let _pilot_response = client.get_pilot_access(
        &login_response.tokens.sso_hopps,
        &login_response.matricule,
        &login_response.societe
    ).await?;

    println!("✅ Pilot access exitoso!");

    // Dashboard info - PROBAR CON CURL PRIMERO
    println!("🔍 Probando Dashboard info con curl...");
    let _dashboard_response_curl = client.get_dashboard_info_curl(
        &login_response.tokens.sso_hopps,
        &login_response.societe,
        &login_response.matricule,
        "2025-08-14"  // FECHA DE HOY
    ).await?;
    
    println!("✅ Dashboard info con curl exitoso!");
    
    // Dashboard info - PROBAR CON REQWEST
    println!("🔍 Probando Dashboard info con reqwest...");
    let _dashboard_response = client.get_dashboard_info(
        &login_response.tokens.sso_hopps,
        &login_response.societe,
        &login_response.matricule,
        "2025-08-14"  // FECHA DE HOY
    ).await?;
    
    println!("✅ Dashboard info con reqwest exitoso!");

    // Obtener tournée con curl (que funciona)
    let date = "2025-08-14"; // FECHA DE HOY
    println!("📅 Obteniendo tournée para la fecha: {}", date);

    match client.get_tournee_curl(&login_response.tokens.sso_hopps, COLIS_PRIVE_SOCIETE, &login_response.matricule, date).await {
        Ok(tournee_data) => {
            println!("✅ Tournée obtenida exitosamente");
            println!("\n🔍 Decodificando datos Base64...");

            match decode_base64(&tournee_data) {
                Ok(decoded_str) => {
                    println!("✅ Datos decodificados correctamente");
                    println!("\n📊 Información de la tournée:");
                    println!("📋 Datos completos de la tournée:");
                    println!("{}", decoded_str);

                    println!("\n🎉 MVP completado exitosamente!");
                }
                Err(e) => {
                    println!("❌ Error decodificando Base64: {}", e);
                    println!("📋 Datos crudos recibidos: {}", tournee_data);
                }
            }
        }
        Err(e) => {
            println!("❌ Error obteniendo tournée: {}", e);
        }
    }

    Ok(())
}
