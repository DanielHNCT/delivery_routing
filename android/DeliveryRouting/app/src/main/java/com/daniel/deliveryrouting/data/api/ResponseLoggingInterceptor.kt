package com.daniel.deliveryrouting.data.api

import android.util.Log
import okhttp3.Interceptor
import okhttp3.Response
import okhttp3.ResponseBody
import okio.Buffer
import java.nio.charset.StandardCharsets

class ResponseLoggingInterceptor : Interceptor {
    
    override fun intercept(chain: Interceptor.Chain): Response {
        val request = chain.request()
        val requestBody = request.body
        
        // Log de la request
        Log.d("ResponseLogging", "=== REQUEST ===")
        Log.d("ResponseLogging", "URL: ${request.url}")
        Log.d("ResponseLogging", "Method: ${request.method}")
        Log.d("ResponseLogging", "Headers: ${request.headers}")
        
        if (requestBody != null) {
            val buffer = Buffer()
            requestBody.writeTo(buffer)
            val requestBodyString = buffer.readString(StandardCharsets.UTF_8)
            Log.d("ResponseLogging", "Request Body: $requestBodyString")
        }
        
        // Ejecutar la request
        val response = chain.proceed(request)
        val responseBody = response.body
        
        // Log de la response
        Log.d("ResponseLogging", "=== RESPONSE ===")
        Log.d("ResponseLogging", "Code: ${response.code}")
        Log.d("ResponseLogging", "Message: ${response.message}")
        Log.d("ResponseLogging", "Headers: ${response.headers}")
        
        if (responseBody != null) {
            val responseBodyString = responseBody.string()
            Log.d("ResponseLogging", "Response Body: $responseBodyString")
            
            // Recrear el response body ya que se consumió
            val newResponseBody = ResponseBody.create(
                responseBody.contentType(),
                responseBodyString
            )
            
            return response.newBuilder()
                .body(newResponseBody)
                .build()
        }
        
        return response
    }
}
