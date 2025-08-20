# 🚚 Delivery Routing - Colis Privé Integration

## 🎯 Descripción

Aplicación Android para optimización de rutas de entrega con integración completa a **Colis Privé**. La app se conecta a un backend Rust local que actúa como proxy para la API de Colis Privé, proporcionando funcionalidades avanzadas de optimización de rutas.

## ✨ Características Principales

### 🔐 Sistema de Autenticación Robusto
- **Auto-refresh automático** de tokens
- **Auto-retry** en caso de tokens expirados
- **Thread-safe** operations con Mutex
- **Persistencia local** de credenciales
- **Manejo inteligente** de sesiones

### 📱 Device Info Manager Único
- **Fingerprint único** por dispositivo para evitar colisiones
- **Install-ID persistente** por instalación de la app
- **IMEI real** cuando hay permisos, fake consistente cuando no
- **Fallbacks seguros** para emuladores y devices sin permisos
- **Logs seguros** sin mostrar datos sensibles completos

### 📦 Gestión de Tournées
- **Carga automática** de paquetes de entrega
- **Auto-retry** con refresh de tokens
- **Manejo robusto** de errores 401
- **Logs detallados** para debugging

### 🗺️ Optimización de Rutas (Futuro)
- **Algoritmos de optimización** avanzados
- **Integración con Mapbox** para visualización
- **Métricas de rendimiento** de entrega
- **Análisis de eficiencia** de rutas

### 📱 UI Moderna y Responsiva
- **Material Design 3** con Jetpack Compose
- **Estados reactivos** con StateFlow
- **Error handling** user-friendly
- **Progress indicators** y feedback visual
- **Auto-retry** en la interfaz

## 🏗️ Arquitectura

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Android App   │    │  Backend Rust    │    │  Colis Privé    │
│                 │    │   (Local Proxy)  │    │     API         │
│  • UI (Compose) │◄──►│  • Auth Proxy    │◄──►│  • Authentication│
│  • ViewModels   │    │  • Route Opt.    │    │  • Tournées     │
│  • Repository   │    │  • Analytics     │    │  • Packages     │
│  • Token Mgr    │    │  • Health Check  │    │  • Status       │
│  • Device Mgr   │    │                   │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## 🚀 Configuración Rápida

### 1. Backend Local
```bash
# El backend Rust debe estar corriendo en:
# Emulador: http://10.0.2.2:3000
# Dispositivo físico: http://192.168.1.X:3000
```

### 2. Credenciales de Test
```kotlin
Username: PCP0010699_A187518
Password: INTI7518
Société: PCP0010699
Date: 2025-08-20
```

### 3. Endpoints Disponibles
- **Auth**: `POST /api/colis-prive/mobile-tournee-with-retry` ✅ (endpoint actualizado)
- **Refresh**: `POST /api/colis-prive/refresh-token`
- **Tournée**: `POST /api/colis-prive/mobile-tournee-with-retry`
- **Health**: `GET /api/colis-prive/health`

## 🛠️ Implementación Completada

### ✅ PROMPT 1: Device Info Manager
- **`DeviceInfoManager.kt`** - Gestión completa de información del dispositivo
- **Fingerprint único** para evitar colisiones con la app oficial
- **Install-ID persistente** por instalación
- **IMEI y Serial** reales o fake consistentes
- **Fallbacks seguros** para emuladores

### ✅ PROMPT 2: Token Manager Completo
- **`ColisTokenManager.kt`** - Gestión robusta de tokens
- **Auto-refresh automático** cuando tokens expiran
- **Extracción automática** de username y societe del matricule
- **Thread-safe** con Mutex
- **Funciones de testing** para debugging

### ✅ PROMPT 3: Repository Integration
- **`ColisRepository.kt`** - Integración completa de managers
- **Auto-retry logic** robusto con máximo 2 intentos
- **Error handling** completo con Result<T>
- **Estado del repository** para UI management
- **Health check** y logout

## 📱 Uso de la App

### 🔐 Inicio de Sesión
1. Abrir la app
2. Las credenciales están pre-llenadas
3. Tocar "🚀 Conectar"
4. La app maneja automáticamente:
   - Generación de device info único
   - Autenticación con backend
   - Guardado automático de tokens

### 📦 Cargar Tournée
1. Una vez autenticado, aparece la sección de tournée
2. Seleccionar fecha (formato: YYYY-MM-DD)
3. Tocar "📋 Cargar Tournée"
4. La app maneja automáticamente:
   - Verificación de tokens
   - Auto-refresh si es necesario
   - Auto-retry en caso de errores 401

### 🔄 Auto-Retry y Refresh
La app implementa un sistema inteligente de auto-retry:

```kotlin
// Flujo automático:
1. Intentar operación con token actual
2. Si 401 (token expirado):
   - Intentar refresh automático
   - Si refresh falla: hacer login fresh
   - Reintentar operación original
3. Máximo 2 intentos para evitar loops
```

## 🧪 Testing

### Tests Unitarios
```bash
# Ejecutar tests del DeviceInfoManager
./gradlew testDebugUnitTest --tests "*DeviceInfoManagerTest*"

# Ejecutar tests del TokenManager
./gradlew testDebugUnitTest --tests "*ColisTokenManagerTest*"

# Ejecutar todos los tests
./gradlew testDebugUnitTest
```

