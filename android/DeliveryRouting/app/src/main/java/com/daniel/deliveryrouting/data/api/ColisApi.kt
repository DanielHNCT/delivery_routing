package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.api.models.*
import retrofit2.Response
import retrofit2.http.*
import java.util.*

interface ColisApi {
    
    // 🆕 NUEVO: FLUJO COMPLETO DE AUTENTICACIÓN v3.3.0.9 (RESUELVE DEFINITIVAMENTE EL 401)
    @POST("api/colis-prive/v3/complete-flow")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun completeAuthenticationFlow(
        @Body request: CompleteAuthFlowRequest,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String,
        @Header("VersionOS") versionOS: String,
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership"
    ): Response<AuthResponse>
    
    // 🆕 NUEVO: MANEJO DE RECONEXIÓN (RESUELVE EL 401)
    // 🆕 NUEVO: RECONEXIÓN AUTOMÁTICA v3.3.0.9 (RESUELVE DEFINITIVAMENTE EL 401)
    @POST("api/colis-prive/v3/reconnect")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun handleReconnection(
        @Body request: ReconnectionRequest,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String,
        @Header("VersionOS") versionOS: String,
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership"
    ): Response<AuthResponse>
    
    // 🎯 ENDPOINT PRINCIPAL: Autenticación a través de tu backend local
    // ✅ CAMBIADO: Usar endpoint que acepta device_info real
    @POST("api/colis-prive/mobile-tournee-with-retry")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun login(
        @Body request: TourneeRequestWithRetry,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String,  // ✅ CAMBIADO: Se pasa dinámicamente
        @Header("VersionOS") versionOS: String,  // ✅ CAMBIADO: Se pasa dinámicamente
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership"
    ): Response<BackendAuthResponse>
    
    // 🔄 REFRESH TOKEN
    @POST("api/colis-prive/refresh-token")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun refreshToken(
        @Body request: RefreshTokenRequest,
        @Header("ActivityId") activityId: String = UUID.randomUUID().toString(),
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String,  // ✅ CAMBIADO: Se pasa dinámicamente
        @Header("VersionOS") versionOS: String,  // ✅ CAMBIADO: Se pasa dinámicamente
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1"
    ): Response<BackendAuthResponse>
    
    // 📦 TOURNÉE CON AUTO-RETRY
    @POST("api/colis-prive/mobile-tournee-with-retry")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun getTourneeWithRetry(
        @Body request: TourneeRequest,
        @Header("ActivityId") activityId: String = UUID.randomUUID().toString(),
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String,  // ✅ CAMBIADO: Se pasa dinámicamente
        @Header("VersionOS") versionOS: String,  // ✅ CAMBIADO: Se pasa dinámicamente
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1"
    ): Response<TourneeResponse>
    
    // 📋 TOURNÉE SIMPLE (legacy)
    @POST("api/colis-prive/mobile-tournee")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun getTournee(
        @Body request: TourneeRequest,
        @Header("ActivityId") activityId: String = UUID.randomUUID().toString(),
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String,  // ✅ CAMBIADO: Se pasa dinámicamente
        @Header("VersionOS") versionOS: String,  // ✅ CAMBIADO: Se pasa dinámicamente
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1"
    ): Response<TourneeResponse>
    
    // 🏥 HEALTH CHECK
    @GET("api/colis-prive/health")
    @Headers(
        "Accept-Charset: UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun healthCheck(): Response<HealthResponse>
    
    // 🗺️ ENDPOINTS DE OPTIMIZACIÓN DE RUTAS (futuro)
    @POST("api/route-optimization/optimize")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun optimizeRoute(
        @Body request: RouteOptimizationRequest,
        @Header("ActivityId") activityId: String = UUID.randomUUID().toString()
    ): Response<RouteOptimizationResponse>
    
    // 📊 ANALYTICS (futuro)
    @GET("api/analytics/delivery-metrics")
    @Headers(
        "Accept-Charset: UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun getDeliveryMetrics(
        @Query("date") date: String,
        @Query("matricule") matricule: String,
        @Header("ActivityId") activityId: String = UUID.randomUUID().toString()
    ): Response<DeliveryMetricsResponse>
}
