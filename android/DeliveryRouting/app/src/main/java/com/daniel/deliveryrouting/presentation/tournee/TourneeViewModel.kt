package com.daniel.deliveryrouting.presentation.tournee

import android.content.Context
import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.daniel.deliveryrouting.data.api.models.*
import com.daniel.deliveryrouting.data.repository.TourneeRepository
import com.daniel.deliveryrouting.data.preferences.PreferencesManager
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * 🎯 VIEWMODEL PARA GESTIÓN DE TOURNÉES
 * 
 * Funcionalidades:
 * - ✅ Carga tournées REALES del backend
 * - ✅ Gestión de filtros y búsqueda
 * - ✅ Optimización de rutas con algoritmo TSP
 * - ✅ Estados de UI reactivos
 * - ✅ Manejo de errores robusto
 * - ✅ Pull-to-refresh y cache
 * 
 * IMPORTANTE: Solo trabaja con datos REALES del backend
 */
class TourneeViewModel(private val context: Context) : ViewModel() {
    
    private val tourneeRepository = TourneeRepository(context)
    private val preferencesManager = PreferencesManager(context)
    
    // Estado de la UI
    private val _uiState = MutableStateFlow(TourneeUiState())
    val uiState: StateFlow<TourneeUiState> = _uiState.asStateFlow()
    
    // Lista completa de paquetes (sin filtrar)
    private var allPackages: List<MobilePackageAction> = emptyList()
    
    companion object {
        private const val TAG = "TourneeViewModel"
    }
    
    init {
        Log.d(TAG, "🚀 TourneeViewModel inicializado")
        
        // Cargar tournée automáticamente si hay credenciales guardadas
        viewModelScope.launch {
            autoLoadTourneeIfPossible()
        }
    }
    
    /**
     * 🎬 MANEJAR ACCIONES DE LA UI
     */
    fun handleAction(action: TourneeAction) {
        Log.d(TAG, "🎬 Acción recibida: ${action::class.simpleName}")
        
        when (action) {
            is TourneeAction.LoadTournee -> loadTournee()
            is TourneeAction.RefreshTournee -> refreshTournee()
            is TourneeAction.ApplyFilters -> applyFilters(action.filters)
            is TourneeAction.SelectPackage -> selectPackage(action.packageAction)
            is TourneeAction.ClearSelectedPackage -> clearSelectedPackage()
            is TourneeAction.ToggleMapView -> toggleMapView()
            is TourneeAction.OptimizeRoute -> optimizeRoute()
            is TourneeAction.UpdatePackageStatus -> updatePackageStatus(action.packageId, action.newStatus)
        }
    }
    
