package com.daniel.deliveryrouting.data.api

import okhttp3.*
import okhttp3.logging.HttpLoggingInterceptor
import java.security.cert.X509Certificate
import javax.net.ssl.*
import java.util.concurrent.TimeUnit
import android.util.Log

object ColisHttpClient {
    
    private fun createUnsafeOkHttpClient(): OkHttpClient {
        return try {
            // SSL Context que acepta todos los certificados
            val trustAllCerts = arrayOf<TrustManager>(object : X509TrustManager {
                override fun checkClientTrusted(chain: Array<X509Certificate>, authType: String) {}
                override fun checkServerTrusted(chain: Array<X509Certificate>, authType: String) {}
                override fun getAcceptedIssuers(): Array<X509Certificate> = arrayOf()
            })

            val sslContext = SSLContext.getInstance("SSL")
            sslContext.init(null, trustAllCerts, java.security.SecureRandom())
            val sslSocketFactory = sslContext.socketFactory

            // Logging interceptor
            val logging = HttpLoggingInterceptor { message ->
                Log.d("ColisHttpClient", message)
            }.apply {
                level = HttpLoggingInterceptor.Level.BODY
            }

            OkHttpClient.Builder()
                .sslSocketFactory(sslSocketFactory, trustAllCerts[0] as X509TrustManager)
                .hostnameVerifier { _, _ -> true } // Acepta todos los hostnames
                .addInterceptor(logging)
                .connectTimeout(30, TimeUnit.SECONDS)
                .readTimeout(30, TimeUnit.SECONDS)
                .writeTimeout(30, TimeUnit.SECONDS)
                .build()
        } catch (e: Exception) {
            Log.e("ColisHttpClient", "Error creating unsafe client: ${e.message}")
            throw RuntimeException(e)
        }
    }
    
    val client: OkHttpClient by lazy { createUnsafeOkHttpClient() }
}

