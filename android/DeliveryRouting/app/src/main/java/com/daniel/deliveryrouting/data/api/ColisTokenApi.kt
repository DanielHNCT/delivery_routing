package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.api.models.ColisAuthResponse
import com.daniel.deliveryrouting.data.api.models.ColisTokenLoginRequest
import com.daniel.deliveryrouting.data.api.models.RefreshTokenRequest
import com.daniel.deliveryrouting.data.api.models.TourneeRequest
import com.daniel.deliveryrouting.data.api.models.TourneeResponse
import retrofit2.Response
import retrofit2.http.*

/**
 * 🎯 API INTERFACE COMPLETA PARA SISTEMA DE TOKENS COLIS PRIVÉ
 * 
 * Endpoints implementados:
 * - ✅ Login inicial
 * - ✅ Refresh de tokens
 * - ✅ Tournée con retry automático
 * - ✅ Health check
 */
interface ColisTokenApi {
    
    /**
     * 🔐 LOGIN INICIAL - Obtener tokens de autenticación
     */
    @POST("api/colis-prive/auth")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8"
    )
    suspend fun login(
        @Body request: ColisTokenLoginRequest,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String = "Samsung SM-S916B",
        @Header("VersionOS") versionOS: String = "5.1.1",
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership",
        @Header("Connection") connection: String = "Keep-Alive",
        @Header("Accept-Encoding") acceptEncoding: String = "gzip",
        @Header("User-Agent") userAgent: String = "okhttp/3.4.1"
    ): Response<ColisAuthResponse>
    
    /**
     * 🔄 REFRESH TOKEN - Renovar tokens expirados
     */
    @POST("api/colis-prive/refresh-token")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8"
    )
    suspend fun refreshToken(
        @Body request: RefreshTokenRequest,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String = "Samsung SM-S916B",
        @Header("VersionOS") versionOS: String = "5.1.1",
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership",
        @Header("Connection") connection: String = "Keep-Alive",
        @Header("Accept-Encoding") acceptEncoding: String = "gzip",
        @Header("User-Agent") userAgent: String = "okhttp/3.4.1"
    ): Response<ColisAuthResponse>
    
    /**
     * 🚚 TOURNÉE CON RETRY - Obtener datos con manejo de tokens
     */
    @POST("api/colis-prive/mobile-tournee-with-retry")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8"
    )
    suspend fun getTourneeWithRetry(
        @Body request: TourneeRequest,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String = "Samsung SM-S916B",
        @Header("VersionOS") versionOS: String = "5.1.1",
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership",
        @Header("Connection") connection: String = "Keep-Alive",
        @Header("Accept-Encoding") acceptEncoding: String = "gzip",
        @Header("User-Agent") userAgent: String = "okhttp/3.4.1"
    ): Response<TourneeResponse>
    
    /**
     * 🚚 TOURNÉE TRADICIONAL - Endpoint original para compatibilidad
     */
    @POST("api/colis-prive/mobile-tournee-updated")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8"
    )
    suspend fun getTourneeUpdated(
        @Body request: TourneeRequest,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String = "Samsung SM-S916B",
        @Header("VersionOS") versionOS: String = "5.1.1",
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership",
        @Header("Connection") connection: String = "Keep-Alive",
        @Header("Accept-Encoding") acceptEncoding: String = "gzip",
        @Header("User-Agent") userAgent: String = "okhttp/3.4.1"
    ): Response<TourneeResponse>
    
    /**
     * 🚚 TOURNÉE STRUCTURED - Endpoint alternativo
     */
    @POST("api/colis-prive/mobile-tournee-structured")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8"
    )
    suspend fun getTourneeStructured(
        @Body request: TourneeRequest,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String = "Samsung SM-S916B",
        @Header("VersionOS") versionOS: String = "5.1.1",
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership",
        @Header("Connection") connection: String = "Keep-Alive",
        @Header("Accept-Encoding") acceptEncoding: String = "gzip",
        @Header("User-Agent") userAgent: String = "okhttp/3.4.1"
    ): Response<TourneeResponse>
    
    /**
     * 🏥 HEALTH CHECK - Verificar estado del backend
     */
    @GET("api/colis-prive/health")
    suspend fun healthCheck(): Response<Map<String, Any>>
    
    /**
     * 🔍 TOKEN VALIDATION - Verificar validez de token
     */
    @POST("api/colis-prive/validate-token")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8"
    )
    suspend fun validateToken(
        @Body request: RefreshTokenRequest,
        @Header("ActivityId") activityId: String,
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("Device") device: String = "Samsung SM-S916B",
        @Header("VersionOS") versionOS: String = "5.1.1",
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1",
        @Header("Domaine") domaine: String = "Membership",
        @Header("Connection") connection: String = "Keep-Alive",
        @Header("Accept-Encoding") acceptEncoding: String = "gzip",
        @Header("User-Agent") userAgent: String = "okhttp/3.4.1"
    ): Response<Map<String, Any>>
}
