package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName

/**
 * Modelos específicos para la gestión de Tournées
 * 
 * IMPORTANTE: Estos modelos solo trabajan con datos REALES del backend.
 * NO se crean datos falsos - solo se procesan los datos que llegan de Colis Privé.
 */

// 📋 MODELO PRINCIPAL DE TOURNÉE
data class Tournee(
    @SerializedName("id") val id: String,
    @SerializedName("date") val date: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("tourneeCode") val tourneeCode: String,
    @SerializedName("packages") val packages: List<MobilePackageAction>,
    @SerializedName("statistics") val statistics: TourneeStatistics,
    @SerializedName("zones") val zones: List<DeliveryZone>,
    @SerializedName("optimizedRoute") val optimizedRoute: OptimizedRoute? = null
)

// 📊 ESTADÍSTICAS CALCULADAS CON DATOS REALES
data class TourneeStatistics(
    @SerializedName("totalPackages") val totalPackages: Int,
    @SerializedName("completedPackages") val completedPackages: Int,
    @SerializedName("pendingPackages") val pendingPackages: Int,
    @SerializedName("totalWeight") val totalWeight: Double? = null,
    @SerializedName("estimatedDuration") val estimatedDuration: String? = null,
    @SerializedName("totalDistance") val totalDistance: Double? = null,
    @SerializedName("averageDeliveryTime") val averageDeliveryTime: String? = null
) {
    val completionPercentage: Float
        get() = if (totalPackages > 0) (completedPackages.toFloat() / totalPackages) * 100 else 0f
}

// 🗺️ ZONAS DE REPARTO (CALCULADAS AUTOMÁTICAMENTE)
data class DeliveryZone(
    @SerializedName("zoneId") val zoneId: String,
    @SerializedName("name") val name: String,
    @SerializedName("postalCodes") val postalCodes: List<String>,
    @SerializedName("packageCount") val packageCount: Int,
    @SerializedName("centerLatitude") val centerLatitude: Double? = null,
    @SerializedName("centerLongitude") val centerLongitude: Double? = null,
    @SerializedName("color") val color: String = "#2196F3"
)

// 🧭 RUTA OPTIMIZADA (TSP - Traveling Salesman Problem)
data class OptimizedRoute(
    @SerializedName("routeId") val routeId: String,
    @SerializedName("packageOrder") val packageOrder: List<String>, // IDs de paquetes en orden óptimo
    @SerializedName("totalDistance") val totalDistance: Double,
    @SerializedName("estimatedDuration") val estimatedDuration: String,
    @SerializedName("optimizationAlgorithm") val optimizationAlgorithm: String = "TSP",
    @SerializedName("waypoints") val waypoints: List<RouteWaypoint>,
    @SerializedName("createdAt") val createdAt: String
)

// 📍 PUNTOS DE LA RUTA
data class RouteWaypoint(
    @SerializedName("packageId") val packageId: String,
    @SerializedName("order") val order: Int,
    @SerializedName("latitude") val latitude: Double,
    @SerializedName("longitude") val longitude: Double,
    @SerializedName("address") val address: String,
    @SerializedName("estimatedArrival") val estimatedArrival: String? = null,
    @SerializedName("distanceFromPrevious") val distanceFromPrevious: Double? = null
)

// 🔍 FILTROS PARA LA LISTA DE PAQUETES
data class TourneeFilters(
    val zones: List<String> = emptyList(),
    val statuses: List<String> = emptyList(),
    val weightRange: WeightRange? = null,
    val searchQuery: String = "",
    val showOnlyWithCoordinates: Boolean = false
)

data class WeightRange(
    val minWeight: Double,
    val maxWeight: Double
)

// 📱 ESTADO DE LA UI DE TOURNÉE
data class TourneeUiState(
    val isLoading: Boolean = false,
    val tournee: Tournee? = null,
    val filteredPackages: List<MobilePackageAction> = emptyList(),
    val activeFilters: TourneeFilters = TourneeFilters(),
    val error: String? = null,
    val isRefreshing: Boolean = false,
    val showMap: Boolean = false,
    val selectedPackage: MobilePackageAction? = null
)

