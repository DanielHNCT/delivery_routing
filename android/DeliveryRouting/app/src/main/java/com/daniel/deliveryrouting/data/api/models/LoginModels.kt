package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName

data class LoginRequest(
    @SerializedName("username") val username: String,
    @SerializedName("password") val password: String,
    @SerializedName("societe") val societe: String
)

data class LoginResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String?,
    @SerializedName("authentication") val authentication: AuthData?,
    @SerializedName("error") val error: ErrorData?
)

data class ErrorData(
    @SerializedName("code") val code: String,
    @SerializedName("message") val message: String
)

data class AuthData(
    @SerializedName("token") val token: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("session_id") val sessionId: String,
    @SerializedName("expires_in") val expiresIn: Long
)
