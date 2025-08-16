use anyhow::Result;

/// Decodifica una cadena Base64 a texto UTF-8
pub fn decode_base64(input: &str) -> Result<String> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    
    let decoded_bytes = STANDARD.decode(input)?;
    let decoded_string = String::from_utf8(decoded_bytes)?;
    
    Ok(decoded_string)
}
