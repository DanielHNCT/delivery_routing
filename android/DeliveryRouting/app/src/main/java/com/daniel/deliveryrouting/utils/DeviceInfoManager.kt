package com.daniel.deliveryrouting.utils

import android.content.Context
import android.content.SharedPreferences
import android.os.Build
import android.provider.Settings
import android.telephony.TelephonyManager
import android.util.Log
import com.google.gson.annotations.SerializedName
import java.security.MessageDigest
import java.util.*

/**
 * 🎯 DEVICE INFO MANAGER PARA COLIS PRIVÉ
 * 
 * Características:
 * - ✅ Obtiene información real del dispositivo
 * - ✅ Genera fingerprint único para evitar colisiones
 * - ✅ Fallbacks seguros para emuladores
 * - ✅ Install-ID único por instalación
 * - ✅ Logs seguros sin mostrar datos sensibles
 */
class DeviceInfoManager(private val context: Context) {
    
    private val prefs: SharedPreferences = context.getSharedPreferences(
        "device_info", Context.MODE_PRIVATE
    )
    
    companion object {
        private const val TAG = "DeviceInfoManager"
        private const val KEY_INSTALL_ID = "install_id"
        private const val KEY_FIRST_INSTALL_TIME = "first_install_time"
        private const val KEY_FAKE_IMEI = "fake_imei"
        private const val KEY_FAKE_SERIAL = "fake_serial"
    }
    
    /**
     * 📱 OBTENER INFORMACIÓN COMPLETA DEL DISPOSITIVO
     */
    fun getDeviceInfo(): DeviceInfo {
        return try {
            val androidId = getAndroidId()
            val installId = getInstallId()
            val imei = getImei(androidId)
            val serial = getSerialNumber(androidId)
            val androidVersion = getAndroidVersion()
            
            DeviceInfo(
                model = getDeviceModel(),
                imei = imei,
                serialNumber = serial,
                androidVersion = androidVersion,
                installId = installId,
                androidId = androidId,
                manufacturer = Build.MANUFACTURER,
                brand = Build.BRAND,
                product = Build.PRODUCT,
                device = Build.DEVICE,
                hardware = Build.HARDWARE
            )
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo device info: ${e.message}", e)
            // Fallback con datos básicos
            DeviceInfo(
                model = "Unknown Device",
                imei = "000000000000000",
                serialNumber = "unknown_serial",
                androidVersion = "Unknown",
                installId = UUID.randomUUID().toString(),
                androidId = "unknown_android_id",
                manufacturer = "Unknown",
                brand = "Unknown",
                product = "Unknown",
                device = "Unknown",
                hardware = "Unknown"
            )
        }
    }
    
    /**
     * 🆔 OBTENER INSTALL-ID ÚNICO (UUID persistente)
     */
    private fun getInstallId(): String {
        var installId = prefs.getString(KEY_INSTALL_ID, null)
        
        if (installId == null) {
            installId = UUID.randomUUID().toString()
            val firstInstallTime = System.currentTimeMillis()
            
            prefs.edit()
                .putString(KEY_INSTALL_ID, installId)
                .putLong(KEY_FIRST_INSTALL_TIME, firstInstallTime)
                .apply()
            
            Log.i(TAG, "🆔 Nuevo Install-ID generado: ${installId.take(8)}...")
        }
        
        return installId
    }
    
    /**
     * 📱 OBTENER MODELO REAL DEL DISPOSITIVO
     */
    private fun getDeviceModel(): String {
        return try {
            val manufacturer = Build.MANUFACTURER.trim()
            val model = Build.MODEL.trim()
            
            // Formato: Samsung SM-S916B, Google Pixel 7, etc.
            val deviceModel = if (manufacturer.isNotEmpty() && model.isNotEmpty()) {
                "$manufacturer $model"
            } else {
                Build.MODEL.ifEmpty { "Unknown Device" }
            }
            
            Log.d(TAG, "📱 Device Model: $deviceModel")
            deviceModel
            
        } catch (e: Exception) {
            Log.w(TAG, "⚠️ Error obteniendo device model: ${e.message}")
            "Unknown Device"
        }
    }
    
