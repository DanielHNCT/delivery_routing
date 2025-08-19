package com.daniel.deliveryrouting.data.repository

import android.content.Context
import android.util.Log
import com.daniel.deliveryrouting.data.api.ColisTokenApi
import com.daniel.deliveryrouting.data.api.models.ColisAuthResponse
import com.daniel.deliveryrouting.data.api.models.ColisTokenLoginRequest
import com.daniel.deliveryrouting.data.api.models.RefreshTokenRequest
import com.daniel.deliveryrouting.data.api.models.TourneeRequest
import com.daniel.deliveryrouting.data.api.models.TourneeResponse
import com.daniel.deliveryrouting.data.token.ColisTokenManager
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.util.UUID

/**
 * 🎯 REPOSITORY COMPLETO CON AUTO-RETRY Y MANEJO DE TOKENS
 * 
 * Funcionalidades implementadas:
 * - ✅ Autenticación con guardado de tokens
 * - ✅ Refresh automático en 401
 * - ✅ Retry logic con máximo 2 intentos
 * - ✅ Manejo de estados de loading
 * - ✅ Logging detallado para debugging
 */
class ColisTokenRepository(
    private val api: ColisTokenApi,
    private val tokenManager: ColisTokenManager
) {
    
    companion object {
        private const val TAG = "ColisTokenRepository"
        private const val MAX_RETRIES = 2
    }
    
    /**
     * 🔐 AUTENTICACIÓN COMPLETA DEL USUARIO
     */
    suspend fun authenticateUser(
        username: String, 
        password: String, 
        societe: String
    ): Result<ColisAuthResponse> = withContext(Dispatchers.IO) {
        
        try {
            Log.d(TAG, "🔐 === INICIANDO AUTENTICACIÓN COMPLETA ===")
            Log.d(TAG, "Username: $username")
            Log.d(TAG, "Societe: $societe")
            Log.d(TAG, "Password length: ${password.length}")
            
            val activityId = UUID.randomUUID().toString()
            Log.d(TAG, "ActivityId: $activityId")
            
            val request = ColisTokenLoginRequest(username, password, societe)
            Log.d(TAG, "Request object: $request")
            
            val response = api.login(request, activityId)
            Log.d(TAG, "📥 Login Response Status: ${response.code()}")
            Log.d(TAG, "📥 Login Response Headers: ${response.headers()}")
            
            if (response.isSuccessful) {
                val authResponse = response.body()
                if (authResponse != null && authResponse.isAuthentif) {
                    
                    // Guardar tokens localmente
                    tokenManager.saveTokens(authResponse)
                    
                    Log.d(TAG, "✅ === AUTENTICACIÓN EXITOSA ===")
                    Log.d(TAG, "🔑 Token guardado: ${authResponse.tokens.ssoHopps.take(50)}...")
                    Log.d(TAG, "🔑 Short Token: ${authResponse.shortToken.ssoHopps.take(50)}...")
                    Log.d(TAG, "👤 Matricule: ${authResponse.matricule}")
                    Log.d(TAG, "🏢 Société: ${authResponse.societe}")
                    Log.d(TAG, "👤 Nom: ${authResponse.nom}")
                    Log.d(TAG, "👤 Prénom: ${authResponse.prenom}")
                    Log.d(TAG, "=== FIN AUTENTICACIÓN ===")
                    
                    Result.success(authResponse)
                } else {
                    Log.e(TAG, "❌ Autenticación falló: isAuthentif = false")
                    Log.e(TAG, "Response: $authResponse")
                    Result.failure(Exception("Authentication failed: Invalid credentials"))
                }
            } else {
                val errorBody = response.errorBody()?.string()
                Log.e(TAG, "❌ Login request falló: ${response.code()} - $errorBody")
                Result.failure(Exception("Login failed: ${response.code()} - $errorBody"))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en autenticación: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🔄 REFRESH AUTOMÁTICO DE TOKEN
     */
    suspend fun refreshTokenIfNeeded(): Result<String> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🔄 === VERIFICANDO NECESIDAD DE REFRESH ===")
            
            // Verificar si tenemos token válido
            val currentToken = tokenManager.getValidToken()
            if (currentToken != null) {
                Log.d(TAG, "✅ Token actual aún válido")
                return@withContext Result.success(currentToken)
            }
            
            // Necesitamos refresh - obtener token anterior para refresh
            val oldToken = tokenManager.getTokenForRefresh()
            if (oldToken == null) {
                Log.w(TAG, "❌ No hay token para refresh - necesita login completo")
                return@withContext Result.failure(Exception("No token available for refresh"))
            }
            
            Log.d(TAG, "🔄 Haciendo refresh token...")
            val activityId = UUID.randomUUID().toString()
            val refreshRequest = RefreshTokenRequest(token = oldToken)
            val response = api.refreshToken(refreshRequest, activityId)
            
            Log.d(TAG, "📥 Refresh Response Status: ${response.code()}")
            
            if (response.isSuccessful) {
                val refreshResponse = response.body()
                if (refreshResponse != null && refreshResponse.isAuthentif) {
                    
                    // Guardar nuevo token
                    tokenManager.saveTokens(refreshResponse)
                    
                    Log.d(TAG, "✅ === TOKEN REFRESH EXITOSO ===")
                    Log.d(TAG, "🔑 Nuevo token: ${refreshResponse.tokens.ssoHopps.take(50)}...")
                    Log.d(TAG, "🔑 Nuevo short token: ${refreshResponse.shortToken.ssoHopps.take(50)}...")
                    Log.d(TAG, "=== FIN REFRESH ===")
                    
                    Result.success(refreshResponse.tokens.ssoHopps)
                } else {
                    Log.e(TAG, "❌ Refresh falló: isAuthentif = false")
                    tokenManager.clearTokens() // Limpiar tokens inválidos
                    Result.failure(Exception("Token refresh failed"))
                }
            } else {
                val errorBody = response.errorBody()?.string()
                Log.e(TAG, "❌ Refresh request falló: ${response.code()} - $errorBody")
                tokenManager.clearTokens() // Limpiar tokens inválidos
                Result.failure(Exception("Refresh failed: ${response.code()} - $errorBody"))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en refresh token: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🚚 OBTENER TOURNÉE CON AUTO-RETRY Y MANEJO DE TOKENS
     */
    suspend fun getTourneeData(
        username: String,
        password: String,
        societe: String,
        date: String,
        matricule: String,
        maxRetries: Int = MAX_RETRIES
    ): Result<TourneeResponse> = withContext(Dispatchers.IO) {
        
        var attempt = 1
        
        while (attempt <= maxRetries) {
            try {
                Log.d(TAG, "🚚 === OBTENIENDO TOURNÉE (Intento $attempt/$maxRetries) ===")
                Log.d(TAG, "Username: $username")
                Log.d(TAG, "Date: $date")
                Log.d(TAG, "Matricule: $matricule")
                
                // Obtener token válido (refresh automático si es necesario)
                val tokenResult = refreshTokenIfNeeded()
                if (tokenResult.isFailure) {
                    Log.e(TAG, "❌ No se pudo obtener token válido")
                    return@withContext Result.failure(tokenResult.exceptionOrNull() ?: Exception("Token unavailable"))
                }
                
                val token = tokenResult.getOrThrow()
                Log.d(TAG, "🔑 Token válido obtenido: ${token.take(50)}...")
                
                // Hacer request de tournée
                val tourneeRequest = TourneeRequest(
                    username = username,
                    password = password,
                    societe = societe,
                    date = date,
                    matricule = matricule,
                    token = token
                )
                
                val activityId = UUID.randomUUID().toString()
                Log.d(TAG, "📱 Enviando request tournée con token: ${token.take(50)}...")
                Log.d(TAG, "ActivityId: $activityId")
                
                val response = api.getTourneeWithRetry(tourneeRequest, activityId)
                
                Log.d(TAG, "📥 Tournée Response Status: ${response.code()}")
                Log.d(TAG, "📥 Tournée Response Headers: ${response.headers()}")
                
                when {
                    response.isSuccessful -> {
                        val tourneeData = response.body()
                        Log.d(TAG, "✅ === TOURNÉE OBTENIDA EXITOSAMENTE ===")
                        Log.d(TAG, "Success: ${tourneeData?.success}")
                        Log.d(TAG, "Message: ${tourneeData?.message}")
                        Log.d(TAG, "Packages count: ${tourneeData?.data?.packages?.size ?: 0}")
                        Log.d(TAG, "=== FIN TOURNÉE ===")
                        return@withContext Result.success(tourneeData ?: TourneeResponse(false, null, "Empty response"))
                    }
                    response.code() == 401 -> {
                        Log.w(TAG, "🔄 Token expirado (401) - Limpiando y reintentando...")
                        tokenManager.clearTokens() // Forzar nuevo login
                        
                        if (attempt < maxRetries) {
                            attempt++
                            Log.d(TAG, "🔄 Reintentando tournée (intento $attempt)...")
                            continue // Retry con nuevo token
                        } else {
                            Log.e(TAG, "❌ Máximo de reintentos alcanzado")
                            return@withContext Result.failure(Exception("Authentication failed after $maxRetries retries"))
                        }
                    }
                    else -> {
                        val errorBody = response.errorBody()?.string()
                        Log.e(TAG, "❌ Tournée request falló: ${response.code()} - $errorBody")
                        return@withContext Result.failure(Exception("Tournée failed: ${response.code()} - $errorBody"))
                    }
                }
                
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error en tournée (intento $attempt): ${e.message}", e)
                
                if (attempt < maxRetries) {
                    attempt++
                    Log.d(TAG, "🔄 Reintentando tournée por error (intento $attempt)...")
                    continue
                } else {
                    return@withContext Result.failure(e)
                }
            }
        }
        
        Result.failure(Exception("Max retries exceeded"))
    }
    
    /**
     * 🚚 TOURNÉE TRADICIONAL (compatibilidad)
     */
    suspend fun getTourneeUpdated(
        username: String,
        password: String,
        societe: String,
        date: String,
        matricule: String
    ): Result<TourneeResponse> = withContext(Dispatchers.IO) {
        
        try {
            Log.d(TAG, "🚚 === OBTENIENDO TOURNÉE UPDATED ===")
            
            val tourneeRequest = TourneeRequest(
                username = username,
                password = password,
                societe = societe,
                date = date,
                matricule = matricule
            )
            
            val activityId = UUID.randomUUID().toString()
            val response = api.getTourneeUpdated(tourneeRequest, activityId)
            
            if (response.isSuccessful) {
                val tourneeData = response.body()
                Log.d(TAG, "✅ Tournée updated obtenido exitosamente")
                Result.success(tourneeData ?: TourneeResponse(false, null, "Empty response"))
            } else {
                val errorBody = response.errorBody()?.string()
                Log.e(TAG, "❌ Tournée updated falló: ${response.code()} - $errorBody")
                Result.failure(Exception("Tournée updated failed: ${response.code()}"))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en tournée updated: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 👋 LOGOUT - Limpiar tokens
     */
    suspend fun logout() = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "👋 === LOGOUT ===")
            tokenManager.clearTokens()
            Log.d(TAG, "✅ Logout completado - tokens eliminados")
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en logout: ${e.message}", e)
        }
    }
    
    /**
     * 🔍 VERIFICAR SI USUARIO ESTÁ LOGUEADO
     */
    suspend fun isUserLoggedIn(): Boolean = withContext(Dispatchers.IO) {
        try {
            val isLoggedIn = tokenManager.isUserLoggedIn()
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
    suspend fun getTokenStatus(): ColisTokenManager.TokenStatus = withContext(Dispatchers.IO) {
        try {
            val status = tokenManager.getTokenStatus()
            status.logStatus()
            status
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo estado de tokens: ${e.message}", e)
            ColisTokenManager.TokenStatus()
        }
    }
    
    /**
     * 🏥 HEALTH CHECK DEL BACKEND
     */
    suspend fun healthCheck(): Result<Map<String, Any>> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🏥 === HEALTH CHECK ===")
            val response = api.healthCheck()
            
            if (response.isSuccessful) {
                val healthData = response.body()
                Log.d(TAG, "✅ Health check exitoso: $healthData")
                Result.success(healthData ?: emptyMap())
            } else {
                Log.e(TAG, "❌ Health check falló: ${response.code()}")
                Result.failure(Exception("Health check failed: ${response.code()}"))
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en health check: ${e.message}", e)
            Result.failure(e)
        }
    }
}