// 🎯 ACCIONES DE LA UI
sealed class TourneeAction {
    object LoadTournee : TourneeAction()
    object RefreshTournee : TourneeAction()
    data class ApplyFilters(val filters: TourneeFilters) : TourneeAction()
    data class SelectPackage(val packageAction: MobilePackageAction) : TourneeAction()
    object ClearSelectedPackage : TourneeAction()
    object ToggleMapView : TourneeAction()
    object OptimizeRoute : TourneeAction()
    data class UpdatePackageStatus(val packageId: String, val newStatus: Status) : TourneeAction()
}

// 🗂️ ORDENAMIENTO DE PAQUETES
enum class PackageSortOrder(val displayName: String) {
    ID("ID"),
    ADDRESS("Dirección"),
    POSTAL_CODE("Código Postal"),
    STATUS("Estado"),
    PRIORITY("Prioridad"),
    DISTANCE("Distancia"),
    WEIGHT("Peso")
}

// 📊 TIPOS DE VISTA
enum class TourneeViewType(val displayName: String) {
    LIST("Lista"),
    MAP("Mapa"),
    STATS("Estadísticas")
}

// 🎨 CONFIGURACIÓN DE COLORES PARA EL MAPA
object MapColors {
    const val PENDING_PACKAGE = "#FF9800"      // Naranja
    const val COMPLETED_PACKAGE = "#4CAF50"    // Verde
    const val IN_PROGRESS_PACKAGE = "#2196F3"  // Azul
    const val CANCELLED_PACKAGE = "#F44336"    // Rojo
    const val OPTIMIZED_ROUTE = "#9C27B0"      // Púrpura
    const val CURRENT_LOCATION = "#000000"     // Negro
}

// 🔧 UTILIDADES PARA TRABAJAR CON DATOS REALES
object TourneeUtils {
    
    /**
     * Agrupa los paquetes por zona postal
     */
    fun groupPackagesByZone(packages: List<MobilePackageAction>): Map<String, List<MobilePackageAction>> {
        return packages.groupBy { it.location.postalCode }
    }
    
    /**
     * Calcula las estadísticas de la tournée con datos reales
     */
    fun calculateStatistics(packages: List<MobilePackageAction>): TourneeStatistics {
        val completed = packages.count { it.status.isCompleted }
        val pending = packages.count { !it.status.isCompleted }
        
        return TourneeStatistics(
            totalPackages = packages.size,
            completedPackages = completed,
            pendingPackages = pending
        )
    }
    
    /**
     * Extrae las zonas de reparto de los paquetes reales
     */
    fun extractDeliveryZones(packages: List<MobilePackageAction>): List<DeliveryZone> {
        return packages
            .groupBy { it.location.postalCode }
            .map { (postalCode, packagesInZone) ->
                val centerLat = packagesInZone
                    .mapNotNull { it.location.latitude }
                    .takeIf { it.isNotEmpty() }
                    ?.average()
                
                val centerLng = packagesInZone
                    .mapNotNull { it.location.longitude }
                    .takeIf { it.isNotEmpty() }
                    ?.average()
                
                DeliveryZone(
                    zoneId = postalCode,
                    name = "Zona $postalCode",
                    postalCodes = listOf(postalCode),
                    packageCount = packagesInZone.size,
                    centerLatitude = centerLat,
                    centerLongitude = centerLng
                )
            }
    }
    
    /**
     * Filtra los paquetes que tienen coordenadas válidas para el mapa
     */
    fun getPackagesWithValidCoordinates(packages: List<MobilePackageAction>): List<MobilePackageAction> {
        return packages.filter { 
            it.location.latitude != null && 
            it.location.longitude != null &&
            it.location.latitude in 41.0..51.5 && // Francia aproximadamente
            it.location.longitude in -5.0..10.0
        }
    }
}