    /**
     * 📞 OBTENER IMEI REAL O GENERAR FAKE CONSISTENTE
     */
    private fun getImei(androidId: String): String {
        return try {
            // Intentar obtener IMEI real con permisos
            val telephonyManager = context.getSystemService(Context.TELEPHONY_SERVICE) as TelephonyManager
            
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                val imei = telephonyManager.imei
                if (!imei.isNullOrEmpty() && imei != "000000000000000") {
                    Log.d(TAG, "📞 IMEI real obtenido: ${imei.take(8)}...")
                    return imei
                }
            } else {
                @Suppress("DEPRECATION")
                val imei = telephonyManager.deviceId
                if (!imei.isNullOrEmpty() && imei != "000000000000000") {
                    Log.d(TAG, "📞 IMEI real obtenido (legacy): ${imei.take(8)}...")
                    return imei
                }
            }
            
            // Generar IMEI fake consistente basado en ANDROID_ID
            generateFakeImei(androidId)
            
        } catch (e: Exception) {
            Log.w(TAG, "⚠️ Error obteniendo IMEI real: ${e.message}")
            generateFakeImei(androidId)
        }
    }
    
    /**
     * 🔧 GENERAR IMEI FAKE CONSISTENTE
     */
    private fun generateFakeImei(androidId: String): String {
        var fakeImei = prefs.getString(KEY_FAKE_IMEI, null)
        
        if (fakeImei == null) {
            // Generar IMEI fake: "35168007" + hash de ANDROID_ID (15 dígitos total)
            val prefix = "35168007"
            val hash = generateHash(androidId)
            val suffix = hash.take(7) // Tomar 7 caracteres del hash
            
            fakeImei = prefix + suffix
            
            prefs.edit().putString(KEY_FAKE_IMEI, fakeImei).apply()
            Log.d(TAG, "🔧 IMEI fake generado: ${fakeImei.take(8)}...")
        }
        
        return fakeImei
    }
    
    /**
     * 🔢 OBTENER SERIAL REAL O GENERAR FAKE CONSISTENTE
     */
    private fun getSerialNumber(androidId: String): String {
        return try {
            // Intentar obtener serial real
            val serial = Build.SERIAL
            if (!serial.isNullOrEmpty() && serial != "unknown") {
                Log.d(TAG, "🔢 Serial real obtenido: ${serial.take(8)}...")
                return serial
            }
            
            // Generar serial fake consistente
            generateFakeSerial(androidId)
            
        } catch (e: Exception) {
            Log.w(TAG, "⚠️ Error obteniendo serial real: ${e.message}")
            generateFakeSerial(androidId)
        }
    }
    
    /**
     * 🔧 GENERAR SERIAL FAKE CONSISTENTE
     */
    private fun generateFakeSerial(androidId: String): String {
        var fakeSerial = prefs.getString(KEY_FAKE_SERIAL, null)
        
        if (fakeSerial == null) {
            // Generar serial fake: "3qtg83z" + ANDROID_ID (consistente por device)
            val prefix = "3qtg83z"
            val suffix = androidId.take(8)
            
            fakeSerial = prefix + suffix
            
            prefs.edit().putString(KEY_FAKE_SERIAL, fakeSerial).apply()
            Log.d(TAG, "🔧 Serial fake generado: ${fakeSerial.take(8)}...")
        }
        
        return fakeSerial
    }
    
    /**
     * 🤖 OBTENER VERSIÓN ANDROID REAL
     */
    private fun getAndroidVersion(): String {
        return try {
            val version = Build.VERSION.RELEASE
            val sdkInt = Build.VERSION.SDK_INT
            
            val androidVersion = "Android $version (API $sdkInt)"
            Log.d(TAG, "🤖 Android Version: $androidVersion")
            androidVersion
            
        } catch (e: Exception) {
            Log.w(TAG, "⚠️ Error obteniendo Android version: ${e.message}")
            "Android Unknown"
        }
    }
    
    /**
     * 🆔 OBTENER ANDROID_ID (identificador único del dispositivo)
     */
    private fun getAndroidId(): String {
        return try {
            val androidId = Settings.Secure.getString(
                context.contentResolver, 
                Settings.Secure.ANDROID_ID
            )
            
            if (!androidId.isNullOrEmpty() && androidId != "9774d56d682e549c") {
                Log.d(TAG, "🆔 Android ID obtenido: ${androidId.take(8)}...")
                androidId
            } else {
                // Fallback para emuladores o devices problemáticos
                val fallbackId = "emulator_${System.currentTimeMillis() % 1000000}"
                Log.w(TAG, "⚠️ Android ID problemático, usando fallback: ${fallbackId.take(8)}...")
                fallbackId
            }
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error obteniendo Android ID: ${e.message}")
            "error_android_id"
        }
    }
    
    /**
     * 🔐 GENERAR HASH PARA IMEI FAKE
     */
    private fun generateHash(input: String): String {
        return try {
            val digest = MessageDigest.getInstance("MD5")
            val hashBytes = digest.digest(input.toByteArray())
            
            // Convertir bytes a hex string
            val hexString = hashBytes.joinToString("") { 
                "%02x".format(it) 
            }
            
            hexString
        } catch (e: Exception) {
            Log.e(TAG, "❌ Error generando hash: ${e.message}")
            "0000000"
        }
    }
    
    /**
     * 📊 OBTENER INFORMACIÓN DE INSTALACIÓN
     */
    fun getInstallationInfo(): InstallationInfo {
        val installId = getInstallId()
        val firstInstallTime = prefs.getLong(KEY_FIRST_INSTALL_TIME, 0)
        val currentTime = System.currentTimeMillis()
        val daysSinceInstall = if (firstInstallTime > 0) {
            (currentTime - firstInstallTime) / (1000 * 60 * 60 * 24)
        } else 0
        
        return InstallationInfo(
            installId = installId,
            firstInstallTime = firstInstallTime,
            daysSinceInstall = daysSinceInstall,
            currentTime = currentTime
        )
    }
    
    /**
     * 🔄 RESET INSTALL-ID PARA TESTING
     */
    fun resetInstallIdForTesting(): String {
        val newInstallId = UUID.randomUUID().toString()
        val currentTime = System.currentTimeMillis()
        
        prefs.edit()
            .putString(KEY_INSTALL_ID, newInstallId)
            .putLong(KEY_FIRST_INSTALL_TIME, currentTime)
            .apply()
        
        Log.i(TAG, "🔄 Install-ID reseteado para testing: ${newInstallId.take(8)}...")
        return newInstallId
    }
    
    /**
     * 📝 LOG DEVICE INFO SIN MOSTRAR DATOS SENSIBLES COMPLETOS
     */
    fun logDeviceInfo() {
        val deviceInfo = getDeviceInfo()
        val installInfo = getInstallationInfo()
        
        Log.i(TAG, "📱 === DEVICE INFO ===")
        Log.i(TAG, "Model: ${deviceInfo.model}")
        Log.i(TAG, "Manufacturer: ${deviceInfo.manufacturer}")
        Log.i(TAG, "Brand: ${deviceInfo.brand}")
        Log.i(TAG, "Android Version: ${deviceInfo.androidVersion}")
        Log.i(TAG, "IMEI: ${deviceInfo.imei.take(8)}...")
        Log.i(TAG, "Serial: ${deviceInfo.serialNumber.take(8)}...")
        Log.i(TAG, "Install ID: ${deviceInfo.installId.take(8)}...")
        Log.i(TAG, "Android ID: ${deviceInfo.androidId.take(8)}...")
        Log.i(TAG, "First Install: ${installInfo.daysSinceInstall} días atrás")
        Log.i(TAG, "=== FIN DEVICE INFO ===")
    }
    
    /**
     * 🧹 CLEANUP PARA TESTING
     */
    fun cleanupForTesting() {
        prefs.edit().clear().apply()
        Log.i(TAG, "🧹 Device info limpiado para testing")
    }
}

