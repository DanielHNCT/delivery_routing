package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.api.EnvironmentConfig

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add map-specific API endpoints if needed
// 2. Add location-based API configuration

object ApiConfig {
    // 🎯 CONFIGURACIÓN ACTUALIZADA: Usar backend local para optimización
    val BASE_URL = EnvironmentConfig.getBaseUrl() + "/"
    const val API_TIMEOUT_SECONDS = 45L
    
    // Endpoints que van a través de tu backend local
    const val LOGIN_ENDPOINT = "api/colis-prive/auth"
    const val TOURNEE_ENDPOINT = "api/colis-prive/mobile-tournee-structured"
    const val TOURNEE_UPDATE_ENDPOINT = "api/colis-prive/mobile-tournee-updated"
    const val HEALTH_ENDPOINT = "api/colis-prive/health"
    
    // 📊 Nuevos endpoints para optimización de rutas
    const val OPTIMIZE_ROUTE_ENDPOINT = "api/route-optimization/optimize"
    const val ANALYTICS_ENDPOINT = "api/analytics/delivery-metrics"
    const val CONFIG_ENDPOINT = "api/config/app-settings"
    
    init {
        // Log de configuración al inicializar
        EnvironmentConfig.logCurrentConfig()
    }
}
