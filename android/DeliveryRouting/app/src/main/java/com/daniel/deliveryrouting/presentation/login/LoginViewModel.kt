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
                Log.d("LoginViewModel", "Llamando repository.authenticate con API: $apiType...")
                
                // 🚀 RUTEAR SEGÚN TIPO DE API
                val result = when (apiType) {
                    "web" -> {
                        Log.d("LoginViewModel", "🌐 Usando API Web (más simple)")
                        repository.authenticateWeb(username, password, societe)
                    }
                    "mobile" -> {
                        Log.d("LoginViewModel", "📱 Usando API Mobile (completa)")
                        repository.authenticate(username, password, societe)
                    }
                    else -> {
                        Log.d("LoginViewModel", "🌐 API no especificada, usando Web por defecto")
                        repository.authenticateWeb(username, password, societe)
                    }
                }
                
                result.fold(
                    onSuccess = { response ->
                        Log.d("LoginViewModel", "✅ LOGIN EXITOSO CON API: $apiType")
                        Log.d("LoginViewModel", "Flow Result: ${response.flowResult?.success}")
                        Log.d("LoginViewModel", "Session ID: ${response.flowResult?.sessionId?.take(50)}...")
                        _loginState.value = LoginState.Success(
                            matricule = response.flowResult?.sessionId?.split("_")?.lastOrNull() ?: username,
                            token = response.flowResult?.sessionId ?: ""
                        )
                    },
                    onFailure = { exception ->
                        Log.e("LoginViewModel", "❌ LOGIN FALLÓ CON API: $apiType", exception)
                        _loginState.value = LoginState.Error(
                            message = "Error con API $apiType: ${exception.message ?: "Error desconocido"}"
                        )
                    }
                )
            } catch (e: Exception) {
                Log.e("LoginViewModel", "❌ EXCEPCIÓN EN LOGIN CON API: $apiType", e)
                _loginState.value = LoginState.Error(
                    message = "Excepción con API $apiType: ${e.message ?: "Excepción inesperada"}"
                )
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
