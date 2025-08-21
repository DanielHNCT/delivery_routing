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
import java.util.Calendar

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
     * 🔐 AUTENTICACIÓN COMPLETA (API Mobile)
     */
    suspend fun authenticate(
        username: String,
        password: String,
        societe: String,
        date: String,
        matricule: String,
        deviceInfo: DeviceInfo
    ): Result<AuthResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🔐 === AUTENTICACIÓN COMPLETA (API MOBILE) ===")
            Log.d(TAG, "Username: $username")
            Log.d(TAG, "Societe: $societe")
            Log.d(TAG, "Matricule: $matricule")
            Log.d(TAG, "Date: $date")
            Log.d(TAG, "Device: ${deviceInfo.model}")
            Log.d(TAG, "Android Version: ${deviceInfo.androidVersion}")
            
            // Corregir username si es necesario
            val usernameCorrected = if (username.contains("_")) username else "${societe}_$username"
            Log.d(TAG, "Username corregido: $usernameCorrected")
            
            val currentDate = if (date.isBlank()) {
                val today = java.time.LocalDate.now()
                today.format(java.time.format.DateTimeFormatter.ISO_LOCAL_DATE)
            } else {
                date
            }
            Log.d(TAG, "Fecha final: $currentDate")
            
            val request = CompleteAuthFlowRequest(
                username = usernameCorrected,
                password = password,
                societe = societe,
                date = currentDate,
                matricule = matricule,
                deviceInfo = deviceInfo,
                apiChoice = "mobile"           // 🆕 NUEVO: Indicar que es API Mobile
            )
            
            Log.d(TAG, "📋 Request completo: $request")
            
            val activityId = UUID.randomUUID().toString()
            val loginDeviceInfo = deviceInfoManager.getDeviceInfo()
            
            Log.d(TAG, "🆔 Activity ID generado: $activityId")
            Log.d(TAG, "📱 Device Info real: ${loginDeviceInfo.model}, ${loginDeviceInfo.androidVersion}")
            
            // 🆕 NUEVO: Llamar al endpoint de flujo completo
            Log.d(TAG, "📡 Enviando request al backend...")
            val response = api.completeAuthenticationFlow(
                request = request,
                activityId = activityId,
                device = loginDeviceInfo.model,  // ✅ Device real del dispositivo
                versionOS = loginDeviceInfo.androidVersion  // ✅ Version real de Android
            )
            
            Log.d(TAG, "📡 Response code: ${response.code()}")
            Log.d(TAG, "📡 Response headers: ${response.headers()}")
            
            when {
                response.isSuccessful -> {
                    val authData = response.body()!!
                    Log.d(TAG, "✅ Flujo completo exitoso - 401 RESUELTO")
                    Log.d(TAG, "📊 Auth Data: success=${authData.success}, message=${authData.message}")
                    
                    // Guardar tokens en el manager (si hay)
                    if (authData.flowResult?.success == true) {
                        Log.d(TAG, "🔑 Guardando tokens en ColisTokenManager...")
                        
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
                        
                        Log.d(TAG, "📋 BackendAuthResponse creado: $backendAuthResponse")
                        
                        // 🆕 NUEVO: Log antes de guardar
                        Log.d(TAG, "💾 === ANTES DE GUARDAR TOKENS ===")
                        Log.d(TAG, "Token a guardar: ${authData.flowResult?.sessionId?.take(50)}...")
                        Log.d(TAG, "Matricule: $matricule")
                        
                        tokenManager.saveTokens(backendAuthResponse)
                        
                        // 🆕 NUEVO: Verificar que se guardaron
                        Log.d(TAG, "🔍 === VERIFICANDO TOKENS GUARDADOS ===")
                        val savedToken = tokenManager.getValidToken()
                        val isLoggedIn = tokenManager.isUserLoggedIn()
                        Log.d(TAG, "Token guardado: ${savedToken != null}")
                        Log.d(TAG, "Usuario logueado: $isLoggedIn")
                        
                        if (savedToken != null) {
                            Log.d(TAG, "✅ Token guardado exitosamente: ${savedToken.take(50)}...")
                        } else {
                            Log.w(TAG, "⚠️ Token NO se guardó correctamente")
                        }
                        
                        // 🆕 NUEVO: Log estado completo
                        tokenManager.logCurrentState()
                        
                    } else {
                        Log.w(TAG, "⚠️ FlowResult no exitoso: ${authData.flowResult}")
                    }
                    
                    // Retornar success
                    Log.d(TAG, "✅ === AUTENTICACIÓN COMPLETA EXITOSA ===")
                    Result.success(authData)
                }
                response.code() == 401 -> {
                    Log.e(TAG, "❌ 401 Unauthorized en flujo completo")
                    Log.e(TAG, "📋 Response body: ${response.errorBody()?.string()}")
                    Result.failure(Exception("Autenticación falló: 401 Unauthorized"))
                }
                else -> {
                    val errorMsg = "Error HTTP en flujo completo: ${response.code()}"
                    Log.e(TAG, "❌ $errorMsg")
                    Log.e(TAG, "📋 Response body: ${response.errorBody()?.string()}")
                    Result.failure(Exception(errorMsg))
                }
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en autenticación completa: ${e.message}", e)
            Log.e(TAG, "📋 Stack trace completo:", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🌐 AUTENTICACIÓN WEB API
     */
    suspend fun authenticateWeb(
        username: String,
        password: String,
        societe: String,
        date: String,
        matricule: String,
        deviceInfo: DeviceInfo
    ): Result<AuthResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🌐 === AUTENTICACIÓN WEB API ===")
            Log.d(TAG, "Username: $username")
            Log.d(TAG, "Societe: $societe")
            Log.d(TAG, "Matricule: $matricule")
            Log.d(TAG, "Date: $date")
            Log.d(TAG, "Device: ${deviceInfo.model}")
            Log.d(TAG, "Android Version: ${deviceInfo.androidVersion}")
            
            // Corregir username si es necesario
            Log.d(TAG, "🔧 Corrigiendo username...")
            val usernameCorrected = try {
                if (username.contains("_")) username else "${societe}_$username"
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error corrigiendo username: ${e.message}", e)
                throw e
            }
            Log.d(TAG, "✅ Username corregido: $usernameCorrected")
            
            Log.d(TAG, "📅 Procesando fecha...")
            val currentDate = try {
                if (date.isBlank()) {
                    // ✅ COMPATIBLE CON ANDROID 5.1.1 (API 22)
                    val calendar = Calendar.getInstance()
                    val year = calendar.get(Calendar.YEAR)
                    val month = calendar.get(Calendar.MONTH) + 1 // Calendar.MONTH es 0-based
                    val day = calendar.get(Calendar.DAY_OF_MONTH)
                    
                    // Formato: YYYY-MM-DD
                    String.format("%04d-%02d-%02d", year, month, day)
                } else {
                    date
                }
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error procesando fecha: ${e.message}", e)
                throw e
            }
            Log.d(TAG, "✅ Fecha final: $currentDate")
            
            Log.d(TAG, "📋 Creando request...")
            val request = try {
                CompleteAuthFlowRequest(
                    username = usernameCorrected,
                    password = password,
                    societe = societe,
                    date = currentDate,
                    matricule = matricule,
                    deviceInfo = deviceInfo,
                    apiChoice = "web"              // 🆕 NUEVO: Indicar que es API Web
                )
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error creando request: ${e.message}", e)
                throw e
            }
            
            Log.d(TAG, "✅ Request completo creado: $request")
            
            Log.d(TAG, "🆔 Generando Activity ID...")
            val activityId = try {
                UUID.randomUUID().toString()
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error generando Activity ID: ${e.message}", e)
                throw e
            }
            Log.d(TAG, "✅ Activity ID generado: $activityId")
            
            Log.d(TAG, "📱 Obteniendo device info real...")
            val loginDeviceInfo = try {
                deviceInfoManager.getDeviceInfo()
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error obteniendo device info real: ${e.message}", e)
                throw e
            }
            
            Log.d(TAG, "✅ Device Info real obtenido: ${loginDeviceInfo.model}, ${loginDeviceInfo.androidVersion}")
            
            // 🆕 NUEVO: Llamar al endpoint de autenticación web
            Log.d(TAG, "📡 Enviando request web al backend...")
            val response = try {
                api.completeAuthenticationFlow(
                    request = request,
                    activityId = activityId,
                    device = loginDeviceInfo.model,  // ✅ Device real del dispositivo
                    versionOS = loginDeviceInfo.androidVersion  // ✅ Version real de Android
                )
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error en llamada a API: ${e.message}", e)
                Log.e(TAG, "📋 Stack trace completo:", e)
                throw e
            }
            
            Log.d(TAG, "✅ Response recibido del backend")
            Log.d(TAG, "📡 Response code: ${response.code()}")
            Log.d(TAG, "📡 Response headers: ${response.headers()}")
            
            when {
                response.isSuccessful -> {
                    Log.d(TAG, "✅ Response exitoso, procesando body...")
                    val authData = try {
                        response.body()!!
                    } catch (e: Exception) {
                        Log.e(TAG, "❌ Error obteniendo response body: ${e.message}", e)
                        throw e
                    }
                    
                    Log.d(TAG, "✅ Autenticación web exitosa")
                    Log.d(TAG, "📊 Auth Data: success=${authData.success}, message=${authData.message}")
                    
                    // Guardar tokens en el manager (si hay)
                    if (authData.flowResult?.success == true) {
                        Log.d(TAG, "🔑 Guardando tokens web en ColisTokenManager...")
                        
                        try {
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
                            
                            Log.d(TAG, "📋 BackendAuthResponse web creado: $backendAuthResponse")
                            
                            // 🆕 NUEVO: Log antes de guardar
                            Log.d(TAG, "💾 === ANTES DE GUARDAR TOKENS WEB ===")
                            Log.d(TAG, "Token a guardar: ${authData.flowResult?.sessionId?.take(50)}...")
                            Log.d(TAG, "Matricule: $matricule")
                            
                            tokenManager.saveTokens(backendAuthResponse)
                            
                            // 🆕 NUEVO: Verificar que se guardaron
                            Log.d(TAG, "🔍 === VERIFICANDO TOKENS WEB GUARDADOS ===")
                            val savedToken = tokenManager.getValidToken()
                            val isLoggedIn = tokenManager.isUserLoggedIn()
                            Log.d(TAG, "Token web guardado: ${savedToken != null}")
                            Log.d(TAG, "Usuario logueado: $isLoggedIn")
                            
                            if (savedToken != null) {
                                Log.d(TAG, "✅ Token web guardado exitosamente: ${savedToken.take(50)}...")
                            } else {
                                Log.w(TAG, "⚠️ Token web NO se guardó correctamente")
                            }
                            
                            // 🆕 NUEVO: Log estado completo
                            tokenManager.logCurrentState()
                            
                        } catch (e: Exception) {
                            Log.e(TAG, "❌ Error guardando tokens: ${e.message}", e)
                            Log.e(TAG, "📋 Stack trace completo:", e)
                            // No lanzar excepción, continuar con el flujo
                        }
                        
                    } else {
                        Log.w(TAG, "⚠️ FlowResult web no exitoso: ${authData.flowResult}")
                    }
                    
                    // Retornar success
                    Log.d(TAG, "✅ === AUTENTICACIÓN WEB EXITOSA ===")
                    Result.success(authData)
                }
                response.code() == 401 -> {
                    Log.e(TAG, "❌ 401 Unauthorized en autenticación web")
                    val errorBody = try {
                        response.errorBody()?.string()
                    } catch (e: Exception) {
                        Log.e(TAG, "❌ Error leyendo error body: ${e.message}", e)
                        "Error body no disponible"
                    }
                    Log.e(TAG, "📋 Response body: $errorBody")
                    Result.failure(Exception("Autenticación web falló: 401 Unauthorized"))
                }
                else -> {
                    val errorMsg = "Error HTTP en autenticación web: ${response.code()}"
                    Log.e(TAG, "❌ $errorMsg")
                    val errorBody = try {
                        response.errorBody()?.string()
                    } catch (e: Exception) {
                        Log.e(TAG, "❌ Error leyendo error body: ${e.message}", e)
                        "Error body no disponible"
                    }
                    Log.e(TAG, "📋 Response body: $errorBody")
                    Result.failure(Exception(errorMsg))
                }
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en autenticación web: ${e.message}", e)
            Log.e(TAG, "📋 Stack trace completo:", e)
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
            
            Log.d(TAG, "🔑 Token anterior encontrado: ${oldToken.take(50)}...")
            
            val deviceInfo = deviceInfoManager.getDeviceInfo()
            Log.d(TAG, "📱 Device Info para refresh: ${deviceInfo.model}, ${deviceInfo.androidVersion}")
            
            val request = RefreshTokenRequest(
                token = oldToken,
                deviceInfo = deviceInfo
            )
            
            Log.d(TAG, "📋 Refresh request: $request")
            Log.d(TAG, "📡 Enviando request de refresh...")
            
            val refreshDeviceInfo = deviceInfoManager.getDeviceInfo()
            val response = api.refreshToken(
                request = request,
                device = refreshDeviceInfo.model,  // ✅ Device real del dispositivo
                versionOS = refreshDeviceInfo.androidVersion  // ✅ Version real de Android
            )
            Log.d(TAG, "📡 Refresh response code: ${response.code()}")
            Log.d(TAG, "📡 Refresh response headers: ${response.headers()}")
            
            if (response.isSuccessful) {
                val newTokenData = response.body()!!
                Log.d(TAG, "✅ Token refresh exitoso")
                Log.d(TAG, "📊 New token data: success=${newTokenData.success}")
                
                // Guardar nuevos tokens
                Log.d(TAG, "🔑 Guardando nuevos tokens después del refresh...")
                tokenManager.saveTokens(newTokenData)
                
                // 🆕 NUEVO: Verificar que se guardaron
                Log.d(TAG, "🔍 === VERIFICANDO TOKENS REFRESHADOS ===")
                val savedToken = tokenManager.getValidToken()
                val isLoggedIn = tokenManager.isUserLoggedIn()
                Log.d(TAG, "Token refreshado guardado: ${savedToken != null}")
                Log.d(TAG, "Usuario logueado después del refresh: $isLoggedIn")
                
                if (savedToken != null) {
                    Log.d(TAG, "✅ Token refreshado guardado exitosamente: ${savedToken.take(50)}...")
                } else {
                    Log.w(TAG, "⚠️ Token refreshado NO se guardó correctamente")
                }
                
                // 🆕 NUEVO: Log estado completo después del refresh
                tokenManager.logCurrentState()
                
                Result.success(newTokenData)
            } else {
                val errorMsg = "Refresh failed with code: ${response.code()}"
                Log.e(TAG, "❌ $errorMsg")
                Log.e(TAG, "📋 Refresh response body: ${response.errorBody()?.string()}")
                Result.failure(Exception(errorMsg))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en refresh token: ${e.message}", e)
            Log.e(TAG, "📋 Stack trace completo:", e)
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
                
                // 🆕 NUEVO: Log antes de generar matricule
                Log.d(TAG, "🆔 Generando matricule...")
                val matricule = try {
                    // ✅ CORREGIDO: Evitar duplicación de societe
                    if (username.startsWith(societe)) {
                        username // Ya tiene el formato correcto
                    } else {
                        "${societe}_$username"
                    }
                } catch (e: Exception) {
                    Log.e(TAG, "❌ Error generando matricule: ${e.message}", e)
                    throw e
                }
                Log.d(TAG, "✅ Matricule generado: $matricule")
                
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
            Log.d(TAG, "📊 === OBTENIENDO ESTADO ACTUAL DEL REPOSITORY ===")
            
            val isAuthenticated = tokenManager.isUserLoggedIn()
            Log.d(TAG, "🔐 Usuario autenticado: $isAuthenticated")
            
            val userData = tokenManager.getSavedUserData()
            Log.d(TAG, "👥 Datos de usuario: $userData")
            
            val tokenExpiration = tokenManager.getTokenExpirationInfo()
            Log.d(TAG, "⏰ Información de expiración: $tokenExpiration")
            
            val deviceInfo = deviceInfoManager.getDeviceInfo()
            Log.d(TAG, "📱 Device Info: ${deviceInfo.model}, ${deviceInfo.androidVersion}")
            
            val installationInfo = deviceInfoManager.getInstallationInfo()
            Log.d(TAG, "📦 Installation Info: $installationInfo")
            
            val lastUpdateTime = System.currentTimeMillis()
            Log.d(TAG, "🕐 Última actualización: ${java.util.Date(lastUpdateTime)}")
            
            // 🆕 NUEVO: Log estado completo del token manager
            Log.d(TAG, "🔍 === ESTADO COMPLETO DEL TOKEN MANAGER ===")
            tokenManager.logCurrentState()
            
            val state = ColisRepositoryState(
                isAuthenticated = isAuthenticated,
                currentUser = userData?.matricule,
                username = userData?.username,
                societe = userData?.societe,
                tokenExpiration = tokenExpiration,
                deviceInfo = deviceInfo,
                installationInfo = installationInfo,
                lastUpdateTime = lastUpdateTime
            )
            
            Log.d(TAG, "📊 Estado del repository creado: $state")
            Log.d(TAG, "✅ === ESTADO DEL REPOSITORY OBTENIDO EXITOSAMENTE ===")
            
            state
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo estado del repository: ${e.message}", e)
            Log.e(TAG, "📋 Stack trace completo:", e)
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
        Log.d(TAG, "🔍 === VERIFICANDO SI USUARIO ESTÁ LOGUEADO ===")
        
        val isLoggedIn = tokenManager.isUserLoggedIn()
        Log.d(TAG, "🔐 Resultado de verificación: $isLoggedIn")
        
        if (isLoggedIn) {
            Log.d(TAG, "✅ Usuario está logueado")
            // 🆕 NUEVO: Log estado detallado si está logueado
            tokenManager.logCurrentState()
        } else {
            Log.w(TAG, "❌ Usuario NO está logueado")
            // 🆕 NUEVO: Log estado detallado si NO está logueado
            tokenManager.logCurrentState()
        }
        
        return isLoggedIn
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
        val deviceInfo = deviceInfoManager.getDeviceInfo()
        val currentDate = getCurrentDate()
        val matricule = extractMatricule(username)
        val loginResult = authenticate(username, password, societe, currentDate, matricule, deviceInfo)
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
