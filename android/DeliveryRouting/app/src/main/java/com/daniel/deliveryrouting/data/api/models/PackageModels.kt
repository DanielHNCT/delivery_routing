package com.daniel.deliveryrouting.data.api.models

import com.google.gson.annotations.SerializedName

// Modelo para obtener paquetes del backend
data class GetPackagesRequest(
    @SerializedName("matricule") val matricule: String,
    @SerializedName("date") val date: String? = null // Opcional, si no se envía usa fecha actual
)

data class GetPackagesResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String?,
    @SerializedName("packages") val packages: List<PackageData>?,
    @SerializedName("error") val error: ErrorData?
)

// Modelo de datos de un paquete
data class PackageData(
    @SerializedName("id") val id: String,
    @SerializedName("tracking_number") val trackingNumber: String,
    @SerializedName("recipient_name") val recipientName: String,
    @SerializedName("address") val address: String,
    @SerializedName("status") val status: String,
    @SerializedName("instructions") val instructions: String = "",
    @SerializedName("phone") val phone: String? = null,
    @SerializedName("delivery_date") val deliveryDate: String? = null,
    @SerializedName("priority") val priority: String = "Normal"
)

// Modelo para actualizar estado de paquete
data class UpdatePackageRequest(
    @SerializedName("package_id") val packageId: String,
    @SerializedName("status") val status: String, // "Entregado", "Fallido", "En camino"
    @SerializedName("notes") val notes: String? = null,
    @SerializedName("delivery_time") val deliveryTime: String? = null
)

data class UpdatePackageResponse(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String?,
    @SerializedName("error") val error: ErrorData?
)
