use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::models::Delivery;

/// Decodifica un string Base64 a texto plano
/// Maneja casos especiales como respuestas null (feriados, días sin trabajo)
pub fn decode_base64(encoded: &str) -> Result<String> {
    // Manejar caso cuando no hay datos (feriados, días sin trabajo)
    if encoded == "null" || encoded.trim().is_empty() {
        return Ok("No hay tournées programadas para esta fecha (posible feriado o día sin trabajo)".to_string());
    }
    
    let decoded_bytes = BASE64.decode(encoded)?;
    let decoded_string = String::from_utf8(decoded_bytes)?;
    Ok(decoded_string)
}

/// Parsea los datos de tournée decodificados
/// Por ahora solo muestra el texto decodificado
/// En el futuro implementaremos parsing completo para extraer estructuras
pub fn parse_tournee_data(decoded: &str) -> Result<Vec<Delivery>> {
    // TODO: Implementar parsing completo de la hoja de ruta
    // Por ahora solo creamos un placeholder
    println!("📋 Datos de tournée decodificados:");
    println!("{}", decoded);
    
    // Placeholder: retornamos un vector vacío por ahora
    // En el futuro aquí parsearemos la hoja de ruta completa
    Ok(Vec::new())
}

/// Extrae información básica de la hoja de ruta
pub fn extract_basic_info(decoded: &str) -> Result<()> {
    let lines: Vec<&str> = decoded.lines().collect();
    
    // Buscar información básica como número de tournée, total de paquetes, etc.
    for line in lines {
        if line.contains("TOURNEE N°") {
            println!("🎯 Tournée encontrada: {}", line.trim());
        }
        if line.contains("NOMBRE DE COLIS TOTAL") {
            println!("📦 Total de paquetes: {}", line.trim());
        }
        if line.contains("POIDS TOTAL") {
            println!("⚖️ Peso total: {}", line.trim());
        }
    }
    
    Ok(())
}
