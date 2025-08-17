//! Shared application state
//! 
//! Este módulo define el estado compartido de la aplicación que se pasa
//! a través del router de Axum.

use sqlx::PgPool;
use crate::config::EnvironmentConfig;
use crate::cache::{RedisClient, AuthCache, TourneeCache};
use crate::migration::services::{MigrationService, MigrationConfig, MigrationStrategy};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: EnvironmentConfig,
    pub redis: RedisClient,
    pub auth_cache: AuthCache,
    pub tournee_cache: TourneeCache,
    pub migration_service: MigrationService,
}

impl AppState {
    pub fn new(pool: PgPool, config: EnvironmentConfig, redis: RedisClient) -> Self {
        let auth_cache = AuthCache::new(redis.clone());
        let tournee_cache = TourneeCache::new(redis.clone());
        
        // Crear configuración personalizada para iniciar con MobileOnly
        let mut migration_config = MigrationConfig::default();
        migration_config.current_strategy = MigrationStrategy::MobileOnly;
        migration_config.auto_progression = false; // Deshabilitar progresión automática
        
        let migration_service = MigrationService::new(
            migration_config,
            redis.clone(),
        );
        
        Self {
            pool,
            config,
            redis,
            auth_cache,
            tournee_cache,
            migration_service,
        }
    }
}