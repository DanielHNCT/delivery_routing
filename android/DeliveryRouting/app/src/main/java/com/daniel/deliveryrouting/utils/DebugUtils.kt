package com.daniel.deliveryrouting.utils

import android.util.Log
import com.daniel.deliveryrouting.data.api.models.LoginResponse

object DebugUtils {
    
    fun logLoginResponse(response: LoginResponse, tag: String = "DebugUtils") {
        Log.d(tag, "=== LOGIN RESPONSE DEBUG ===")
        Log.d(tag, "Response completa: $response")
        Log.d(tag, "response.success: ${response.success}")
        Log.d(tag, "response.status: ${response.status}")
        Log.d(tag, "response.code: ${response.code}")
        Log.d(tag, "response.message: ${response.message}")
        Log.d(tag, "response.token: ${response.token}")
        Log.d(tag, "response.authentication: ${response.authentication}")
        Log.d(tag, "response.authentication?.token: ${response.authentication?.token}")
        Log.d(tag, "response.authentication?.message: ${response.authentication?.message}")
        Log.d(tag, "response.error: ${response.error}")
        Log.d(tag, "response.error?.message: ${response.error?.message}")
        Log.d(tag, "response.timestamp: ${response.timestamp}")
        Log.d(tag, "response.credentials_used: ${response.credentials_used}")
        Log.d(tag, "=== FIN DEBUG ===")
    }
    
    fun analyzeLoginResponse(response: LoginResponse): String {
        return buildString {
            appendLine("=== ANÁLISIS DE RESPUESTA ===")
            appendLine("Success: ${response.success}")
            appendLine("Status: ${response.status}")
            appendLine("Code: ${response.code}")
            appendLine("Token directo: ${response.token}")
            appendLine("Token en auth: ${response.authentication?.token}")
            appendLine("Message directo: ${response.message}")
            appendLine("Message en auth: ${response.authentication?.message}")
            appendLine("Error: ${response.error?.message}")
            appendLine("=== FIN ANÁLISIS ===")
        }
    }
}

