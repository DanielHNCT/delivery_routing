package com.daniel.deliveryrouting.data.repository

import android.content.Context
import android.provider.Settings
import com.daniel.deliveryrouting.data.api.ColisApiService
import com.daniel.deliveryrouting.data.api.ColisSoapClient
import com.daniel.deliveryrouting.data.api.EnvironmentConfig
import com.daniel.deliveryrouting.data.api.models.*
import com.daniel.deliveryrouting.data.api.models.BackendResponseMapper
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import android.util.Log
import java.util.*

class ColisRepository(private val context: Context) {
    
    private val api = ColisApiService.api
    private val soapClient = ColisSoapClient()
    
    suspend fun authenticate(
        username: String,
        password: String,
        societe: String
    ): Result<ColisLoginResponse> = withContext(Dispatchers.IO) {
        
        try {
            Log.d("ColisRepository", "=== INICIO AUTENTICACIÓN VÍA BACKEND LOCAL ===")
            Log.d("ColisRepository", "🎯 USANDO BACKEND LOCAL PARA OPTIMIZACIÓN DE RUTAS")
            Log.d("ColisRepository", "Backend URL: ${EnvironmentConfig.getBaseUrl()}")
            Log.d("ColisRepository", "Username: $username")
            Log.d("ColisRepository", "Societe: $societe")
            Log.d("ColisRepository", "Password length: ${password.length}")
            
            // Paso 1: SOAP BonjourDistri (opcional pero replicando app oficial)
            val imei = getDeviceImei()
            Log.d("ColisRepository", "Device IMEI: $imei")
            
            soapClient.bonjourDistri(username, imei).onFailure { error -> 
                // Log error pero continúa (como en app oficial)
                Log.w("ColisRepository", "SOAP BonjourDistri failed: ${error.message}")
            }.onSuccess { response ->
                Log.d("ColisRepository", "SOAP BonjourDistri success: $response")
            }
            
            // Paso 2: REST Authentication (principal)
            val deviceInfo = getDeviceInfo()
            Log.d("ColisRepository", "Device Info: $deviceInfo")
            
            val request = ColisLoginRequest(
                audit = AuditData(
                    appName = "CP DISTRI V2",
                    cle1 = "",
                    cle2 = "",
                    cle3 = "",
                    deviceModelName = deviceInfo.model,
                    iccid = deviceInfo.iccid,
                    imei = deviceInfo.imei,
                    msisdn = deviceInfo.msisdn,
                    noSerie = deviceInfo.serialNumber
                ),
                commun = CommunData(dureeTokenInHour = 0),
                username = username,  // ✅ CAMBIADO: login → username para coincidir con backend
                password = password,
                societe = societe
            )
            
            Log.d("ColisRepository", "REST Request: $request")
            
            val activityId = UUID.randomUUID().toString()
            Log.d("ColisRepository", "ActivityId: $activityId")
            
            val response = api.login(request, activityId)
            Log.d("ColisRepository", "REST Response Code: ${response.code()}")
            Log.d("ColisRepository", "REST Response Headers: ${response.headers()}")
            
            response.body()?.let { backendResponse ->
                Log.d("ColisRepository", "=== RESPUESTA COMPLETA DEL BACKEND ===")
                Log.d("ColisRepository", "Backend success: ${backendResponse.success}")
                Log.d("ColisRepository", "Backend matricule: ${backendResponse.authentication.matricule}")
                Log.d("ColisRepository", "Backend message: ${backendResponse.authentication.message}")
                Log.d("ColisRepository", "Backend token: ${backendResponse.authentication.token.take(100)}...")
                Log.d("ColisRepository", "Backend timestamp: ${backendResponse.timestamp}")
                Log.d("ColisRepository", "=== FIN RESPUESTA BACKEND ===")
                
                // ✅ MAPEAR BACKEND → COLIS FORMAT
                val colisResponse = BackendResponseMapper.mapBackendAuthToColisLogin(backendResponse)
                BackendResponseMapper.logMappingDetails(backendResponse, colisResponse)
                
                if (backendResponse.success) {
                    Log.d("ColisRepository", "✅ AUTENTICACIÓN EXITOSA VÍA BACKEND")
                    Log.d("ColisRepository", "Matricule: ${backendResponse.authentication.matricule}")
                    Log.d("ColisRepository", "Token: ${backendResponse.authentication.token.take(50)}...")
                    Log.d("ColisRepository", "Message: ${backendResponse.authentication.message}")
                    Result.success(colisResponse)
                } else {
                    val errorMsg = backendResponse.authentication.message ?: "Backend authentication failed"
                    Log.e("ColisRepository", "❌ AUTENTICACIÓN BACKEND FALLÓ: $errorMsg")
                    Result.failure(Exception(errorMsg))
                }
            } ?: Result.failure(Exception("Empty response"))
            
        } catch (e: Exception) {
            Log.e("ColisRepository", "❌ EXCEPCIÓN EN AUTENTICACIÓN: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    private fun getDeviceInfo(): DeviceInfo {
        return DeviceInfo(
            model = android.os.Build.MODEL,
            serialNumber = generateDeviceSerial(),
            imei = getDeviceImei(),
            iccid = "indisponible",
            msisdn = "indisponible"
        )
    }
    
    private fun getDeviceImei(): String {
        return try {
            // Usar ANDROID_ID como fallback (más confiable que IMEI)
            Settings.Secure.getString(context.contentResolver, Settings.Secure.ANDROID_ID)
        } catch (e: Exception) {
            Log.w("ColisRepository", "Error getting device ID: ${e.message}")
            "ce31ec9aab417230" // Fallback al IMEI capturado
        }
    }
    
    private fun generateDeviceSerial(): String {
        // Usar serial real o generar uno basado en device
        return try {
            android.os.Build.SERIAL.takeIf { it.isNotBlank() && it != "unknown" }
                ?: "oj13vnazlp1ymruibx978j13" // Fallback al serial capturado
        } catch (e: Exception) {
            Log.w("ColisRepository", "Error getting device serial: ${e.message}")
            "oj13vnazlp1ymruibx978j13"
        }
    }
    
    // 🗺️ NUEVOS MÉTODOS PARA OPTIMIZACIÓN DE RUTAS
    // TODO: Implementar métodos de optimización una vez que el backend esté completamente funcional
}
