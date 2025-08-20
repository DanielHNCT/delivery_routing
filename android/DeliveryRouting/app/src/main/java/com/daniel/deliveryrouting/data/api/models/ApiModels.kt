package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName
import com.daniel.deliveryrouting.utils.DeviceInfo

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add Mapbox dependencies to build.gradle
// 2. Implement map display in PackageListScreen
// 3. Add map markers for packages with coordinates
// 4. Implement route optimization display

// 🎯 MODELOS PARA SISTEMA DE TOKENS COLIS PRIVÉ
data class ColisAuthResponse(
    @SerializedName("infoConsolidee") val infoConsolidee: String? = null,
    @SerializedName("isAuthentif") val isAuthentif: Boolean,
    @SerializedName("accountExpirationDate") val accountExpirationDate: String? = null,
    @SerializedName("roleSGBD") val roleSGBD: List<String>? = null,
    @SerializedName("roleSI") val roleSI: String? = null,
    val identity: String? = null,
    @SerializedName("isAdminMetier") val isAdminMetier: Boolean = false,
    @SerializedName("isAdminIndiana") val isAdminIndiana: Boolean = false,
    val matricule: String? = null,
    val nom: String? = null,
    val prenom: String? = null,
    @SerializedName("codeAnalytique") val codeAnalytique: String? = null,
    val domaine: String? = null,
    val tenant: String? = null,
    val societe: String? = null,
    @SerializedName("libelleSociete") val libelleSociete: String? = null,
    @SerializedName("typeClient") val typeClient: String? = null,
    @SerializedName("habilitationAD") val habilitationAD: Any? = null,
    @SerializedName("habilitationInterprete") val habilitationInterprete: Any? = null,
    val roles: List<String>? = null,
    val tokens: TokenData,
    @SerializedName("shortToken") val shortToken: TokenData,
    @SerializedName("profilUtilisateur") val profilUtilisateur: List<Any>? = null
)

// 🔑 DATOS DE TOKENS
data class TokenData(
    @SerializedName("SsoHopps") val ssoHopps: String
)

// 🔐 REQUEST MODELS ACTUALIZADOS
// ColisLoginRequest movida a ColisModels.kt para evitar duplicación

data class RefreshTokenRequest(
    @SerializedName("token") val token: String,
    @SerializedName("device_info") val deviceInfo: DeviceInfo
)

// 🎯 REQUEST PARA TOURNÉE CON AUTO-RETRY (endpoint principal)
data class TourneeRequest(
    @SerializedName("username") val username: String,
    @SerializedName("password") val password: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("date") val date: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("token") val token: String? = null,
    @SerializedName("device_info") val deviceInfo: DeviceInfo
)

// 🎯 REQUEST PARA AUTENTICACIÓN CON TOURNÉE (nuevo endpoint principal)
data class TourneeRequestWithRetry(
    @SerializedName("username") val username: String,
    @SerializedName("password") val password: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("date") val date: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("token") val token: String? = null,
    @SerializedName("device_info") val deviceInfo: DeviceInfo
)

data class TourneeUpdateRequest(
    @SerializedName("date") val dateDebut: String,     // ✅ CAMBIADO: DateDebut → date
    @SerializedName("matricule") val username: String, // ✅ CAMBIADO: username → matricule
    @SerializedName("password") val password: String,  // ✅ CAMBIADO: Password → password (minúscula)
    @SerializedName("societe") val societe: String     // ✅ CAMBIADO: Societe → societe (minúscula)
)

// Response Models
data class LoginResponse(
    @SerializedName("success") val success: Boolean? = null,
    @SerializedName("authentication") val authentication: Authentication? = null,
    @SerializedName("credentials_used") val credentials_used: CredentialsUsed? = null,
    @SerializedName("timestamp") val timestamp: String? = null,
    @SerializedName("error") val error: ErrorInfo? = null,
    
    // Campos alternativos que puede enviar el backend
    @SerializedName("token") val token: String? = null,
    @SerializedName("message") val message: String? = null,
    @SerializedName("status") val status: String? = null,
    @SerializedName("code") val code: String? = null
)

data class Authentication(
    @SerializedName("token") val token: String?,
    @SerializedName("matricule") val matricule: String?,
    @SerializedName("message") val message: String
)

data class CredentialsUsed(
    @SerializedName("username") val username: String,
    @SerializedName("societe") val societe: String
)

data class ErrorInfo(
    @SerializedName("code") val code: String,
    @SerializedName("message") val message: String
)

// 📦 NUEVOS MODELOS PARA TOURNÉE CON AUTO-RETRY
data class TourneeResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String,
    @SerializedName("data") val data: List<MobilePackageAction>?,
    @SerializedName("endpoint_used") val endpoint_used: String,
    @SerializedName("total_packages") val total_packages: Int
)

data class MobilePackageAction(
    @SerializedName("package_info") val package_info: PackageInfo,
    @SerializedName("customer") val customer: Customer,
    @SerializedName("location") val location: Location,
    @SerializedName("timing") val timing: Timing,
    @SerializedName("status") val status: Status
)

data class PackageInfo(
    @SerializedName("id") val id: String,
    @SerializedName("reference") val reference: String,
    @SerializedName("barcode") val barcode: String,
    @SerializedName("tourneeCode") val tourneeCode: String
)

data class Customer(
    @SerializedName("name") val name: String,
    @SerializedName("phone") val phone: String?,
    @SerializedName("email") val email: String?
)

data class Location(
    @SerializedName("formattedAddress") val formattedAddress: String,
    @SerializedName("city") val city: String,
    @SerializedName("postalCode") val postalCode: String,
    @SerializedName("latitude") val latitude: Double?,
    @SerializedName("longitude") val longitude: Double?
)

