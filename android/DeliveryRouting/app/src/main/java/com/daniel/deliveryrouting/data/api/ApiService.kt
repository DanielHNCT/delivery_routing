package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.api.models.LoginRequest
import com.daniel.deliveryrouting.data.api.models.LoginResponse
import com.daniel.deliveryrouting.data.api.models.TourneeRequest
import com.daniel.deliveryrouting.data.api.models.TourneeResponse
import com.daniel.deliveryrouting.data.api.models.TourneeUpdateRequest
import com.daniel.deliveryrouting.data.api.ApiConfig
import com.google.gson.JsonObject
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.POST

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add Mapbox dependencies to build.gradle
// 2. Add location-based endpoints if needed
// 3. Implement route optimization endpoints

interface ApiService {
    
    @POST("api/colis-prive/auth")
    suspend fun login(@Body request: LoginRequest): LoginResponse
    
    @POST("api/colis-prive/mobile-tournee-structured")
    suspend fun getTourneeStructured(@Body request: TourneeRequest): TourneeResponse
    
    @POST("api/colis-prive/mobile-tournee-with-retry")
    suspend fun getTourneeUpdated(@Body request: TourneeRequest): TourneeResponse
    
    @GET("api/colis-prive/health")
    suspend fun healthCheck(): Response<JsonObject>
    
    companion object {
        val BASE_URL = ApiConfig.BASE_URL  // No const porque ApiConfig.BASE_URL es dinámico
    }
}
