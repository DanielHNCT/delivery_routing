package com.daniel.deliveryrouting.presentation.main

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.daniel.deliveryrouting.data.api.models.*
import com.daniel.deliveryrouting.data.repository.ColisRepository
import com.daniel.deliveryrouting.data.token.ColisTokenManager
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import android.util.Log
import java.text.SimpleDateFormat
import java.util.*

/**
 * 🎯 MAIN VIEWMODEL ACTUALIZADO PARA COLIS PRIVÉ
 * 
 * Características:
 * - ✅ Usa el nuevo ColisRepository
 * - ✅ Integra DeviceInfoManager y TokenManager
 * - ✅ Auto-retry automático para tournées
 * - ✅ Manejo robusto de errores
 */
class MainViewModel(
    private val colisRepository: ColisRepository,
    private val tokenManager: ColisTokenManager
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(MainUiState())
    val uiState = _uiState.asStateFlow()
    
    private val dateFormat = SimpleDateFormat("yyyy-MM-dd", Locale.getDefault())
    
    companion object {
        private const val TAG = "MainViewModel"
    }
    
    /**
     * 🔄 CARGAR TOURNÉE AUTOMÁTICAMENTE
     */
    fun loadCurrentDayTournee() {
        viewModelScope.launch {
            try {
                _uiState.value = _uiState.value.copy(isLoading = true, error = null)
                
                // Usar fecha actual
                val currentDate = dateFormat.format(Date()) // YYYY-MM-DD
                
                // Obtener datos del usuario logueado
                val userData = tokenManager.getSavedUserData()
                if (userData == null) {
                    Log.w(TAG, "⚠️ No hay datos de usuario, usando valores por defecto")
                    _uiState.value = _uiState.value.copy(
                        isLoading = false,
                        error = "Usuario no autenticado"
                    )
                    return@launch
                }
                
                Log.d(TAG, "🔄 Cargando tournée automáticamente para fecha: $currentDate, matricule: ${userData.matricule}")
                
                // Usar el nuevo ColisRepository con auto-retry
                val result = colisRepository.getTourneeWithAutoRetry(
                    username = userData.username,
                    password = "INTI7518", // TODO: Obtener de forma segura
                    societe = userData.societe,
                    date = currentDate,
                    matricule = userData.matricule
                )
                
                result.fold(
                    onSuccess = { response ->
                        if (response.success) {
                            val packages = response.data?.map { mobilePackage ->
                                // Convertir MobilePackageAction a Package
                                Package(
                                    id = mobilePackage.package_info.id,
                                    locationId = mobilePackage.package_info.id,
                                    reference = mobilePackage.package_info.reference,
                                    barcode = mobilePackage.package_info.barcode,
                                    tourneeCode = mobilePackage.package_info.tourneeCode,
                                    action = PackageAction(
                                        code = mobilePackage.status.code,
                                        label = mobilePackage.status.label
                                    ),
                                    location = PackageLocation(
                                        hasCoordinates = mobilePackage.location.latitude != null && mobilePackage.location.longitude != null,
                                        latitude = mobilePackage.location.latitude,
                                        longitude = mobilePackage.location.longitude,
                                        formattedAddress = mobilePackage.location.formattedAddress,
                                        city = mobilePackage.location.city,
                                        postalCode = mobilePackage.location.postalCode
                                    ),
                                    timing = PackageTiming(
                                        estimatedTime = mobilePackage.timing.estimatedTime,
                                        priority = mobilePackage.timing.priority
                                    ),
                                    status = PackageStatus(
                                        code = mobilePackage.status.code,
                                        label = mobilePackage.status.label,
                                        isCompleted = mobilePackage.status.isCompleted
                                    ),
                                    sender = PackageSender(
                                        name = mobilePackage.customer.name,
                                        phone = mobilePackage.customer.phone,
                                        email = mobilePackage.customer.email
                                    )
                                )
                            } ?: emptyList()
                            
                            _uiState.value = _uiState.value.copy(
                                packages = packages,
                                selectedDate = currentDate,
                                tourneeCode = extractTourneeFromMatricule(userData.matricule),
                                isLoading = false,
                                error = null
                            )
                            Log.d(TAG, "✅ Tournée cargada automáticamente: ${packages.size} paquetes")
                        } else {
                            _uiState.value = _uiState.value.copy(
                                isLoading = false,
                                error = response.message ?: "Error desconocido"
                            )
                        }
                    },
                    onFailure = { exception ->
                        Log.e(TAG, "❌ Error cargando tournée automáticamente: ${exception.message}")
                        _uiState.value = _uiState.value.copy(
                            isLoading = false,
                            error = "Error de conexión: ${exception.message}"
                        )
                    }
                )
            } catch (e: Exception) {
                Log.e(TAG, "❌ Excepción cargando tournée: ${e.message}")
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    error = "Error inesperado: ${e.message}"
                )
            }
        }
    }
    
    /**
     * 🔍 VERIFICAR ESTADO DE AUTENTICACIÓN
     */
    fun checkAuthenticationStatus() {
        viewModelScope.launch {
            try {
                val isAuthenticated = tokenManager.isUserLoggedIn()
                val userData = tokenManager.getSavedUserData()
                
                if (isAuthenticated && userData != null) {
                    Log.d(TAG, "✅ Usuario autenticado: ${userData.matricule}")
                    // Usuario está logueado, cargar tournée automáticamente
                    loadCurrentDayTournee()
                } else {
                    Log.d(TAG, "⚠️ Usuario no autenticado")
                    _uiState.value = _uiState.value.copy(
                        error = "Usuario no autenticado"
                    )
                }
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error verificando estado de autenticación: ${e.message}")
            }
        }
    }
    
    /**
     * 🗑️ LOGOUT
     */
    fun logout() {
        viewModelScope.launch {
            try {
                colisRepository.logout()
                _uiState.value = MainUiState() // Reset completo
                Log.d(TAG, "✅ Logout exitoso")
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error en logout: ${e.message}")
            }
        }
    }
    
    /**
     * 📊 OBTENER ESTADO DEL REPOSITORY
     */
    fun getRepositoryState() {
        viewModelScope.launch {
            try {
                val state = colisRepository.getCurrentState()
                Log.d(TAG, "📊 Estado del repository: ${state.isAuthenticated}")
                Log.d(TAG, "Usuario: ${state.currentUser}")
                Log.d(TAG, "Token expira en: ${state.tokenExpiration.getFormattedExpiry()}")
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error obteniendo estado: ${e.message}")
            }
        }
    }
    
    fun clearError() {
        _uiState.value = _uiState.value.copy(error = null)
    }
    
    private fun extractTourneeFromMatricule(matricule: String): String {
        return matricule.split('_').lastOrNull() ?: ""
    }
}

data class MainUiState(
    val tourneeCode: String = "",
    val selectedDate: String = "",
    val isLoading: Boolean = false,
    val packages: List<Package> = emptyList(),
    val error: String? = null
)
