//! Shared application state
//! 
//! Este módulo define el estado compartido de la aplicación que se pasa
//! a través del router de Axum.

use sqlx::PgPool;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::EnvironmentConfig;
use crate::cache::RedisClient;

/// Estructura para almacenar tokens de autenticación
#[derive(Clone, Debug)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub username: String,
    pub societe: String,
}

impl AuthToken {
    pub fn new(token: String, username: String, societe: String, expires_in_hours: i32) -> Self {
        Self {
            token,
            expires_at: chrono::Utc::now() + chrono::Duration::hours(expires_in_hours as i64),
            username,
            societe,
        }
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: EnvironmentConfig,
    pub redis: RedisClient,
    pub http_client: Client,
    pub auth_tokens: Arc<RwLock<HashMap<String, AuthToken>>>,
}

impl AppState {
    pub fn new(pool: PgPool, config: EnvironmentConfig, redis: RedisClient) -> Self {
        Self {
            pool,
            config,
            redis,
            http_client: Client::new(),
            auth_tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Obtener token de autenticación para un usuario específico
    pub async fn get_auth_token(&self, username: &str, societe: &str) -> Option<AuthToken> {
        let key = format!("{}:{}", societe, username);
        let tokens = self.auth_tokens.read().await;
        tokens.get(&key).cloned()
    }

    /// Almacenar token de autenticación
    pub async fn store_auth_token(&self, username: String, societe: String, token: String, expires_in_hours: i32) {
        let key = format!("{}:{}", societe, username);
        let auth_token = AuthToken::new(token, username, societe, expires_in_hours);
        let mut tokens = self.auth_tokens.write().await;
        tokens.insert(key, auth_token);
    }

    /// Limpiar tokens expirados
    pub async fn cleanup_expired_tokens(&self) {
        let mut tokens = self.auth_tokens.write().await;
        tokens.retain(|_, token| !token.is_expired());
    }
}