### Tests de Integración
```bash
# Tests de UI
./gradlew connectedDebugAndroidTest
```

### Flujo de Testing
1. **DeviceInfoManager** genera device info único
2. **Repository.authenticate()** con credenciales
3. **TokenManager** guarda tokens automáticamente
4. **Repository.getTourneeWithAutoRetry()** con auto-refresh
5. Verificar logs de cada step

### Logs Esperados
```
📱 === DEVICE INFO ===
Model: Samsung SM-S916B
Install ID: abc12345...
IMEI: 35168007...
=== FIN DEVICE INFO ===

🔐 === INICIO AUTENTICACIÓN ===
Username: PCP0010699_A187518
Backend: http://10.0.2.2:3000

🔑 === TOKENS GUARDADOS EXITOSAMENTE ===
Token: Xal5G2w1CDR1AMe6uElQw...
Username extraído: A187518
Societe extraída: PCP0010699
=== FIN TOKENS GUARDADOS ===

📦 === TOURNÉE CON AUTO-RETRY ===
🔄 Intento 1/2
✅ Token válido encontrado
📡 Tournée response code: 200
✅ Tournée exitosa: 15 paquetes
```

## 🔧 Configuración Avanzada

### Device Info
```kotlin
// Configurar device info
val deviceInfoManager = DeviceInfoManager(context)
deviceInfoManager.logDeviceInfo()

// Reset para testing
val newInstallId = deviceInfoManager.resetInstallIdForTesting()
```

### Token Management
```kotlin
// Configurar validez de tokens (por defecto: 1 hora)
companion object {
    private const val TOKEN_VALIDITY_HOURS = 1L
}

// Forzar expiración para testing
tokenManager.forceTokenExpiry()
```

### Repository State
```kotlin
// Obtener estado completo
val state = repository.getCurrentState()
Log.d(TAG, "Authenticated: ${state.isAuthenticated}")
Log.d(TAG, "Token expira en: ${state.tokenExpiration.minutesUntilExpiry} min")
Log.d(TAG, "Device fingerprint: ${state.getDeviceFingerprint()}")
```

## 🚨 Troubleshooting

### Problemas Comunes

#### ❌ Error de Conexión
```
Error: "Error de conexión: Network timeout"
Solución: Verificar que el backend esté corriendo en http://10.0.2.2:3000
```

#### 🔑 Token Expirado
```
Error: "Tournée failed with code: 401"
Solución: La app maneja esto automáticamente con auto-refresh
```

#### 📱 Device Info Problemático
```
Error: "Android ID problemático, usando fallback"
Solución: Normal en emuladores, la app genera fallback automáticamente
```

### Logs de Debug
```bash
# Filtrar logs de Colis Privé
adb logcat | grep "ColisApp"

# Ver logs de device info
adb logcat | grep "DeviceInfoManager"

# Ver logs de autenticación
adb logcat | grep "AUTH_FLOW"

# Ver logs de API
adb logcat | grep "API_CALL"

# Ver logs de tokens
adb logcat | grep "TOKEN_EVENT"
```

## 🔮 Roadmap

### ✅ Completado
- [x] DeviceInfoManager completo con fingerprint único
- [x] ColisTokenManager con auto-refresh y extracción automática
- [x] ColisRepository con auto-retry logic robusto
- [x] Integración completa de los 3 managers
- [x] Tests unitarios para DeviceInfoManager y TokenManager
- [x] Logs comprehensivos para debugging

### 🚧 En Desarrollo
- [ ] Integración con Mapbox
- [ ] Algoritmos de optimización de rutas
- [ ] Métricas de rendimiento
- [ ] Modo offline

### 📋 Próximas Funcionalidades
- [ ] Dashboard de analytics
- [ ] Notificaciones push
- [ ] Sincronización en background
- [ ] Exportación de datos
- [ ] Integración con GPS

## 🤝 Contribución

### Estándares de Código
- **Kotlin** con coroutines para async operations
- **Jetpack Compose** para UI
- **MVVM** architecture pattern
- **Repository pattern** para data layer
- **StateFlow** para reactive UI

### Convenciones de Naming
```kotlin
// Archivos
DeviceInfoManager.kt          // Manager de device info
ColisTokenManager.kt          // Manager de tokens
ColisRepository.kt            // Repository principal

// Funciones
suspend fun getDeviceInfo()   // Obtener device info
suspend fun getValidToken()   // Obtener token válido
suspend fun authenticate()    // Autenticar usuario

// Variables
private val _authState        // StateFlow privado
val authState                 // StateFlow público
```

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver `LICENSE` para más detalles.

## 📞 Soporte

Para soporte técnico o preguntas:
- **Issues**: Crear issue en GitHub
- **Documentación**: Ver código fuente y comentarios
- **Logs**: Usar logs estructurados para debugging

---

**🚀 ¡La app está lista para usar con tu backend Rust!**

**✅ Todos los 3 prompts han sido implementados completamente:**
1. **DeviceInfoManager** - Fingerprint único por dispositivo
2. **TokenManager** - Auto-refresh y extracción automática
3. **Repository** - Auto-retry logic robusto con integración completa
