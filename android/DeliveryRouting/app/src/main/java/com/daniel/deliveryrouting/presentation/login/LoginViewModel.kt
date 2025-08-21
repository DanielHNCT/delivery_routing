package com.daniel.deliveryrouting.presentation.login

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import com.daniel.deliveryrouting.data.repository.ColisRepository
import com.daniel.deliveryrouting.data.api.models.AuthResponse
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import android.content.Context
import android.util.Log
import java.util.Calendar

class LoginViewModel(
    private val repository: ColisRepository
) : ViewModel() {

    private val _loginState = MutableStateFlow<LoginState>(LoginState.Idle)
    val loginState: StateFlow<LoginState> = _loginState.asStateFlow()

    fun login(username: String, password: String, societe: String, apiType: String = "web") {
        Log.d("LoginViewModel", "=== INICIO LOGIN ===")
        Log.d("LoginViewModel", "Username: $username")
        Log.d("LoginViewModel", "Password length: ${password.length}")
        Log.d("LoginViewModel", "Societe: $societe")
        Log.d("LoginViewModel", "API Type: $apiType")

        _loginState.value = LoginState.Loading

        viewModelScope.launch {
            try {
                Log.d("LoginViewModel", "🚀 Iniciando corrutina de login...")
                Log.d("LoginViewModel", "Llamando repository.authenticate con API: $apiType...")
                
                // 🆕 NUEVO: Log antes de obtener device info
                Log.d("LoginViewModel", "📱 Obteniendo device info...")
                val deviceInfo = try {
                    repository.getDeviceInfo()
                } catch (e: Exception) {
                    Log.e("LoginViewModel", "❌ Error obteniendo device info: ${e.message}", e)
                    throw e
                }
                Log.d("LoginViewModel", "✅ Device info obtenido: ${deviceInfo.model}, ${deviceInfo.androidVersion}")
                
                // 🆕 NUEVO: Log antes de generar fecha
                Log.d("LoginViewModel", "📅 Generando fecha actual...")
                val currentDate = try {
                    // ✅ COMPATIBLE CON ANDROID 5.1.1 (API 22)
                    val calendar = Calendar.getInstance()
                    val year = calendar.get(Calendar.YEAR)
                    val month = calendar.get(Calendar.MONTH) + 1 // Calendar.MONTH es 0-based
                    val day = calendar.get(Calendar.DAY_OF_MONTH)
                    
                    // Formato: YYYY-MM-DD
                    String.format("%04d-%02d-%02d", year, month, day)
                } catch (e: Exception) {
                    Log.e("LoginViewModel", "❌ Error generando fecha: ${e.message}", e)
                    throw e
                }
                Log.d("LoginViewModel", "✅ Fecha generada: $currentDate")
                
                // 🆕 NUEVO: Log antes de generar matricule
                Log.d("LoginViewModel", "🆔 Generando matricule...")
                val matricule = try {
                    // ✅ CORREGIDO: Evitar duplicación de societe
                    if (username.startsWith(societe)) {
                        username // Ya tiene el formato correcto
                    } else {
                        "${societe}_$username"
                    }
                } catch (e: Exception) {
                    Log.e("LoginViewModel", "❌ Error generando matricule: ${e.message}", e)
                    throw e
                }
                Log.d("LoginViewModel", "✅ Matricule generado: $matricule")
                
                // 🚀 RUTEAR SEGÚN TIPO DE API
                Log.d("LoginViewModel", "🔄 Ruteando según API type: $apiType")
                val result = when (apiType) {
                    "web" -> {
                        Log.d("LoginViewModel", "🌐 Usando API Web (más simple)")
                        Log.d("LoginViewModel", "📋 Parámetros: username=$username, password=***, societe=$societe, date=$currentDate, matricule=$matricule")
                        repository.authenticateWeb(username, password, societe, currentDate, matricule, deviceInfo)
                    }
                    "mobile" -> {
                        Log.d("LoginViewModel", "📱 Usando API Mobile (completa)")
                        Log.d("LoginViewModel", "📋 Parámetros: username=$username, password=***, societe=$societe, date=$currentDate, matricule=$matricule")
                        repository.authenticate(username, password, societe, currentDate, matricule, deviceInfo)
                    }
                    else -> {
                        Log.d("LoginViewModel", "🌐 API no especificada, usando Web por defecto")
                        Log.d("LoginViewModel", "📋 Parámetros: username=$username, password=***, societe=$societe, date=$currentDate, matricule=$matricule")
                        repository.authenticateWeb(username, password, societe, currentDate, matricule, deviceInfo)
                    }
                }
                
                Log.d("LoginViewModel", "✅ Llamada al repository completada, procesando resultado...")
                
                result.fold(
                    onSuccess = { response ->
                        Log.d("LoginViewModel", "✅ LOGIN EXITOSO CON API: $apiType")
                        Log.d("LoginViewModel", "📊 Response completa: $response")
                        Log.d("LoginViewModel", "Flow Result: ${response.flowResult?.success}")
                        Log.d("LoginViewModel", "Session ID: ${response.flowResult?.sessionId?.take(50)}...")
                        
                        val matricule = response.flowResult?.sessionId?.split("_")?.lastOrNull() ?: username
                        val token = response.flowResult?.sessionId ?: ""
                        
                        Log.d("LoginViewModel", "🔑 Matricule extraído: $matricule")
                        Log.d("LoginViewModel", "🔑 Token extraído: ${token.take(50)}...")
                        
                        _loginState.value = LoginState.Success(
                            matricule = matricule,
                            token = token
                        )
                        
                        Log.d("LoginViewModel", "✅ Estado de login actualizado a Success")
                    },
                    onFailure = { exception ->
                        Log.e("LoginViewModel", "❌ LOGIN FALLÓ CON API: $apiType", exception)
                        Log.e("LoginViewModel", "📋 Stack trace completo:", exception)
                        _loginState.value = LoginState.Error(
                            message = "Error con API $apiType: ${exception.message ?: "Error desconocido"}"
                        )
                        Log.d("LoginViewModel", "✅ Estado de login actualizado a Error")
                    }
                )
                
                Log.d("LoginViewModel", "✅ === LOGIN COMPLETADO ===")
                
            } catch (e: Exception) {
                Log.e("LoginViewModel", "❌ EXCEPCIÓN EN LOGIN CON API: $apiType", e)
                Log.e("LoginViewModel", "📋 Stack trace completo:", e)
                _loginState.value = LoginState.Error(
                    message = "Excepción con API $apiType: ${e.message ?: "Excepción inesperada"}"
                )
                Log.d("LoginViewModel", "✅ Estado de login actualizado a Error después de excepción")
            }
        }
    }

    fun resetState() {
        _loginState.value = LoginState.Idle
    }

    sealed class LoginState {
        object Idle : LoginState()
        object Loading : LoginState()
        data class Success(val matricule: String, val token: String) : LoginState()
        data class Error(val message: String) : LoginState()
    }
}

/**
 * Factory para crear LoginViewModel con ColisRepository
 */
class LoginViewModelFactory(private val context: Context) : ViewModelProvider.Factory {
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(LoginViewModel::class.java)) {
            val repository = ColisRepository(context)
            @Suppress("UNCHECKED_CAST")
            return LoginViewModel(repository) as T
        }
        throw IllegalArgumentException("Unknown ViewModel class")
    }
}