/**
 * 📱 INFORMACIÓN COMPLETA DEL DISPOSITIVO
 * 
 * ✅ COMPATIBLE CON BACKEND RUST (snake_case)
 * - Internamente usa camelCase para legibilidad en Kotlin
 * - Se serializa como snake_case para compatibilidad con Rust
 */
data class DeviceInfo(
    @SerializedName("model") val model: String,
    @SerializedName("imei") val imei: String,
    @SerializedName("serial_number") val serialNumber: String,
    @SerializedName("android_version") val androidVersion: String,
    @SerializedName("install_id") val installId: String,
    @SerializedName("android_id") val androidId: String,
    @SerializedName("manufacturer") val manufacturer: String,
    @SerializedName("brand") val brand: String,
    @SerializedName("product") val product: String,
    @SerializedName("device") val device: String,
    @SerializedName("hardware") val hardware: String
) {
    /**
     * 🔍 OBTENER FINGERPRINT ÚNICO PARA COLIS PRIVÉ
     */
    fun getFingerprint(): String {
        return "$manufacturer|$model|$androidId|$installId"
    }
    
    /**
     * 📱 OBTENER MODELO COMPACTO PARA HEADERS
     */
    fun getCompactModel(): String {
        return model.replace(" ", "").take(20)
    }
}

/**
 * 🆔 INFORMACIÓN DE INSTALACIÓN
 * 
 * ✅ COMPATIBLE CON BACKEND RUST (snake_case)
 */
data class InstallationInfo(
    @SerializedName("install_id") val installId: String,
    @SerializedName("first_install_time") val firstInstallTime: Long,
    @SerializedName("days_since_install") val daysSinceInstall: Long,
    @SerializedName("current_time") val currentTime: Long
) {
    /**
     * 📅 FORMATO LEGIBLE DE INSTALACIÓN
     */
    fun getFormattedInstallDate(): String {
        val date = Date(firstInstallTime)
        val formatter = java.text.SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault())
        return formatter.format(date)
    }
}
