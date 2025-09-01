package com.daniel.deliveryrouting.presentation.login

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import com.daniel.deliveryrouting.data.api.models.LoginResponse
import com.daniel.deliveryrouting.data.repository.SimpleRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import android.content.Context
import android.util.Log

class LoginViewModel(
    private val repository: SimpleRepository
) : ViewModel() {

    private val _loginState = MutableStateFlow<LoginState>(LoginState.Idle)
    val loginState: StateFlow<LoginState> = _loginState.asStateFlow()

    fun login(username: String, password: String, societe: String) {
        Log.d("LoginViewModel", "=== INICIO LOGIN ===")
        Log.d("LoginViewModel", "Username: $username")
        Log.d("LoginViewModel", "Password length: ${password.length}")
        Log.d("LoginViewModel", "Societe: $societe")

        _loginState.value = LoginState.Loading

        viewModelScope.launch {
            try {
                Log.d("LoginViewModel", "🚀 Iniciando login al backend...")
                
                // ✅ SIMPLIFICADO: Usar el endpoint del backend
                val result = repository.loginToBackend(username, password, societe)
                
                Log.d("LoginViewModel", "✅ Login completado, procesando resultado...")
                
                result.fold(
                    onSuccess = { loginResponse ->
                        Log.d("LoginViewModel", "🎉 Login exitoso: ${loginResponse.authentication?.message}")
                        _loginState.value = LoginState.Success(loginResponse)
                    },
                    onFailure = { error ->
                        Log.e("LoginViewModel", "❌ Error en login: ${error.message}")
                        _loginState.value = LoginState.Error(error.message ?: "Error desconocido")
                    }
                )
                
            } catch (e: Exception) {
                Log.e("LoginViewModel", "💥 Excepción en login: ${e.message}", e)
                _loginState.value = LoginState.Error("Error inesperado: ${e.message}")
            }
        }
    }

    sealed class LoginState {
        object Idle : LoginState()
        object Loading : LoginState()
        data class Success(val data: LoginResponse) : LoginState()
        data class Error(val message: String) : LoginState()
    }
}

/**
 * Factory para crear LoginViewModel con SimpleRepository
 */
class LoginViewModelFactory(private val context: Context) : ViewModelProvider.Factory {
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(LoginViewModel::class.java)) {
            val repository = SimpleRepository(context)
            @Suppress("UNCHECKED_CAST")
            return LoginViewModel(repository) as T
        }
        throw IllegalArgumentException("Unknown ViewModel class")
    }
}
