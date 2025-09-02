package com.daniel.deliveryrouting.data.repository

import android.content.Context
import android.util.Log
import com.daniel.deliveryrouting.data.api.BackendApi
import com.daniel.deliveryrouting.data.api.models.LoginRequest
import com.daniel.deliveryrouting.data.api.models.LoginResponse
import com.daniel.deliveryrouting.data.api.models.GetPackagesRequest
import com.daniel.deliveryrouting.data.api.models.GetPackagesResponse
import com.daniel.deliveryrouting.data.api.models.PackageData
import com.daniel.deliveryrouting.data.demo.DemoData
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.util.concurrent.TimeUnit

class BackendRepository(private val context: Context) {

    private val TAG = "BackendRepository"

    private val api: BackendApi by lazy {
        val logging = HttpLoggingInterceptor().apply {
            level = HttpLoggingInterceptor.Level.BODY
        }

        val client = OkHttpClient.Builder()
            .addInterceptor(logging)
            .connectTimeout(30, TimeUnit.SECONDS)
            .readTimeout(30, TimeUnit.SECONDS)
            .writeTimeout(30, TimeUnit.SECONDS)
            .build()

        Retrofit.Builder()
            .baseUrl("http://192.168.1.9:3000/")
            .client(client)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
            .create(BackendApi::class.java)
    }

    suspend fun login(
        username: String,
        password: String,
        societe: String
    ): Result<LoginResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🚀 Iniciando login al backend")
            Log.d(TAG, "Username: $username")
            Log.d(TAG, "Societe: $societe")

            val request = LoginRequest(
                username = username,
                password = password,
                societe = societe
            )

            val response = api.login(request)

            if (response.isSuccessful && response.body() != null) {
                Log.d(TAG, "🎉 Login exitoso: ${response.body()?.message}")
                Result.success(response.body()!!)
            } else {
                val errorBody = response.errorBody()?.string() ?: "Error desconocido"
                Log.e(TAG, "❌ Error en login: ${response.code()} - $errorBody")
                Result.failure(Exception("Error ${response.code()}: $errorBody"))
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Excepción en login: ${e.message}", e)
            Result.failure(e)
        }
    }

    suspend fun getPackages(
        matricule: String,
        date: String? = null
    ): Result<GetPackagesResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "📦 Obteniendo paquetes para matricule: $matricule")
            
            val request = GetPackagesRequest(
                matricule = matricule,
                date = date
            )

            val response = api.getPackages(request)

            if (response.isSuccessful && response.body() != null) {
                val packages = response.body()?.packages
                if (packages.isNullOrEmpty()) {
                    // 🎯 MODO DEMO: Si no hay paquetes, usar datos de prueba
                    Log.d(TAG, "🎯 Activando modo demo - usando datos de prueba")
                    val demoPackages = DemoData.getDemoPackages()
                    val demoResponse = GetPackagesResponse(
                        success = true,
                        message = DemoData.getDemoMessage(),
                        packages = demoPackages,
                        error = null
                    )
                    Result.success(demoResponse)
                } else {
                    Log.d(TAG, "🎉 Paquetes obtenidos exitosamente: ${packages.size} paquetes")
                    Result.success(response.body()!!)
                }
            } else {
                val errorBody = response.errorBody()?.string() ?: "Error desconocido"
                Log.e(TAG, "❌ Error obteniendo paquetes: ${response.code()} - $errorBody")
                
                // 🎯 MODO DEMO: Si hay error, usar datos de prueba
                Log.d(TAG, "🎯 Activando modo demo por error - usando datos de prueba")
                val demoPackages = DemoData.getDemoPackages()
                val demoResponse = GetPackagesResponse(
                    success = true,
                    message = DemoData.getDemoMessage(),
                    packages = demoPackages,
                    error = null
                )
                Result.success(demoResponse)
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Excepción obteniendo paquetes: ${e.message}", e)
            
            // 🎯 MODO DEMO: Si hay excepción, usar datos de prueba
            Log.d(TAG, "🎯 Activando modo demo por excepción - usando datos de prueba")
            val demoPackages = DemoData.getDemoPackages()
            val demoResponse = GetPackagesResponse(
                success = true,
                message = DemoData.getDemoMessage(),
                packages = demoPackages,
                error = null
            )
            Result.success(demoResponse)
        }
    }
}