    /**
     * 📦 CARGAR TOURNÉE DESDE EL BACKEND
     */
    private fun loadTournee(forceRefresh: Boolean = false) {
        viewModelScope.launch {
            try {
                Log.d(TAG, "📦 === CARGANDO TOURNÉE ===")
                _uiState.value = _uiState.value.copy(
                    isLoading = true,
                    error = null
                )
                
                // Obtener credenciales guardadas
                val credentials = getStoredCredentials()
                if (credentials == null) {
                    Log.w(TAG, "⚠️ No hay credenciales guardadas")
                    _uiState.value = _uiState.value.copy(
                        isLoading = false,
                        error = "No hay credenciales guardadas. Por favor, inicia sesión primero."
                    )
                    return@launch
                }
                
                Log.d(TAG, "🔑 Usando credenciales: ${credentials.username} - ${credentials.societe}")
                
                // Cargar tournée del backend
                val result = tourneeRepository.getCurrentTournee(
                    username = credentials.username,
                    password = credentials.password,
                    societe = credentials.societe,
                    forceRefresh = forceRefresh
                )
                
                when {
                    result.isSuccess -> {
                        val tournee = result.getOrNull()!!
                        allPackages = tournee.packages
                        
                        Log.d(TAG, "✅ Tournée cargada: ${tournee.packages.size} paquetes")
                        
                        // Aplicar filtros actuales a los nuevos datos
                        val filteredPackages = filterPackages(allPackages, _uiState.value.activeFilters)
                        
                        _uiState.value = _uiState.value.copy(
                            isLoading = false,
                            isRefreshing = false,
                            tournee = tournee,
                            filteredPackages = filteredPackages,
                            error = null
                        )
                        
                        Log.d(TAG, "✅ UI actualizada con ${filteredPackages.size} paquetes filtrados")
                    }
                    else -> {
                        val error = result.exceptionOrNull()
                        Log.e(TAG, "❌ Error cargando tournée: ${error?.message}")
                        
                        _uiState.value = _uiState.value.copy(
                            isLoading = false,
                            isRefreshing = false,
                            error = "Error cargando tournée: ${error?.message ?: "Error desconocido"}"
                        )
                    }
                }
                
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error en loadTournee: ${e.message}", e)
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    isRefreshing = false,
                    error = "Error inesperado: ${e.message}"
                )
            }
        }
    }
    
    /**
     * 🔄 REFRESH TOURNÉE (PULL-TO-REFRESH)
     */
    private fun refreshTournee() {
        Log.d(TAG, "🔄 Actualizando tournée...")
        _uiState.value = _uiState.value.copy(isRefreshing = true)
        loadTournee(forceRefresh = true)
    }
    
    /**
     * 🔍 APLICAR FILTROS
     */
    private fun applyFilters(filters: TourneeFilters) {
        Log.d(TAG, "🔍 Aplicando filtros: $filters")
        
        val filteredPackages = filterPackages(allPackages, filters)
        
        _uiState.value = _uiState.value.copy(
            activeFilters = filters,
            filteredPackages = filteredPackages
        )
        
        Log.d(TAG, "✅ Filtros aplicados: ${filteredPackages.size} paquetes resultantes")
    }
    
    /**
     * 📍 SELECCIONAR PAQUETE
     */
    private fun selectPackage(packageAction: MobilePackageAction) {
        Log.d(TAG, "📍 Paquete seleccionado: ${packageAction.package_info.id}")
        
        _uiState.value = _uiState.value.copy(
            selectedPackage = packageAction
        )
    }
    
    /**
     * ❌ DESELECCIONAR PAQUETE
     */
    private fun clearSelectedPackage() {
        Log.d(TAG, "❌ Deseleccionando paquete")
        
        _uiState.value = _uiState.value.copy(
            selectedPackage = null
        )
    }
    
    /**
     * 🗺️ TOGGLE VISTA DE MAPA
     */
    private fun toggleMapView() {
        val newShowMap = !_uiState.value.showMap
        Log.d(TAG, "🗺️ Toggle mapa: $newShowMap")
        
        _uiState.value = _uiState.value.copy(
            showMap = newShowMap
        )
    }
    
    /**
     * 🧭 OPTIMIZAR RUTA
     */
    private fun optimizeRoute() {
        viewModelScope.launch {
            try {
                Log.d(TAG, "🧭 === OPTIMIZANDO RUTA ===")
                
                val currentPackages = _uiState.value.filteredPackages
                if (currentPackages.isEmpty()) {
                    Log.w(TAG, "⚠️ No hay paquetes para optimizar")
                    return@launch
                }
                
                // Mostrar loading
                _uiState.value = _uiState.value.copy(isLoading = true)
                
                val result = tourneeRepository.optimizeRoute(currentPackages)
                
                when {
                    result.isSuccess -> {
                        val optimizedRoute = result.getOrNull()!!
                        Log.d(TAG, "✅ Ruta optimizada: ${optimizedRoute.waypoints.size} puntos")
                        
                        // Actualizar tournée con ruta optimizada
                        val updatedTournee = _uiState.value.tournee?.copy(
                            optimizedRoute = optimizedRoute
                        )
                        
                        _uiState.value = _uiState.value.copy(
                            isLoading = false,
                            tournee = updatedTournee,
                            showMap = true // Mostrar mapa automáticamente
                        )
                        
                        Log.d(TAG, "✅ Ruta optimizada aplicada y mapa activado")
                    }
                    else -> {
                        val error = result.exceptionOrNull()
                        Log.e(TAG, "❌ Error optimizando ruta: ${error?.message}")
                        
                        _uiState.value = _uiState.value.copy(
                            isLoading = false,
                            error = "Error optimizando ruta: ${error?.message}"
                        )
                    }
                }
                
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error en optimizeRoute: ${e.message}", e)
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    error = "Error inesperado optimizando ruta: ${e.message}"
                )
            }
        }
    }
    
    /**
     * 🔄 ACTUALIZAR ESTADO DE PAQUETE
     */
    private fun updatePackageStatus(packageId: String, newStatus: Status) {
        viewModelScope.launch {
            try {
                Log.d(TAG, "🔄 Actualizando estado de paquete $packageId")
                
                val result = tourneeRepository.updatePackageStatus(packageId, newStatus)
                
                when {
                    result.isSuccess -> {
                        Log.d(TAG, "✅ Estado de paquete actualizado exitosamente")
                        // Recargar datos para reflejar los cambios
                        loadTournee(forceRefresh = true)
                    }
                    else -> {
                        val error = result.exceptionOrNull()
                        Log.e(TAG, "❌ Error actualizando estado: ${error?.message}")
                        
                        _uiState.value = _uiState.value.copy(
                            error = "Error actualizando estado: ${error?.message}"
                        )
                    }
                }
                
            } catch (e: Exception) {
                Log.e(TAG, "❌ Error en updatePackageStatus: ${e.message}", e)
                _uiState.value = _uiState.value.copy(
                    error = "Error inesperado actualizando estado: ${e.message}"
                )
            }
        }
    }
    
    /**
     * 🗑️ LIMPIAR ERROR
     */
    fun clearError() {
        _uiState.value = _uiState.value.copy(error = null)
    }
    
    /**
     * 📊 OBTENER ESTADÍSTICAS ACTUALES
     */
    fun getCurrentStatistics(): TourneeStatistics? {
        return _uiState.value.tournee?.statistics
    }
    
    /**
     * 🗺️ OBTENER PAQUETES CON COORDENADAS VÁLIDAS
     */
    fun getPackagesWithCoordinates(): List<MobilePackageAction> {
        return _uiState.value.filteredPackages.filter { pkg ->
            pkg.location.latitude != null && 
            pkg.location.longitude != null &&
            pkg.location.latitude in 41.0..51.5 && // Francia aproximadamente
            pkg.location.longitude in -5.0..10.0
        }
    }
    
    /**
     * 🎯 OBTENER FILTROS DISPONIBLES
     */
    fun getAvailableFilters(): FilterOptions {
        val zones = allPackages.map { it.location.postalCode }.distinct().sorted()
        val statuses = allPackages.map { it.status.code }.distinct().sorted()
        
        return FilterOptions(
            availableZones = zones,
            availableStatuses = statuses
        )
    }
    
    // 🛠️ FUNCIONES PRIVADAS
    
    /**
     * 🔑 OBTENER CREDENCIALES ALMACENADAS
     */
    private fun getStoredCredentials(): StoredCredentials? {
        return try {
                    val username = preferencesManager.getUsername()
        val password = preferencesManager.getPassword()
        val societe = preferencesManager.getSociete()
            
                    if (username?.isNotBlank() == true && password?.isNotBlank() == true && societe?.isNotBlank() == true) {
            StoredCredentials(username, password, societe)
        } else {
            null
        }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo credenciales: ${e.message}", e)
            null
        }
    }
    
    /**
     * 🚀 AUTO-CARGA SI ES POSIBLE
     */
    private suspend fun autoLoadTourneeIfPossible() {
        try {
            val credentials = getStoredCredentials()
            if (credentials != null) {
                Log.d(TAG, "🚀 Auto-cargando tournée con credenciales guardadas")
                loadTournee()
            } else {
                Log.d(TAG, "ℹ️ No hay credenciales guardadas, esperando login manual")
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en auto-carga: ${e.message}", e)
        }
    }
    
    /**
     * 🔍 FILTRAR PAQUETES LOCALMENTE
     */
    private fun filterPackages(
        packages: List<MobilePackageAction>,
        filters: TourneeFilters
    ): List<MobilePackageAction> {
        var result = packages
        
        // Filtrar por zonas
        if (filters.zones.isNotEmpty()) {
            result = result.filter { pkg ->
                filters.zones.contains(pkg.location.postalCode)
            }
        }
        
        // Filtrar por estados
        if (filters.statuses.isNotEmpty()) {
            result = result.filter { pkg ->
                filters.statuses.contains(pkg.status.code)
            }
        }
        
        // Filtrar por búsqueda
        if (filters.searchQuery.isNotBlank()) {
            result = result.filter { pkg ->
                pkg.package_info.reference.contains(filters.searchQuery, ignoreCase = true) ||
                pkg.package_info.barcode.contains(filters.searchQuery, ignoreCase = true) ||
                pkg.location.formattedAddress.contains(filters.searchQuery, ignoreCase = true) ||
                pkg.customer.name.contains(filters.searchQuery, ignoreCase = true)
            }
        }
        
        // Filtrar solo con coordenadas
        if (filters.showOnlyWithCoordinates) {
            result = result.filter { pkg ->
                pkg.location.latitude != null && pkg.location.longitude != null
            }
        }
        
        return result
    }
    
    override fun onCleared() {
        super.onCleared()
        Log.d(TAG, "🧹 TourneeViewModel limpiado")
        tourneeRepository.clearCache()
    }
}

/**
 * 🔑 CREDENCIALES ALMACENADAS
 */
private data class StoredCredentials(
    val username: String,
    val password: String,
    val societe: String
)

/**
 * 🎛️ OPCIONES DE FILTRADO DISPONIBLES
 */
data class FilterOptions(
    val availableZones: List<String>,
    val availableStatuses: List<String>
)