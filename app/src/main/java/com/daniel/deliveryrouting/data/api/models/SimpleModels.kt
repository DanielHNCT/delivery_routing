package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName

// ✅ MODELOS SIMPLIFICADOS PARA COMPILACIÓN

data class LoginRequest(
    @SerializedName("username") val username: String,
    @SerializedName("password") val password: String,
    @SerializedName("societe") val societe: String
)

data class LoginResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String,
    @SerializedName("authentication") val authentication: AuthenticationData?
)

data class AuthenticationData(
    @SerializedName("matricule") val matricule: String,
    @SerializedName("message") val message: String
)

data class TourneeRequest(
    @SerializedName("username") val username: String,
    @SerializedName("password") val password: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("date") val date: String?
)

data class TourneeResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String,
    @SerializedName("data") val data: TourneeData?
)

data class TourneeData(
    @SerializedName("matricule") val matricule: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("date") val date: String
)
