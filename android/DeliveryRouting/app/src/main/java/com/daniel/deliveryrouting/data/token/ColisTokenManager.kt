package com.daniel.deliveryrouting.data.token

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import com.daniel.deliveryrouting.data.api.models.BackendAuthResponse
import com.daniel.deliveryrouting.data.api.models.ColisAuthResponse
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import java.util.concurrent.TimeUnit
import java.util.Date
import java.util.Locale

/**
 * 🔐 TOKEN MANAGER COMPLETO PARA COLIS PRIVÉ
 * 
 * Características:
 * - ✅ Compatible con BackendAuthResponse
 * - ✅ Auto-refresh automático
 * - ✅ Thread-safe con Mutex
 * - ✅ Persistencia en SharedPreferences
 * - ✅ Extracción automática de username y societe
 * - ✅ Logs seguros (tokens truncados)
 * - ✅ Funciones de testing
 */
class ColisTokenManager(context: Context) {
    
    private val prefs: SharedPreferences = context.getSharedPreferences(
        "colis_tokens", Context.MODE_PRIVATE
    )
    private val mutex = Mutex()
    
    companion object {
        private const val TAG = "ColisTokenManager"
        private const val KEY_TOKEN = "sso_hopps_token"
        private const val KEY_EXPIRY = "token_expiry"
        private const val KEY_MATRICULE = "matricule"
        private const val KEY_USERNAME = "username"
        private const val KEY_SOCIETE = "societe"
        private const val KEY_LAST_REFRESH_TIME = "last_refresh_time"
        private const val KEY_INSTALL_TIME = "install_time"
        private const val TOKEN_VALIDITY_HOURS = 1L // Tokens válidos por 1 hora
        private const val MAX_TOKEN_LOG_LENGTH = 50 // Máximo caracteres para logs
    }
    
    /**
     * 🔑 GUARDAR TOKENS DESPUÉS DE LOGIN EXITOSO
     */
    suspend fun saveTokens(response: BackendAuthResponse) = mutex.withLock {
        try {
            val currentTime = System.currentTimeMillis()
            val expiryTime = currentTime + TimeUnit.HOURS.toMillis(TOKEN_VALIDITY_HOURS)
            
            // Extraer username y societe del matricule
            val matricule = response.authentication.matricule ?: ""
            val username = extractUsernameFromMatricule(matricule)
            val societe = extractSocieteFromMatricule(matricule)
            
            prefs.edit()
                .putString(KEY_TOKEN, response.authentication.token)
                .putLong(KEY_EXPIRY, expiryTime)
                .putString(KEY_MATRICULE, matricule)
                .putString(KEY_USERNAME, username)
                .putString(KEY_SOCIETE, societe)
                .putLong(KEY_LAST_REFRESH_TIME, currentTime)
                .putLong(KEY_INSTALL_TIME, getInstallTime())
                .apply()
                
            Log.d(TAG, "🔑 === TOKENS GUARDADOS EXITOSAMENTE ===")
            Log.d(TAG, "Token: ${response.authentication.token?.take(MAX_TOKEN_LOG_LENGTH)}...")
            Log.d(TAG, "Matricule: $matricule")
            Log.d(TAG, "Username extraído: $username")
            Log.d(TAG, "Societe extraída: $societe")
            Log.d(TAG, "Expira: ${java.util.Date(expiryTime)}")
            Log.d(TAG, "=== FIN TOKENS GUARDADOS ===")
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error guardando tokens: ${e.message}", e)
        }
    }
    
    /**
     * 🔑 GUARDAR TOKENS (LEGACY - ColisAuthResponse)
     */
    suspend fun saveTokens(response: ColisAuthResponse) = mutex.withLock {
        try {
            val currentTime = System.currentTimeMillis()
            val expiryTime = currentTime + TimeUnit.HOURS.toMillis(TOKEN_VALIDITY_HOURS)
            
            val matricule = response.matricule ?: ""
            val username = extractUsernameFromMatricule(matricule)
            val societe = extractSocieteFromMatricule(matricule)
            
            prefs.edit()
                .putString(KEY_TOKEN, response.tokens.ssoHopps)
                .putLong(KEY_EXPIRY, expiryTime)
                .putString(KEY_MATRICULE, matricule)
                .putString(KEY_USERNAME, username)
                .putString(KEY_SOCIETE, societe)
                .putLong(KEY_LAST_REFRESH_TIME, currentTime)
                .putLong(KEY_INSTALL_TIME, getInstallTime())
                .apply()
                
            Log.d(TAG, "🔑 === TOKENS GUARDADOS (LEGACY) ===")
            Log.d(TAG, "Token: ${response.tokens.ssoHopps.take(MAX_TOKEN_LOG_LENGTH)}...")
            Log.d(TAG, "Matricule: $matricule")
            Log.d(TAG, "Username extraído: $username")
            Log.d(TAG, "Societe extraída: $societe")
            Log.d(TAG, "=== FIN TOKENS GUARDADOS ===")
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error guardando tokens legacy: ${e.message}", e)
        }
    }
    
