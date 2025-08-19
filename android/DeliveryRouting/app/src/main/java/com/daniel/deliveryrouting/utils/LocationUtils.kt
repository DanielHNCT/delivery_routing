package com.daniel.deliveryrouting.utils

import com.daniel.deliveryrouting.data.api.models.Package
import com.daniel.deliveryrouting.data.api.models.LatLng
import kotlin.math.*

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add Mapbox-specific location utilities
// 2. Add route calculation utilities
// 3. Add map annotation utilities

object LocationUtils {
    
    // Validar coordenadas GPS
    fun isValidCoordinate(lat: Double?, lng: Double?): Boolean {
        return lat != null && lng != null && 
               lat >= -90 && lat <= 90 && 
               lng >= -180 && lng <= 180
    }
    
    // Calcular distancia entre dos puntos (Haversine)
    fun calculateDistance(
        lat1: Double, lng1: Double,
        lat2: Double, lng2: Double
    ): Double {
        val earthRadius = 6371.0 // km
        
        val dLat = Math.toRadians(lat2 - lat1)
        val dLng = Math.toRadians(lng2 - lng1)
        
        val a = sin(dLat / 2) * sin(dLat / 2) +
                cos(Math.toRadians(lat1)) * cos(Math.toRadians(lat2)) *
                sin(dLng / 2) * sin(dLng / 2)
        
        val c = 2 * atan2(sqrt(a), sqrt(1 - a))
        return earthRadius * c
    }
    
    // Calcular distancia desde un punto a una lista de paquetes
    fun calculateDistancesFromPoint(
        userLat: Double,
        userLng: Double,
        packages: List<Package>
    ): Map<String, Double> {
        return packages.associate { pkg ->
            pkg.id to if (pkg.location.hasCoordinates && 
                         isValidCoordinate(pkg.location.latitude, pkg.location.longitude)) {
                calculateDistance(
                    userLat, userLng,
                    pkg.location.latitude!!, pkg.location.longitude!!
                )
            } else Double.MAX_VALUE
        }
    }
    
    // Obtener paquetes ordenados por distancia
    fun getPackagesSortedByDistance(
        userLat: Double,
        userLng: Double,
        packages: List<Package>
    ): List<Package> {
        return packages.sortedBy { pkg ->
            if (pkg.location.hasCoordinates && 
                isValidCoordinate(pkg.location.latitude, pkg.location.longitude)) {
                calculateDistance(
                    userLat, userLng,
                    pkg.location.latitude!!, pkg.location.longitude!!
                )
            } else Double.MAX_VALUE
        }
    }
    
    // Calcular bounds para el mapa
    fun calculateMapBounds(packages: List<Package>): Pair<LatLng, LatLng>? {
        val validLocations = packages.mapNotNull { pkg ->
            if (pkg.location.hasCoordinates && 
                isValidCoordinate(pkg.location.latitude, pkg.location.longitude)) {
                LatLng(pkg.location.latitude!!, pkg.location.longitude!!)
            } else null
        }
        
        if (validLocations.isEmpty()) return null
        
        val minLat = validLocations.minOf { it.latitude }
        val maxLat = validLocations.maxOf { it.latitude }
        val minLng = validLocations.minOf { it.longitude }
        val maxLng = validLocations.maxOf { it.longitude }
        
        // Agregar padding para mejor visualización
        val latPadding = (maxLat - minLat) * 0.1
        val lngPadding = (maxLng - minLng) * 0.1
        
        return Pair(
            LatLng(minLat - latPadding, minLng - lngPadding),
            LatLng(maxLat + latPadding, maxLng + lngPadding)
        )
    }
    
    // Formatear distancia para mostrar
    fun formatDistance(distanceKm: Double): String {
        return when {
            distanceKm < 1 -> "${(distanceKm * 1000).toInt()}m"
            distanceKm < 10 -> "${String.format("%.1f", distanceKm)}km"
            else -> "${distanceKm.toInt()}km"
        }
    }
    
    // Calcular tiempo estimado de viaje (aproximado)
    fun calculateEstimatedTime(distanceKm: Double, averageSpeedKmh: Double = 30.0): Int {
        return (distanceKm / averageSpeedKmh * 60).toInt() // minutos
    }
}

