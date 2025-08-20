package com.daniel.deliveryrouting

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.daniel.deliveryrouting.utils.DeviceInfoManager
import org.junit.Assert.*
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

/**
 * 🧪 TESTS UNITARIOS PARA DEVICE INFO MANAGER
 * 
 * Prueba:
 * - ✅ Generación de device info
 * - ✅ Install-ID único
 * - ✅ Fingerprint único
 * - ✅ Fallbacks seguros
 */
@RunWith(RobolectricTestRunner::class)
@Config(sdk = [28])
class DeviceInfoManagerTest {
    
    private lateinit var context: Context
    private lateinit var deviceInfoManager: DeviceInfoManager
    
    @Before
    fun setup() {
        context = ApplicationProvider.getApplicationContext()
        deviceInfoManager = DeviceInfoManager(context)
        
        // Limpiar para testing
        deviceInfoManager.cleanupForTesting()
    }
    
    @Test
    fun `getDeviceInfo should return valid device info`() {
        // When
        val deviceInfo = deviceInfoManager.getDeviceInfo()
        
        // Then
        assertNotNull("Device info should not be null", deviceInfo)
        assertTrue("Model should not be empty", deviceInfo.model.isNotEmpty())
        assertTrue("IMEI should be 15 digits", deviceInfo.imei.length == 15)
        assertTrue("Serial should not be empty", deviceInfo.serialNumber.isNotEmpty())
        assertTrue("Install ID should not be empty", deviceInfo.installId.isNotEmpty())
        assertTrue("Android ID should not be empty", deviceInfo.androidId.isNotEmpty())
    }
    
    @Test
    fun `getInstallId should generate unique ID`() {
        // When
        val installId1 = deviceInfoManager.getInstallationInfo().installId
        val installId2 = deviceInfoManager.getInstallationInfo().installId
        
        // Then
        assertNotNull("Install ID should not be null", installId1)
        assertEquals("Install ID should be consistent", installId1, installId2)
        assertTrue("Install ID should be UUID format", installId1.length == 36)
    }
    
    @Test
    fun `resetInstallIdForTesting should generate new ID`() {
        // Given
        val originalId = deviceInfoManager.getInstallationInfo().installId
        
        // When
        val newId = deviceInfoManager.resetInstallIdForTesting()
        
        // Then
        assertNotEquals("New ID should be different", originalId, newId)
        assertTrue("New ID should be UUID format", newId.length == 36)
    }
    
    @Test
    fun `getDeviceInfo should generate unique fingerprint`() {
        // When
        val deviceInfo = deviceInfoManager.getDeviceInfo()
        val fingerprint = deviceInfo.getFingerprint()
        
        // Then
        assertNotNull("Fingerprint should not be null", fingerprint)
        assertTrue("Fingerprint should contain manufacturer", fingerprint.contains(deviceInfo.manufacturer))
        assertTrue("Fingerprint should contain model", fingerprint.contains(deviceInfo.model))
        assertTrue("Fingerprint should contain Android ID", fingerprint.contains(deviceInfo.androidId))
        assertTrue("Fingerprint should contain Install ID", fingerprint.contains(deviceInfo.installId))
    }
    
    @Test
    fun `getCompactModel should return compact model string`() {
        // When
        val deviceInfo = deviceInfoManager.getDeviceInfo()
        val compactModel = deviceInfo.getCompactModel()
        
        // Then
        assertNotNull("Compact model should not be null", compactModel)
        assertTrue("Compact model should not contain spaces", !compactModel.contains(" "))
        assertTrue("Compact model should be <= 20 chars", compactModel.length <= 20)
    }
    
    @Test
    fun `getInstallationInfo should return valid info`() {
        // When
        val installInfo = deviceInfoManager.getInstallationInfo()
        
        // Then
        assertNotNull("Installation info should not be null", installInfo)
        assertTrue("Install ID should not be empty", installInfo.installId.isNotEmpty())
        assertTrue("First install time should be > 0", installInfo.firstInstallTime > 0)
        assertTrue("Days since install should be >= 0", installInfo.daysSinceInstall >= 0)
        assertTrue("Current time should be > 0", installInfo.currentTime > 0)
    }
    
    @Test
    fun `cleanupForTesting should clear all data`() {
        // Given - generate some data first
        deviceInfoManager.getDeviceInfo()
        deviceInfoManager.getInstallationInfo()
        
        // When
        deviceInfoManager.cleanupForTesting()
        
        // Then - should still work but with fresh data
        val deviceInfo = deviceInfoManager.getDeviceInfo()
        assertNotNull("Device info should still work after cleanup", deviceInfo)
    }
}
