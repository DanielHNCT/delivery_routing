package com.daniel.deliveryrouting.data

/**
 * 🔄 CLASE RESULT PARA MANEJAR OPERACIONES ASÍNCRONAS
 * 
 * Esta clase encapsula el resultado de una operación que puede ser:
 * - Success: Operación exitosa con datos
 * - Error: Operación fallida con mensaje de error
 */
sealed class Result<out T> {
    data class Success<out T>(val data: T) : Result<T>()
    data class Error(val message: String) : Result<Nothing>()
    
    /**
     * 🔄 TRANSFORMAR RESULTADO EXITOSO
     */
    fun <R> map(transform: (T) -> R): Result<R> {
        return when (this) {
            is Success -> Success(transform(data))
            is Error -> Error(message)
        }
    }
    
    /**
     * 🔄 TRANSFORMAR RESULTADO EXITOSO CON FUNCIÓN ASÍNCRONA
     */
    suspend fun <R> mapAsync(transform: suspend (T) -> R): Result<R> {
        return when (this) {
            is Success -> Success(transform(data))
            is Error -> Error(message)
        }
    }
    
    /**
     * 🔄 EJECUTAR ACCIÓN SEGÚN EL RESULTADO
     */
    fun fold(
        onSuccess: (T) -> Unit,
        onError: (String) -> Unit
    ) {
        when (this) {
            is Success -> onSuccess(data)
            is Error -> onError(message)
        }
    }
}
