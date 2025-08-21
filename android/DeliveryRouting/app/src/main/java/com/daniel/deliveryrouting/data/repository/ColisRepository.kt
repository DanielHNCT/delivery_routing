package com.daniel.deliveryrouting.data.repository

import android.content.Context
import android.util.Log
import com.daniel.deliveryrouting.data.api.ColisApiService
import com.daniel.deliveryrouting.data.api.models.*
import com.daniel.deliveryrouting.data.token.ColisTokenManager
import com.daniel.deliveryrouting.data.token.UserTokenData
import com.daniel.deliveryrouting.data.token.TokenExpirationInfo
import com.daniel.deliveryrouting.utils.DeviceInfo
import com.daniel.deliveryrouting.utils.DeviceInfoManager
import com.daniel.deliveryrouting.utils.InstallationInfo
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.util.*

/**
 * 🔗 REPOSITORY COMPLETO PARA COLIS PRIVÉ
 * 
 * Características:
 * - ✅ Integra DeviceInfoManager, TokenManager y API
 * - ✅ Auto-retry logic robusto con refresh automático
 * - ✅ Error handling completo con Result<T>
 * - ✅ Logs detallados para debugging
 * - ✅ Estado del repository para UI management
 */
class ColisRepository(private val context: Context) {
    
    private val api = ColisApiService.api
    private val tokenManager = ColisTokenManager(context)
    private val deviceInfoManager = DeviceInfoManager(context)
    
    companion object {
        private const val TAG = "ColisRepository"
        private const val MAX_RETRY_ATTEMPTS = 2
        
        // ✅ CONFIGURACIÓN INTELIGENTE DEL BACKEND
        private const val EMULATOR_BACKEND_URL = "http://10.0.2.2:3000"  // Para emuladores
        private const val PHYSICAL_DEVICE_BACKEND_URL = "http://192.168.1.9:3000"  // Para tu Xperia Z1
        
        /**
         * 🌐 OBTENER URL DEL BACKEND SEGÚN EL DISPOSITIVO
         */
        private fun getBackendUrl(): String {
            // ✅ FORZAR IP CORRECTA PARA TU XPERIA Z1
            // Tu dispositivo: Sony D5503 (CB5A1YGJL7)
            return if (android.os.Build.MODEL.contains("D5503") || 
                       android.os.Build.MODEL.contains("Sony") ||
                       android.os.Build.MANUFACTURER.contains("Sony")) {
                // Es tu Sony Xperia Z1 - usar IP real
                PHYSICAL_DEVICE_BACKEND_URL
            } else if (android.os.Build.FINGERPRINT.contains("generic") || 
                       android.os.Build.FINGERPRINT.contains("unknown") ||
                       android.os.Build.MODEL.contains("google_sdk") ||
                       android.os.Build.MODEL.contains("Emulator") ||
                       android.os.Build.MODEL.contains("Android SDK built for x86")) {
                // Es un emulador
                EMULATOR_BACKEND_URL
            } else {
                // Otros dispositivos físicos
                PHYSICAL_DEVICE_BACKEND_URL
            }
        }
        
        /**
         * 📅 OBTENER FECHA ACTUAL EN FORMATO YYYY-MM-DD
         */
        private fun getCurrentDate(): String {
            val formatter = java.text.SimpleDateFormat("yyyy-MM-dd", java.util.Locale.getDefault())
            return formatter.format(java.util.Date())
        }
        
        /**
         * 🆔 EXTRAER MATRÍCULA CORRECTA PARA COLIS PRIVÉ
         * 
         * Colis Privé necesita: "INTI_A187518" (sin duplicación)
         * El username ya viene como "INTI_A187518" desde LoginScreen
         */
        private fun extractMatricule(username: String): String {
            // ✅ Usar directamente el username que ya tiene el formato correcto
            return username
        }
        
        /**
         * 👤 EXTRAER USERNAME CORRECTO PARA COLIS PRIVÉ
         * 
         * Username debe ser solo la parte final después del último "_"
         * Ejemplo: "INTI_A187518" -> "A187518"
         */
        private fun extractUsername(username: String): String {
            // ✅ Extraer solo la parte final después del último "_"
            return username.split("_").lastOrNull() ?: username
        }
    }
    
