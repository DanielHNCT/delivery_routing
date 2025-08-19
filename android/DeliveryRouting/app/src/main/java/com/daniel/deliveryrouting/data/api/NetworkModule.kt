package com.daniel.deliveryrouting.data.api

import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.util.concurrent.TimeUnit
import android.util.Log

object NetworkModule {
    private val loggingInterceptor = HttpLoggingInterceptor { message ->
        Log.d("NetworkDebug", message)
    }.apply {
        level = HttpLoggingInterceptor.Level.BODY
    }

    private val okHttpClient: OkHttpClient = OkHttpClient.Builder()
        .connectTimeout(60, TimeUnit.SECONDS)
        .readTimeout(60, TimeUnit.SECONDS)
        .writeTimeout(60, TimeUnit.SECONDS)
        .addInterceptor(loggingInterceptor)
        .addInterceptor { chain ->
            val request = chain.request()
            Log.d("NetworkDebug", "=== REQUEST ===")
            Log.d("NetworkDebug", "URL: ${request.url}")
            Log.d("NetworkDebug", "Method: ${request.method}")
            Log.d("NetworkDebug", "Headers: ${request.headers}")

            val response = chain.proceed(request)

            Log.d("NetworkDebug", "=== RESPONSE ===")
            Log.d("NetworkDebug", "Code: ${response.code}")
            Log.d("NetworkDebug", "Message: ${response.message}")
            Log.d("NetworkDebug", "Headers: ${response.headers}")

            response
        }
        .build()

    val apiService: ApiService by lazy {
        Retrofit.Builder()
            .baseUrl(ApiConfig.BASE_URL)
            .client(okHttpClient)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
            .create(ApiService::class.java)
    }
}

