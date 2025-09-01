package com.daniel.deliveryrouting.data.api

import com.daniel.deliveryrouting.data.api.models.LoginRequest
import com.daniel.deliveryrouting.data.api.models.LoginResponse
import com.daniel.deliveryrouting.data.api.models.GetPackagesRequest
import com.daniel.deliveryrouting.data.api.models.GetPackagesResponse
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.Headers
import retrofit2.http.POST

interface BackendApi {
    
    @POST("api/colis-prive/auth")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun login(
        @Body request: LoginRequest
    ): Response<LoginResponse>

    @POST("api/colis-prive/packages")
    @Headers(
        "Accept-Charset: UTF-8",
        "Content-Type: application/json; charset=UTF-8",
        "Connection: Keep-Alive",
        "Accept-Encoding: gzip",
        "User-Agent: okhttp/3.4.1"
    )
    suspend fun getPackages(
        @Body request: GetPackagesRequest
    ): Response<GetPackagesResponse>
}
