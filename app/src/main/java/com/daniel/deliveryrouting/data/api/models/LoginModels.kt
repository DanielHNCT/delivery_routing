package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName

/**
 * Request para login al backend
 */
data class LoginRequest(
    @SerializedName("username") val username: String,
    @SerializedName("password") val password: String,
    @SerializedName("societe") val societe: String
)

/**
 * Response del backend después del login
 */
data class LoginResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String,
    @SerializedName("authentication") val authentication: AuthData?,
    @SerializedName("error") val error: String?
)

/**
 * Datos de autenticación
 */
data class AuthData(
    @SerializedName("token") val token: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("session_id") val sessionId: String,
    @SerializedName("expires_in") val expiresIn: Long
)

/**
 * Request para obtener tournée
 */
data class TourneeRequest(
    @SerializedName("matricule") val matricule: String,
    @SerializedName("date") val date: String
)

/**
 * Response de tournée
 */
data class TourneeResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String,
    @SerializedName("tournee_data") val tourneeData: List<TourneeData>?,
    @SerializedName("error") val error: String?
)

/**
 * Datos de tournée
 */
data class TourneeData(
    @SerializedName("code_tournee") val codeTournee: String,
    @SerializedName("matricule_distributeur") val matriculeDistributeur: String,
    @SerializedName("date_tournee") val dateTournee: String,
    @SerializedName("nombre_colis") val nombreColis: Int
)