data class Timing(
    @SerializedName("estimatedTime") val estimatedTime: String?,
    @SerializedName("priority") val priority: String?
)

data class Status(
    @SerializedName("code") val code: String,
    @SerializedName("label") val label: String,
    @SerializedName("isCompleted") val isCompleted: Boolean
)

// 🆕 NUEVO: Request para flujo completo de autenticación (RESUELVE EL 401)
data class CompleteAuthFlowRequest(
    @SerializedName("username") val username: String,
    @SerializedName("password") val password: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("date") val date: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("device_info") val deviceInfo: DeviceInfo
)

// 🆕 NUEVO: Request para reconexión (RESUELVE EL 401)
data class ReconnectionRequest(
    @SerializedName("username") val username: String,
    @SerializedName("password") val password: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("date") val date: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("device_info") val deviceInfo: DeviceInfo
)

// 🆕 NUEVO: Response del flujo completo
data class AuthResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String,
    @SerializedName("flow_result") val flowResult: FlowResult?,
    @SerializedName("reconnection_result") val reconnectionResult: FlowResult?,
    @SerializedName("timestamp") val timestamp: String,
    @SerializedName("error") val error: ErrorInfo?
)

data class FlowResult(
    @SerializedName("success") val success: Boolean,
    @SerializedName("flow_completed") val flowCompleted: Boolean,
    @SerializedName("session_id") val sessionId: String,
    @SerializedName("activity_id") val activityId: String,
    @SerializedName("timestamp") val timestamp: String,
    @SerializedName("steps") val steps: FlowSteps,
    @SerializedName("message") val message: String
)

data class FlowSteps(
    @SerializedName("device_audit") val deviceAudit: StepResult,
    @SerializedName("version_check") val versionCheck: StepResult,
    @SerializedName("login") val login: StepResult,
    @SerializedName("logging") val logging: StepResult
)

data class StepResult(
    @SerializedName("success") val success: Boolean,
    @SerializedName("status") val status: Int,
    @SerializedName("message") val message: String,
    @SerializedName("response") val response: String
)

// 🏥 HEALTH CHECK RESPONSE
data class HealthResponse(
    @SerializedName("status") val status: String,
    @SerializedName("colis_prive_connection") val colis_prive_connection: Boolean,
    @SerializedName("timestamp") val timestamp: String,
    @SerializedName("version") val version: String
)

// 🗺️ ROUTE OPTIMIZATION MODELS (futuro)
// Eliminadas las clases duplicadas - ver OptimizationModels.kt

// Core Models with GPS Coordinates (Ready for Mapbox)
data class Package(
    @SerializedName("id") val id: String,
    @SerializedName("locationId") val locationId: String,
    @SerializedName("reference") val reference: String,
    @SerializedName("barcode") val barcode: String,
    @SerializedName("tourneeCode") val tourneeCode: String,
    @SerializedName("action") val action: PackageAction,
    @SerializedName("location") val location: PackageLocation, // ¡CON COORDENADAS!
    @SerializedName("timing") val timing: PackageTiming,
    @SerializedName("status") val status: PackageStatus,
    @SerializedName("sender") val sender: PackageSender
)

data class PackageLocation(
    @SerializedName("hasCoordinates") val hasCoordinates: Boolean,
    @SerializedName("latitude") val latitude: Double? = null,
    @SerializedName("longitude") val longitude: Double? = null,
    @SerializedName("gpsQualityMeters") val gpsQualityMeters: String? = null,
    @SerializedName("lastUpdated") val lastUpdated: String? = null,
    @SerializedName("coordinateSource") val coordinateSource: String? = null,
    @SerializedName("formattedAddress") val formattedAddress: String? = null,
    @SerializedName("city") val city: String? = null,
    @SerializedName("postalCode") val postalCode: String? = null
) {
    fun isValidForMapping(): Boolean {
        return hasCoordinates && 
               latitude != null && longitude != null &&
               latitude in 41.0..51.5 && longitude in -5.0..10.0 &&
               (gpsQualityMeters?.toDoubleOrNull() ?: 999.0) < 50.0
    }
}

data class PackageAction(
    @SerializedName("code") val code: String,
    @SerializedName("label") val label: String,
    @SerializedName("color") val color: String? = null
)

data class PackageTiming(
    @SerializedName("estimatedTime") val estimatedTime: String? = null,
    @SerializedName("priority") val priority: String? = null
)

data class PackageStatus(
    @SerializedName("code") val code: String,
    @SerializedName("label") val label: String,
    @SerializedName("isCompleted") val isCompleted: Boolean
)

data class PackageSender(
    @SerializedName("name") val name: String,
    @SerializedName("phone") val phone: String? = null,
    @SerializedName("email") val email: String? = null
)

// Clases preparadas para Mapbox
data class MapBounds(
    val southwest: LatLng,
    val northeast: LatLng
)

data class LatLng(
    val latitude: Double,
    val longitude: Double
)

// Enums
enum class PackageActionType(val code: String, val label: String, val color: String) {
    DELIVERY("DELIVERY", "Entrega", "#4CAF50"),
    PICKUP("PICKUP", "Recogida", "#2196F3"),
    EXCHANGE("EXCHANGE", "Cambio", "#FF9800"),
    RETURN("RETURN", "Devolución", "#F44336")
}

enum class PackageStatusType(val code: String, val label: String, val isCompleted: Boolean) {
    PENDING("PENDING", "Pendiente", false),
    IN_PROGRESS("IN_PROGRESS", "En Progreso", false),
    COMPLETED("COMPLETED", "Completado", true),
    CANCELLED("CANCELLED", "Cancelado", true)
}
