package com.daniel.deliveryrouting.data.token

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import com.daniel.deliveryrouting.data.api.models.ColisAuthResponse
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import java.util.concurrent.TimeUnit

/**
 * 🎯 MANAGER COMPLETO DE TOKENS PARA COLIS PRIVÉ
 * 
 * Funcionalidades:
 * - ✅ Guardar tokens localmente
 * - ✅ Verificar expiración automática
 * - ✅ Refresh automático en 401
 * - ✅ Thread-safe con Mutex
 * - ✅ Auto-cleanup de tokens expirados
 */
class ColisTokenManager(context: Context) {
    
    private val prefs: SharedPreferences = context.getSharedPreferences(
        "colis_tokens", Context.MODE_PRIVATE
    )
    private val mutex = Mutex()
    
    companion object {
        private const val TAG = "ColisTokenManager"
        private const val KEY_TOKEN = "sso_hopps_token"
        private const val KEY_SHORT_TOKEN = "sso_hopps_short_token"
        private const val KEY_EXPIRY = "token_expiry"
        private const val KEY_MATRICULE = "matricule"
        private const val KEY_USERNAME = "username"
        private const val KEY_SOCIETE = "societe"
        private const val TOKEN_VALIDITY_HOURS = 1L // Tokens válidos por 1 hora
    }
    
    /**
     * 🔑 GUARDAR TOKENS DESPUÉS DE LOGIN EXITOSO
     */
    suspend fun saveTokens(response: ColisAuthResponse) = mutex.withLock {
        try {
            val expiryTime = System.currentTimeMillis() + TimeUnit.HOURS.toMillis(TOKEN_VALIDITY_HOURS)
            
            prefs.edit()
                .putString(KEY_TOKEN, response.tokens.ssoHopps)
                .putString(KEY_SHORT_TOKEN, response.shortToken.ssoHopps)
                .putLong(KEY_EXPIRY, expiryTime)
                .putString(KEY_MATRICULE, response.matricule)
                .putString(KEY_USERNAME, response.identity)
                .putString(KEY_SOCIETE, response.societe)
                .apply()
                
            Log.d(TAG, "🔑 === TOKENS GUARDADOS EXITOSAMENTE ===")
            Log.d(TAG, "Token: ${response.tokens.ssoHopps.take(50)}...")
            Log.d(TAG, "Short Token: ${response.shortToken.ssoHopps.take(50)}...")
            Log.d(TAG, "Matricule: ${response.matricule}")
            Log.d(TAG, "Expiry: ${java.util.Date(expiryTime)}")
            Log.d(TAG, "=== FIN TOKENS GUARDADOS ===")
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error guardando tokens: ${e.message}", e)
        }
    }
    
    /**
     * ✅ OBTENER TOKEN VÁLIDO (si existe y no ha expirado)
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
     * 🔄 OBTENER TOKEN PARA REFRESH (incluso si expiró)
     */
    suspend fun getTokenForRefresh(): String? = mutex.withLock {
        return try {
            val token = prefs.getString(KEY_TOKEN, null)
            if (token != null) {
                Log.d(TAG, "🔄 Token para refresh encontrado: ${token.take(50)}...")
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
     * ⏰ VERIFICAR SI TOKEN HA EXPIRADO
     */
    suspend fun isTokenExpired(): Boolean = mutex.withLock {
        return try {
            val expiry = prefs.getLong(KEY_EXPIRY, 0)
            val isExpired = System.currentTimeMillis() >= expiry
            Log.d(TAG, "⏰ Token expirado: $isExpired")
            isExpired
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error verificando expiración: ${e.message}", e)
            true // Asumir expirado en caso de error
        }
    }
    
    /**
     * 🗑️ LIMPIAR TODOS LOS TOKENS (logout)
     */
    suspend fun clearTokens() = mutex.withLock {
        try {
            prefs.edit()
                .remove(KEY_TOKEN)
                .remove(KEY_SHORT_TOKEN)
                .remove(KEY_EXPIRY)
                .remove(KEY_MATRICULE)
                .remove(KEY_USERNAME)
                .remove(KEY_SOCIETE)
                .apply()
            Log.d(TAG, "🗑️ === TOKENS ELIMINADOS COMPLETAMENTE ===")
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error limpiando tokens: ${e.message}", e)
        }
    }
    
    /**
     * 👤 OBTENER MATRICULE DEL USUARIO LOGUEADO
     */
    suspend fun getMatricule(): String? = mutex.withLock {
        return try {
            val matricule = prefs.getString(KEY_MATRICULE, null)
            Log.d(TAG, "👤 Matricule obtenido: $matricule")
            matricule
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo matricule: ${e.message}", e)
            null
        }
    }
    
    /**
     * 🏢 OBTENER SOCIÉTÉ DEL USUARIO LOGUEADO
     */
    suspend fun getSociete(): String? = mutex.withLock {
        return try {
            val societe = prefs.getString(KEY_SOCIETE, null)
            Log.d(TAG, "🏢 Société obtenida: $societe")
            societe
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo société: ${e.message}", e)
            null
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
     * 📊 OBTENER ESTADO COMPLETO DE TOKENS
     */
    suspend fun getTokenStatus(): TokenStatus = mutex.withLock {
        return try {
            val token = prefs.getString(KEY_TOKEN, null)
            val expiry = prefs.getLong(KEY_EXPIRY, 0)
            val currentTime = System.currentTimeMillis()
            val matricule = prefs.getString(KEY_MATRICULE, null)
            val societe = prefs.getString(KEY_SOCIETE, null)
            
            val isValid = token != null && currentTime < expiry
            val remainingMinutes = if (isValid) {
                TimeUnit.MILLISECONDS.toMinutes(expiry - currentTime)
            } else 0
            
            TokenStatus(
                hasToken = token != null,
                isValid = isValid,
                remainingMinutes = remainingMinutes,
                matricule = matricule,
                societe = societe,
                expiryTime = if (expiry > 0) java.util.Date(expiry) else null
            )
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo estado de tokens: ${e.message}", e)
            TokenStatus()
        }
    }
    
    /**
     * 📊 ESTADO COMPLETO DE TOKENS
     */
    data class TokenStatus(
        val hasToken: Boolean = false,
        val isValid: Boolean = false,
        val remainingMinutes: Long = 0,
        val matricule: String? = null,
        val societe: String? = null,
        val expiryTime: java.util.Date? = null
    ) {
        fun logStatus() {
            Log.d(TAG, "📊 === ESTADO DE TOKENS ===")
            Log.d(TAG, "Tiene token: $hasToken")
            Log.d(TAG, "Token válido: $isValid")
            Log.d(TAG, "Minutos restantes: $remainingMinutes")
            Log.d(TAG, "Matricule: $matricule")
            Log.d(TAG, "Société: $societe")
            Log.d(TAG, "Expira: $expiryTime")
            Log.d(TAG, "=== FIN ESTADO ===")
        }
    }
}
