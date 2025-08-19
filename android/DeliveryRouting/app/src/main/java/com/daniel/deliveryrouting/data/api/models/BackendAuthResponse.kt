package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName

/**
 * Respuesta de autenticación del backend local
 * Formato consistente para multi-plataforma (Android + iOS)
 */
data class BackendAuthResponse(
    val authentication: AuthenticationData,
    val success: Boolean,
    val timestamp: String
)

/**
 * Datos de autenticación del backend
 */
data class AuthenticationData(
    val matricule: String,
    val message: String,
    val token: String
)

/**
 * Respuesta de error del backend (si success = false)
 */
data class BackendErrorResponse(
    val error: String,
    val message: String,
    val timestamp: String
)
