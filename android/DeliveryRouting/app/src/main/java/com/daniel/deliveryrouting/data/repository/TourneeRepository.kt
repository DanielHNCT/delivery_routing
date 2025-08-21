package com.daniel.deliveryrouting.data.repository

import android.content.Context
import android.util.Log
import com.daniel.deliveryrouting.data.api.models.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.text.SimpleDateFormat
import java.util.*

/**
 * 📦 REPOSITORY ESPECÍFICO PARA GESTIÓN DE TOURNÉES
 * 
 * Este repository se enfoca exclusivamente en:
 * - Obtener datos REALES de tournées del backend
 * - Procesar y filtrar paquetes recibidos
 * - Gestionar caché local de tournées
 * - Optimización de rutas con datos REALES
 * 
 * IMPORTANTE: NO crea datos falsos, solo trabaja con datos del backend
 */
class TourneeRepository(private val context: Context) {
    
    private val colisRepository = ColisRepository(context)
    
    // Cache local de la tournée actual
    private var cachedTournee: Tournee? = null
    private var lastCacheTime: Long = 0
    private val cacheValidityMs = 5 * 60 * 1000L // 5 minutos
    
    companion object {
        private const val TAG = "TourneeRepository"
        
        /**
         * 📅 OBTENER FECHA ACTUAL EN FORMATO YYYY-MM-DD
         */
        private fun getCurrentDate(): String {
            val formatter = SimpleDateFormat("yyyy-MM-dd", Locale.getDefault())
            return formatter.format(Date())
        }
    }
    
