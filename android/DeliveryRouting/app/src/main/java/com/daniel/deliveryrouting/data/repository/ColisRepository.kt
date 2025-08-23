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
 * 🔗 REPOSITORY SIMPLIFICADO PARA COLIS PRIVÉ
 * 
 * Características:
 * - ✅ Solo endpoints esenciales: login directo y lettre de voiture
 * - ✅ Integra DeviceInfoManager y TokenManager
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
    }
    
    /**
     * 🆕 NUEVO: LOGIN DIRECTO A COLIS PRIVE (PROXY)
     * 
     * Este método envía las credenciales directamente al backend
     * que actúa como proxy hacia Colis Prive
     */
    suspend fun loginDirectToColisPrive(
        username: String,
        password: String,
        societe: String,
        apiChoice: String = "web"
    ): Result<ColisPriveLoginResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🚀 Iniciando login directo a Colis Prive")
            
            val request = ColisPriveLoginRequest(
                username = username,
                password = password,
                societe = societe,
                apiChoice = apiChoice
            )
            
            val response = api.loginDirectToColisPrive(request)
            
            if (response.isSuccessful) {
                val loginData = response.body()!!
                Log.d(TAG, "✅ Login directo exitoso")
                
                // Guardar token si la autenticación fue exitosa
                if (loginData.success && loginData.data != null) {
                    val tokenData = UserTokenData(
                        token = loginData.data.token,
                        matricule = loginData.data.matricule,
                        societe = loginData.data.societe,
                        timestamp = System.currentTimeMillis()
                    )
                    tokenManager.saveTokens(tokenData)
                    Log.d(TAG, "💾 Token guardado exitosamente")
                }
                
                Result.success(loginData)
            } else {
                val errorMsg = "Login falló con código: ${response.code()}"
                Log.e(TAG, "❌ $errorMsg")
                Result.failure(Exception(errorMsg))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en login directo: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🆕 NUEVO: OBTENER LETTRE DE VOITURE COMPLETO
     * 
     * Este método usa el token guardado para obtener el lettre de voiture
     * sin necesidad de hacer login completo
     */
    suspend fun getLettreDeVoiture(
        matricule: String,
        societe: String,
        date: String? = null
    ): Result<LettreDeVoitureResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "📋 Obteniendo Lettre de Voiture para: $matricule")
            
            // Obtener token guardado
            val tokenData = tokenManager.getTokens()
            if (tokenData == null) {
                Log.e(TAG, "❌ No hay token guardado para obtener lettre de voiture")
                return@withContext Result.failure(Exception("No hay token de autenticación"))
            }
            
            val currentDate = date ?: getCurrentDate()
            
            val request = LettreDeVoitureRequest(
                token = tokenData.token,
                matricule = matricule,
                societe = societe,
                date = currentDate
            )
            
            val activityId = UUID.randomUUID().toString()
            val response = api.getLettreDeVoiture(
                request = request,
                activityId = activityId
            )
            
            if (response.isSuccessful) {
                val lettreData = response.body()!!
                Log.d(TAG, "✅ Lettre de Voiture obtenido exitosamente")
                
                Result.success(lettreData)
            } else {
                val errorMsg = "Error obteniendo lettre de voiture: ${response.code()}"
                Log.e(TAG, "❌ $errorMsg")
                Result.failure(Exception(errorMsg))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo lettre de voiture: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🏥 HEALTH CHECK
     */
    suspend fun healthCheck(): Result<HealthResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🏥 Verificando salud del backend")
            
            val response = api.healthCheck()
            
            if (response.isSuccessful) {
                val healthData = response.body()!!
                Log.d(TAG, "✅ Health check exitoso: ${healthData.status}")
                Result.success(healthData)
            } else {
                val errorMsg = "Health check falló con código: ${response.code()}"
                Log.e(TAG, "❌ $errorMsg")
                Result.failure(Exception(errorMsg))
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en health check: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🔍 OBTENER ESTADO ACTUAL DEL REPOSITORY
     */
    fun getRepositoryState(): ColisRepositoryState {
        val tokenData = tokenManager.getTokens()
        val deviceInfo = deviceInfoManager.getDeviceInfo()
        val installationInfo = deviceInfoManager.getInstallationInfo()
        
        return ColisRepositoryState(
            isAuthenticated = tokenData != null,
            currentUser = tokenData?.matricule,
            username = tokenData?.matricule,
            societe = tokenData?.societe,
            tokenExpiration = tokenData?.let { TokenExpirationInfo(it.timestamp) } ?: TokenExpirationInfo(),
            deviceInfo = deviceInfo,
            installationInfo = installationInfo,
            lastUpdateTime = System.currentTimeMillis()
        )
    }
    
    /**
     * 🧹 LIMPIAR TOKENS
     */
    fun clearTokens() {
        tokenManager.clearTokens()
        Log.d(TAG, "🧹 Tokens limpiados")
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
