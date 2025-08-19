package com.daniel.deliveryrouting.data.preferences

import android.content.Context
import android.content.SharedPreferences

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add map preferences (zoom level, center location)
// 2. Add route preferences (avoid tolls, highways, etc.)
// 3. Add user location preferences

class PreferencesManager(context: Context) {
    
    private val sharedPreferences: SharedPreferences = context.getSharedPreferences(
        PREF_NAME, Context.MODE_PRIVATE
    )
    
    fun saveAuthToken(token: String) {
        sharedPreferences.edit().putString(KEY_AUTH_TOKEN, token).apply()
    }
    
    fun getAuthToken(): String? {
        return sharedPreferences.getString(KEY_AUTH_TOKEN, null)
    }
    
    fun clearAuthToken() {
        sharedPreferences.edit().remove(KEY_AUTH_TOKEN).apply()
    }
    
    fun saveUsername(username: String) {
        sharedPreferences.edit().putString(KEY_USERNAME, username).apply()
    }
    
    fun getUsername(): String? = sharedPreferences.getString(KEY_USERNAME, null)
    
    fun saveSociete(societe: String) {
        sharedPreferences.edit().putString(KEY_SOCIETE, societe).apply()
    }
    
    fun getSociete(): String? = sharedPreferences.getString(KEY_SOCIETE, null)
    
    fun savePassword(password: String) {
        sharedPreferences.edit().putString(KEY_PASSWORD, password).apply()
    }
    
    fun getPassword(): String? = sharedPreferences.getString(KEY_PASSWORD, null)
    
    fun saveTourneeCode(tourneeCode: String) {
        sharedPreferences.edit().putString(KEY_TOURNEE_CODE, tourneeCode).apply()
    }
    
    fun getTourneeCode(): String? {
        return sharedPreferences.getString(KEY_TOURNEE_CODE, null)
    }
    
    fun saveLastSyncTime(timestamp: Long) {
        sharedPreferences.edit().putLong(KEY_LAST_SYNC, timestamp).apply()
    }
    
    fun getLastSyncTime(): Long {
        return sharedPreferences.getLong(KEY_LAST_SYNC, 0L)
    }
    
    fun isLoggedIn(): Boolean {
        return getAuthToken() != null
    }
    
    companion object {
        private const val PREF_NAME = "delivery_routing_prefs"
        private const val KEY_AUTH_TOKEN = "auth_token"
        private const val KEY_USERNAME = "username"
        private const val KEY_SOCIETE = "societe"
        private const val KEY_PASSWORD = "password"
        private const val KEY_TOURNEE_CODE = "tournee_code"
        private const val KEY_LAST_SYNC = "last_sync"
    }
}
