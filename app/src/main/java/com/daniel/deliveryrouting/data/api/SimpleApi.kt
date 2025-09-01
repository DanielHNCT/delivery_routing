package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.api.models.*
import retrofit2.Response
import retrofit2.http.*

interface SimpleApi {
    
    @POST("api/colis-prive/auth")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun login(@Body request: LoginRequest): Response<LoginResponse>
    
    @POST("api/colis-prive/tournee")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun getTournee(@Body request: TourneeRequest): Response<TourneeResponse>
    
    @GET("api/colis-prive/health")
    suspend fun healthCheck(): Response<Map<String, String>>
}
