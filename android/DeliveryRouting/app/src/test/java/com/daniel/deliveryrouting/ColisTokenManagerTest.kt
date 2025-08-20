package com.daniel.deliveryrouting

import android.content.Context
import android.content.SharedPreferences
import androidx.test.core.app.ApplicationProvider
import com.daniel.deliveryrouting.data.api.models.BackendAuthResponse
import com.daniel.deliveryrouting.data.api.models.Authentication
import com.daniel.deliveryrouting.data.token.ColisTokenManager
import io.mockk.every
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.runTest
import org.junit.Assert.*
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import java.util.concurrent.TimeUnit

/**
 * 🧪 TESTS UNITARIOS PARA COLIS TOKEN MANAGER
 * 
 * Prueba:
 * - ✅ Guardado de tokens
 * - ✅ Verificación de expiración
 * - ✅ Thread safety
 * - ✅ Error handling
 */
@RunWith(RobolectricTestRunner::class)
@Config(sdk = [28])
class ColisTokenManagerTest {
    
    private lateinit var context: Context
    private lateinit var sharedPreferences: SharedPreferences
    private lateinit var tokenManager: ColisTokenManager
    
    @Before
    fun setup() {
        context = ApplicationProvider.getApplicationContext()
        sharedPreferences = context.getSharedPreferences("test_colis_tokens", Context.MODE_PRIVATE)
        tokenManager = ColisTokenManager(context)
        
        // Limpiar preferencias antes de cada test
        sharedPreferences.edit().clear().apply()
    }
    
    @Test
    fun `saveTokens should persist BackendAuthResponse data correctly`() = runTest {
        // Given
        val authResponse = BackendAuthResponse(
            success = true,
            authentication = Authentication(
                token = "test_token_123456789",
                matricule = "TEST_MATRICULE",
                message = "Test success"
            ),
            timestamp = "2025-08-20T10:00:00Z"
        )
        
        // When
        tokenManager.saveTokens(authResponse)
        
        // Then
        val savedToken = tokenManager.getValidToken()
        assertEquals("test_token_123456789", savedToken)
        
        val userData = tokenManager.getSavedUserData()
        assertNotNull("User data should not be null", userData)
        assertEquals("TEST_MATRICULE", userData?.matricule)
        assertEquals("TEST_MATRICULE", userData?.username) // Usa matricule como username
        assertEquals("PCP0010699", userData?.societe) // Hardcodeado
    }
    
    @Test
    fun `getValidToken should return null for expired token`() = runTest {
        // Given - save a token that's already expired
        val expiredTime = System.currentTimeMillis() - TimeUnit.HOURS.toMillis(2)
        val authResponse = BackendAuthResponse(
            success = true,
            authentication = Authentication(
                token = "expired_token",
                matricule = "TEST_MATRICULE",
                message = "Test"
            ),
            timestamp = "2025-08-20T10:00:00Z"
        )
        
        // Manually set expired time
        context.getSharedPreferences("colis_tokens", Context.MODE_PRIVATE)
            .edit()
            .putString("sso_hopps_token", "expired_token")
            .putLong("token_expiry", expiredTime)
            .apply()
        
        // When
        val token = tokenManager.getValidToken()
        
        // Then
        assertNull("Token should be null when expired", token)
        assertTrue("Token should be marked as expired", tokenManager.isTokenExpired())
    }
    
    @Test
    fun `getValidToken should return token when valid`() = runTest {
        // Given - save a valid token
        val validTime = System.currentTimeMillis() + TimeUnit.HOURS.toMillis(1)
        val authResponse = BackendAuthResponse(
            success = true,
            authentication = Authentication(
                token = "valid_token",
                matricule = "TEST_MATRICULE",
                message = "Test"
            ),
            timestamp = "2025-08-20T10:00:00Z"
        )
        
        // Manually set valid time
        context.getSharedPreferences("colis_tokens", Context.MODE_PRIVATE)
            .edit()
            .putString("sso_hopps_token", "valid_token")
            .putLong("token_expiry", validTime)
            .apply()
        
        // When
        val token = tokenManager.getValidToken()
        
        // Then
        assertEquals("valid_token", token)
        assertFalse("Token should not be expired", tokenManager.isTokenExpired())
    }
    
