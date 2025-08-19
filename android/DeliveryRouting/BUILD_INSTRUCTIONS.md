# 🚀 Instrucciones de Build - Delivery Routing

## 📋 Prerrequisitos

### Android Studio
- **Versión**: Android Studio Hedgehog (2023.2.1) o superior
- **Gradle**: 8.10.1+
- **Kotlin**: 2.0.21+

### SDK
- **Compile SDK**: 35
- **Target SDK**: 35
- **Min SDK**: 24
- **Build Tools**: 35.0.0+

## 🔧 Configuración del Proyecto

### 1. Sincronizar Gradle
```bash
# En Android Studio: File -> Sync Project with Gradle Files
# O en terminal:
./gradlew clean
./gradlew build
```

### 2. Verificar Dependencias
Asegurarse de que todas las dependencias estén resueltas:
- androidx.core:core-ktx:1.16.0
- androidx.lifecycle:lifecycle-viewmodel-compose:2.7.0
- androidx.navigation:navigation-compose:2.7.6
- retrofit:2.9.0
- okhttp:4.12.0

### 3. Configuración de Red
La app está configurada para:
```
Base URL: http://192.168.1.9:3000/
```

**⚠️ IMPORTANTE**: Para producción, cambiar a HTTPS y actualizar en:
- `ApiService.kt`
- `Constants.kt`

## 📱 Build y Deployment

### Build de Debug
```bash
./gradlew assembleDebug
```

### Build de Release
```bash
./gradlew assembleRelease
```

### APK de Debug
```bash
./gradlew assembleDebug
# APK en: app/build/outputs/apk/debug/app-debug.apk
```

### Bundle de Release
```bash
./gradlew bundleRelease
# Bundle en: app/build/outputs/bundle/release/app-release.aab
```

## 🧪 Testing

### Tests Unitarios
```bash
./gradlew test
```

### Tests de Instrumentación
```bash
./gradlew connectedAndroidTest
```

### Tests Completos
```bash
./gradlew check
```

## 🚨 Solución de Problemas

### Error de Compilación
```bash
# Limpiar y rebuild
./gradlew clean
./gradlew build
```

### Dependencias no Resueltas
```bash
# Invalidar caches en Android Studio
File -> Invalidate Caches and Restart
```

### Problemas de Red
- Verificar que el dispositivo tenga acceso a `192.168.1.101:3000`
- Comprobar firewall y configuración de red
- Usar `adb reverse` para desarrollo local

### Permisos
La app requiere:
- `INTERNET`
- `ACCESS_NETWORK_STATE`

## 📊 Configuración de Build Variants

### Debug
- `debuggable = true`
- `minifyEnabled = false`
- Logging completo habilitado

### Release
- `debuggable = false`
- `minifyEnabled = true`
- ProGuard habilitado

## 🔐 Configuración de Seguridad

### Debug
- `usesCleartextTraffic = true` (para desarrollo local)
- Logging de requests HTTP habilitado

### Release
- `usesCleartextTraffic = false`
- Logging deshabilitado
- ProGuard rules aplicadas

## 📱 Instalación en Dispositivo

### 1. Habilitar Depuración USB
- Configuración -> Acerca del teléfono
- Tocar 7 veces en "Número de compilación"
- Configuración -> Opciones de desarrollador -> Depuración USB

### 2. Conectar Dispositivo
```bash
adb devices
# Debe mostrar el dispositivo conectado
```

### 3. Instalar APK
```bash
adb install app/build/outputs/apk/debug/app-debug.apk
```

## 🗺️ Preparación para Mapbox

### Dependencias a Agregar
```gradle
// En app/build.gradle.kts
dependencies {
    // Mapbox Maps
    implementation 'com.mapbox.maps:android:11.0.0'
    
    // Mapbox Navigation
    implementation 'com.mapbox.navigation:android:2.17.0'
    
    // Mapbox Search
    implementation 'com.mapbox.search:android:2.0.0'
}
```

### Permisos a Agregar
```xml
<!-- En AndroidManifest.xml -->
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
<uses-permission android:name="android.permission.FOREGROUND_SERVICE" />
```

### Configuración de API Key
```kotlin
// En DeliveryRoutingApplication.kt
MapboxOptions.setAccessToken("tu_api_key_aqui")
```

## 📈 Métricas de Build

### Tiempo de Build
- **Clean Build**: ~2-3 minutos
- **Incremental Build**: ~30-60 segundos
- **APK Size**: ~15-20 MB (debug)

### Optimizaciones
- R8/ProGuard habilitado para release
- Compresión de recursos
- D8 compiler optimizado

## 🎯 Checklist de Build

### Antes del Build
- [ ] Todas las dependencias sincronizadas
- [ ] Tests pasando
- [ ] Configuración de red correcta
- [ ] Permisos configurados

### Durante el Build
- [ ] Sin errores de compilación
- [ ] APK generado correctamente
- [ ] Tamaño de APK razonable
- [ ] Logs de build limpios

### Después del Build
- [ ] APK instalado en dispositivo
- [ ] App inicia correctamente
- [ ] Login funciona
- [ ] Lista de paquetes carga
- [ ] Coordenadas GPS visibles

## 🚀 Deployment

### Para Demo
1. Build debug APK
2. Instalar en dispositivo de demo
3. Verificar conectividad de red
4. Probar funcionalidades principales

### Para Producción
1. Build release bundle
2. Firmar con keystore de producción
3. Subir a Google Play Console
4. Configurar rollout gradual

---

**¡La aplicación está lista para el demo del martes! 🎉**

**Para agregar Mapbox después, solo seguir las instrucciones del README principal.**
