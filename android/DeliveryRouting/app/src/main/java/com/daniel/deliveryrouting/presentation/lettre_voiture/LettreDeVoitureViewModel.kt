package com.daniel.deliveryrouting.presentation.lettre_voiture

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.daniel.deliveryrouting.data.api.models.LettreDeVoitureResponse
import com.daniel.deliveryrouting.data.repository.ColisRepository
import com.daniel.deliveryrouting.data.Result
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import android.util.Log

/**
 * 🆕 NUEVO: ViewModel para Lettre de Voiture
 * 
 * Maneja la obtención del lettre de voiture usando el token guardado
 */
class LettreDeVoitureViewModel(
    private val repository: ColisRepository
) : ViewModel() {
    
    companion object {
        private const val TAG = "LettreDeVoitureViewModel"
    }
    
    // Estado del ViewModel
    private val _uiState = MutableStateFlow<LettreDeVoitureUiState>(LettreDeVoitureUiState.Idle)
    val uiState: StateFlow<LettreDeVoitureUiState> = _uiState.asStateFlow()
    
    /**
     * 📋 OBTENER LETTRE DE VOITURE
     */
    fun getLettreDeVoiture(
        matricule: String,
        societe: String,
        date: String? = null
    ) {
        viewModelScope.launch {
            try {
                Log.d(TAG, "🚀 Iniciando obtención de Lettre de Voiture")
                _uiState.value = LettreDeVoitureUiState.Loading
                
                val result = repository.getLettreDeVoiture(
                    matricule = matricule,
                    societe = societe,
                    date = date
                )
                
                when (result) {
                    is Result.Success -> {
                        Log.d(TAG, "✅ Lettre de Voiture obtenido exitosamente")
                        _uiState.value = LettreDeVoitureUiState.Success(result.data)
                    }
                    is Result.Error -> {
                        Log.e(TAG, "❌ Error obteniendo Lettre de Voiture: ${result.message}")
                        _uiState.value = LettreDeVoitureUiState.Error(result.message)
                    }
                }
                
            } catch (e: Exception) {
                Log.e(TAG, "❌ Excepción obteniendo Lettre de Voiture", e)
                _uiState.value = LettreDeVoitureUiState.Error("Error inesperado: ${e.message}")
            }
        }
    }
    
    /**
     * 🔄 REINTENTAR OBTENCIÓN
     */
    fun retry() {
        // Obtener el último estado exitoso o usar valores por defecto
        val currentState = _uiState.value
        if (currentState is LettreDeVoitureUiState.Success) {
            getLettreDeVoiture(
                matricule = currentState.data.data?.matricule ?: "",
                societe = currentState.data.data?.societe ?: ""
            )
        }
    }
    
    /**
     * 🧹 LIMPIAR ESTADO
     */
    fun clearState() {
        _uiState.value = LettreDeVoitureUiState.Idle
    }
}

/**
 * 📱 ESTADOS DE LA UI PARA LETTRE DE VOITURE
 */
sealed class LettreDeVoitureUiState {
    object Idle : LettreDeVoitureUiState()
    object Loading : LettreDeVoitureUiState()
    data class Success(val data: LettreDeVoitureResponse) : LettreDeVoitureUiState()
    data class Error(val message: String) : LettreDeVoitureUiState()
}
