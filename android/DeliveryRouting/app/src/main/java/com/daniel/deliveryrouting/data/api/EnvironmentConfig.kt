package com.daniel.deliveryrouting.data.api

import android.util.Log

/**
 * Configuración de entorno para la aplicación
 * Controla si usar el backend local (para optimización) o Colis Privé directo
 */
object EnvironmentConfig {
    
    enum class Environment {
        DEVELOPMENT,  // Usa backend local para optimización de rutas
        PRODUCTION   // Usa Colis Privé directo (si es necesario)
    }
    
    // 🎯 CONFIGURACIÓN ACTUAL: DEVELOPMENT para usar tu backend local
    val currentEnvironment = Environment.DEVELOPMENT
    
    /**
     * Obtiene la URL base según el entorno
     */
    fun getBaseUrl(): String {
        return when (currentEnvironment) {
            Environment.DEVELOPMENT -> {
                // Para emulador Android Studio
                "http://10.0.2.2:3000"
                
                // Para dispositivo físico en misma red, usar:
                // "http://192.168.1.9:3000"  // IP de tu máquina
            }
            Environment.PRODUCTION -> {
                // Colis Privé directo (no recomendado, perdemos optimización)
                "https://wsauthentificationexterne.colisprive.com"
            }
        }
    }
    
    /**
     * Obtiene los endpoints según el entorno
     */
    fun getAuthEndpoint(): String {
        return when (currentEnvironment) {
            Environment.DEVELOPMENT -> "/api/colis-prive/auth"  // Tu backend
            Environment.PRODUCTION -> "/api/auth/login/Membership"  // Colis Privé directo
        }
    }
    
    fun getTourneeEndpoint(): String {
        return when (currentEnvironment) {
            Environment.DEVELOPMENT -> "/api/colis-prive/mobile-tournee-updated"  // Tu backend
            Environment.PRODUCTION -> "/WS-TourneeColis/api/getListTourneeMobileByMatriculeDistributeurDateDebut_POST"  // Colis Privé directo
        }
    }
    
    /**
     * Verifica si estamos usando el backend local (recomendado)
     */
    fun isUsingLocalBackend(): Boolean {
        return currentEnvironment == Environment.DEVELOPMENT
    }
    
    /**
     * Log de configuración actual
     */
    fun logCurrentConfig() {
        Log.d("EnvironmentConfig", "=== CONFIGURACIÓN DE ENTORNO ===")
        Log.d("EnvironmentConfig", "Entorno: $currentEnvironment")
        Log.d("EnvironmentConfig", "Base URL: ${getBaseUrl()}")
        Log.d("EnvironmentConfig", "Auth Endpoint: ${getAuthEndpoint()}")
        Log.d("EnvironmentConfig", "Tournée Endpoint: ${getTourneeEndpoint()}")
        Log.d("EnvironmentConfig", "Usando backend local: ${isUsingLocalBackend()}")
        
        if (isUsingLocalBackend()) {
            Log.d("EnvironmentConfig", "✅ PERFECTO: Usando backend local para optimización de rutas")
        } else {
            Log.w("EnvironmentConfig", "⚠️ ADVERTENCIA: Usando Colis Privé directo, sin optimización")
        }
    }
}
