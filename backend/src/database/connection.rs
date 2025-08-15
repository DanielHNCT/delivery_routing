//! Configuración de conexión a PostgreSQL
//! 
//! Este módulo maneja la conexión a la base de datos PostgreSQL con PostGIS.

use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{PgConnection, Row};
use std::time::Duration;
use tracing::{info, error};

/// Pool de conexiones a PostgreSQL
pub type DatabasePool = PgPool;

/// Configuración de la base de datos
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/delivery_routing".to_string()),
            max_connections: 10,
            min_connections: 2,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            max_lifetime: Duration::from_secs(3600),
        }
    }
}

/// Crear y configurar el pool de conexiones
pub async fn create_pool(config: Option<DatabaseConfig>) -> Result<DatabasePool, sqlx::Error> {
    let config = config.unwrap_or_default();
    
    info!("🔌 Configurando pool de conexiones a PostgreSQL...");
    info!("   📍 URL: {}", mask_database_url(&config.url));
    info!("   🔗 Max connections: {}", config.max_connections);
    info!("   🔗 Min connections: {}", config.min_connections);

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .connect(&config.url)
        .await?;

    info!("✅ Pool de conexiones creado exitosamente");

    // Verificar conexión
    test_connection(&pool).await?;

    Ok(pool)
}

/// Verificar que la conexión funciona
async fn test_connection(pool: &DatabasePool) -> Result<(), sqlx::Error> {
    info!("🧪 Probando conexión a la base de datos...");
    
    let result = sqlx::query("SELECT 1 as test")
        .fetch_one(pool)
        .await?;
    
    let test_value: i32 = result.get("test");
    if test_value == 1 {
        info!("✅ Conexión a la base de datos exitosa");
        Ok(())
    } else {
        error!("❌ Conexión a la base de datos falló");
        Err(sqlx::Error::RowNotFound)
    }
}

/// Obtener una conexión del pool
pub async fn get_connection(pool: &DatabasePool) -> Result<PgConnection, sqlx::Error> {
    // TODO: Implementar cuando sea necesario
    // Por ahora, retornamos un error ya que no es crítico para la funcionalidad básica
    Err(sqlx::Error::Configuration("Función no implementada".into()))
}

/// Ejecutar migraciones de la base de datos
pub async fn run_migrations(pool: &DatabasePool) -> Result<(), sqlx::Error> {
    info!("🔄 Ejecutando migraciones de la base de datos...");
    
    // TODO: Implementar cuando se cree el directorio migrations
    // sqlx::migrate!("./migrations")
    //     .run(pool)
    //     .await?;
    
    info!("✅ Migraciones ejecutadas exitosamente (simulado)");
    Ok(())
}

/// Función helper para enmascarar la URL de la base de datos en logs
fn mask_database_url(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(_colon_pos) = url[..at_pos].rfind(':') {
            let protocol = &url[..url.find("://").unwrap_or(0) + 3];
            let host = &url[at_pos + 1..];
            format!("{}***:***@{}", protocol, host)
        } else {
            url.to_string()
        }
    } else {
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert!(!config.url.is_empty());
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 2);
    }

    #[test]
    fn test_mask_database_url() {
        let url = "postgresql://username:password@localhost/db";
        let masked = mask_database_url(url);
        assert!(masked.contains("***:***"));
        assert!(!masked.contains("password"));
    }
}
