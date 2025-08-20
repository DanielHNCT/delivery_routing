package com.daniel.deliveryrouting.utils

import android.util.Log
import java.text.SimpleDateFormat
import java.util.*

/**
 * 🎯 LOGGER COMPREHENSIVO PARA COLIS PRIVÉ
 * 
 * Características:
 * - ✅ Logs estructurados por categorías
 * - ✅ Timestamps automáticos
 * - ✅ Niveles de log configurables
 * - ✅ Filtrado por tags
 * - ✅ Performance tracking
 * - ✅ Security (no log tokens completos)
 */
object ColisLogger {
    
    private const val TAG = "ColisApp"
    private const val MAX_TOKEN_PREVIEW = 20
    
    // 🎯 NIVELES DE LOG
    enum class LogLevel {
        VERBOSE, DEBUG, INFO, WARN, ERROR
    }
    
    // 🔧 CONFIGURACIÓN
    private var currentLogLevel = LogLevel.DEBUG
    private var enablePerformanceLogging = true
    private var enableSecurityLogging = true
    
    /**
     * 🔐 LOGS DE AUTENTICACIÓN
     */
    fun logAuthFlow(step: String, success: Boolean, details: String? = null) {
        val message = buildString {
            append("AUTH_FLOW | Step: $step | Success: $success")
            details?.let { append(" | Details: $it") }
        }
        
        if (success) {
            Log.d(TAG, "✅ $message")
        } else {
            Log.e(TAG, "❌ $message")
        }
    }
    
    fun logLoginAttempt(username: String, societe: String, success: Boolean, error: String? = null) {
        val message = buildString {
            append("LOGIN_ATTEMPT | Username: $username | Société: $societe | Success: $success")
            error?.let { append(" | Error: $it") }
        }
        
        if (success) {
            Log.i(TAG, "🔐 $message")
        } else {
            Log.e(TAG, "🚫 $message")
        }
    }
    
    fun logTokenEvent(event: String, tokenPreview: String? = null) {
        val preview = tokenPreview?.take(MAX_TOKEN_PREVIEW) ?: "null"
        val message = "TOKEN_EVENT | Event: $event | Token: $preview..."
        
        if (enableSecurityLogging) {
            Log.d(TAG, "🔑 $message")
        }
    }
    
    fun logTokenExpiry(expiryTime: Long, remainingMinutes: Long) {
        val message = "TOKEN_EXPIRY | Expires: ${formatTimestamp(expiryTime)} | Remaining: ${remainingMinutes}min"
        
        when {
            remainingMinutes <= 5 -> Log.w(TAG, "⚠️ $message")
            remainingMinutes <= 30 -> Log.i(TAG, "⏰ $message")
            else -> Log.d(TAG, "⏰ $message")
        }
    }
    
    /**
     * 📡 LOGS DE API
     */
    fun logApiCall(endpoint: String, method: String, statusCode: Int?, duration: Long) {
        val message = buildString {
            append("API_CALL | $method $endpoint")
            append(" | Status: $statusCode")
            append(" | Duration: ${duration}ms")
        }
        
        when {
            statusCode in 200..299 -> Log.d(TAG, "📡 $message")
            statusCode in 400..499 -> Log.w(TAG, "⚠️ $message")
            statusCode in 500..599 -> Log.e(TAG, "💥 $message")
            else -> Log.d(TAG, "📡 $message")
        }
        
        // Performance logging
        if (enablePerformanceLogging && duration > 5000) {
            Log.w(TAG, "🐌 SLOW_API_CALL | $endpoint took ${duration}ms")
        }
    }
    
    fun logApiError(endpoint: String, method: String, error: Throwable, responseCode: Int? = null) {
        val message = buildString {
            append("API_ERROR | $method $endpoint")
            responseCode?.let { append(" | Response: $it") }
            append(" | Error: ${error.message}")
        }
        
        Log.e(TAG, "💥 $message", error)
    }
    
