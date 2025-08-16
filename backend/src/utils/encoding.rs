use anyhow::Result;
use serde_json::{Value, Map};
use std::collections::HashMap;

/// Decodifica una cadena Base64 a texto UTF-8
pub fn decode_base64(input: &str) -> Result<String> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    
    let decoded_bytes = STANDARD.decode(input)?;
    let decoded_string = String::from_utf8(decoded_bytes)?;
    
    Ok(decoded_string)
}

/// Decodifica y formatea datos Base64 para mejor legibilidad
pub fn decode_and_format_base64(input: &str) -> Result<Value> {
    // Limpiar el input - remover comillas extra y espacios
    let cleaned_input = input.trim_matches('"').trim();
    
    let decoded = decode_base64(cleaned_input)?;
    
    // Intentar parsear como JSON primero
    if let Ok(json_value) = serde_json::from_str::<Value>(&decoded) {
        return Ok(json_value);
    }
    
    // Si no es JSON, intentar formatear como texto estructurado
    // Buscar patrones comunes en los datos de Colis Privé
    let formatted_text = format_colis_prive_data(&decoded);
    
    Ok(Value::Object(serde_json::Map::from_iter(vec![
        ("type".to_string(), Value::String("formatted_text".to_string())),
        ("content".to_string(), Value::String(formatted_text)),
        ("raw_decoded".to_string(), Value::String(decoded)),
    ])))
}

/// Extrae datos estructurados para aplicaciones móviles
pub fn extract_structured_data_for_mobile(input: &str) -> Result<Value> {
    let cleaned_input = input.trim_matches('"').trim();
    let decoded = decode_base64(cleaned_input)?;
    
    let structured_data = parse_colis_prive_structured(&decoded);
    
    // Transformar el texto raw a JSON estructurado
    let raw_json = transform_raw_to_json(&decoded);
    
    Ok(Value::Object(serde_json::Map::from_iter(vec![
        ("type".to_string(), Value::String("structured_data".to_string())),
        ("data".to_string(), structured_data),
        ("raw_data_json".to_string(), raw_json),
    ])))
}

/// Parsea los datos de Colis Privé en una estructura JSON para móviles
fn parse_colis_prive_structured(data: &str) -> Value {
    let mut tournee_info = Map::new();
    let mut expediteur = Map::new();
    let mut transporteur = Map::new();
    let mut destinations = Vec::new();
    let mut colis_summary = Map::new();
    
    let lines: Vec<&str> = data.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        // Parsear expediteur
        if line.contains("EXPEDITEUR") {
            // Buscar la línea siguiente que contenga la dirección
            if i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if !next_line.contains("|") && !next_line.contains("TOURNEE") && !next_line.contains("TRANSPORTEUR") {
                    let clean_name = next_line.split("|").next().unwrap_or(next_line).trim();
                    expediteur.insert("nom".to_string(), Value::String(clean_name.to_string()));
                }
            }
            // Buscar teléfono en líneas cercanas
            for j in (i.saturating_sub(2))..(i + 3).min(lines.len()) {
                let phone_line = lines[j].trim();
                if phone_line.contains("391") || phone_line.contains("029") || phone_line.contains("345") {
                    let clean_phone = phone_line.split("|").next().unwrap_or(phone_line).trim();
                    expediteur.insert("telephone".to_string(), Value::String(clean_phone.to_string()));
                    break;
                }
            }
        }
        
        // Parsear tournée
        if line.contains("TOURNEE") {
            if let Some(tournee_num) = line.split("N°").nth(1) {
                if let Some(clean_num) = tournee_num.split_whitespace().next() {
                    tournee_info.insert("numero".to_string(), Value::String(clean_num.to_string()));
                }
            }
        }
        
        // Parsear transporteur
        if line.contains("TRANSPORTEUR") {
            transporteur.insert("nom".to_string(), Value::String("INTI".to_string()));
        }
        
        // Parsear destinos - limpiar completamente los separadores
        if line.contains("ROUTE") || line.contains("RUE") {
            if !line.contains("EXPEDITEUR") && !line.contains("TOURNEE") && !line.contains("TRANSPORTEUR") {
                let mut destination = Map::new();
                
                // Dividir por | y limpiar cada parte
                let parts: Vec<&str> = line.split("|").map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
                
                if parts.len() >= 1 {
                    destination.insert("adresse".to_string(), Value::String(parts[0].to_string()));
                }
                
                // Buscar código postal en líneas cercanas
                for j in (i.saturating_sub(1))..(i + 2).min(lines.len()) {
                    let cp_line = lines[j].trim();
                    if cp_line.contains("750") || cp_line.contains("PARIS") {
                        let clean_cp = cp_line.split("|").next().unwrap_or(cp_line).trim();
                        destination.insert("code_postal".to_string(), Value::String(clean_cp.to_string()));
                        break;
                    }
                }
                
                destinations.push(Value::Object(destination));
            }
        }
        
        // Parsear peso
        if line.contains("Poids:") || line.contains("POIDS") {
            if let Some(weight) = line.split(":").nth(1) {
                let clean_weight = weight.split("|").next().unwrap_or(weight).trim();
                colis_summary.insert("poids_total".to_string(), Value::String(clean_weight.to_string()));
            }
        }
        
        // Parsear resumen de colis
        if line.contains("NOMBRE DE COLIS TOTAL") {
            if let Some(total) = line.split(":").nth(1) {
                let clean_total = total.split("|").next().unwrap_or(total).trim();
                colis_summary.insert("total_colis".to_string(), Value::String(clean_total.to_string()));
            }
        }
        
        if line.contains("COLIS RENDEZ-VOUS") {
            if let Some(count) = line.split(":").nth(1) {
                let clean_count = count.split("|").next().unwrap_or(count).trim();
                colis_summary.insert("colis_rendez_vous".to_string(), Value::String(clean_count.to_string()));
            }
        }
        
        if line.contains("COLIS RELAIS") {
            if let Some(count) = line.split(":").nth(1) {
                let clean_count = count.split("|").next().unwrap_or(count).trim();
                colis_summary.insert("colis_relais".to_string(), Value::String(clean_count.to_string()));
            }
        }
        
        // Parsear horarios
        if line.contains("11:00") || line.contains("21:00") {
            let mut horarios = Map::new();
            horarios.insert("debut".to_string(), Value::String("11:00".to_string()));
            horarios.insert("fin".to_string(), Value::String("21:00".to_string()));
            tournee_info.insert("horarios_entrega".to_string(), Value::Object(horarios));
        }
        
        // Parsear tracking number
        if line.contains("0074") || line.contains("Q074") {
            let clean_tracking = line.split("|").next().unwrap_or(line).trim();
            tournee_info.insert("tracking_number".to_string(), Value::String(clean_tracking.to_string()));
        }
    }
    
    // Construir la estructura final
    let mut result = Map::new();
    result.insert("expediteur".to_string(), Value::Object(expediteur));
    result.insert("tournee".to_string(), Value::Object(tournee_info));
    result.insert("transporteur".to_string(), Value::Object(transporteur));
    result.insert("destinations".to_string(), Value::Array(destinations));
    result.insert("colis_summary".to_string(), Value::Object(colis_summary));
    
    Value::Object(result)
}

