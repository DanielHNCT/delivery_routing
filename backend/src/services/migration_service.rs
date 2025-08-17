use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn, error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::external_models::{MobileTourneeRequest, MobileTourneeResponse};
use crate::cache::{AuthCache, TourneeCache, RedisClient, CacheConfig};

/// Estrategia de migración para decidir qué API usar
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MigrationStrategy {
    /// 100% API Web (estado inicial)
    WebOnly,
    /// 20% API Móvil, 80% API Web
    Mobile20,
    /// 50% API Móvil, 50% API Web
    Mobile50,
    /// 80% API Móvil, 20% API Web (fallback)
    Mobile80,
    /// 100% API Móvil, API Web solo para emergencias
    MobileOnly,
}

impl MigrationStrategy {
    /// Obtener el porcentaje de tráfico para API móvil
    pub fn mobile_percentage(&self) -> f64 {
        match self {
            MigrationStrategy::WebOnly => 0.0,
            MigrationStrategy::Mobile20 => 0.2,
            MigrationStrategy::Mobile50 => 0.5,
            MigrationStrategy::Mobile80 => 0.8,
            MigrationStrategy::MobileOnly => 1.0,
        }
    }
    
    /// Obtener el porcentaje de tráfico para API web
    pub fn web_percentage(&self) -> f64 {
        1.0 - self.mobile_percentage()
    }
    
    /// Obtener la siguiente estrategia en la secuencia
    pub fn next(&self) -> Option<Self> {
        match self {
            MigrationStrategy::WebOnly => Some(MigrationStrategy::Mobile20),
            MigrationStrategy::Mobile20 => Some(MigrationStrategy::Mobile50),
            MigrationStrategy::Mobile50 => Some(MigrationStrategy::Mobile80),
            MigrationStrategy::Mobile80 => Some(MigrationStrategy::MobileOnly),
            MigrationStrategy::MobileOnly => None,
        }
    }
    
    /// Obtener la estrategia anterior
    pub fn previous(&self) -> Option<Self> {
        match self {
            MigrationStrategy::WebOnly => None,
            MigrationStrategy::Mobile20 => Some(MigrationStrategy::WebOnly),
            MigrationStrategy::Mobile50 => Some(MigrationStrategy::Mobile20),
            MigrationStrategy::Mobile80 => Some(MigrationStrategy::Mobile50),
            MigrationStrategy::MobileOnly => Some(MigrationStrategy::Mobile80),
        }
    }
}

/// Configuración de migración
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    pub current_strategy: MigrationStrategy,
    pub auto_progression: bool,
    pub progression_threshold: f64, // Tasa de éxito mínima para progresar
    pub rollback_threshold: f64,    // Tasa de éxito mínima para no hacer rollback
    pub min_requests_before_progression: u32,
    pub health_check_interval_seconds: u64,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            current_strategy: MigrationStrategy::WebOnly,
            auto_progression: true,
            progression_threshold: 0.95, // 95% de éxito para progresar
            rollback_threshold: 0.90,    // 90% de éxito para no hacer rollback
            min_requests_before_progression: 100,
            health_check_interval_seconds: 300, // 5 minutos
        }
    }
}

/// Métricas de migración por estrategia
#[derive(Debug, Clone, Default)]
pub struct MigrationMetrics {
    pub total_requests: u32,
    pub successful_requests: u32,
    pub failed_requests: u32,
    pub avg_response_time_ms: f64,
    pub last_updated: u64,
}

impl MigrationMetrics {
    /// Calcular tasa de éxito
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.successful_requests as f64 / self.total_requests as f64
        }
    }
    
    /// Calcular tasa de fallo
    pub fn failure_rate(&self) -> f64 {
        1.0 - self.success_rate()
    }
    
    /// Actualizar métricas con un nuevo request
    pub fn update(&mut self, success: bool, response_time_ms: f64) {
        self.total_requests += 1;
        
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
        
        // Actualizar tiempo de respuesta promedio
        let total_time = self.avg_response_time_ms * (self.total_requests - 1) as f64 + response_time_ms;
        self.avg_response_time_ms = total_time / self.total_requests as f64;
        
        self.last_updated = chrono::Utc::now().timestamp() as u64;
    }
}

/// Servicio de migración gradual
pub struct MigrationService {
    config: MigrationConfig,
    metrics: Arc<RwLock<HashMap<MigrationStrategy, MigrationMetrics>>>,
    cache: Arc<RedisClient>,
    auth_cache: Arc<AuthCache>,
    tournee_cache: Arc<TourneeCache>,
}

impl MigrationService {
    /// Crear nuevo servicio de migración
    pub fn new(
        config: MigrationConfig,
        cache: RedisClient,
    ) -> Self {
        let auth_cache = AuthCache::new(cache.clone());
        let tournee_cache = TourneeCache::new(cache.clone());
        
        Self {
            config,
            metrics: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(cache),
            auth_cache: Arc::new(auth_cache),
            tournee_cache: Arc::new(tournee_cache),
        }
    }
    
    /// Decidir qué API usar para un request específico
    pub async fn decide_api_strategy(&self, request: &MobileTourneeRequest) -> MigrationStrategy {
        let strategy = self.config.current_strategy;
        
        // Si es 100% web o 100% móvil, no hay decisión que tomar
        if strategy == MigrationStrategy::WebOnly {
            return MigrationStrategy::WebOnly;
        }
        if strategy == MigrationStrategy::MobileOnly {
            return MigrationStrategy::MobileOnly;
        }
        
        // Para estrategias mixtas, usar un hash determinístico del request
        let hash = self.hash_request(request);
        let hash_percentage = (hash % 100) as f64 / 100.0;
        
        if hash_percentage < strategy.mobile_percentage() {
            MigrationStrategy::MobileOnly
        } else {
            MigrationStrategy::WebOnly
        }
    }
    
