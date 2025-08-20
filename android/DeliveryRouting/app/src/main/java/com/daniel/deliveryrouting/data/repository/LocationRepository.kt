package com.daniel.deliveryrouting.data.repository

import com.daniel.deliveryrouting.data.api.models.Package
import com.daniel.deliveryrouting.data.api.models.MapBounds
import com.daniel.deliveryrouting.data.api.models.LatLng

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add Mapbox SDK dependencies
// 2. Implement actual map operations
// 3. Add route calculation methods
// 4. Add location clustering methods

class LocationRepository {
    
    // Placeholder methods for future Mapbox integration
    
    fun getPackagesWithCoordinates(packages: List<Package>): List<Package> {
        return packages.filter { it.location.hasCoordinates }
    }
    
    fun calculateMapBounds(packages: List<Package>): MapBounds? {
        val validLocations = packages.mapNotNull { pkg ->
            if (pkg.location.hasCoordinates && 
                pkg.location.latitude != null && 
                pkg.location.longitude != null) {
                LatLng(pkg.location.latitude, pkg.location.longitude)
            } else null
        }
        
        if (validLocations.isEmpty()) return null
        
        val minLat = validLocations.minOf { it.latitude }
        val maxLat = validLocations.maxOf { it.latitude }
        val minLng = validLocations.minOf { it.longitude }
        val maxLng = validLocations.maxOf { it.longitude }
        
        return MapBounds(
            southwest = LatLng(minLat, minLng),
            northeast = LatLng(maxLat, maxLng)
        )
    }
    
    fun sortPackagesByDistance(
        packages: List<Package>,
        userLat: Double,
        userLng: Double
    ): List<Package> {
        return packages.sortedBy { pkg ->
            if (pkg.location.hasCoordinates && 
                pkg.location.latitude != null && 
                pkg.location.longitude != null) {
                calculateDistance(
                    userLat, userLng,
                    pkg.location.latitude, pkg.location.longitude
                )
            } else Double.MAX_VALUE
        }
    }
    
    private fun calculateDistance(
        lat1: Double, lng1: Double,
        lat2: Double, lng2: Double
    ): Double {
        // Implementación Haversine para calcular distancia
        val earthRadius = 6371.0 // km
        
        val dLat = Math.toRadians(lat2 - lat1)
        val dLng = Math.toRadians(lng2 - lng1)
        
        val a = kotlin.math.sin(dLat / 2) * kotlin.math.sin(dLat / 2) +
                kotlin.math.cos(Math.toRadians(lat1)) * kotlin.math.cos(Math.toRadians(lat2)) *
                kotlin.math.sin(dLng / 2) * kotlin.math.sin(dLng / 2)
        
        val c = 2 * kotlin.math.atan2(kotlin.math.sqrt(a), kotlin.math.sqrt(1 - a))
        return earthRadius * c
    }
}


