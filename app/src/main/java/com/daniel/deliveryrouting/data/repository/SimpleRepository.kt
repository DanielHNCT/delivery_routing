package com.daniel.deliveryrouting.data.repository

import android.content.Context
import android.util.Log
import com.daniel.deliveryrouting.data.api.SimpleApi
import com.daniel.deliveryrouting.data.api.models.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory

class SimpleRepository(private val context: Context) {
    
    private val api: SimpleApi by lazy {
        Retrofit.Builder()
            .baseUrl("http://192.168.1.9:3000/")
            .addConverterFactory(GsonConverterFactory.create())
            .build()
            .create(SimpleApi::class.java)
    }
    
    suspend fun loginToBackend(
        username: String,
        password: String,
        societe: String
    ): Result<LoginResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d("SimpleRepository", "🚀 Iniciando login al backend")
            
            val request = LoginRequest(
                username = username,
                password = password,
                societe = societe
            )
            
            val response = api.login(request)
            
            if (response.isSuccessful) {
                val loginResponse = response.body()
                if (loginResponse?.success == true) {
                    Log.d("SimpleRepository", "✅ Login exitoso")
                    Result.success(loginResponse)
                } else {
                    Log.e("SimpleRepository", "❌ Login falló: ${loginResponse?.message}")
                    Result.failure(Exception(loginResponse?.message ?: "Login falló"))
                }
            } else {
                Log.e("SimpleRepository", "❌ Error HTTP: ${response.code()}")
                Result.failure(Exception("Error HTTP: ${response.code()}"))
            }
            
        } catch (e: Exception) {
            Log.e("SimpleRepository", "❌ Error en login: ${e.message}", e)
            Result.failure(e)
        }
    }
    
    suspend fun getTourneeFromBackend(
        username: String,
        password: String,
        societe: String,
        matricule: String,
        date: String? = null
    ): Result<TourneeResponse> = withContext(Dispatchers.IO) {
        try {
            Log.d("SimpleRepository", "🚀 Obteniendo tournée del backend")
            
            val request = TourneeRequest(
                username = username,
                password = password,
                societe = societe,
                matricule = matricule,
                date = date
            )
            
            val response = api.getTournee(request)
            
            if (response.isSuccessful) {
                val tourneeResponse = response.body()
                if (tourneeResponse?.success == true) {
                    Log.d("SimpleRepository", "✅ Tournée obtenida exitosamente")
                    Result.success(tourneeResponse)
                } else {
                    Log.e("SimpleRepository", "❌ Error al obtener tournée: ${tourneeResponse?.message}")
                    Result.failure(Exception(tourneeResponse?.message ?: "Error al obtener tournée"))
                }
            } else {
                Log.e("SimpleRepository", "❌ Error HTTP: ${response.code()}")
                Result.failure(Exception("Error HTTP: ${response.code()}"))
            }
            
        } catch (e: Exception) {
            Log.e("SimpleRepository", "❌ Error al obtener tournée: ${e.message}", e)
            Result.failure(e)
        }
    }
}
