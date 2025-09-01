//! Shared application state
//! 
//! Este módulo define el estado compartido de la aplicación que se pasa
//! a través del router de Axum.

use sqlx::PgPool;
use reqwest::Client;
use crate::config::EnvironmentConfig;
use crate::cache::RedisClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: EnvironmentConfig,
    pub redis: RedisClient,
    pub http_client: Client,
}

impl AppState {
    pub fn new(pool: PgPool, config: EnvironmentConfig, redis: RedisClient) -> Self {
        Self {
            pool,
            config,
            redis,
            http_client: Client::new(),
        }
    }
}