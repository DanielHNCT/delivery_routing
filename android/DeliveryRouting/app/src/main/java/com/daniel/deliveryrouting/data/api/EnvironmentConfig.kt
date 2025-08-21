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
                // ✅ DETECCIÓN AUTOMÁTICA DEL DISPOSITIVO
                getBackendUrlForDevice()
            }
            Environment.PRODUCTION -> {
                // Colis Privé directo (no recomendado, perdemos optimización)
                "https://wsauthentificationexterne.colisprive.com"
            }
        }
    }
    
    /**
     * 🌐 OBTENER URL DEL BACKEND SEGÚN EL DISPOSITIVO
     */
    private fun getBackendUrlForDevice(): String {
        return if (android.os.Build.MODEL.contains("D5503") || 
                   android.os.Build.MODEL.contains("Sony") ||
                   android.os.Build.MANUFACTURER.contains("Sony")) {
            // Es tu Sony Xperia Z1 - usar IP real del backend
            "http://192.168.1.9:3000"  // ✅ BACKEND REAL EN 192.168.1.9
        } else if (android.os.Build.FINGERPRINT.contains("generic") || 
                   android.os.Build.FINGERPRINT.contains("unknown") ||
                   android.os.Build.MODEL.contains("google_sdk") ||
                   android.os.Build.MODEL.contains("Emulator") ||
                   android.os.Build.MODEL.contains("Android SDK built for x86")) {
            // Es un emulador
            "http://10.0.2.2:3000"
        } else {
            // Otros dispositivos físicos
            "http://192.168.1.9:3000"  // ✅ BACKEND REAL EN 192.168.1.9
        }
    }
    
    /**
     * Obtiene los endpoints según el entorno
     */
    fun getAuthEndpoint(): String {
        return when (currentEnvironment) {
            Environment.DEVELOPMENT -> "/api/colis-prive/mobile-tournee-with-retry"  // ✅ CAMBIADO: Endpoint que acepta device_info real
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
        
        // ✅ AGREGADO: Información del dispositivo
        Log.d("EnvironmentConfig", "📱 Device Model: ${android.os.Build.MODEL}")
        Log.d("EnvironmentConfig", "📱 Device Manufacturer: ${android.os.Build.MANUFACTURER}")
        Log.d("EnvironmentConfig", "📱 Device Fingerprint: ${android.os.Build.FINGERPRINT}")
        
        if (isUsingLocalBackend()) {
            Log.d("EnvironmentConfig", "✅ PERFECTO: Usando backend local para optimización de rutas")
        } else {
            Log.w("EnvironmentConfig", "⚠️ ADVERTENCIA: Usando Colis Privé directo, sin optimización")
        }
    }
}

