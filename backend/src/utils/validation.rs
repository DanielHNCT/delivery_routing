//! Funciones de validación para la API
//! 
//! Este módulo contiene funciones de validación adicionales y helpers.

use validator::{Validate, ValidationError};
use uuid::Uuid;

/// Validar que un UUID es válido
pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
    if Uuid::parse_str(uuid).is_ok() {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_uuid");
        error.message = Some("UUID inválido".into());
        Err(error)
    }
}

/// Validar que un string no está vacío
pub fn validate_not_empty(value: &str) -> Result<(), ValidationError> {
    if !value.trim().is_empty() {
        Ok(())
    } else {
        let mut error = ValidationError::new("empty_string");
        error.message = Some("Campo no puede estar vacío".into());
        Err(error)
    }
}

/// Validar que un número está en un rango específico
pub fn validate_range<T: PartialOrd + std::fmt::Display>(
    value: T,
    min: T,
    max: T,
    field_name: &str,
) -> Result<(), ValidationError> {
    if value >= min && value <= max {
        Ok(())
    } else {
        let mut error = ValidationError::new("out_of_range");
        error.message = Some(format!("{} debe estar entre {} y {}", field_name, min, max).into());
        Err(error)
    }
}

/// Validar formato de email
pub fn validate_email_format(email: &str) -> Result<(), ValidationError> {
    if email.contains('@') && email.contains('.') {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_email_format");
        error.message = Some("Formato de email inválido".into());
        Err(error)
    }
}

/// Validar formato de teléfono (básico)
pub fn validate_phone_format(phone: &str) -> Result<(), ValidationError> {
    let clean_phone = phone.chars().filter(|c| c.is_digit(10)).collect::<String>();
    if clean_phone.len() >= 8 && clean_phone.len() <= 15 {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_phone_format");
        error.message = Some("Formato de teléfono inválido".into());
        Err(error)
    }
}

/// Validar que una fecha está en el futuro
pub fn validate_future_date(date: &chrono::DateTime<chrono::Utc>) -> Result<(), ValidationError> {
    let now = chrono::Utc::now();
    if *date > now {
        Ok(())
    } else {
        let mut error = ValidationError::new("past_date");
        error.message = Some("La fecha debe estar en el futuro".into());
        Err(error)
    }
}

/// Validar que una fecha está en el pasado
pub fn validate_past_date(date: &chrono::DateTime<chrono::Utc>) -> Result<(), ValidationError> {
    let now = chrono::Utc::now();
    if *date < now {
        Ok(())
    } else {
        let mut error = ValidationError::new("future_date");
        error.message = Some("La fecha debe estar en el pasado".into());
        Err(error)
    }
}

/// Validar que un string contiene solo caracteres alfanuméricos
pub fn validate_alphanumeric(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(|c| c.is_alphanumeric() || c.is_whitespace()) {
        Ok(())
    } else {
        let mut error = ValidationError::new("non_alphanumeric");
        error.message = Some("Solo se permiten caracteres alfanuméricos y espacios".into());
        Err(error)
    }
}

/// Validar que un string contiene solo letras
pub fn validate_letters_only(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        Ok(())
    } else {
        let mut error = ValidationError::new("non_letters");
        error.message = Some("Solo se permiten letras y espacios".into());
        Err(error)
    }
}

/// Validar que un string contiene solo números
pub fn validate_numbers_only(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(|c| c.is_digit(10)) {
        Ok(())
    } else {
        let mut error = ValidationError::new("non_numbers");
        error.message = Some("Solo se permiten números".into());
        Err(error)
    }
}

/// Validar longitud de string con límites personalizados
pub fn validate_string_length(
    value: &str,
    min: usize,
    max: usize,
    field_name: &str,
) -> Result<(), ValidationError> {
    let len = value.len();
    if len >= min && len <= max {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_length");
        error.message = Some(format!("{} debe tener entre {} y {} caracteres", field_name, min, max).into());
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_uuid() {
        let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
        let invalid_uuid = "invalid-uuid";
        
        assert!(validate_uuid(valid_uuid).is_ok());
        assert!(validate_uuid(invalid_uuid).is_err());
    }

    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty("hello").is_ok());
        assert!(validate_not_empty("").is_err());
        assert!(validate_not_empty("   ").is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, 1, 10, "test").is_ok());
        assert!(validate_range(0, 1, 10, "test").is_err());
        assert!(validate_range(15, 1, 10, "test").is_err());
    }

    #[test]
    fn test_validate_email_format() {
        assert!(validate_email_format("test@example.com").is_ok());
        assert!(validate_email_format("invalid-email").is_err());
        assert!(validate_email_format("test@").is_err());
    }

    #[test]
    fn test_validate_phone_format() {
        assert!(validate_phone_format("1234567890").is_ok());
        assert!(validate_phone_format("123-456-7890").is_ok());
        assert!(validate_phone_format("123").is_err());
    }

    #[test]
    fn test_validate_alphanumeric() {
        assert!(validate_alphanumeric("Hello World 123").is_ok());
        assert!(validate_alphanumeric("Hello@World").is_err());
    }

    #[test]
    fn test_validate_letters_only() {
        assert!(validate_letters_only("Hello World").is_ok());
        assert!(validate_letters_only("Hello123").is_err());
    }

    #[test]
    fn test_validate_numbers_only() {
        assert!(validate_numbers_only("12345").is_ok());
        assert!(validate_numbers_only("123abc").is_err());
    }

    #[test]
    fn test_validate_string_length() {
        assert!(validate_string_length("hello", 3, 10, "test").is_ok());
        assert!(validate_string_length("hi", 3, 10, "test").is_err());
        assert!(validate_string_length("hello world", 3, 10, "test").is_err());
    }
}
