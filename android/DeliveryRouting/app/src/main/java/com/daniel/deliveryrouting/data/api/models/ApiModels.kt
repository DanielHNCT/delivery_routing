package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName
import com.daniel.deliveryrouting.utils.DeviceInfo

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add Mapbox dependencies to build.gradle
// 2. Implement map display in PackageListScreen
// 3. Add map markers for packages with coordinates
// 4. Implement route optimization display

// 🆕 NUEVO: MODELOS PARA LOGIN DIRECTO A COLIS PRIVE
data class ColisPriveLoginRequest(
    @SerializedName("username") val username: String,        // Ej: "A187518"
    @SerializedName("password") val password: String,        // Ej: "INTI7518"
    @SerializedName("societe") val societe: String,          // Ej: "PCP0010699"
    @SerializedName("api_choice") val apiChoice: String = "web"  // "web" o "mobile"
)

data class ColisPriveLoginResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String,
    @SerializedName("data") val data: ColisPriveAuthData?,
    @SerializedName("error") val error: String?
)

data class ColisPriveAuthData(
    @SerializedName("token") val token: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("roles") val roles: List<String>,
    @SerializedName("isAuthentif") val isAuthentif: Boolean
)

// 🆕 NUEVO: MODELOS PARA LETTRE DE VOITURE COMPLETO
data class LettreDeVoitureRequest(
    @SerializedName("token") val token: String,
    @SerializedName("matricule") val matricule: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("date") val date: String  // YYYY-MM-DD
)

data class LettreDeVoitureResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String,
    @SerializedName("data") val data: LettreDeVoitureData?,
    @SerializedName("error") val error: String?
)

data class LettreDeVoitureData(
    @SerializedName("matricule") val matricule: String,
    @SerializedName("societe") val societe: String,
    @SerializedName("date") val date: String,
    @SerializedName("tournee_info") val tourneeInfo: TourneeInfo?,
    @SerializedName("colis_summary") val colisSummary: ColisSummary,
    @SerializedName("lettre_content") val lettreContent: String,
    @SerializedName("timestamp") val timestamp: String
)

data class TourneeInfo(
    @SerializedName("code_tournee") val codeTournee: String,
    @SerializedName("statut") val statut: String,
    @SerializedName("distributeur") val distributeur: String,
    @SerializedName("centre") val centre: String,
    @SerializedName("point_concentration") val pointConcentration: String
)

data class ColisSummary(
    @SerializedName("total_colis") val totalColis: Int,
    @SerializedName("colis_distribue") val colisDistribue: Int,
    @SerializedName("colis_restant") val colisRestant: Int,
    @SerializedName("colis_premium") val colisPremium: Int,
    @SerializedName("colis_relais") val colisRelais: Int,
    @SerializedName("colis_casier") val colisCasier: Int
)

// 🏥 HEALTH CHECK
data class HealthResponse(
    @SerializedName("status") val status: String,
    @SerializedName("timestamp") val timestamp: String,
    @SerializedName("version") val version: String
)