    /**
     * 🔄 LOGS DE RETRY Y REFRESH
     */
    fun logRetryAttempt(operation: String, attempt: Int, maxAttempts: Int, reason: String? = null) {
        val message = buildString {
            append("RETRY_ATTEMPT | Operation: $operation | Attempt: $attempt/$maxAttempts")
            reason?.let { append(" | Reason: $it") }
        }
        
        when (attempt) {
            1 -> Log.i(TAG, "🔄 $message")
            else -> Log.w(TAG, "🔄 $message")
        }
    }
    
    fun logTokenRefresh(success: Boolean, oldTokenPreview: String? = null, newTokenPreview: String? = null) {
        val message = buildString {
            append("TOKEN_REFRESH | Success: $success")
            oldTokenPreview?.let { append(" | Old: ${it.take(MAX_TOKEN_PREVIEW)}...") }
            newTokenPreview?.let { append(" | New: ${it.take(MAX_TOKEN_PREVIEW)}...") }
        }
        
        if (success) {
            Log.i(TAG, "🔄 $message")
        } else {
            Log.w(TAG, "⚠️ $message")
        }
    }
    
    /**
     * 📦 LOGS DE TOURNÉE
     */
    fun logTourneeLoad(date: String, matricule: String, success: Boolean, packageCount: Int? = null) {
        val message = buildString {
            append("TOURNEE_LOAD | Date: $date | Matricule: $matricule | Success: $success")
            packageCount?.let { append(" | Packages: $it") }
        }
        
        if (success) {
            Log.i(TAG, "📦 $message")
        } else {
            Log.e(TAG, "❌ $message")
        }
    }
    
    fun logPackageProcessing(packageId: String, action: String, success: Boolean, details: String? = null) {
        val message = buildString {
            append("PACKAGE_PROCESSING | ID: $packageId | Action: $action | Success: $success")
            details?.let { append(" | Details: $it") }
        }
        
        if (success) {
            Log.d(TAG, "📦 $message")
        } else {
            Log.w(TAG, "⚠️ $message")
        }
    }
    
    /**
     * 🗺️ LOGS DE OPTIMIZACIÓN
     */
    fun logRouteOptimization(packageCount: Int, optimizationType: String, success: Boolean, savings: String? = null) {
        val message = buildString {
            append("ROUTE_OPTIMIZATION | Packages: $packageCount | Type: $optimizationType | Success: $success")
            savings?.let { append(" | Savings: $it") }
        }
        
        if (success) {
            Log.i(TAG, "🗺️ $message")
        } else {
            Log.e(TAG, "❌ $message")
        }
    }
    
    /**
     * 📱 LOGS DE UI
     */
    fun logUiEvent(screen: String, action: String, success: Boolean, details: String? = null) {
        val message = buildString {
            append("UI_EVENT | Screen: $screen | Action: $action | Success: $success")
            details?.let { append(" | Details: $it") }
        }
        
        if (success) {
            Log.d(TAG, "📱 $message")
        } else {
            Log.w(TAG, "⚠️ $message")
        }
    }
    
    fun logUserInteraction(element: String, action: String, additionalInfo: String? = null) {
        val message = buildString {
            append("USER_INTERACTION | Element: $element | Action: $action")
            additionalInfo?.let { append(" | Info: $it") }
        }
        
        Log.d(TAG, "👆 $message")
    }
    
    /**
     * 🔍 LOGS DE DEBUGGING
     */
    fun logDebug(message: String, data: Any? = null) {
        if (currentLogLevel <= LogLevel.DEBUG) {
            val fullMessage = buildString {
                append("DEBUG | $message")
                data?.let { append(" | Data: $it") }
            }
            Log.d(TAG, "🔍 $fullMessage")
        }
    }
    
    fun logVerbose(message: String, data: Any? = null) {
        if (currentLogLevel <= LogLevel.VERBOSE) {
            val fullMessage = buildString {
                append("VERBOSE | $message")
                data?.let { append(" | Data: $it") }
            }
            Log.v(TAG, "🔍 $fullMessage")
        }
    }
    
    /**
     * ⚠️ LOGS DE WARNING
     */
    fun logWarning(message: String, context: String? = null) {
        val fullMessage = buildString {
            append("WARNING | $message")
            context?.let { append(" | Context: $it") }
        }
        Log.w(TAG, "⚠️ $fullMessage")
    }
    
