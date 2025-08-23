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
import com.daniel.deliveryrouting.data.Result
import com.daniel.deliveryrouting.data.api.models.ColisPriveLoginResponse

class LoginViewModel(
    private val repository: ColisRepository
) : ViewModel() {

    private val _loginState = MutableStateFlow<LoginState>(LoginState.Idle)
    val loginState: StateFlow<LoginState> = _loginState.asStateFlow()

    fun login(username: String, password: String, societe: String, apiType: String = "web") {
        Log.d("LoginViewModel", "=== INICIO LOGIN DIRECTO A COLIS PRIVE ===")
        Log.d("LoginViewModel", "Username: $username")
        Log.d("LoginViewModel", "Password length: ${password.length}")
        Log.d("LoginViewModel", "Societe: $societe")
        Log.d("LoginViewModel", "API Type: $apiType")

        _loginState.value = LoginState.Loading

        viewModelScope.launch {
            try {
                Log.d("LoginViewModel", "🚀 Iniciando login directo a Colis Prive...")
                
                // 🆕 NUEVO: Usar el endpoint directo a Colis Prive
                val result = repository.loginDirectToColisPrive(username, password, societe, apiType)
                
                Log.d("LoginViewModel", "✅ Login completado, procesando resultado...")
                
                when (result) {
                    is Result.Success -> {
                        Log.d("LoginViewModel", "🎉 Login exitoso: ${result.data.message}")
                        _loginState.value = LoginState.Success(result.data)
                    }
                    is Result.Error -> {
                        Log.e("LoginViewModel", "❌ Error en login: ${result.message}")
                        _loginState.value = LoginState.Error(result.message)
                    }
                }
                
            } catch (e: Exception) {
                Log.e("LoginViewModel", "💥 Excepción en login: ${e.message}", e)
                _loginState.value = LoginState.Error("Error inesperado: ${e.message}")
            }
        }
    }

    fun resetState() {
        _loginState.value = LoginState.Idle
    }

    sealed class LoginState {
        object Idle : LoginState()
        object Loading : LoginState()
        data class Success(val data: ColisPriveLoginResponse) : LoginState()
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
