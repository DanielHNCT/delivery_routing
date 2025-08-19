package com.daniel.deliveryrouting.utils

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add Mapbox API key constant
// 2. Add map style constants
// 3. Add default map center constants

object Constants {
    
    // API Configuration
    const val API_BASE_URL = "http://192.168.1.9:3000/" // Mantener para compatibilidad
    const val API_TIMEOUT_SECONDS = 30L
    
    // Navigation
    const val LOGIN_ROUTE = "login"
    const val MAIN_ROUTE = "main"
    const val PACKAGE_LIST_ROUTE = "package_list"
    const val PACKAGE_DETAIL_ROUTE = "package_detail"
    
    // Preferences
    const val PREF_NAME = "delivery_routing_prefs"
    const val PREF_AUTH_TOKEN = "auth_token"
    const val PREF_TOURNEE_CODE = "tournee_code"
    
    // UI
    const val ANIMATION_DURATION = 300L
    const val DEBOUNCE_DELAY = 500L
    
    // Package Status Colors
    const val COLOR_PENDING = "#FF9800"
    const val COLOR_IN_PROGRESS = "#2196F3"
    const val COLOR_COMPLETED = "#4CAF50"
    const val COLOR_CANCELLED = "#F44336"
    
    // Package Action Colors
    const val COLOR_DELIVERY = "#4CAF50"
    const val COLOR_PICKUP = "#2196F3"
    const val COLOR_EXCHANGE = "#FF9800"
    const val COLOR_RETURN = "#F44336"
}