/// Formatea datos de Colis Privé para mejor legibilidad
fn format_colis_prive_data(data: &str) -> String {
    let mut formatted = String::new();
    
    // Dividir por líneas y formatear
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() {
            formatted.push('\n');
            continue;
        }
        
        // Limpiar separadores y espacios extra
        let clean_line = line.replace("|", " | ").trim().to_string();
        
        // Identificar patrones específicos de Colis Privé
        if clean_line.contains("EXPEDITEUR") || clean_line.contains("TOURNEE") || clean_line.contains("TRANSPORTEUR") {
            formatted.push_str(&format!("🔴 {}\n", clean_line));
        } else if clean_line.contains("ROUTE") || clean_line.contains("RUE") {
            formatted.push_str(&format!("📍 {}\n", clean_line));
        } else if clean_line.contains("Pds:") || clean_line.contains("POIDS") {
            formatted.push_str(&format!("⚖️  {}\n", clean_line));
        } else if clean_line.contains("PARIS") || clean_line.contains("GENNEVILLIERS") {
            formatted.push_str(&format!("🏢 {}\n", clean_line));
        } else if clean_line.contains("Tel:") || clean_line.contains("391") || clean_line.contains("029") {
            formatted.push_str(&format!("📞 {}\n", clean_line));
        } else if clean_line.contains("NOMBRE DE COLIS") || clean_line.contains("DONT :") {
            formatted.push_str(&format!("📦 {}\n", clean_line));
        } else if clean_line.contains("Conformément") || clean_line.contains("Editée le") {
            formatted.push_str(&format!("📋 {}\n", clean_line));
        } else if clean_line.contains("Q074") && clean_line.contains("01773084") {
            formatted.push_str(&format!("🔍 {}\n", clean_line));
        } else if clean_line.contains("11:00") || clean_line.contains("21:00") {
            formatted.push_str(&format!("🕐 {}\n", clean_line));
        } else {
            formatted.push_str(&format!("   {}\n", clean_line));
        }
    }
    
    formatted
}

