package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName

// 🗺️ MODELOS PARA OPTIMIZACIÓN DE RUTAS

// Tournée Optimizada Request
data class TourneeOptimizedRequest(
    @SerializedName("DateDebut") val dateDebut: String,
    @SerializedName("Matricule") val matricule: String,
    @SerializedName("Password") val password: String,
    @SerializedName("Societe") val societe: String,
    @SerializedName("optimizeRoute") val optimizeRoute: Boolean = true,  // Solicitar optimización
    @SerializedName("includeTrafficData") val includeTrafficData: Boolean = true
)

// Tournée Optimizada Response
data class TourneeOptimizedResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("data") val data: OptimizedTourneeData?,
    @SerializedName("message") val message: String?,
    @SerializedName("optimization") val optimization: OptimizationMetadata?
)

// Datos de Tournée Optimizada
data class OptimizedTourneeData(
    @SerializedName("tourneeCode") val tourneeCode: String,
    @SerializedName("date") val date: String,
    @SerializedName("packages") val packages: List<OptimizedPackage>,
    @SerializedName("optimizedRoute") val optimizedRoute: OptimizedRoute,
    @SerializedName("totalDistance") val totalDistance: Double,
    @SerializedName("estimatedDuration") val estimatedDuration: Int  // minutos
)

// Paquete Optimizado
data class OptimizedPackage(
    @SerializedName("id") val id: String,
    @SerializedName("reference") val reference: String,
    @SerializedName("location") val location: OptimizedLocation,
    @SerializedName("action") val action: PackageAction,
    @SerializedName("status") val status: PackageStatus,
    @SerializedName("routeOrder") val routeOrder: Int,  // Orden optimizado
    @SerializedName("estimatedArrival") val estimatedArrival: String?,
    @SerializedName("priority") val priority: String = "NORMAL"
)

// Ubicación Optimizada
data class OptimizedLocation(
    @SerializedName("hasCoordinates") val hasCoordinates: Boolean,
    @SerializedName("latitude") val latitude: Double?,
    @SerializedName("longitude") val longitude: Double?,
    @SerializedName("formattedAddress") val formattedAddress: String?,
    @SerializedName("city") val city: String?,
    @SerializedName("postalCode") val postalCode: String?,
    @SerializedName("distanceFromPrevious") val distanceFromPrevious: Double?,  // metros
    @SerializedName("drivingTimeFromPrevious") val drivingTimeFromPrevious: Int?  // minutos
)

// Ruta Optimizada
data class OptimizedRoute(
    @SerializedName("waypoints") val waypoints: List<RouteWaypoint>,
    @SerializedName("totalDistance") val totalDistance: Double,  // metros
    @SerializedName("totalDuration") val totalDuration: Int,     // minutos
    @SerializedName("optimizationAlgorithm") val optimizationAlgorithm: String,
    @SerializedName("trafficConditions") val trafficConditions: String?
)

// Punto de la Ruta
data class RouteWaypoint(
    @SerializedName("latitude") val latitude: Double,
    @SerializedName("longitude") val longitude: Double,
    @SerializedName("packageId") val packageId: String?,
    @SerializedName("address") val address: String?,
    @SerializedName("order") val order: Int
)

// Metadata de Optimización
data class OptimizationMetadata(
    @SerializedName("algorithm") val algorithm: String,
    @SerializedName("optimizationTimeMs") val optimizationTimeMs: Long,
    @SerializedName("improvementPercentage") val improvementPercentage: Double,
    @SerializedName("trafficDataUsed") val trafficDataUsed: Boolean,
    @SerializedName("totalPackages") val totalPackages: Int,
    @SerializedName("optimizedPackages") val optimizedPackages: Int
)

// 🚀 OPTIMIZACIÓN DE RUTAS REQUEST
data class RouteOptimizationRequest(
    @SerializedName("packages") val packages: List<PackageForOptimization>,
    @SerializedName("startLocation") val startLocation: LocationCoordinate,
    @SerializedName("endLocation") val endLocation: LocationCoordinate?,
    @SerializedName("optimizationGoal") val optimizationGoal: String = "MINIMIZE_TIME", // MINIMIZE_TIME, MINIMIZE_DISTANCE, BALANCED
    @SerializedName("includeTraffic") val includeTraffic: Boolean = true,
    @SerializedName("maxDuration") val maxDuration: Int? = null  // minutos máximos
)

// Paquete para Optimización
data class PackageForOptimization(
    @SerializedName("id") val id: String,
    @SerializedName("location") val location: LocationCoordinate,
    @SerializedName("priority") val priority: String = "NORMAL",  // HIGH, NORMAL, LOW
    @SerializedName("serviceTime") val serviceTime: Int = 5,     // minutos en ubicación
    @SerializedName("timeWindow") val timeWindow: TimeWindow?
)

// Ventana de Tiempo
data class TimeWindow(
    @SerializedName("earliest") val earliest: String?,  // HH:mm
    @SerializedName("latest") val latest: String?       // HH:mm
)

// Coordenada de Ubicación
data class LocationCoordinate(
    @SerializedName("latitude") val latitude: Double,
    @SerializedName("longitude") val longitude: Double,
    @SerializedName("address") val address: String?
)

// 🚀 OPTIMIZACIÓN DE RUTAS RESPONSE
data class RouteOptimizationResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("optimizedRoute") val optimizedRoute: OptimizedRoute,
    @SerializedName("optimization") val optimization: OptimizationMetadata,
    @SerializedName("message") val message: String?
)

// 📈 MÉTRICAS DE ENTREGA
data class DeliveryMetricsResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("metrics") val metrics: DeliveryMetrics?,
    @SerializedName("message") val message: String?
)

data class DeliveryMetrics(
    @SerializedName("totalDeliveries") val totalDeliveries: Int,
    @SerializedName("completedDeliveries") val completedDeliveries: Int,
    @SerializedName("pendingDeliveries") val pendingDeliveries: Int,
    @SerializedName("averageDeliveryTime") val averageDeliveryTime: Double,  // minutos
    @SerializedName("totalDistance") val totalDistance: Double,  // kilómetros
    @SerializedName("fuelEfficiency") val fuelEfficiency: Double,  // L/100km
    @SerializedName("customerSatisfaction") val customerSatisfaction: Double,  // 0-5
    @SerializedName("routeOptimizationSavings") val routeOptimizationSavings: RouteSavings
)

data class RouteSavings(
    @SerializedName("timeSavedMinutes") val timeSavedMinutes: Int,
    @SerializedName("distanceSavedKm") val distanceSavedKm: Double,
    @SerializedName("fuelSavedLiters") val fuelSavedLiters: Double,
    @SerializedName("co2ReductionKg") val co2ReductionKg: Double
)

