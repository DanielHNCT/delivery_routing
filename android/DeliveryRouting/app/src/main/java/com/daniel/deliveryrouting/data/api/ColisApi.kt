package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.api.models.ColisLoginRequest
import com.daniel.deliveryrouting.data.api.models.ColisLoginResponse
import com.daniel.deliveryrouting.data.api.models.BackendAuthResponse
import com.daniel.deliveryrouting.data.api.models.*
import retrofit2.Response
import retrofit2.http.*

interface ColisApi {
    
    // 🎯 ENDPOINT PRINCIPAL: Autenticación a través de tu backend local
    @POST("api/colis-prive/auth")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8"
    )
    suspend fun login(
        @Body request: ColisLoginRequest,
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
    ): Response<BackendAuthResponse>  // ✅ CAMBIADO: Ahora devuelve BackendAuthResponse
    
    // 🗺️ TODO: Agregar endpoints de optimización una vez que el backend esté listo
    // @POST("api/colis-prive/mobile-tournee-updated")
    // @POST("api/route-optimization/optimize")
    // @GET("api/analytics/delivery-metrics")
}
