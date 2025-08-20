use crate::client::ColisPriveClient;
use crate::external_models::{LoginRequest, Commun, RefreshTokenRequest};
use serde_json::json;

/// Test básico de creación del cliente
#[tokio::test]
async fn test_client_creation() {
    let client = ColisPriveClient::new();
    assert!(client.is_ok());
    
    let client = client.unwrap();
    assert_eq!(client.auth_base_url, "https://wsauthentificationexterne.colisprive.com");
    assert_eq!(client.tournee_base_url, "https://wstournee-v2.colisprive.com");
}

/// Test de headers de Colis Privé
#[tokio::test]
async fn test_colis_headers() {
    let client = ColisPriveClient::new().unwrap();
    
    // Test headers para login
    let login_headers = client.get_colis_headers("login", Some("test_user"), None);
    assert!(login_headers.contains_key("ActivityId"));
    assert!(login_headers.contains_key("AppName"));
    assert!(login_headers.contains_key("VersionApplication"));
    assert!(login_headers.contains_key("UserName"));
    assert_eq!(login_headers.get("AppName").unwrap(), "CP DISTRI V2");
    assert_eq!(login_headers.get("VersionApplication").unwrap(), "3.3.0.9");
    
    // Test headers para refresh
    let refresh_headers = client.get_colis_headers("refresh", None, None);
    assert!(refresh_headers.contains_key("ActivityId"));
    assert!(refresh_headers.contains_key("AppName"));
    assert!(!refresh_headers.contains_key("UserName")); // No username para refresh
    
    // Test headers para tournée
    let tournee_headers = client.get_colis_headers("tournee", Some("test_user"), Some("test_token"));
    assert!(tournee_headers.contains_key("SsoHopps"));
    assert!(tournee_headers.contains_key("UserName"));
    assert_eq!(tournee_headers.get("SsoHopps").unwrap(), "test_token");
}

/// Test de request body para refresh token
#[tokio::test]
async fn test_refresh_token_request() {
    let refresh_request = json!({
        "dureeTokenInHour": 0,
        "token": "test_token_123"
    });
    
    assert_eq!(refresh_request["dureeTokenInHour"], 0);
    assert_eq!(refresh_request["token"], "test_token_123");
}

/// Test de request body para login
#[tokio::test]
async fn test_login_request() {
    let login_req = LoginRequest {
        login: "test_user".to_string(),
        password: "test_password".to_string(),
        societe: "test_societe".to_string(),
        commun: Commun {
            duree_token_in_hour: 24,
        },
    };
    
    assert_eq!(login_req.login, "test_user");
    assert_eq!(login_req.societe, "test_societe");
    assert_eq!(login_req.commun.duree_token_in_hour, 24);
}

/// Test de RefreshTokenRequest
#[tokio::test]
async fn test_refresh_token_request_struct() {
    let refresh_req = RefreshTokenRequest {
        duree_token_in_hour: 0,
        token: "test_token_456".to_string(),
    };
    
    assert_eq!(refresh_req.duree_token_in_hour, 0);
    assert_eq!(refresh_req.token, "test_token_456");
}

/// Test de compatibilidad de headers legacy
#[tokio::test]
async fn test_legacy_headers_compatibility() {
    let client = ColisPriveClient::new().unwrap();
    
    // El método legacy debe seguir funcionando
    let legacy_headers = client.get_common_headers();
    assert!(legacy_headers.contains_key("ActivityId"));
    assert!(legacy_headers.contains_key("AppName"));
    
    // Debe ser equivalente a get_colis_headers("default", None, None)
    let default_headers = client.get_colis_headers("default", None, None);
    assert_eq!(legacy_headers.len(), default_headers.len());
}

/// Test de ActivityId único por request
#[tokio::test]
async fn test_activity_id_uniqueness() {
    let client = ColisPriveClient::new().unwrap();
    
    let headers1 = client.get_colis_headers("login", Some("user1"), None);
    let headers2 = client.get_colis_headers("login", Some("user2"), None);
    
    let activity_id1 = headers1.get("ActivityId").unwrap();
    let activity_id2 = headers2.get("ActivityId").unwrap();
    
    // Los ActivityId deben ser diferentes
    assert_ne!(activity_id1, activity_id2);
}

/// Test de headers específicos por endpoint
#[tokio::test]
async fn test_endpoint_specific_headers() {
    let client = ColisPriveClient::new().unwrap();
    
    // Login debe tener headers específicos
    let login_headers = client.get_colis_headers("login", Some("user"), None);
    assert!(login_headers.contains_key("Accept"));
    assert!(login_headers.contains_key("Accept-Language"));
    assert!(login_headers.contains_key("Origin"));
    
    // Refresh no debe tener headers adicionales
    let refresh_headers = client.get_colis_headers("refresh", None, None);
    assert!(!refresh_headers.contains_key("Accept"));
    assert!(!refresh_headers.contains_key("Origin"));
    
    // Tournée debe tener headers específicos
    let tournee_headers = client.get_colis_headers("tournee", Some("user"), Some("token"));
    assert!(tournee_headers.contains_key("X-Requested-With"));
    assert!(tournee_headers.contains_key("X-Device-Info"));
}
