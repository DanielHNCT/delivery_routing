package com.daniel.deliveryrouting

import android.app.Application
import android.util.Log
import com.daniel.deliveryrouting.data.api.ApiService
import com.daniel.deliveryrouting.data.api.ApiConfig
import com.daniel.deliveryrouting.data.api.AuthInterceptor
import com.daniel.deliveryrouting.data.api.ResponseLoggingInterceptor
import com.daniel.deliveryrouting.data.api.NetworkModule
import com.daniel.deliveryrouting.data.preferences.PreferencesManager
import com.daniel.deliveryrouting.data.repository.DeliveryRepository
import com.daniel.deliveryrouting.data.repository.LocationRepository
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.util.concurrent.TimeUnit

class DeliveryRoutingApplication : Application() {
    
    companion object {
        private lateinit var instance: DeliveryRoutingApplication
        
        fun getInstance(): DeliveryRoutingApplication = instance
    }
    
    override fun onCreate() {
        super.onCreate()
        instance = this
        
        Log.d("DeliveryRoutingApp", "Inicializando aplicación...")
        
        // Inicializar servicios usando NetworkModule (timeouts 60s + logging detallado)
        apiService = NetworkModule.apiService
        
        Log.d("DeliveryRoutingApp", "Aplicación inicializada correctamente")
    }
    
    // Lazy initialization de dependencias
    val preferencesManager: PreferencesManager by lazy {
        PreferencesManager(this)
    }
    
    lateinit var apiService: ApiService
    
    val deliveryRepository: DeliveryRepository by lazy {
        DeliveryRepository(apiService, preferencesManager)
    }
    
    val locationRepository: LocationRepository by lazy {
        LocationRepository()
    }
}
