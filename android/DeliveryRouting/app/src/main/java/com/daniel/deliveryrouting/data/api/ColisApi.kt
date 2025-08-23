package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.api.models.*
import retrofit2.Response
import retrofit2.http.*
import java.util.*

interface ColisApi {
    
    // 🆕 NUEVO: LOGIN DIRECTO A COLIS PRIVE (PROXY)
    @POST("api/colis-prive/login")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun loginDirectToColisPrive(
        @Body request: ColisPriveLoginRequest
    ): Response<ColisPriveLoginResponse>
    
    // 🆕 NUEVO: LETTRE DE VOITURE COMPLETO (CON TOKEN)
    @POST("api/colis-prive/lettre-voiture")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun getLettreDeVoiture(
        @Body request: LettreDeVoitureRequest,
        @Header("ActivityId") activityId: String = UUID.randomUUID().toString(),
        @Header("AppName") appName: String = "CP DISTRI V2",
        @Header("AppIdentifier") appIdentifier: String = "com.danem.cpdistriv2",
        @Header("VersionApplication") versionApp: String = "3.3.0.9",
        @Header("VersionCode") versionCode: String = "1"
    ): Response<LettreDeVoitureResponse>
    
    // 🏥 HEALTH CHECK
    @GET("api/colis-prive/health")
    @Headers(
        "Accept-Charset: UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun healthCheck(): Response<HealthResponse>
}
