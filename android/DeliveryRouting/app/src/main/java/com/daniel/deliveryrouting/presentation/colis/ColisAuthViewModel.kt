package com.daniel.deliveryrouting.presentation.colis

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.daniel.deliveryrouting.data.api.models.ColisLoginResponse
import com.daniel.deliveryrouting.data.repository.ColisRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import android.util.Log

class ColisAuthViewModel(private val repository: ColisRepository) : ViewModel() {
    
    private val _authState = MutableStateFlow<AuthState>(AuthState.Idle)
    val authState: StateFlow<AuthState> = _authState
    
    fun login(username: String, password: String, societe: String) {
        Log.d("ColisAuthViewModel", "Iniciando login con username: $username, societe: $societe")
        
        viewModelScope.launch {
            _authState.value = AuthState.Loading
            
            repository.authenticate(username, password, societe)
                .onSuccess { response ->
                    Log.d("ColisAuthViewModel", "✅ Login exitoso con Colis Privé!")
                    Log.d("ColisAuthViewModel", "Identity: ${response.identity}")
                    Log.d("ColisAuthViewModel", "Matricule: ${response.matricule}")
                    Log.d("ColisAuthViewModel", "SsoHopps Token disponible: ${response.tokens?.ssoHopps != null}")
                    _authState.value = AuthState.Success(response)
                }
                .onFailure { error ->
                    Log.e("ColisAuthViewModel", "❌ Login falló: ${error.message}")
                    _authState.value = AuthState.Error(error.message ?: "Unknown error")
                }
        }
    }
    
    fun resetState() {
        _authState.value = AuthState.Idle
    }
    
    sealed class AuthState {
        object Idle : AuthState()
        object Loading : AuthState()
        data class Success(val response: ColisLoginResponse) : AuthState()
        data class Error(val message: String) : AuthState()
    }
}