    /// Hash determinístico del request para routing consistente
    fn hash_request(&self, request: &MobileTourneeRequest) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        request.username.hash(&mut hasher);
        request.societe.hash(&mut hasher);
        request.matricule.hash(&mut hasher);
        request.date.hash(&mut hasher);
        
        hasher.finish() as u32
    }
    
    /// Registrar métricas de un request
    pub async fn record_metrics(
        &self,
        strategy: MigrationStrategy,
        success: bool,
        response_time_ms: f64,
    ) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        
        let entry = metrics.entry(strategy).or_insert_with(MigrationMetrics::default);
        entry.update(success, response_time_ms);
        
        debug!("📊 Métricas actualizadas para estrategia {:?}: éxito={}, tiempo={}ms", strategy, success, response_time_ms);
        
        Ok(())
    }
    
    /// Evaluar si se debe progresar a la siguiente estrategia
    pub async fn evaluate_progression(&self) -> Result<Option<MigrationStrategy>> {
        if !self.config.auto_progression {
            return Ok(None);
        }
        
        let current_metrics = self.get_strategy_metrics(self.config.current_strategy).await?;
        
        // Verificar si hay suficientes requests para evaluar
        if current_metrics.total_requests < self.config.min_requests_before_progression {
            debug!("📊 Insuficientes requests para evaluar progresión: {}/{}", 
                   current_metrics.total_requests, 
                   self.config.min_requests_before_progression);
            return Ok(None);
        }
        
        let success_rate = current_metrics.success_rate();
        
        // Si la tasa de éxito es alta, progresar
        if success_rate >= self.config.progression_threshold {
            if let Some(next_strategy) = self.config.current_strategy.next() {
                info!("🚀 Progresando a estrategia {:?} (tasa de éxito: {:.2}%)", 
                      next_strategy, success_rate * 100.0);
                return Ok(Some(next_strategy));
            }
        }
        
        // Si la tasa de éxito es muy baja, considerar rollback
        if success_rate < self.config.rollback_threshold {
            if let Some(prev_strategy) = self.config.current_strategy.previous() {
                warn!("⚠️ Considerando rollback a estrategia {:?} (tasa de éxito: {:.2}%)", 
                      prev_strategy, success_rate * 100.0);
                return Ok(Some(prev_strategy));
            }
        }
        
        Ok(None)
    }
    
    /// Cambiar a una nueva estrategia
    pub async fn change_strategy(&mut self, new_strategy: MigrationStrategy) -> Result<()> {
        let old_strategy = self.config.current_strategy;
        
        info!("🔄 Cambiando estrategia de {:?} a {:?}", old_strategy, new_strategy);
        
        // Guardar configuración en Redis
        self.save_config_to_cache(&new_strategy).await?;
        
        // Actualizar configuración local
        self.config.current_strategy = new_strategy;
        
        // Limpiar métricas de la estrategia anterior
        self.clear_strategy_metrics(old_strategy).await?;
        
        info!("✅ Estrategia cambiada exitosamente");
        Ok(())
    }
    
    /// Obtener métricas de una estrategia específica
    pub async fn get_strategy_metrics(&self, strategy: MigrationStrategy) -> Result<MigrationMetrics> {
        let metrics = self.metrics.read().await;
        
        Ok(metrics.get(&strategy).cloned().unwrap_or_default())
    }
    
    /// Obtener métricas de todas las estrategias
    pub async fn get_all_metrics(&self) -> Result<HashMap<MigrationStrategy, MigrationMetrics>> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }
    
    /// Obtener resumen de migración
    pub async fn get_migration_summary(&self) -> Result<MigrationSummary> {
        let all_metrics = self.get_all_metrics().await?;
        let current_metrics = self.get_strategy_metrics(self.config.current_strategy).await?;
        
        let summary = MigrationSummary {
            current_strategy: self.config.current_strategy,
            config: self.config.clone(),
            current_metrics,
            all_metrics,
            last_updated: chrono::Utc::now().timestamp() as u64,
        };
        
        Ok(summary)
    }
    
    /// Guardar configuración en cache
    async fn save_config_to_cache(&self, strategy: &MigrationStrategy) -> Result<()> {
        let config_key = "migration:config";
        let config_data = serde_json::to_string(&self.config)?;
        
        self.cache.set(config_key, &config_data, 86400).await?; // 24 horas
        
        Ok(())
    }
    
    /// Limpiar métricas de una estrategia
    async fn clear_strategy_metrics(&self, strategy: MigrationStrategy) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        metrics.remove(&strategy);
        
        debug!("🧹 Métricas limpiadas para estrategia {:?}", strategy);
        Ok(())
    }
    
    /// Simular actividad para testing
    pub async fn simulate_activity(&self) -> Result<()> {
        // Crear actividad falsa en el cache para camuflaje
        self.auth_cache.simulate_user_activity().await?;
        self.tournee_cache.simulate_tournee_activity().await?;
        
        debug!("🎭 Actividad simulada para camuflaje");
        Ok(())
    }
}

/// Resumen de migración
#[derive(Debug, Serialize)]
pub struct MigrationSummary {
    pub current_strategy: MigrationStrategy,
    pub config: MigrationConfig,
    pub current_metrics: MigrationMetrics,
    pub all_metrics: HashMap<MigrationStrategy, MigrationMetrics>,
    pub last_updated: u64,
}
