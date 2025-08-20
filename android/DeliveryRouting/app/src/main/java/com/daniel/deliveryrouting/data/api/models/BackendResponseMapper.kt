package com.daniel.deliveryrouting.data.api.models

import android.util.Log

/**
 * Mapper para convertir respuestas del backend al formato Colis
 * Permite que Android use el formato consistente del backend
 */
object BackendResponseMapper {
    
    private const val TAG = "BackendResponseMapper"
    
    /**
     * Convierte BackendAuthResponse a ColisLoginResponse
     * Mantiene compatibilidad con la app Android existente
     */
    fun mapBackendAuthToColisLogin(backendResponse: BackendAuthResponse): ColisLoginResponse {
        Log.d(TAG, "=== MAPPING BACKEND → COLIS FORMAT ===")
        Log.d(TAG, "Backend success: ${backendResponse.success}")
        Log.d(TAG, "Backend matricule: ${backendResponse.authentication.matricule}")
        Log.d(TAG, "Backend message: ${backendResponse.authentication.message}")
        Log.d(TAG, "Backend token: ${backendResponse.authentication.token.take(50)}...")
        
        return ColisLoginResponse(
            // ✅ Mapeo directo del backend
            isAuthentif = backendResponse.success,
            code = if (backendResponse.success) 200 else 401,
            duration = null,
            type = "BACKEND_AUTH",
            errorBody = if (!backendResponse.success) backendResponse.authentication.message else null,
            data = backendResponse.authentication.matricule,
            titreFromBean = "Backend Authentication",
            errorMessageFromBean = if (!backendResponse.success) backendResponse.authentication.message else null,
            exception = null,
            
            // ✅ Datos específicos del backend
            identity = backendResponse.authentication.matricule,
            matricule = backendResponse.authentication.matricule,
            
            // ✅ Tokens del backend
            tokens = TokensData(
                ssoHopps = backendResponse.authentication.token
            ),
            shortToken = ShortTokenData(
                ssoHopps = backendResponse.authentication.token
            )
        )
    }
    
    /**
     * Convierte BackendErrorResponse a ColisLoginResponse de error
     */
    fun mapBackendErrorToColisLogin(backendError: BackendErrorResponse): ColisLoginResponse {
        Log.d(TAG, "=== MAPPING BACKEND ERROR → COLIS FORMAT ===")
        Log.d(TAG, "Backend error: ${backendError.error}")
        Log.d(TAG, "Backend message: ${backendError.message}")
        
        return ColisLoginResponse(
            isAuthentif = false,
            code = 401,
            duration = null,
            type = "BACKEND_ERROR",
            errorBody = backendError.message,
            data = null,
            titreFromBean = "Backend Error",
            errorMessageFromBean = backendError.message,
            exception = backendError.error,
            identity = null,
            matricule = null,
            tokens = null,
            shortToken = null
        )
    }
    
    /**
     * Valida si la respuesta del backend es válida
     */
    fun isValidBackendResponse(backendResponse: BackendAuthResponse): Boolean {
        return backendResponse.success && 
               backendResponse.authentication.matricule.isNotBlank() &&
               backendResponse.authentication.token.isNotBlank()
    }
    
    /**
     * Log de mapeo completo para debugging
     */
    fun logMappingDetails(backendResponse: BackendAuthResponse, colisResponse: ColisLoginResponse) {
        Log.d(TAG, "=== MAPPING COMPLETADO ===")
        Log.d(TAG, "Backend → Colis:")
        Log.d(TAG, "  success: ${backendResponse.success} → isAuthentif: ${colisResponse.isAuthentif}")
        Log.d(TAG, "  matricule: ${backendResponse.authentication.matricule} → matricule: ${colisResponse.matricule}")
        Log.d(TAG, "  token: ${backendResponse.authentication.token.take(30)}... → tokens.ssoHopps: ${colisResponse.tokens?.ssoHopps?.take(30)}...")
        Log.d(TAG, "  message: ${backendResponse.authentication.message}")
        Log.d(TAG, "  timestamp: ${backendResponse.timestamp}")
    }
}