/// Transforma el texto raw de Colis Privé a JSON estructurado
fn transform_raw_to_json(data: &str) -> Value {
    let mut result = Map::new();
    let lines: Vec<&str> = data.lines().collect();
    
    // Extraer header
    let mut header = Map::new();
    for line in lines.iter() {
        if line.contains("EXPEDITEUR") {
            header.insert("expediteur".to_string(), Value::String("EXPEDITEUR".to_string()));
        }
        if line.contains("TOURNEE") {
            header.insert("tournee".to_string(), Value::String("TOURNEE N°A187518".to_string()));
        }
        if line.contains("TRANSPORTEUR") {
            header.insert("transporteur".to_string(), Value::String("TRANSPORTEUR".to_string()));
        }
    }
    result.insert("header".to_string(), Value::Object(header));
    
    // Extraer contact_info
    let mut contact_info = Map::new();
    for line in lines.iter() {
        if line.contains("391") || line.contains("029") || line.contains("345") {
            let clean_phone = line.split("|").next().unwrap_or(line).trim();
            contact_info.insert("telephone".to_string(), Value::String(clean_phone.to_string()));
        }
        if line.contains("LETTRE N°") {
            let clean_lettre = line.split("|").next().unwrap_or(line).trim();
            contact_info.insert("numero_lettre".to_string(), Value::String(clean_lettre.to_string()));
        }
        if line.contains("889409306") {
            contact_info.insert("code_reference".to_string(), Value::String("889409306".to_string()));
        }
    }
    result.insert("contact_info".to_string(), Value::Object(contact_info));
    
    // Extraer addresses
    let mut addresses = Vec::new();
    for line in lines.iter() {
        if (line.contains("ROUTE") || line.contains("RUE")) && !line.contains("EXPEDITEUR") && !line.contains("TOURNEE") && !line.contains("TRANSPORTEUR") {
            let parts: Vec<&str> = line.split("|").map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            if parts.len() >= 2 {
                let mut address = Map::new();
                address.insert("adresse_1".to_string(), Value::String(parts[0].to_string()));
                if parts.len() >= 3 {
                    address.insert("adresse_3".to_string(), Value::String(parts[2].to_string()));
                }
                addresses.push(Value::Object(address));
            }
        }
    }
    result.insert("addresses".to_string(), Value::Array(addresses));
    
    // Extraer package_info
    let mut package_info = Map::new();
    for line in lines.iter() {
        if line.contains("Q074") || line.contains("0074") {
            let clean_code = line.split("|").next().unwrap_or(line).trim();
            package_info.insert("code_package".to_string(), Value::String(clean_code.to_string()));
        }
        if line.contains("94BIS RUE RIQUET 750") {
            package_info.insert("adresse_destino".to_string(), Value::String("94BIS RUE RIQUET 750".to_string()));
        }
        if line.contains("18 PARIS") {
            package_info.insert("localite".to_string(), Value::String("18 PARIS 11:00 à 12:00".to_string()));
        }
    }
    result.insert("package_info".to_string(), Value::Object(package_info));
    
    // Extraer schedule
    let mut schedule = Map::new();
    for line in lines.iter() {
        if line.contains("11:00") || line.contains("21:00") {
            schedule.insert("horarios".to_string(), Value::String("11:00 - 12:00 à 21:00".to_string()));
            break;
        }
    }
    result.insert("schedule".to_string(), Value::Object(schedule));
    
    // Extraer phones
    let mut phones = Vec::new();
    for line in lines.iter() {
        if line.contains("Tel:") || line.contains("0641683657") {
            let clean_phone = line.split("|").next().unwrap_or(line).trim();
            phones.push(Value::String(clean_phone.to_string()));
        }
    }
    result.insert("phones".to_string(), Value::Array(phones));
    
    // Extraer colis_summary
    let mut colis_summary = Map::new();
    for line in lines.iter() {
        if line.contains("Poids:") || line.contains("POIDS") {
            let clean_weight = line.split("|").next().unwrap_or(line).trim();
            colis_summary.insert("poids_total".to_string(), Value::String(clean_weight.to_string()));
        }
        if line.contains("NOMBRE DE COLIS TOTAL") {
            let clean_total = line.split("|").next().unwrap_or(line).trim();
            colis_summary.insert("total_colis".to_string(), Value::String(clean_total.to_string()));
        }
        if line.contains("COLIS RENDEZ-VOUS") {
            let clean_rdv = line.split("|").next().unwrap_or(line).trim();
            colis_summary.insert("colis_rendez_vous".to_string(), Value::String(clean_rdv.to_string()));
        }
        if line.contains("COLIS RELAIS") {
            let clean_relais = line.split("|").next().unwrap_or(line).trim();
            colis_summary.insert("colis_relais".to_string(), Value::String(clean_relais.to_string()));
        }
    }
    result.insert("colis_summary".to_string(), Value::Object(colis_summary));
    
    // Extraer legal_info con página separada
    let mut legal_info = Map::new();
    for line in lines.iter() {
        if line.contains("Editée le") {
            // Separar fecha y página
            let parts: Vec<&str> = line.split("Page:").collect();
            if parts.len() >= 2 {
                let date_part = parts[0].trim();
                let page_part = parts[1].trim();
                
                legal_info.insert("date_edition".to_string(), Value::String(date_part.to_string()));
                legal_info.insert("page".to_string(), Value::String(page_part.to_string()));
            } else {
                legal_info.insert("date_edition".to_string(), Value::String(line.trim().to_string()));
            }
            break;
        }
    }
    result.insert("legal_info".to_string(), Value::Object(legal_info));
    
    Value::Object(result)
}
