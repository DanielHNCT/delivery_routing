package com.daniel.deliveryrouting.data.repository

import com.daniel.deliveryrouting.data.api.ApiService
import com.daniel.deliveryrouting.data.api.models.LoginRequest
import com.daniel.deliveryrouting.data.api.models.LoginResponse
import com.daniel.deliveryrouting.data.api.models.TourneeRequest
import com.daniel.deliveryrouting.data.api.models.TourneeResponse
import com.daniel.deliveryrouting.data.api.models.TourneeUpdateRequest
import com.daniel.deliveryrouting.data.preferences.PreferencesManager
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import android.util.Log
import com.daniel.deliveryrouting.utils.DebugUtils

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add location-based repository methods
// 2. Add route optimization methods
// 3. Add distance calculation methods

class DeliveryRepository(
    private val apiService: ApiService,
    private val preferencesManager: PreferencesManager
) {
    
    suspend fun login(username: String, password: String, societe: String): Result<LoginResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d("RepoDebug", "=== REPOSITORY LOGIN START ===")
            Log.d("RepoDebug", "Username: '$username'")
            Log.d("RepoDebug", "Societe: '$societe'")
            Log.d("RepoDebug", "Password length: ${password.length}")
            
            val request = LoginRequest(username, password, societe)
            Log.d("RepoDebug", "Request object: $request")
            
            Log.d("RepoDebug", "Llamando apiService.login...")
            val response = apiService.login(request)
            
            Log.d("RepoDebug", "=== RESPONSE RECIBIDA ===")
            Log.d("RepoDebug", "Response object: $response")
            Log.d("RepoDebug", "Response.success: ${response.success}")
            Log.d("RepoDebug", "Response.status: '${response.status}'")
            Log.d("RepoDebug", "Response.code: '${response.code}'")
            Log.d("RepoDebug", "Response.token: '${response.token}'")
            Log.d("RepoDebug", "Response.message: '${response.message}'")
            Log.d("RepoDebug", "Response.authentication: ${response.authentication}")
            Log.d("RepoDebug", "Response.error: ${response.error}")
            
            // Usar utilidades de debugging existentes
            DebugUtils.logLoginResponse(response, "RepoDebug")
            
            // LÓGICA MUY PERMISIVA - Si hay token, es éxito
            val token = response.authentication?.token ?: response.token
            
            Log.d("RepoDebug", "Token extraído: '$token'")
            
            if (!token.isNullOrBlank()) {
                Log.d("RepoDebug", "=== GUARDANDO TOKEN Y RETORNANDO SUCCESS ===")
                preferencesManager.saveAuthToken(token)
                // Guardar también username, societe y password para futuras peticiones
                preferencesManager.saveUsername(username)
                preferencesManager.saveSociete(societe)
                preferencesManager.savePassword(password)
                
                Log.d("RepoDebug", "Token guardado exitosamente")
                Result.success(response)
            } else {
                Log.e("RepoDebug", "=== ERROR: NO HAY TOKEN ===")
                Log.e("RepoDebug", "response.success: ${response.success}")
                Log.e("RepoDebug", "response.authentication?.token: '${response.authentication?.token}'")
                Log.e("RepoDebug", "response.token: '${response.token}'")
                
                val errorMessage = response.error?.message 
                                 ?: response.message 
                                 ?: "Token no encontrado en respuesta"
                
                Log.e("RepoDebug", "Error message: $errorMessage")
                Result.failure(Exception(errorMessage))
            }
            
        } catch (e: Exception) {
            Log.e("RepoDebug", "=== EXCEPCIÓN EN REPOSITORY ===")
            Log.e("RepoDebug", "Exception type: ${e::class.java.simpleName}")
            Log.e("RepoDebug", "Exception message: ${e.message}")
            Log.e("RepoDebug", "Stack trace: ${e.stackTraceToString()}")
            
            Result.failure(e)
        }
    }
    
    suspend fun getTourneeStructured(tourneeCode: String, date: String): Result<TourneeResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d("TourneeDebug", "=== CARGA DE TOURNÉE START ===")
            Log.d("TourneeDebug", "TourneeCode: '$tourneeCode'")
            Log.d("TourneeDebug", "Date: '$date'")
            
            // Incluir username y password en la petición de tournée
            val username = preferencesManager.getUsername() ?: ""
            val password = preferencesManager.getPassword() ?: ""
            Log.d("TourneeDebug", "Username para tournée: '$username'")
            Log.d("TourneeDebug", "Password para tournée: '${password.take(3)}...'")
            val request = TourneeRequest(
                username = username,
                password = password,
                societe = preferencesManager.getSociete() ?: "",
                date = date,
                matricule = username // Usar username como matricule para compatibilidad
            )
            Log.d("TourneeDebug", "Request object: $request")
            
            Log.d("TourneeDebug", "Llamando apiService.getTourneeStructured...")
            val response = apiService.getTourneeStructured(request)
            
            Log.d("TourneeDebug", "=== RESPUESTA TOURNÉE RECIBIDA ===")
            Log.d("TourneeDebug", "Response object: $response")
            Log.d("TourneeDebug", "Response.success: ${response.success}")
            Log.d("TourneeDebug", "Response.message: '${response.message}'")
            Log.d("TourneeDebug", "Response.data: ${response.data}")
            Log.d("TourneeDebug", "Response.data?.packages?.size: ${response.data?.packages?.size}")
            
            if (response.success) {
                Log.d("TourneeDebug", "=== TOURNÉE EXITOSO ===")
                preferencesManager.saveTourneeCode(tourneeCode)
                preferencesManager.saveLastSyncTime(System.currentTimeMillis())
                Log.d("TourneeDebug", "Tournée guardado en preferences")
                Result.success(response)
            } else {
                Log.e("TourneeDebug", "=== TOURNÉE FALLÓ ===")
                Log.e("TourneeDebug", "response.success: ${response.success}")
                Log.e("TourneeDebug", "response.message: '${response.message}'")
                
                val errorMessage = response.message ?: "Failed to get tournee"
                Log.e("TourneeDebug", "Error message: $errorMessage")
                Result.failure(Exception(errorMessage))
            }
        } catch (e: Exception) {
            Log.e("TourneeDebug", "=== EXCEPCIÓN EN TOURNÉE ===")
            Log.e("TourneeDebug", "Exception type: ${e::class.java.simpleName}")
            Log.e("TourneeDebug", "Exception message: ${e.message}")
            Log.e("TourneeDebug", "Stack trace: ${e.stackTraceToString()}")
            Result.failure(e)
        }
    }
    
    suspend fun getTourneeUpdated(matricule: String, date: String): Result<TourneeResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d("TourneeUpdatedDebug", "=== CARGA TOURNÉE MISE À JOUR START ===")
            Log.d("TourneeUpdatedDebug", "Matricule: '$matricule'")
            Log.d("TourneeUpdatedDebug", "Date: '$date'")
            
            val password = preferencesManager.getPassword() ?: ""
            val societe = preferencesManager.getSociete() ?: ""
            Log.d("TourneeUpdatedDebug", "Societe: '$societe'")
            Log.d("TourneeUpdatedDebug", "Password len: ${password.length}")
            val request = TourneeRequest(
                username = matricule,  // ✅ username para el backend
                password = password,
                societe = societe,
                date = date,
                matricule = matricule,  // ✅ matricule también se envía
                token = null  // ✅ token opcional
            )
            Log.d("TourneeUpdatedDebug", "Request object: $request")
            
            Log.d("TourneeUpdatedDebug", "Llamando apiService.getTourneeUpdated...")
            val response = apiService.getTourneeUpdated(request)
            
            Log.d("TourneeUpdatedDebug", "=== RESPUESTA TOURNÉE MISE À JOUR ===")
            Log.d("TourneeUpdatedDebug", "Response object: $response")
            Log.d("TourneeUpdatedDebug", "Response.success: ${response.success}")
            Log.d("TourneeUpdatedDebug", "Response.message: '${response.message}'")
            Log.d("TourneeUpdatedDebug", "Response.data: ${response.data}")
            Log.d("TourneeUpdatedDebug", "Response.data?.packages?.size: ${response.data?.packages?.size}")
            
            if (response.success) {
                Log.d("TourneeUpdatedDebug", "=== TOURNÉE MISE À JOUR EXITOSO ===")
                // Guardar la tournée cargada para uso posterior
                response.data?.tourneeCode?.let { tourneeCode ->
                    preferencesManager.saveTourneeCode(tourneeCode)
                    preferencesManager.saveLastSyncTime(System.currentTimeMillis())
                    Log.d("TourneeUpdatedDebug", "Tournée guardada: $tourneeCode")
                }
                Result.success(response)
            } else {
                Log.e("TourneeUpdatedDebug", "=== TOURNÉE MISE À JOUR FALLÓ ===")
                val errorMessage = response.message ?: "Failed to get tournee updated"
                Log.e("TourneeUpdatedDebug", "Error message: $errorMessage")
                Result.failure(Exception(errorMessage))
            }
            
        } catch (e: Exception) {
            Log.e("TourneeUpdatedDebug", "=== EXCEPCIÓN EN TOURNÉE MISE À JOUR ===")
            Log.e("TourneeUpdatedDebug", "Exception type: ${e::class.java.simpleName}")
            Log.e("TourneeUpdatedDebug", "Exception message: ${e.message}")
            Log.e("TourneeUpdatedDebug", "Stack trace: ${e.stackTraceToString()}")
            Result.failure(e)
        }
    }
    
    suspend fun healthCheck(): Result<Boolean> = withContext(Dispatchers.IO) {
        try {
            val response = apiService.healthCheck()
            Result.success(response.isSuccessful)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
    
    fun isLoggedIn(): Boolean {
        return preferencesManager.isLoggedIn()
    }
    
    fun logout() {
        preferencesManager.clearAuthToken()
    }
    
    fun getStoredTourneeCode(): String? {
        return preferencesManager.getTourneeCode()
    }
}