    /**
     * 🔐 AUTENTICACIÓN PRINCIPAL CON FLUJO COMPLETO (RESUELVE EL 401)
     */
    suspend fun authenticate(
        username: String,
        password: String,
        societe: String
    ): Result<AuthResponse> = withContext(Dispatchers.IO) {
        
        try {
            val backendUrl = getBackendUrl()
            Log.d(TAG, "🚀 === INICIO FLUJO COMPLETO DE AUTENTICACIÓN (RESUELVE EL 401) ===")
            Log.d(TAG, "Username: $username")
            Log.d(TAG, "Societe: $societe")
            Log.d(TAG, "Backend: $backendUrl")
            Log.d(TAG, "📱 Device Model: ${android.os.Build.MODEL}")
            Log.d(TAG, "📱 Device Manufacturer: ${android.os.Build.MANUFACTURER}")
            Log.d(TAG, "📱 Device Fingerprint: ${android.os.Build.FINGERPRINT}")
            
            // Obtener device info único
            val deviceInfo = deviceInfoManager.getDeviceInfo()
            deviceInfoManager.logDeviceInfo()
            
            // ✅ CORREGIDO: Usar username directamente sin duplicar societe
            val currentDate = getCurrentDate()
            val matricule = extractMatricule(username)  // ✅ "INTI_A187518" (sin duplicación)
            val usernameCorrected = extractUsername(username) // ✅ "A187518"
            
            Log.d(TAG, "🆔 Username recibido: $username")
            Log.d(TAG, "🆔 Matrícula para Colis Privé: $matricule")
            Log.d(TAG, "🆔 Username corregido: $usernameCorrected")
            
            // 🆕 NUEVO: Usar flujo completo de autenticación
            val request = CompleteAuthFlowRequest(
                username = usernameCorrected,  // ✅ Usar username corregido: "A187518"
                password = password,
                societe = societe,
                date = currentDate,
                matricule = matricule,         // ✅ Usar matrícula extraída: "PCP0010699_A187518"
                deviceInfo = deviceInfo,
                apiChoice = "mobile"           // 🆕 NUEVO: Indicar que es API Mobile
            )
            
            Log.d(TAG, "📡 Enviando request de flujo completo...")
            
            val activityId = UUID.randomUUID().toString()
            val loginDeviceInfo = deviceInfoManager.getDeviceInfo()
            
            // 🆕 NUEVO: Llamar al endpoint de flujo completo
            val response = api.completeAuthenticationFlow(
                request = request,
                activityId = activityId,
                device = loginDeviceInfo.model,  // ✅ Device real del dispositivo
                versionOS = loginDeviceInfo.androidVersion  // ✅ Version real de Android
            )
            
            Log.d(TAG, "📡 Response code: ${response.code()}")
            
            when {
                response.isSuccessful -> {
                    val authData = response.body()!!
                    Log.d(TAG, "✅ Flujo completo exitoso - 401 RESUELTO")
                    
                    // Guardar tokens en el manager (si hay)
                    if (authData.flowResult?.success == true) {
                        // Crear BackendAuthResponse compatible
                        val authenticationData = AuthenticationData(
                            matricule = matricule,
                            message = authData.message,
                            token = authData.flowResult?.sessionId ?: ""
                        )
                        val backendAuthResponse = BackendAuthResponse(
                            authentication = authenticationData,
                            success = authData.success,
                            timestamp = authData.timestamp
                        )
                        tokenManager.saveTokens(backendAuthResponse)
                    }
                    
                    // Retornar success
                    Result.success(authData)
                }
                response.code() == 401 -> {
                    Log.e(TAG, "❌ 401 Unauthorized - intentando reconexión...")
                    // Intentar reconexión automática
                    val reconnectionResult = handleReconnection(username, password, societe)
                    reconnectionResult
                }
                else -> {
                    val errorMsg = "Error HTTP: ${response.code()}"
                    Log.e(TAG, "❌ $errorMsg")
                    Result.failure(Exception(errorMsg))
                }
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en flujo completo: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🌐 AUTENTICACIÓN WEB (API SIMPLE)
     */
    suspend fun authenticateWeb(
        username: String,
        password: String,
        societe: String
    ): Result<AuthResponse> = withContext(Dispatchers.IO) {
        
        try {
            val backendUrl = getBackendUrl()
            Log.d(TAG, "🌐 === INICIO AUTENTICACIÓN WEB (API SIMPLE) ===")
            Log.d(TAG, "Username: $username")
            Log.d(TAG, "Societe: $societe")
            Log.d(TAG, "Backend: $backendUrl")
            Log.d(TAG, "📱 Device Model: ${android.os.Build.MODEL}")
            Log.d(TAG, "📱 Device Manufacturer: ${android.os.Build.MANUFACTURER}")
            
            // Obtener device info único
            val deviceInfo = deviceInfoManager.getDeviceInfo()
            deviceInfoManager.logDeviceInfo()
            
            // ✅ CORREGIDO: Usar username directamente sin duplicar societe
            val currentDate = getCurrentDate()
            val matricule = extractMatricule(username)  // ✅ "INTI_A187518" (sin duplicación)
            val usernameCorrected = extractUsername(username) // ✅ "A187518"
            
            Log.d(TAG, "🆔 Username recibido: $username")
            Log.d(TAG, "🆔 Matrícula para Colis Privé: $matricule")
            Log.d(TAG, "🆔 Username corregido: $usernameCorrected")
            
            // 🆕 NUEVO: Usar autenticación web simple
            val request = CompleteAuthFlowRequest(
                username = usernameCorrected,  // ✅ Usar username corregido: "A187518"
                password = password,
                societe = societe,
                date = currentDate,
                matricule = matricule,         // ✅ Usar matrícula extraída: "PCP0010699_A187518"
                deviceInfo = deviceInfo,
                apiChoice = "web"              // 🆕 NUEVO: Indicar que es API Web
            )
            
            Log.d(TAG, "📡 Enviando request de autenticación web...")
            
            val activityId = UUID.randomUUID().toString()
            val loginDeviceInfo = deviceInfoManager.getDeviceInfo()
            
            // 🆕 NUEVO: Llamar al endpoint de autenticación web
            val response = api.completeAuthenticationFlow(
                request = request,
                activityId = activityId,
                device = loginDeviceInfo.model,  // ✅ Device real del dispositivo
                versionOS = loginDeviceInfo.androidVersion  // ✅ Version real de Android
            )
            
            Log.d(TAG, "📡 Response code: ${response.code()}")
            
            when {
                response.isSuccessful -> {
                    val authData = response.body()!!
                    Log.d(TAG, "✅ Autenticación web exitosa")
                    
                    // Guardar tokens en el manager (si hay)
                    if (authData.flowResult?.success == true) {
                        // Crear BackendAuthResponse compatible
                        val authenticationData = AuthenticationData(
                            matricule = matricule,
                            message = authData.message,
                            token = authData.flowResult?.sessionId ?: ""
                        )
                        val backendAuthResponse = BackendAuthResponse(
                            authentication = authenticationData,
                            success = authData.success,
                            timestamp = authData.timestamp
                        )
                        tokenManager.saveTokens(backendAuthResponse)
                    }
                    
                    // Retornar success
                    Result.success(authData)
                }
                response.code() == 401 -> {
                    Log.e(TAG, "❌ 401 Unauthorized en autenticación web")
                    Result.failure(Exception("Autenticación web falló: 401 Unauthorized"))
                }
                else -> {
                    val errorMsg = "Error HTTP en autenticación web: ${response.code()}"
                    Log.e(TAG, "❌ $errorMsg")
                    Result.failure(Exception(errorMsg))
                }
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en autenticación web: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🔄 REFRESH TOKEN
     */
    suspend fun refreshToken(): Result<BackendAuthResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🔄 === REFRESH TOKEN ===")
            
            val oldToken = tokenManager.getTokenForRefresh()
            if (oldToken == null) {
                Log.w(TAG, "⚠️ No hay token para refresh")
                return@withContext Result.failure(Exception("No token available for refresh"))
            }
            
            val deviceInfo = deviceInfoManager.getDeviceInfo()
            val request = RefreshTokenRequest(
                token = oldToken,
                deviceInfo = deviceInfo
            )
            
            Log.d(TAG, "📡 Enviando request de refresh...")
            
            val refreshDeviceInfo = deviceInfoManager.getDeviceInfo()
            val response = api.refreshToken(
                request = request,
                device = refreshDeviceInfo.model,  // ✅ Device real del dispositivo
                versionOS = refreshDeviceInfo.androidVersion  // ✅ Version real de Android
            )
            Log.d(TAG, "📡 Refresh response code: ${response.code()}")
            
            if (response.isSuccessful) {
                val newTokenData = response.body()!!
                Log.d(TAG, "✅ Token refresh exitoso")
                
                // Guardar nuevos tokens
                tokenManager.saveTokens(newTokenData)
                
                Result.success(newTokenData)
            } else {
                val errorMsg = "Refresh failed with code: ${response.code()}"
                Log.e(TAG, "❌ $errorMsg")
                Result.failure(Exception(errorMsg))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en refresh token: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 📦 TOURNÉE CON AUTO-RETRY ROBUSTO
     */
    suspend fun getTourneeWithAutoRetry(
        username: String,
        password: String,
        societe: String,
        date: String,
        matricule: String
    ): Result<TourneeResponse> = withContext(Dispatchers.IO) {
        
        Log.d(TAG, "📦 === TOURNÉE CON AUTO-RETRY ===")
        Log.d(TAG, "Username: $username")
        Log.d(TAG, "Date: $date")
        Log.d(TAG, "Matricule: $matricule")
        
        var lastError: Exception? = null
        
        for (attempt in 0 until MAX_RETRY_ATTEMPTS) {
            Log.d(TAG, "🔄 Intento ${attempt + 1}/$MAX_RETRY_ATTEMPTS")
            
            try {
                // Obtener token válido (auto-refresh si es necesario)
                val token = getValidTokenWithRefresh(username, password, societe)
                if (token == null) {
                    Log.e(TAG, "❌ No se pudo obtener token válido")
                    lastError = Exception("Unable to get valid token")
                    continue
                }
                
                Log.d(TAG, "🔑 Using token: ${token.take(50)}...")
                
                // Obtener device info
                val deviceInfo = deviceInfoManager.getDeviceInfo()
                
                // ✅ CORREGIDO: Usar username directamente sin duplicar societe
                val matriculeCorrected = extractMatricule(username)  // ✅ "INTI_A187518" (sin duplicación)
                val usernameCorrected = extractUsername(username) // ✅ "A187518"
                
                Log.d(TAG, "🆔 Username recibido: $username")
                Log.d(TAG, "🆔 Matrícula para Colis Privé: $matriculeCorrected")
                Log.d(TAG, "🆔 Username corregido: $usernameCorrected")
                
                // Crear request de tournée
                val request = TourneeRequest(
                    username = usernameCorrected,  // ✅ Usar username corregido: "A187518"
                    password = password,
                    societe = societe,
                    date = date,
                    matricule = matriculeCorrected,  // ✅ Usar matrícula corregida: "PCP0010699_A187518"
                    token = token,
                    deviceInfo = deviceInfo
                )
                
                // ✅ AGREGADO: Headers con device info real
                val tourneeDeviceInfo = deviceInfoManager.getDeviceInfo()
                val response = api.getTourneeWithRetry(
                    request = request,
                    device = tourneeDeviceInfo.model,  // ✅ Device real del dispositivo
                    versionOS = tourneeDeviceInfo.androidVersion  // ✅ Version real de Android
                )
                Log.d(TAG, "📡 Tournée response code: ${response.code()}")
                
                when {
                    response.isSuccessful -> {
                        val tourneeData = response.body()!!
                        Log.d(TAG, "✅ Tournée exitosa: ${tourneeData.total_packages} paquetes")
                        return@withContext Result.success(tourneeData)
                    }
                    response.code() == 401 && attempt == 0 -> {
                        Log.w(TAG, "🔄 Token expirado (401), limpiando y retry...")
                        tokenManager.clearTokens()
                        lastError = Exception("Token expired, retrying with fresh auth")
                        continue
                    }
                    else -> {
                        val errorMsg = "Tournée failed with code: ${response.code()}"
                        Log.e(TAG, "❌ $errorMsg")
                        lastError = Exception(errorMsg)
                        continue
                    }
                }
                
            } catch (e: Exception) {
                Log.e(TAG, "❌ Network error on attempt ${attempt + 1}: ${e.message}", e)
                lastError = e
                if (attempt == MAX_RETRY_ATTEMPTS - 1) { // Last attempt
                    break
                }
            }
        }
        
        Result.failure(lastError ?: Exception("Max retries exceeded"))
    }
    
    /**
     * 🏥 HEALTH CHECK
     */
    suspend fun healthCheck(): Result<HealthResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🏥 === HEALTH CHECK ===")
            
            val response = api.healthCheck()
            Log.d(TAG, "📡 Health response code: ${response.code()}")
            
            if (response.isSuccessful) {
                val healthData = response.body()!!
                Log.d(TAG, "✅ Health check exitoso: ${healthData.status}")
                Result.success(healthData)
            } else {
                val errorMsg = "Health check failed with code: ${response.code()}"
                Log.e(TAG, "❌ $errorMsg")
                Result.failure(Exception(errorMsg))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en health check: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 📊 OBTENER ESTADO ACTUAL DEL REPOSITORY
     */
    suspend fun getCurrentState(): ColisRepositoryState = withContext(Dispatchers.IO) {
        try {
            val isAuthenticated = tokenManager.isUserLoggedIn()
            val userData = tokenManager.getSavedUserData()
            val tokenExpiration = tokenManager.getTokenExpirationInfo()
            val deviceInfo = deviceInfoManager.getDeviceInfo()
            val installationInfo = deviceInfoManager.getInstallationInfo()
            
            ColisRepositoryState(
                isAuthenticated = isAuthenticated,
                currentUser = userData?.matricule,
                username = userData?.username,
                societe = userData?.societe,
                tokenExpiration = tokenExpiration,
                deviceInfo = deviceInfo,
                installationInfo = installationInfo,
                lastUpdateTime = System.currentTimeMillis()
            )
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo estado del repository: ${e.message}", e)
            ColisRepositoryState()
        }
    }
    
    /**
     * 🗑️ LOGOUT
     */
    suspend fun logout() {
        try {
            Log.d(TAG, "🚪 === LOGOUT ===")
            
            tokenManager.clearTokens()
            Log.d(TAG, "✅ Logout exitoso")
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en logout: ${e.message}", e)
        }
    }
    
    /**
     * 🔍 OBTENER CREDENCIALES GUARDADAS
     */
    suspend fun getSavedCredentials(): UserTokenData? {
        return tokenManager.getSavedUserData()
    }
    
    /**
     * 🔍 VERIFICAR SI USUARIO ESTÁ LOGUEADO
     */
    suspend fun isUserLoggedIn(): Boolean {
        return tokenManager.isUserLoggedIn()
    }
    
    /**
     * 📱 OBTENER DEVICE INFO
     */
    fun getDeviceInfo() = deviceInfoManager.getDeviceInfo()
    
    /**
     * 🆔 OBTENER INSTALLATION INFO
     */
    fun getInstallationInfo() = deviceInfoManager.getInstallationInfo()
    
    // 🛠️ FUNCIONES PRIVADAS DE UTILIDAD
    
    /**
     * 🔑 OBTENER TOKEN VÁLIDO CON AUTO-REFRESH
     */
    private suspend fun getValidTokenWithRefresh(
        username: String,
        password: String,
        societe: String
    ): String? {
        
        // Verificar token actual
        val currentToken = tokenManager.getValidToken()
        if (currentToken != null) {
            Log.d(TAG, "✅ Token válido encontrado")
            return currentToken
        }
        
        Log.d(TAG, "🔄 Token expirado o no encontrado, intentando refresh...")
        
        // Intentar refresh
        val refreshResult = refreshToken()
        if (refreshResult.isSuccess) {
            val newToken = tokenManager.getValidToken()
            if (newToken != null) {
                Log.d(TAG, "✅ Token refresh exitoso")
                return newToken
            }
        }
        
        // Si refresh falla, hacer login fresh
        Log.d(TAG, "🔑 Haciendo login fresh...")
        val loginResult = authenticate(username, password, societe)
        if (loginResult.isSuccess) {
            val newToken = tokenManager.getValidToken()
            if (newToken != null) {
                Log.d(TAG, "✅ Login fresh exitoso")
                return newToken
            }
        }
        
        Log.e(TAG, "❌ No se pudo obtener token válido")
        return null
    }
    
    /**
     * 🔄 MANEJO DE RECONEXIÓN AUTOMÁTICA (RESUELVE EL 401)
     */
    private suspend fun handleReconnection(
        username: String,
        password: String,
        societe: String
    ): Result<AuthResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🔄 === MANEJANDO RECONEXIÓN (RESUELVE EL 401) ===")
            
            val deviceInfo = deviceInfoManager.getDeviceInfo()
            val currentDate = getCurrentDate()
            val matricule = extractMatricule(username)  // ✅ Usar username directamente
            val usernameCorrected = extractUsername(username)
            
            val request = ReconnectionRequest(
                username = usernameCorrected,
                password = password,
                societe = societe,
                date = currentDate,
                matricule = matricule,
                deviceInfo = deviceInfo
            )
            
            val activityId = UUID.randomUUID().toString()
            val response = api.handleReconnection(
                request = request,
                activityId = activityId,
                device = deviceInfo.model,
                versionOS = deviceInfo.androidVersion
            )
            
            if (response.isSuccessful) {
                val authData = response.body()!!
                Log.d(TAG, "✅ Reconexión exitosa - 401 RESUELTO")
                
                // Guardar tokens si hay
                if (authData.reconnectionResult?.success == true) {
                    val authenticationData = AuthenticationData(
                        matricule = matricule,
                        message = authData.message,
                        token = authData.reconnectionResult?.sessionId ?: ""
                    )
                    val backendAuthResponse = BackendAuthResponse(
                        authentication = authenticationData,
                        success = authData.success,
                        timestamp = authData.timestamp
                    )
                    tokenManager.saveTokens(backendAuthResponse)
                }
                
                Result.success(authData)
            } else {
                val errorMsg = "Reconexión falló con código: ${response.code()}"
                Log.e(TAG, "❌ $errorMsg")
                Result.failure(Exception(errorMsg))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en reconexión: ${e.message}", e)
            Result.failure(e)
        }
    }
}

/**
 * 📊 ESTADO COMPLETO DEL REPOSITORY
 */
data class ColisRepositoryState(
    val isAuthenticated: Boolean = false,
    val currentUser: String? = null,
    val username: String? = null,
    val societe: String? = null,
    val tokenExpiration: TokenExpirationInfo = TokenExpirationInfo(),
    val deviceInfo: DeviceInfo? = null,
    val installationInfo: InstallationInfo? = null,
    val lastUpdateTime: Long = 0
) {
    /**
     * 🔍 VERIFICAR SI TOKEN EXPIRA PRONTO
     */
    fun isTokenExpiringSoon(): Boolean {
        return tokenExpiration.isExpiringSoon()
    }
    
    /**
     * 🚨 VERIFICAR SI TOKEN EXPIRA MUY PRONTO
     */
    fun isTokenExpiringVerySoon(): Boolean {
        return tokenExpiration.isExpiringVerySoon()
    }
    
    /**
     * 📱 OBTENER FINGERPRINT DEL DISPOSITIVO
     */
    fun getDeviceFingerprint(): String? {
        return deviceInfo?.getFingerprint()
    }
    
    /**
     * 📅 FORMATO LEGIBLE DE ÚLTIMA ACTUALIZACIÓN
     */
    fun getFormattedLastUpdate(): String {
        val date = Date(lastUpdateTime)
        val formatter = java.text.SimpleDateFormat("HH:mm:ss", Locale.getDefault())
        return formatter.format(date)
    }
}
