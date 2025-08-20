package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.preferences.PreferencesManager
import okhttp3.Interceptor
import okhttp3.Response

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add location-based headers if needed
// 2. Add device location headers for route optimization

class AuthInterceptor(
    private val preferencesManager: PreferencesManager
) : Interceptor {
    
    override fun intercept(chain: Interceptor.Chain): Response {
        val originalRequest = chain.request()
        
        val token = preferencesManager.getAuthToken()
        
        val newRequest = if (token != null) {
            originalRequest.newBuilder()
                .header("Authorization", "Bearer $token")
                .header("Content-Type", "application/json")
                .build()
        } else {
            originalRequest.newBuilder()
                .header("Content-Type", "application/json")
                .build()
        }
        
        return chain.proceed(newRequest)
    }
}


