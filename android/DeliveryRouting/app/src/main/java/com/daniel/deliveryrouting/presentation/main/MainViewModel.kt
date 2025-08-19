package com.daniel.deliveryrouting.presentation.main

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.daniel.deliveryrouting.data.api.models.Package
import com.daniel.deliveryrouting.data.repository.DeliveryRepository
import com.daniel.deliveryrouting.data.repository.LocationRepository
import com.daniel.deliveryrouting.utils.LocationUtils
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import java.text.SimpleDateFormat
import java.util.*
import android.util.Log

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add map view state management
// 2. Add location tracking
// 3. Add route optimization state

class MainViewModel(
    private val deliveryRepository: DeliveryRepository,
    private val locationRepository: LocationRepository
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(MainUiState())
    val uiState: StateFlow<MainUiState> = _uiState.asStateFlow()
    
    private val dateFormat = SimpleDateFormat("yyyy-MM-dd", Locale.getDefault())
    
    init {
        loadStoredTournee()
        // Si hay una tournée guardada, cargarla automáticamente
        loadStoredTourneeIfAvailable()
    }
    
    fun onTourneeCodeChange(tourneeCode: String) {
        _uiState.value = _uiState.value.copy(
            tourneeCode = tourneeCode,
            tourneeCodeError = null
        )
    }
    
    fun onDateChange(date: String) {
        _uiState.value = _uiState.value.copy(
            selectedDate = date,
            dateError = null
        )
    }
    
    fun onLoadTourneeClick() {
        val tourneeCode = _uiState.value.tourneeCode.trim()
        val date = _uiState.value.selectedDate
        
        // Validaciones
        if (tourneeCode.isEmpty()) {
            _uiState.value = _uiState.value.copy(
                tourneeCodeError = "El código de tournée es requerido"
            )
            return
        }
        
        if (date.isEmpty()) {
            _uiState.value = _uiState.value.copy(
                dateError = "La fecha es requerida"
            )
            return
        }
        
        loadTournee(tourneeCode, date)
    }
    
    fun onRefresh() {
        val tourneeCode = _uiState.value.tourneeCode
        val date = _uiState.value.selectedDate
        
        if (tourneeCode.isNotEmpty() && date.isNotEmpty()) {
            loadTournee(tourneeCode, date)
        }
    }
    
    fun onPackageClick(packageItem: Package) {
        _uiState.value = _uiState.value.copy(
            selectedPackage = packageItem
        )
    }
    
    fun onPackageDismiss() {
        _uiState.value = _uiState.value.copy(
            selectedPackage = null
        )
    }
    
    fun onToggleViewMode() {
        val currentMode = _uiState.value.viewMode
        val newMode = if (currentMode == ViewMode.LIST) ViewMode.MAP else ViewMode.LIST
        
        _uiState.value = _uiState.value.copy(
            viewMode = newMode
        )
        
        // TODO: Implementar cambio de vista cuando se agregue Mapbox
        if (newMode == ViewMode.MAP) {
            // Placeholder para implementación futura
        }
    }
    
    fun onSortByDistance() {
        val packages = _uiState.value.packages
        if (packages.isNotEmpty()) {
            // Por ahora usamos coordenadas fijas, después se obtendrán del GPS del dispositivo
            val userLat = 48.8566 // París como ejemplo
            val userLng = 2.3522
            
            val sortedPackages = LocationUtils.getPackagesSortedByDistance(
                userLat, userLng, packages
            )
            
            _uiState.value = _uiState.value.copy(
                packages = sortedPackages,
                sortOrder = SortOrder.DISTANCE
            )
        }
    }
    
    fun onSortByReference() {
        val packages = _uiState.value.packages
        if (packages.isNotEmpty()) {
            val sortedPackages = packages.sortedBy { it.reference }
            _uiState.value = _uiState.value.copy(
                packages = sortedPackages,
                sortOrder = SortOrder.REFERENCE
            )
        }
    }
    
    private fun loadStoredTournee() {
        val storedTournee = deliveryRepository.getStoredTourneeCode()
        if (storedTournee != null) {
            _uiState.value = _uiState.value.copy(
                tourneeCode = storedTournee
            )
        }
        
        // Establecer fecha actual por defecto
        val currentDate = dateFormat.format(Date())
        _uiState.value = _uiState.value.copy(
            selectedDate = currentDate
        )
    }
    
    private fun loadStoredTourneeIfAvailable() {
        val storedTournee = deliveryRepository.getStoredTourneeCode()
        val currentDate = dateFormat.format(Date())
        
        if (storedTournee != null) {
            Log.d("MainViewModel", "Tournée guardada encontrada: $storedTournee, cargando automáticamente...")
            loadTournee(storedTournee, currentDate)
        } else {
            Log.d("MainViewModel", "No hay tournée guardada, esperando login...")
        }
    }
    
    private fun loadTournee(tourneeCode: String, date: String) {
        _uiState.value = _uiState.value.copy(
            isLoading = true,
            error = null
        )
        
        viewModelScope.launch {
            try {
                val result = deliveryRepository.getTourneeStructured(tourneeCode, date)
                result.fold(
                    onSuccess = { response ->
                        val packages = response.data?.packages ?: emptyList()
                        _uiState.value = _uiState.value.copy(
                            isLoading = false,
                            packages = packages,
                            tourneeData = response.data,
                            error = null
                        )
                    },
                    onFailure = { exception ->
                        _uiState.value = _uiState.value.copy(
                            isLoading = false,
                            error = exception.message ?: "Error al cargar tournée"
                        )
                    }
                )
            } catch (e: Exception) {
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    error = e.message ?: "Error inesperado"
                )
            }
        }
    }
    
    fun clearError() {
        _uiState.value = _uiState.value.copy(error = null)
    }
    
    /**
     * Cargar automáticamente la tournée del día actual
     */
    fun loadCurrentDayTournee() {
        viewModelScope.launch {
            try {
                _uiState.value = _uiState.value.copy(isLoading = true, error = null)
                
                // Usar fecha actual
                val currentDate = dateFormat.format(Date()) // YYYY-MM-DD
                
                // Obtener matricule del usuario logueado (desde preferencias o estado global)
                val matricule = getLoggedUserMatricule() ?: "PCP0010699_A187518"
                
                Log.d("MainViewModel", "🔄 Cargando tournée automáticamente para fecha: $currentDate, matricule: $matricule")
                
                val result = deliveryRepository.getTourneeUpdated(matricule, currentDate)
                
                result.fold(
                    onSuccess = { response ->
                        if (response.success) {
                            val packages = response.data?.packages ?: emptyList()
                            _uiState.value = _uiState.value.copy(
                                packages = packages,
                                selectedDate = currentDate,
                                tourneeCode = extractTourneeFromMatricule(matricule),
                                isLoading = false,
                                error = null
                            )
                            Log.d("MainViewModel", "✅ Tournée cargada automáticamente: ${packages.size} paquetes")
                        } else {
                            _uiState.value = _uiState.value.copy(
                                isLoading = false,
                                error = response.message ?: "Error desconocido"
                            )
                        }
                    },
                    onFailure = { exception ->
                        Log.e("MainViewModel", "❌ Error cargando tournée automáticamente: ${exception.message}")
                        _uiState.value = _uiState.value.copy(
                            isLoading = false,
                            error = "Error de conexión: ${exception.message}"
                        )
                    }
                )
            } catch (e: Exception) {
                Log.e("MainViewModel", "❌ Excepción cargando tournée: ${e.message}")
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    error = "Error inesperado: ${e.message}"
                )
            }
        }
    }
    
    private fun getLoggedUserMatricule(): String? {
        // TODO: Implementar según tu sistema de preferencias
        // Por ahora retornar valor por defecto
        return "PCP0010699_A187518"
    }
    
    private fun extractTourneeFromMatricule(matricule: String): String {
        return matricule.split('_').lastOrNull() ?: ""
    }
}

data class MainUiState(
    val tourneeCode: String = "",
    val selectedDate: String = "",
    val tourneeCodeError: String? = null,
    val dateError: String? = null,
    val isLoading: Boolean = false,
    val packages: List<Package> = emptyList(),
    val tourneeData: com.daniel.deliveryrouting.data.api.models.TourneeData? = null,
    val selectedPackage: Package? = null,
    val viewMode: ViewMode = ViewMode.LIST,
    val sortOrder: SortOrder = SortOrder.REFERENCE,
    val error: String? = null
)

enum class ViewMode {
    LIST,
    MAP // Para implementar después con Mapbox
}

enum class SortOrder {
    REFERENCE,
    DISTANCE,
    STATUS
}