    /**
     * 📦 OBTENER TOURNÉE CON DATOS REALES DEL BACKEND
     */
    suspend fun getCurrentTournee(
        username: String,
        password: String,
        societe: String,
        forceRefresh: Boolean = false
    ): Result<Tournee> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "📦 === OBTENIENDO TOURNÉE REAL ===")
            Log.d(TAG, "Username: $username")
            Log.d(TAG, "Societe: $societe")
            Log.d(TAG, "Force refresh: $forceRefresh")
            
            // Verificar cache si no es force refresh
            if (!forceRefresh && isCacheValid()) {
                cachedTournee?.let { tournee ->
                    Log.d(TAG, "✅ Usando tournée desde cache (${tournee.packages.size} paquetes)")
                    return@withContext Result.success(tournee)
                }
            }
            
            // Obtener datos reales del backend
            val currentDate = getCurrentDate()
            val matricule = username // El username ya incluye la matrícula
            
            val tourneeResult = colisRepository.getTourneeWithAutoRetry(
                username = username,
                password = password,
                societe = societe,
                date = currentDate,
                matricule = matricule
            )
            
            when {
                tourneeResult.isSuccess -> {
                    val tourneeResponse = tourneeResult.getOrNull()!!
                    Log.d(TAG, "✅ Tournée recibida del backend: ${tourneeResponse.total_packages} paquetes")
                    
                    // Convertir response a modelo de Tournée
                    val tournee = convertResponseToTournee(
                        response = tourneeResponse,
                        username = username,
                        date = currentDate
                    )
                    
                    // Guardar en cache
                    cachedTournee = tournee
                    lastCacheTime = System.currentTimeMillis()
                    
                    Log.d(TAG, "✅ Tournée procesada y cacheada exitosamente")
                    Result.success(tournee)
                }
                else -> {
                    val error = tourneeResult.exceptionOrNull()
                    Log.e(TAG, "❌ Error obteniendo tournée: ${error?.message}")
                    Result.failure(error ?: Exception("Error desconocido obteniendo tournée"))
                }
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error en getCurrentTournee: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🔍 FILTRAR PAQUETES SEGÚN CRITERIOS
     */
    suspend fun getFilteredPackages(
        filters: TourneeFilters
    ): Result<List<MobilePackageAction>> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🔍 === FILTRANDO PAQUETES ===")
            Log.d(TAG, "Filtros aplicados: $filters")
            
            val tournee = cachedTournee ?: run {
                Log.w(TAG, "⚠️ No hay tournée en cache para filtrar")
                return@withContext Result.failure(Exception("No hay tournée cargada"))
            }
            
            var filteredPackages = tournee.packages
            
            // Filtrar por zonas (códigos postales)
            if (filters.zones.isNotEmpty()) {
                filteredPackages = filteredPackages.filter { pkg ->
                    filters.zones.contains(pkg.location.postalCode)
                }
                Log.d(TAG, "📍 Filtrado por zonas: ${filteredPackages.size} paquetes")
            }
            
            // Filtrar por estados
            if (filters.statuses.isNotEmpty()) {
                filteredPackages = filteredPackages.filter { pkg ->
                    filters.statuses.contains(pkg.status.code)
                }
                Log.d(TAG, "📊 Filtrado por estados: ${filteredPackages.size} paquetes")
            }
            
            // Filtrar por búsqueda de texto
            if (filters.searchQuery.isNotBlank()) {
                filteredPackages = filteredPackages.filter { pkg ->
                    pkg.package_info.reference.contains(filters.searchQuery, ignoreCase = true) ||
                    pkg.package_info.barcode.contains(filters.searchQuery, ignoreCase = true) ||
                    pkg.location.formattedAddress.contains(filters.searchQuery, ignoreCase = true) ||
                    pkg.customer.name.contains(filters.searchQuery, ignoreCase = true)
                }
                Log.d(TAG, "🔎 Filtrado por búsqueda: ${filteredPackages.size} paquetes")
            }
            
            // Filtrar solo paquetes con coordenadas
            if (filters.showOnlyWithCoordinates) {
                filteredPackages = filteredPackages.filter { pkg ->
                    pkg.location.latitude != null && pkg.location.longitude != null
                }
                Log.d(TAG, "🗺️ Filtrado por coordenadas: ${filteredPackages.size} paquetes")
            }
            
            Log.d(TAG, "✅ Filtrado completado: ${filteredPackages.size} paquetes resultantes")
            Result.success(filteredPackages)
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error filtrando paquetes: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 📊 OBTENER ESTADÍSTICAS DE LA TOURNÉE
     */
    suspend fun getTourneeStatistics(): Result<TourneeStatistics> = withContext(Dispatchers.IO) {
        try {
            val tournee = cachedTournee ?: run {
                Log.w(TAG, "⚠️ No hay tournée en cache para estadísticas")
                return@withContext Result.failure(Exception("No hay tournée cargada"))
            }
            
            Log.d(TAG, "📊 Calculando estadísticas de ${tournee.packages.size} paquetes")
            Result.success(tournee.statistics)
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo estadísticas: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🗺️ OBTENER ZONAS DE REPARTO
     */
    suspend fun getDeliveryZones(): Result<List<DeliveryZone>> = withContext(Dispatchers.IO) {
        try {
            val tournee = cachedTournee ?: run {
                Log.w(TAG, "⚠️ No hay tournée en cache para zonas")
                return@withContext Result.failure(Exception("No hay tournée cargada"))
            }
            
            Log.d(TAG, "🗺️ Obteniendo ${tournee.zones.size} zonas de reparto")
            Result.success(tournee.zones)
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo zonas: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🧭 OPTIMIZAR RUTA DE PAQUETES
     */
    suspend fun optimizeRoute(
        packages: List<MobilePackageAction>
    ): Result<OptimizedRoute> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🧭 === OPTIMIZANDO RUTA ===")
            Log.d(TAG, "Optimizando ruta para ${packages.size} paquetes")
            
            // Filtrar solo paquetes con coordenadas válidas
            val packagesWithCoords = packages.filter { pkg ->
                pkg.location.latitude != null && 
                pkg.location.longitude != null &&
                pkg.location.latitude in 41.0..51.5 && // Francia aproximadamente
                pkg.location.longitude in -5.0..10.0
            }
            
            if (packagesWithCoords.isEmpty()) {
                Log.w(TAG, "⚠️ No hay paquetes con coordenadas válidas para optimizar")
                return@withContext Result.failure(Exception("No hay paquetes con coordenadas válidas"))
            }
            
            Log.d(TAG, "📍 ${packagesWithCoords.size} paquetes tienen coordenadas válidas")
            
            // Implementar algoritmo TSP simple (para demo, se puede mejorar)
            val optimizedOrder = optimizeWithTSP(packagesWithCoords)
            
            // Crear waypoints en orden optimizado
            val waypoints = optimizedOrder.mapIndexed { index, pkg ->
                RouteWaypoint(
                    packageId = pkg.package_info.id,
                    order = index + 1,
                    latitude = pkg.location.latitude!!,
                    longitude = pkg.location.longitude!!,
                    address = pkg.location.formattedAddress,
                    estimatedArrival = calculateEstimatedArrival(index),
                    distanceFromPrevious = if (index > 0) {
                        calculateDistance(
                            optimizedOrder[index - 1],
                            pkg
                        )
                    } else null
                )
            }
            
            // Calcular distancia total
            val totalDistance = waypoints.sumOf { it.distanceFromPrevious ?: 0.0 }
            
            val optimizedRoute = OptimizedRoute(
                routeId = UUID.randomUUID().toString(),
                packageOrder = optimizedOrder.map { it.package_info.id },
                totalDistance = totalDistance,
                estimatedDuration = formatDuration(totalDistance),
                optimizationAlgorithm = "TSP_NEAREST_NEIGHBOR",
                waypoints = waypoints,
                createdAt = SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault()).format(Date())
            )
            
            // Actualizar cache con ruta optimizada
            cachedTournee = cachedTournee?.copy(optimizedRoute = optimizedRoute)
            
            Log.d(TAG, "✅ Ruta optimizada: ${optimizedRoute.waypoints.size} puntos, ${String.format("%.2f", totalDistance)} km")
            Result.success(optimizedRoute)
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error optimizando ruta: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🔄 ACTUALIZAR ESTADO DE PAQUETE
     */
    suspend fun updatePackageStatus(
        packageId: String,
        newStatus: Status
    ): Result<Boolean> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🔄 Actualizando estado del paquete $packageId a ${newStatus.code}")
            
            val tournee = cachedTournee ?: run {
                Log.w(TAG, "⚠️ No hay tournée en cache para actualizar")
                return@withContext Result.failure(Exception("No hay tournée cargada"))
            }
            
            // Actualizar paquete en cache local
            val updatedPackages = tournee.packages.map { pkg ->
                if (pkg.package_info.id == packageId) {
                    pkg.copy(status = newStatus)
                } else {
                    pkg
                }
            }
            
            // Recalcular estadísticas
            val updatedStats = TourneeUtils.calculateStatistics(updatedPackages)
            
            // Actualizar cache
            cachedTournee = tournee.copy(
                packages = updatedPackages,
                statistics = updatedStats
            )
            
            Log.d(TAG, "✅ Estado del paquete actualizado en cache local")
            
            // TODO: Aquí se podría sincronizar con el backend si es necesario
            // val syncResult = syncPackageStatusWithBackend(packageId, newStatus)
            
            Result.success(true)
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error actualizando estado: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    /**
     * 🗑️ LIMPIAR CACHE
     */
    fun clearCache() {
        Log.d(TAG, "🗑️ Limpiando cache de tournée")
        cachedTournee = null
        lastCacheTime = 0
    }
    
    /**
     * 📱 OBTENER ESTADO DEL REPOSITORY
     */
    fun getRepositoryState(): TourneeRepositoryState {
        return TourneeRepositoryState(
            hasActiveTournee = cachedTournee != null,
            packageCount = cachedTournee?.packages?.size ?: 0,
            lastUpdateTime = lastCacheTime,
            cacheValid = isCacheValid()
        )
    }
    
    // 🛠️ FUNCIONES PRIVADAS
    
    /**
     * ✅ VERIFICAR SI EL CACHE ES VÁLIDO
     */
    private fun isCacheValid(): Boolean {
        return cachedTournee != null && 
               (System.currentTimeMillis() - lastCacheTime) < cacheValidityMs
    }
    
    /**
     * 🔄 CONVERTIR RESPONSE DEL BACKEND A MODELO TOURNÉE
     */
    private fun convertResponseToTournee(
        response: TourneeResponse,
        username: String,
        date: String
    ): Tournee {
        val packages = response.data ?: emptyList()
        
        // Calcular estadísticas con datos reales
        val statistics = TourneeUtils.calculateStatistics(packages)
        
        // Extraer zonas de reparto de los datos reales
        val zones = TourneeUtils.extractDeliveryZones(packages)
        
        return Tournee(
            id = UUID.randomUUID().toString(),
            date = date,
            matricule = username,
            tourneeCode = packages.firstOrNull()?.package_info?.tourneeCode ?: "UNKNOWN",
            packages = packages,
            statistics = statistics,
            zones = zones,
            optimizedRoute = null // Se genera bajo demanda
        )
    }
    
    /**
     * 🧭 ALGORITMO TSP SIMPLE (NEAREST NEIGHBOR)
     */
    private fun optimizeWithTSP(packages: List<MobilePackageAction>): List<MobilePackageAction> {
        if (packages.isEmpty()) return emptyList()
        
        val unvisited = packages.toMutableList()
        val optimized = mutableListOf<MobilePackageAction>()
        
        // Empezar con el primer paquete
        var current = unvisited.removeAt(0)
        optimized.add(current)
        
        // Algoritmo greedy: siempre ir al punto más cercano
        while (unvisited.isNotEmpty()) {
            val nearest = unvisited.minByOrNull { pkg ->
                calculateDistance(current, pkg)
            }!!
            
            unvisited.remove(nearest)
            optimized.add(nearest)
            current = nearest
        }
        
        return optimized
    }
    
    /**
     * 📏 CALCULAR DISTANCIA ENTRE DOS PAQUETES (HAVERSINE)
     */
    private fun calculateDistance(
        package1: MobilePackageAction,
        package2: MobilePackageAction
    ): Double {
        val lat1 = package1.location.latitude ?: return Double.MAX_VALUE
        val lon1 = package1.location.longitude ?: return Double.MAX_VALUE
        val lat2 = package2.location.latitude ?: return Double.MAX_VALUE
        val lon2 = package2.location.longitude ?: return Double.MAX_VALUE
        
        // Fórmula de Haversine
        val R = 6371.0 // Radio de la Tierra en km
        val dLat = Math.toRadians(lat2 - lat1)
        val dLon = Math.toRadians(lon2 - lon1)
        val a = Math.sin(dLat / 2) * Math.sin(dLat / 2) +
                Math.cos(Math.toRadians(lat1)) * Math.cos(Math.toRadians(lat2)) *
                Math.sin(dLon / 2) * Math.sin(dLon / 2)
        val c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a))
        return R * c
    }
    
    /**
     * ⏰ CALCULAR TIEMPO ESTIMADO DE LLEGADA
     */
    private fun calculateEstimatedArrival(orderIndex: Int): String {
        val baseTime = Calendar.getInstance().apply {
            set(Calendar.HOUR_OF_DAY, 9) // Empezar a las 9:00
            set(Calendar.MINUTE, 0)
        }
        
        // Agregar 15 minutos por cada entrega
        baseTime.add(Calendar.MINUTE, orderIndex * 15)
        
        val formatter = SimpleDateFormat("HH:mm", Locale.getDefault())
        return formatter.format(baseTime.time)
    }
    
    /**
     * ⏱️ FORMATEAR DURACIÓN ESTIMADA
     */
    private fun formatDuration(totalDistanceKm: Double): String {
        // Asumir velocidad promedio de 30 km/h en ciudad
        val durationHours = totalDistanceKm / 30.0
        val hours = durationHours.toInt()
        val minutes = ((durationHours - hours) * 60).toInt()
        
        return when {
            hours > 0 -> "${hours}h ${minutes}min"
            else -> "${minutes}min"
        }
    }
}

/**
 * 📊 ESTADO DEL TOURNEE REPOSITORY
 */
data class TourneeRepositoryState(
    val hasActiveTournee: Boolean = false,
    val packageCount: Int = 0,
    val lastUpdateTime: Long = 0,
    val cacheValid: Boolean = false
) {
    fun getFormattedLastUpdate(): String {
        val date = Date(lastUpdateTime)
        val formatter = SimpleDateFormat("HH:mm:ss", Locale.getDefault())
        return formatter.format(date)
    }
}