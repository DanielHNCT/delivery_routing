package com.daniel.deliveryrouting

import android.app.Application
import android.util.Log
import com.daniel.deliveryrouting.data.api.NetworkModule
import com.daniel.deliveryrouting.data.preferences.PreferencesManager

class DeliveryRoutingApplication : Application() {
    
    companion object {
        private lateinit var instance: DeliveryRoutingApplication
        
        fun getInstance(): DeliveryRoutingApplication = instance
    }
    
    override fun onCreate() {
        super.onCreate()
        instance = this
        
        Log.d("DeliveryRoutingApp", "Inicializando aplicación...")
        
        // Inicializar servicios usando NetworkModule
        okHttpClient = NetworkModule.okHttpClient
        
        Log.d("DeliveryRoutingApp", "Aplicación inicializada correctamente")
    }
    
    // Lazy initialization de dependencias
    val preferencesManager: PreferencesManager by lazy {
        PreferencesManager(this)
    }
    
    lateinit var okHttpClient: okhttp3.OkHttpClient
}