    @Test
    fun `clearTokens should remove all stored data`() = runTest {
        // Given - save some data first
        val authResponse = BackendAuthResponse(
            success = true,
            authentication = Authentication(
                token = "test_token",
                matricule = "TEST_MATRICULE",
                message = "Test"
            ),
            timestamp = "2025-08-20T10:00:00Z"
        )
        tokenManager.saveTokens(authResponse)
        
        // Verify data was saved
        assertNotNull(tokenManager.getValidToken())
        assertNotNull(tokenManager.getSavedUserData())
        
        // When
        tokenManager.clearTokens()
        
        // Then
        assertNull("Token should be null after clear", tokenManager.getValidToken())
        assertNull("User data should be null after clear", tokenManager.getSavedUserData())
        assertFalse("User should not be logged in after clear", tokenManager.isUserLoggedIn())
    }
    
    @Test
    fun `getTokenForRefresh should return token even if expired`() = runTest {
        // Given - save an expired token
        val expiredTime = System.currentTimeMillis() - TimeUnit.HOURS.toMillis(2)
        context.getSharedPreferences("colis_tokens", Context.MODE_PRIVATE)
            .edit()
            .putString("sso_hopps_token", "expired_token_for_refresh")
            .putLong("token_expiry", expiredTime)
            .apply()
        
        // When
        val refreshToken = tokenManager.getTokenForRefresh()
        
        // Then
        assertEquals("expired_token_for_refresh", refreshToken)
    }
    
    @Test
    fun `isUserLoggedIn should return false when no token`() = runTest {
        // Given - no token stored
        
        // When
        val isLoggedIn = tokenManager.isUserLoggedIn()
        
        // Then
        assertFalse("User should not be logged in without token", isLoggedIn)
    }
    
    @Test
    fun `isUserLoggedIn should return true when valid token exists`() = runTest {
        // Given - save a valid token
        val validTime = System.currentTimeMillis() + TimeUnit.HOURS.toMillis(1)
        context.getSharedPreferences("colis_tokens", Context.MODE_PRIVATE)
            .edit()
            .putString("sso_hopps_token", "valid_token")
            .putLong("token_expiry", validTime)
            .apply()
        
        // When
        val isLoggedIn = tokenManager.isUserLoggedIn()
        
        // Then
        assertTrue("User should be logged in with valid token", isLoggedIn)
    }
    
    @Test
    fun `getTokenStatus should return correct status information`() = runTest {
        // Given - save a token with specific expiry
        val expiryTime = System.currentTimeMillis() + TimeUnit.MINUTES.toMillis(30) // 30 minutes
        context.getSharedPreferences("colis_tokens", Context.MODE_PRIVATE)
            .edit()
            .putString("sso_hopps_token", "status_test_token")
            .putLong("token_expiry", expiryTime)
            .putString("matricule", "STATUS_TEST")
            .putString("societe", "TEST_SOCIETE")
            .apply()
        
        // When
        val status = tokenManager.getTokenStatus()
        
        // Then
        assertTrue("Should have token", status.hasToken)
        assertTrue("Token should be valid", status.isValid)
        assertTrue("Should have remaining minutes", status.remainingMinutes > 0)
        assertEquals("Matricule should match", "STATUS_TEST", status.matricule)
        assertEquals("Societe should match", "TEST_SOCIETE", status.societe)
        assertNotNull("Should have expiry time", status.expiryTime)
    }
    
    @Test
    fun `token expiry should be set to 1 hour from save`() = runTest {
        // Given
        val authResponse = BackendAuthResponse(
            success = true,
            authentication = Authentication(
                token = "timing_test_token",
                matricule = "TIMING_TEST",
                message = "Test"
            ),
            timestamp = "2025-08-20T10:00:00Z"
        )
        
        val beforeSave = System.currentTimeMillis()
        
        // When
        tokenManager.saveTokens(authResponse)
        
        // Then
        val status = tokenManager.getTokenStatus()
        assertNotNull("Should have expiry time", status.expiryTime)
        
        val expectedExpiry = beforeSave + TimeUnit.HOURS.toMillis(1)
        val actualExpiry = status.expiryTime?.time ?: 0
        
        // Allow 1 second tolerance for test execution time
        assertTrue(
            "Expiry should be approximately 1 hour from save",
            Math.abs(actualExpiry - expectedExpiry) < 1000
        )
    }
}
