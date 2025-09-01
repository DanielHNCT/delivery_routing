package com.daniel.deliveryrouting.data.repository

import android.content.Context
import android.util.Log
import com.daniel.deliveryrouting.data.api.BackendApi
import com.daniel.deliveryrouting.data.api.models.LoginRequest
import com.daniel.deliveryrouting.data.api.models.LoginResponse
import com.daniel.deliveryrouting.data.api.models.TourneeRequest
import com.daniel.deliveryrouting.data.api.models.TourneeResponse
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.util.concurrent.TimeUnit

/**
 * Repository para comunicarse con el backend Rust
 */
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
            .baseUrl("http://192.168.1.9:3000/") // TODO: Configurar URL base dinámicamente
            .client(client)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
            .create(BackendApi::class.java)
    }

    /**
     * Login al backend
     */
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

    /**
     * Obtener tournée
     */
    suspend fun getTournee(
        matricule: String,
        date: String
    ): Result<TourneeResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🚀 Obteniendo tournée")
            Log.d(TAG, "Matricule: $matricule")
            Log.d(TAG, "Date: $date")

            val request = TourneeRequest(
                matricule = matricule,
                date = date
            )

            val response = api.getTournee(request)

            if (response.isSuccessful && response.body() != null) {
                Log.d(TAG, "🎉 Tournée obtenida: ${response.body()?.message}")
                Result.success(response.body()!!)
            } else {
                val errorBody = response.errorBody()?.string() ?: "Error desconocido"
                Log.e(TAG, "❌ Error en tournée: ${response.code()} - $errorBody")
                Result.failure(Exception("Error ${response.code()}: $errorBody"))
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Excepción en tournée: ${e.message}", e)
            Result.failure(e)
        }
    }

    /**
     * Health check del backend
     */
    suspend fun healthCheck(): Result<Map<String, String>> = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "🚀 Verificando salud del backend")

            val response = api.healthCheck()

            if (response.isSuccessful && response.body() != null) {
                Log.d(TAG, "🎉 Backend saludable: ${response.body()}")
                Result.success(response.body()!!)
            } else {
                val errorBody = response.errorBody()?.string() ?: "Error desconocido"
                Log.e(TAG, "❌ Error en health check: ${response.code()} - $errorBody")
                Result.failure(Exception("Error ${response.code()}: $errorBody"))
            }
        } catch (e: Exception) {
            Log.e(TAG, "❌ Excepción en health check: ${e.message}", e)
            Result.failure(e)
        }
    }
}
