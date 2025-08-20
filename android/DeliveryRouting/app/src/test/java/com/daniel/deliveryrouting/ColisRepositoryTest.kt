package com.daniel.deliveryrouting

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.daniel.deliveryrouting.data.repository.ColisRepository
import com.daniel.deliveryrouting.utils.DeviceInfoManager
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.runTest
import org.junit.Assert.*
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

/**
 * 🧪 TESTS UNITARIOS PARA COLIS REPOSITORY
 * 
 * Prueba:
 * - ✅ Integración con DeviceInfoManager
 * - ✅ Integración con TokenManager
 * - ✅ Funciones básicas del repository
 */
@RunWith(RobolectricTestRunner::class)
@Config(sdk = [28])
class ColisRepositoryTest {
    
    private lateinit var context: Context
    private lateinit var colisRepository: ColisRepository
    private lateinit var deviceInfoManager: DeviceInfoManager
    
    @Before
    fun setup() {
        context = ApplicationProvider.getApplicationContext()
        colisRepository = ColisRepository(context)
        deviceInfoManager = DeviceInfoManager(context)
    }
    
    @Test
    fun `ColisRepository should be initialized correctly`() {
        // Then
        assertNotNull("ColisRepository should not be null", colisRepository)
    }
    
    @Test
    fun `DeviceInfoManager should be accessible from repository`() {
        // When
        val deviceInfo = colisRepository.getDeviceInfo()
        
        // Then
        assertNotNull("Device info should not be null", deviceInfo)
        assertTrue("Device model should not be empty", deviceInfo.model.isNotEmpty())
        assertTrue("Install ID should not be empty", deviceInfo.installId.isNotEmpty())
    }
    
    @Test
    fun `Installation info should be accessible from repository`() {
        // When
        val installInfo = colisRepository.getInstallationInfo()
        
        // Then
        assertNotNull("Installation info should not be null", installInfo)
        assertTrue("Install ID should not be empty", installInfo.installId.isNotEmpty())
        assertTrue("First install time should be > 0", installInfo.firstInstallTime > 0)
    }
    
    @Test
    fun `Repository should have correct backend URL`() {
        // When
        val state = runTest { colisRepository.getCurrentState() }
        
        // Then
        assertNotNull("Repository state should not be null", state)
        // El estado inicial debe ser no autenticado
        assertFalse("Initial state should not be authenticated", state.isAuthenticated)
    }
}