    /**
     * ✅ OBTENER TOKEN VÁLIDO (solo si no ha expirado)
     */
    suspend fun getValidToken(): String? = mutex.withLock {
        try {
            val token = prefs.getString(KEY_TOKEN, null)
            val expiry = prefs.getLong(KEY_EXPIRY, 0)
            val currentTime = System.currentTimeMillis()
            
            return if (token != null && currentTime < expiry) {
                val remainingMinutes = TimeUnit.MILLISECONDS.toMinutes(expiry - currentTime)
                Log.d(TAG, "✅ Token válido encontrado - Restan $remainingMinutes minutos")
                token
            } else {
                if (token != null) {
                    Log.w(TAG, "❌ Token expirado - Expiry: ${java.util.Date(expiry)}, Actual: ${java.util.Date(currentTime)}")
                } else {
                    Log.w(TAG, "❌ No hay token guardado")
                }
                null
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo token válido: ${e.message}", e)
            null
        }
    }
    
    /**
     * 🔍 VERIFICAR SI TOKEN ES VÁLIDO (sin devolver token)
     */
    suspend fun isTokenValid(): Boolean = mutex.withLock {
        return try {
            val token = prefs.getString(KEY_TOKEN, null)
            val expiry = prefs.getLong(KEY_EXPIRY, 0)
            val currentTime = System.currentTimeMillis()
            
            val isValid = token != null && currentTime < expiry
            Log.d(TAG, "🔍 Token válido: $isValid")
            isValid
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error verificando validez del token: ${e.message}", e)
            false
        }
    }
    
    /**
     * 🔄 OBTENER TOKEN PARA REFRESH (incluso si expiró)
     */
    suspend fun getTokenForRefresh(): String? = mutex.withLock {
        return try {
            val token = prefs.getString(KEY_TOKEN, null)
            if (token != null) {
                Log.d(TAG, "🔄 Token para refresh encontrado: ${token.take(MAX_TOKEN_LOG_LENGTH)}...")
            } else {
                Log.w(TAG, "❌ No hay token para refresh")
            }
            token
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo token para refresh: ${e.message}", e)
            null
        }
    }
    
    /**
     * 🗑️ LIMPIAR TODOS LOS TOKENS
     */
    suspend fun clearTokens() = mutex.withLock {
        try {
            prefs.edit()
                .remove(KEY_TOKEN)
                .remove(KEY_EXPIRY)
                .remove(KEY_MATRICULE)
                .remove(KEY_USERNAME)
                .remove(KEY_SOCIETE)
                .remove(KEY_LAST_REFRESH_TIME)
                .apply()
            Log.d(TAG, "🗑️ === TOKENS ELIMINADOS COMPLETAMENTE ===")
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error limpiando tokens: ${e.message}", e)
        }
    }
    
    /**
     * 👥 OBTENER DATOS DE USUARIO GUARDADOS
     */
    suspend fun getSavedUserData(): UserTokenData? = mutex.withLock {
        return try {
            val matricule = prefs.getString(KEY_MATRICULE, null)
            val username = prefs.getString(KEY_USERNAME, null)
            val societe = prefs.getString(KEY_SOCIETE, null)
            val lastRefreshTime = prefs.getLong(KEY_LAST_REFRESH_TIME, 0)
            
            if (matricule != null && username != null && societe != null) {
                Log.d(TAG, "👥 Datos de usuario obtenidos: $matricule, $username, $societe")
                UserTokenData(
                    matricule = matricule,
                    username = username,
                    societe = societe,
                    lastRefreshTime = lastRefreshTime
                )
            } else {
                Log.w(TAG, "⚠️ Datos de usuario incompletos: matricule=$matricule, username=$username, societe=$societe")
                null
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo datos de usuario: ${e.message}", e)
            null
        }
    }
    
    /**
     * ⏰ OBTENER INFORMACIÓN DETALLADA DE EXPIRACIÓN
     */
    suspend fun getTokenExpirationInfo(): TokenExpirationInfo = mutex.withLock {
        return try {
            val token = prefs.getString(KEY_TOKEN, null)
            val expiry = prefs.getLong(KEY_EXPIRY, 0)
            val currentTime = System.currentTimeMillis()
            
            val isExpired = token == null || currentTime >= expiry
            val minutesUntilExpiry = if (!isExpired) {
                TimeUnit.MILLISECONDS.toMinutes(expiry - currentTime)
            } else 0
            
            TokenExpirationInfo(
                expiryTime = expiry,
                currentTime = currentTime,
                isExpired = isExpired,
                minutesUntilExpiry = minutesUntilExpiry,
                hasToken = token != null
            )
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo info de expiración: ${e.message}", e)
            TokenExpirationInfo()
        }
    }
    
    /**
     * 🧪 FORZAR EXPIRACIÓN DEL TOKEN (para testing)
     */
    suspend fun forceTokenExpiry(): Boolean = mutex.withLock {
        return try {
            val currentTime = System.currentTimeMillis()
            val pastTime = currentTime - TimeUnit.HOURS.toMillis(2) // 2 horas en el pasado
            
            prefs.edit()
                .putLong(KEY_EXPIRY, pastTime)
                .apply()
            
            Log.i(TAG, "🧪 Token expiración forzada para testing")
            true
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error forzando expiración: ${e.message}", e)
            false
        }
    }
    
    /**
     * 📊 LOG ESTADO ACTUAL COMPLETO
     */
    suspend fun logCurrentState() = mutex.withLock {
        try {
            val token = prefs.getString(KEY_TOKEN, null)
            val expiry = prefs.getLong(KEY_EXPIRY, 0)
            val currentTime = System.currentTimeMillis()
            val matricule = prefs.getString(KEY_MATRICULE, null)
            val username = prefs.getString(KEY_USERNAME, null)
            val societe = prefs.getString(KEY_SOCIETE, null)
            val lastRefresh = prefs.getLong(KEY_LAST_REFRESH_TIME, 0)
            val installTime = prefs.getLong(KEY_INSTALL_TIME, 0)
            
            val isValid = token != null && currentTime < expiry
            val remainingMinutes = if (isValid) {
                TimeUnit.MILLISECONDS.toMinutes(expiry - currentTime)
            } else 0
            
            Log.d(TAG, "📊 === ESTADO ACTUAL DE TOKENS ===")
            Log.d(TAG, "Tiene token: ${token != null}")
            Log.d(TAG, "Token válido: $isValid")
            Log.d(TAG, "Minutos restantes: $remainingMinutes")
            Log.d(TAG, "Matricule: $matricule")
            Log.d(TAG, "Username: $username")
            Log.d(TAG, "Société: $societe")
            Log.d(TAG, "Expira: ${if (expiry > 0) java.util.Date(expiry) else "N/A"}")
            Log.d(TAG, "Último refresh: ${if (lastRefresh > 0) java.util.Date(lastRefresh) else "N/A"}")
            Log.d(TAG, "Instalación: ${if (installTime > 0) java.util.Date(installTime) else "N/A"}")
            Log.d(TAG, "=== FIN ESTADO ===")
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error loggeando estado actual: ${e.message}", e)
        }
    }
    
    /**
     * 🔍 VERIFICAR SI USUARIO ESTÁ LOGUEADO
     */
    suspend fun isUserLoggedIn(): Boolean = withContext(Dispatchers.IO) {
        try {
            val token = getValidToken()
            val isLoggedIn = token != null
            Log.d(TAG, "🔍 Usuario logueado: $isLoggedIn")
            isLoggedIn
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error verificando login: ${e.message}", e)
            false
        }
    }
    
    /**
     * 🔄 OBTENER TIEMPO DESDE ÚLTIMO REFRESH
     */
    suspend fun getTimeSinceLastRefresh(): Long = mutex.withLock {
        return try {
            val lastRefresh = prefs.getLong(KEY_LAST_REFRESH_TIME, 0)
            val currentTime = System.currentTimeMillis()
            
            if (lastRefresh > 0) {
                val timeSince = currentTime - lastRefresh
                val minutesSince = TimeUnit.MILLISECONDS.toMinutes(timeSince)
                Log.d(TAG, "⏰ Tiempo desde último refresh: $minutesSince minutos")
                timeSince
            } else {
                Log.d(TAG, "⏰ No hay registro de refresh previo")
                0L
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo tiempo desde refresh: ${e.message}", e)
            0L
        }
    }
    
    // 🛠️ FUNCIONES PRIVADAS DE UTILIDAD
    
    /**
     * 👤 EXTRAER USERNAME DEL MATRICULE
     * PCP0010699_A187518 → A187518
     */
    private fun extractUsernameFromMatricule(matricule: String): String {
        return try {
            if (matricule.contains("_")) {
                val parts = matricule.split("_")
                if (parts.size >= 2) {
                    parts[1] // Retorna la parte después del underscore
                } else {
                    matricule
                }
            } else {
                matricule
            }
        } catch (e: Exception) {
            Log.w(TAG, "⚠️ Error extrayendo username de matricule: ${e.message}")
            matricule
        }
    }
    
    /**
     * 🏢 EXTRAER SOCIÉTÉ DEL MATRICULE
     * PCP0010699_A187518 → PCP0010699
     */
    private fun extractSocieteFromMatricule(matricule: String): String {
        return try {
            if (matricule.contains("_")) {
                val parts = matricule.split("_")
                if (parts.size >= 2) {
                    parts[0] // Retorna la parte antes del underscore
                } else {
                    matricule
                }
            } else {
                matricule
            }
        } catch (e: Exception) {
            Log.w(TAG, "⚠️ Error extrayendo societe de matricule: ${e.message}")
            matricule
        }
    }
    
    /**
     * 📅 OBTENER TIEMPO DE INSTALACIÓN
     */
    private fun getInstallTime(): Long {
        var installTime = prefs.getLong(KEY_INSTALL_TIME, 0L)
        
        if (installTime == 0L) {
            installTime = System.currentTimeMillis()
            prefs.edit().putLong(KEY_INSTALL_TIME, installTime).apply()
        }
        
        return installTime
    }
}

/**
 * 👥 DATOS DE USUARIO GUARDADOS
 */
data class UserTokenData(
    val matricule: String,
    val username: String,
    val societe: String,
    val lastRefreshTime: Long
) {
    /**
     * 📅 FORMATO LEGIBLE DE ÚLTIMO REFRESH
     */
    fun getFormattedLastRefresh(): String {
        val date = Date(lastRefreshTime)
        val formatter = java.text.SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault())
        return formatter.format(date)
    }
    
    /**
     * ⏰ MINUTOS DESDE ÚLTIMO REFRESH
     */
    fun getMinutesSinceLastRefresh(): Long {
        val currentTime = System.currentTimeMillis()
        return (currentTime - lastRefreshTime) / (1000 * 60)
    }
}

/**
 * ⏰ INFORMACIÓN DE EXPIRACIÓN DEL TOKEN
 */
data class TokenExpirationInfo(
    val expiryTime: Long = 0,
    val currentTime: Long = 0,
    val isExpired: Boolean = true,
    val minutesUntilExpiry: Long = 0,
    val hasToken: Boolean = false
) {
    /**
     * 📅 FORMATO LEGIBLE DE EXPIRACIÓN
     */
    fun getFormattedExpiry(): String {
        return if (expiryTime > 0) {
            val date = Date(expiryTime)
            val formatter = java.text.SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault())
            formatter.format(date)
        } else {
            "N/A"
        }
    }
    
    /**
     * ⚠️ VERIFICAR SI TOKEN EXPIRA PRONTO (≤ 30 minutos)
     */
    fun isExpiringSoon(): Boolean {
        return !isExpired && minutesUntilExpiry <= 30
    }
    
    /**
     * 🚨 VERIFICAR SI TOKEN EXPIRA MUY PRONTO (≤ 5 minutos)
     */
    fun isExpiringVerySoon(): Boolean {
        return !isExpired && minutesUntilExpiry <= 5
    }
}
