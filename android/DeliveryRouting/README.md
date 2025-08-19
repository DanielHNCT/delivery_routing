# 🚚 Delivery Routing - Android MVP

Aplicación Android MVP para gestión de rutas de delivery, **preparada arquitecturalmente para Mapbox** pero sin implementar mapas por ahora.

## 🎯 Objetivo

App funcional para demo del martes con arquitectura lista para agregar Mapbox después sin refactoring masivo.

## ✨ Características

### ✅ Implementado (MVP)
- **Login funcional** con credenciales reales
- **Lista de paquetes** con coordenadas GPS mostradas
- **Detalles de paquete** con información de ubicación
- **FAB preparado** para toggle lista/mapa
- **Arquitectura lista** para Mapbox
- **Utils de ubicación** implementados
- **Material Design 3** con Compose

### 🗺️ Preparado para Mapbox (Futuro)
- Modelos de datos con coordenadas GPS
- Estructura de repositorios para ubicación
- Utils de cálculo de distancias y bounds
- Toggle entre vista lista y mapa
- Placeholder para vista de mapa

## 🏗️ Arquitectura

```
com.deliveryrouting.android/
├── data/
│   ├── api/
│   │   ├── ApiService.kt          # Endpoints de la API
│   │   ├── AuthInterceptor.kt     # Interceptor de autenticación
│   │   └── models/                # Modelos de datos con GPS
│   ├── repository/
│   │   ├── DeliveryRepository.kt  # Lógica de negocio
│   │   └── LocationRepository.kt  # Para futuro Mapbox
│   └── preferences/
│       └── PreferencesManager.kt  # Gestión de tokens
├── presentation/
│   ├── login/                     # Pantalla de login
│   ├── main/                      # Pantalla principal
│   └── common/                    # Componentes comunes
└── utils/
    ├── Constants.kt               # Constantes de la app
    ├── LocationUtils.kt           # Utils de ubicación
    └── Extensions.kt              # Extensiones de Compose
```

## 🚀 Tecnologías

- **Kotlin 100%**
- **Jetpack Compose** (no Views tradicionales)
- **Material Design 3**
- **Retrofit** para API
- **Coroutines** para async
- **MVVM** con ViewModels
- **Repository Pattern**

## 📱 Funcionalidades MVP

### Login
- Campos de usuario y contraseña
- Validación de formulario
- Manejo de errores de API
- Almacenamiento de token JWT

### Lista de Paquetes
- Carga de tournée por código y fecha
- Lista con RecyclerView (Compose LazyColumn)
- Información de coordenadas GPS visible
- Ordenamiento por referencia o distancia
- Chips de estado y acción con colores

### Detalles de Paquete
- Dialog con información completa
- Coordenadas GPS formateadas
- Información del remitente
- Botón preparado para mostrar en mapa

### Preparación para Mapbox
- Toggle entre vista lista y mapa
- FAB para cambiar vista
- Placeholder de mapa
- Estructura de datos GPS lista

## 🔧 Instalación

### Prerrequisitos
- Android Studio Hedgehog o superior
- Android SDK 24+
- Kotlin 2.0.21+

### Pasos
1. Clonar el repositorio
2. Abrir en Android Studio
3. Sincronizar Gradle
4. Ejecutar en dispositivo/emulador

### Configuración
La aplicación está configurada para conectarse a:
```
http://192.168.1.9:3000/
```

Modificar en `ApiService.kt` si es necesario.

## 🗺️ Integración Futura con Mapbox

### Dependencias a agregar
```gradle
implementation 'com.mapbox.maps:android:11.0.0'
implementation 'com.mapbox.navigation:android:2.17.0'
```

### Archivos a modificar
1. **build.gradle.kts** - Agregar dependencias Mapbox
2. **MainScreen.kt** - Implementar `showMapView()`
3. **LocationRepository.kt** - Implementar métodos de mapa
4. **AndroidManifest.xml** - Agregar permisos de ubicación

### Funcionalidades a implementar
- Vista de mapa con MapView
- Marcadores para paquetes con coordenadas
- Cálculo y visualización de rutas
- Optimización de rutas
- Tracking de ubicación del usuario

## 📊 Estructura de Datos

### Package con Coordenadas GPS
```kotlin
data class Package(
    val id: String,
    val locationId: String,
    val reference: String,
    val location: PackageLocation, // ¡CON COORDENADAS!
    val action: PackageAction,
    val status: PackageStatus,
    // ... otros campos
)

data class PackageLocation(
    val hasCoordinates: Boolean,
    val latitude: Double?,
    val longitude: Double?,
    val gpsQualityMeters: String?,
    val formattedAddress: String?,
    val city: String?,
    val postalCode: String?
)
```

### Coordenadas y Bounds
```kotlin
data class LatLng(
    val latitude: Double,
    val longitude: Double
)

data class MapBounds(
    val southwest: LatLng,
    val northeast: LatLng
)
```

## 🎨 UI/UX

### Material Design 3
- Colores del sistema
- Tipografía escalable
- Componentes modernos
- Temas dinámicos

### Componentes Compose
- Cards elevadas
- Chips informativos
- Botones con estados
- Campos de texto con validación
- Indicadores de progreso

### Responsive Design
- Adaptable a diferentes tamaños
- Orientación portrait
- Scroll vertical cuando es necesario

## 🔐 Seguridad

### Autenticación
- Interceptor HTTP para tokens JWT
- Almacenamiento seguro de credenciales
- Manejo de sesiones

### Red
- HTTPS (configurable)
- Timeouts de conexión
- Logging de requests para debug

## 🧪 Testing

### Estructura preparada
- Tests unitarios básicos
- Tests de instrumentación
- Mocks para repositorios

### Ejecutar tests
```bash
./gradlew test                    # Tests unitarios
./gradlew connectedAndroidTest    # Tests de instrumentación
```

## 📝 TODO para Mapbox

### Alta Prioridad
- [ ] Agregar dependencias Mapbox
- [ ] Implementar MapView en MainScreen
- [ ] Agregar marcadores para paquetes
- [ ] Implementar navegación básica

### Media Prioridad
- [ ] Cálculo de rutas optimizadas
- [ ] Tracking de ubicación del usuario
- [ ] Clustering de marcadores
- [ ] Offline maps

### Baja Prioridad
- [ ] Personalización de estilos de mapa
- [ ] Análisis de tráfico en tiempo real
- [ ] Integración con sensores del dispositivo

## 🚀 Ventajas de este Enfoque

1. **No debugging de mapa ahora** - App funcional para demo
2. **Arquitectura preparada** - Sin refactoring masivo
3. **Datos GPS visibles** - Coordenadas en lista
4. **Un solo comando** para agregar mapa
5. **Demo funcional** para martes

## 📞 Soporte

Para preguntas o problemas:
- Revisar logs de la aplicación
- Verificar conectividad de red
- Comprobar configuración de API

## 📄 Licencia

Proyecto interno para demo de delivery routing.

---

**¡Listo para agregar Mapbox cuando sea necesario! 🗺️✨**
