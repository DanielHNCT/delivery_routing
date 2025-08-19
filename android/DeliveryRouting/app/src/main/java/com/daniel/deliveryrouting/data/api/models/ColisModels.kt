package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName

// Colis Privé Authentication Models
data class ColisLoginRequest(
    val audit: AuditData,
    val commun: CommunData,
    @SerializedName("username")  // ✅ CAMBIADO: login → username para coincidir con backend
    val username: String,        // ✅ CAMBIADO: login → username para coincidir con backend
    val password: String,
    val societe: String
)

data class AuditData(
    @SerializedName("appName") val appName: String,
    val cle1: String,
    val cle2: String,
    val cle3: String,
    @SerializedName("deviceModelName") val deviceModelName: String,
    val iccid: String,
    val imei: String,
    val msisdn: String,
    @SerializedName("noSerie") val noSerie: String
)

data class CommunData(
    @SerializedName("dureeTokenInHour") val dureeTokenInHour: Int
)

data class ColisLoginResponse(
    @SerializedName("isAuthentif") val isAuthentif: Boolean,  // ← CAMBIADO: isOk → isAuthentif
    val code: Int,
    val duration: Int?,
    @SerializedName("type") val type: String?,
    @SerializedName("errorBody") val errorBody: String?,
    val data: String?,  // ← CAMBIADO: Any? → String? (como en logs)
    @SerializedName("titreFromBean") val titreFromBean: String?,
    @SerializedName("errorMessageFromBean") val errorMessageFromBean: String?,
    val exception: String?,
    
    // ← NUEVOS CAMPOS IDENTIFICADOS EN LOGS:
    val identity: String?,
    val matricule: String?,
    val tokens: TokensData?,
    val shortToken: ShortTokenData?
)

// Device Information Models
data class DeviceInfo(
    val model: String,
    val serialNumber: String,
    val imei: String = "indisponible",
    val iccid: String = "indisponible",
    val msisdn: String = "indisponible"
)

// SOAP Response Models
data class SoapBonjourDistriResponse(
    val success: Boolean,
    val message: String?,
    val data: String?
)

// Token Data Models (nuevos campos identificados en logs)
data class TokensData(
    @SerializedName("SsoHopps") val ssoHopps: String?
)

data class ShortTokenData(
    @SerializedName("SsoHopps") val ssoHopps: String?
)