    /**
     * 💥 LOGS DE ERROR
     */
    fun logError(context: String, error: Throwable, additionalInfo: String? = null) {
        val message = buildString {
            append("ERROR | Context: $context")
            additionalInfo?.let { append(" | Info: $it") }
            append(" | Message: ${error.message}")
        }
        
        Log.e(TAG, "💥 $message", error)
    }
    
    fun logCriticalError(context: String, error: Throwable, userImpact: String? = null) {
        val message = buildString {
            append("CRITICAL_ERROR | Context: $context")
            userImpact?.let { append(" | User Impact: $it") }
            append(" | Message: ${error.message}")
        }
        
        Log.e(TAG, "🚨 $message", error)
    }
    
    /**
     * 📊 LOGS DE PERFORMANCE
     */
    fun logPerformance(operation: String, duration: Long, threshold: Long = 1000) {
        if (enablePerformanceLogging) {
            val message = "PERFORMANCE | Operation: $operation | Duration: ${duration}ms"
            
            when {
                duration > threshold * 2 -> Log.w(TAG, "🐌 $message (SLOW)")
                duration > threshold -> Log.w(TAG, "⚠️ $message (SLOW)")
                else -> Log.d(TAG, "⚡ $message")
            }
        }
    }
    
    fun logMemoryUsage(operation: String, memoryUsage: Long) {
        if (enablePerformanceLogging) {
            val message = "MEMORY_USAGE | Operation: $operation | Usage: ${memoryUsage / 1024 / 1024}MB"
            Log.d(TAG, "💾 $message")
        }
    }
    
    /**
     * 🌐 LOGS DE CONECTIVIDAD
     */
    fun logNetworkStatus(isOnline: Boolean, connectionType: String? = null) {
        val message = buildString {
            append("NETWORK_STATUS | Online: $isOnline")
            connectionType?.let { append(" | Type: $it") }
        }
        
        if (isOnline) {
            Log.i(TAG, "🌐 $message")
        } else {
            Log.w(TAG, "📡 $message")
        }
    }
    
    fun logBackendHealth(status: String, responseTime: Long? = null) {
        val message = buildString {
            append("BACKEND_HEALTH | Status: $status")
            responseTime?.let { append(" | Response Time: ${it}ms") }
        }
        
        when (status.lowercase()) {
            "healthy" -> Log.i(TAG, "🏥 $message")
            "degraded" -> Log.w(TAG, "⚠️ $message")
            "unhealthy" -> Log.e(TAG, "💥 $message")
            else -> Log.d(TAG, "🏥 $message")
        }
    }
    
    /**
     * 🔧 CONFIGURACIÓN
     */
    fun setLogLevel(level: LogLevel) {
        currentLogLevel = level
        Log.i(TAG, "🔧 LOG_LEVEL_CHANGED | New Level: $level")
    }
    
    fun enablePerformanceLogging(enable: Boolean) {
        enablePerformanceLogging = enable
        Log.i(TAG, "🔧 PERFORMANCE_LOGGING | Enabled: $enable")
    }
    
    fun enableSecurityLogging(enable: Boolean) {
        enableSecurityLogging = enable
        Log.i(TAG, "🔧 SECURITY_LOGGING | Enabled: $enable")
    }
    
    /**
     * 📅 UTILITY FUNCTIONS
     */
    private fun formatTimestamp(timestamp: Long): String {
        val formatter = SimpleDateFormat("HH:mm:ss", Locale.getDefault())
        return formatter.format(Date(timestamp))
    }
    
    /**
     * 🧹 CLEANUP
     */
    fun cleanup() {
        Log.i(TAG, "🧹 LOGGER_CLEANUP | Cleaning up logger resources")
        // Aquí podrías agregar cleanup de archivos de log, etc.
    }
}

/**
 * 🎯 EXTENSION FUNCTIONS PARA LOGGING CONVENIENTE
 */
fun Any.logDebug(message: String) = ColisLogger.logDebug(message, this)
fun Any.logError(context: String, error: Throwable) = ColisLogger.logError(context, error)
fun Any.logPerformance(operation: String, duration: Long) = ColisLogger.logPerformance(operation, duration)
