use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::json;
use delivery_optimizer::main;

#[tokio::test]
async fn test_health_check() {
    let app = create_test_app().await;
    let response = app.get("/api/colis-prive/health").await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: serde_json::Value = response.json();
    assert_eq!(body["service"], "colis-prive");
    assert_eq!(body["status"], "healthy");
}

#[tokio::test]
async fn test_auth_endpoint_invalid_credentials() {
    let app = create_test_app().await;
    let response = app
        .post("/api/colis-prive/auth")
        .json(&json!({
            "username": "invalid_user",
            "password": "invalid_password",
            "societe": "INVALID_SOCIETE"
        }))
        .await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: serde_json::Value = response.json();
    assert_eq!(body["success"], false);
    assert_eq!(body["error"]["code"], "AUTH_FAILED");
}

#[tokio::test]
async fn test_tournee_endpoint_invalid_credentials() {
    let app = create_test_app().await;
    let response = app
        .post("/api/colis-prive/tournee")
        .json(&json!({
            "username": "invalid_user",
            "password": "invalid_password",
            "societe": "INVALID_SOCIETE",
            "date": "2025-08-18",
            "matricule": "INVALID_MATRICULE"
        }))
        .await;
    
    // Debería fallar pero no dar error 500
    assert_ne!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_mobile_tournee_endpoint_invalid_credentials() {
    let app = create_test_app().await;
    let response = app
        .post("/api/colis-prive/mobile-tournee")
        .json(&json!({
            "username": "invalid_user",
            "password": "invalid_password",
            "societe": "INVALID_SOCIETE",
            "date": "2025-08-18",
            "matricule": "INVALID_MATRICULE"
        }))
        .await;
    
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: serde_json::Value = response.json();
    assert_eq!(body["success"], false);
    assert_eq!(body["endpoint_used"], "mobile");
    assert_eq!(body["total_packages"], 0);
}

#[tokio::test]
async fn test_endpoint_comparison() {
    let app = create_test_app().await;
    
    // Test data común
    let test_data = json!({
        "username": "test_user",
        "password": "test_password",
        "societe": "PCP0010699",
        "date": "2025-08-18",
        "matricule": "PCP0010699_A187518"
    });
    
    // Probar endpoint web
    let web_response = app
        .post("/api/colis-prive/tournee")
        .json(&test_data)
        .await;
    
    // Probar endpoint móvil
    let mobile_response = app
        .post("/api/colis-prive/mobile-tournee")
        .json(&test_data)
        .await;
    
    // Ambos deberían responder (aunque fallen por credenciales inválidas)
    assert_ne!(web_response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_ne!(mobile_response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    
    // Verificar que el endpoint móvil tenga la estructura correcta
    let mobile_body: serde_json::Value = mobile_response.json();
    assert!(mobile_body.is_object());
    assert!(mobile_body.get("endpoint_used").is_some());
    assert!(mobile_body.get("total_packages").is_some());
}

// Función helper para crear la app de test
async fn create_test_app() -> TestServer {
    // Aquí deberías crear una instancia de tu app para testing
    // Por ahora, esto es un placeholder
    todo!("Implementar creación de app de test")
}

