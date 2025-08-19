package com.daniel.deliveryrouting.data.api

import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import android.util.Log

object ColisApiService {
    
    // 🎯 USAR BACKEND LOCAL para optimización de rutas
    private val BASE_URL = EnvironmentConfig.getBaseUrl() + "/"
    
    val api: ColisApi by lazy {
        if (EnvironmentConfig.isUsingLocalBackend()) {
            Log.d("ColisApiService", "✅ Inicializando Colis API con BACKEND LOCAL para optimización")
        } else {
            Log.d("ColisApiService", "⚠️ Inicializando Colis API DIRECTO (sin optimización)")
        }
        
        Log.d("ColisApiService", "Base URL: $BASE_URL")
        
        Retrofit.Builder()
            .baseUrl(BASE_URL)
            .client(ColisHttpClient.client)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
            .create(ColisApi::class.java)
    }
}
