# 🚛 Delivery Route Optimizer

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Android](https://img.shields.io/badge/Android-3DDC84?style=for-the-badge&logo=android&logoColor=white)
![Mapbox](https://img.shields.io/badge/Mapbox-000000?style=for-the-badge&logo=mapbox&logoColor=white)
![Status](https://img.shields.io/badge/Status-MVP%20Development-orange?style=for-the-badge)

**Sistema de Optimización de Rutas de Entrega con Visualización Geográfica**

</div>

---

## 📋 **DESCRIPCIÓN DEL PROYECTO**

### **🎯 Objetivo**
Desarrollar una **aplicación móvil Android** para optimización de rutas de entrega que reemplace el sistema actual de scanner lento y proporcione visualización geográfica en tiempo real usando Mapbox.

### **❌ Problema a Resolver**
- **Scanner actual:** Lento y poco eficiente
- **Visualización:** Solo muestra nombres y números en texto
- **Ubicación:** Sin referencia geográfica de paquetes
- **Productividad:** Pérdida de tiempo en planificación de rutas

### **✅ Solución Implementada**
- **App Android nativa** con interfaz moderna e intuitiva
- **Mapa interactivo** usando Mapbox para visualización geográfica
- **Backend API Rust** como intermediario inteligente con Colis Privé
- **Sincronización en tiempo real** de datos de entrega

---

## 🏗️ **ARQUITECTURA DEL PROYECTO**

```
delivery_routing/
├── 📖 README.md                   # Este archivo - Documentación principal
├── 📚 docs/                       # Documentación completa del proyecto
│   ├── IMPLEMENTATION_LOGS.md     # 🚛 Logs generales del proyecto
│   ├── PROJECT_LOGS_INDEX.md      # 📋 Índice de toda la documentación
│   ├── backend/                   # 🚀 Logs específicos del backend
│   ├── android/                   # 📱 Logs específicos de Android
│   └── ios/                       # 🍎 Logs específicos de iOS
├── 🚀 backend/                    # API Rust intermediaria
├── 📱 android/                    # Aplicación Android nativa
└── 🍎 ios/                        # Aplicación iOS nativa
```

---

## 🛠️ **STACK TECNOLÓGICO**

### **🚀 Backend API (Rust)**
- **Framework:** Axum 0.7
- **Base de Datos:** PostgreSQL + SQLx
- **HTTP Client:** Reqwest
- **Serialización:** Serde + Serde JSON
- **Runtime:** Tokio

### **📱 Aplicación Android**
- **Lenguaje:** Kotlin
- **UI Framework:** Jetpack Compose
- **Arquitectura:** MVVM + Clean Architecture
- **Networking:** Retrofit + OkHttp
- **Base de Datos:** Room Database

### **🗺️ Integración de Mapas**
- **Proveedor:** Mapbox
- **Geolocalización:** Google Play Services
- **Routing:** Mapbox Directions API
- **Offline:** Mapbox Offline

---

## 🗺️ **ROADMAP DEL PROYECTO**

| **Fase** | **Estado** | **Progreso** | **Descripción** |
|----------|------------|---------------|-----------------|
| **🚀 Backend API** | ✅ **COMPLETADO** | 100% | API Rust intermediaria con Colis Privé |
| **📱 App Android** | 🚧 **EN DESARROLLO** | 25% | Estructura base y planificación |
| **🗺️ Visualización Mapa** | 📋 **PLANIFICADO** | 0% | Integración Mapbox y geolocalización |
| **⚡ Optimización Rutas** | 🔮 **FUTURO** | 0% | Algoritmos de optimización avanzada |

---

## 🚀 **INICIO RÁPIDO**

### **📋 Prerrequisitos**
- Rust 1.70+
- PostgreSQL 13+
- Android Studio (para desarrollo Android)
- Xcode (para desarrollo iOS)

### **🔧 Instalación del Backend**
```bash
# Clonar el repositorio
git clone https://github.com/username/delivery_routing.git
cd delivery_routing

# Configurar base de datos
cd backend
cp env.example .env
# Editar .env con tus credenciales

# Instalar dependencias y ejecutar
cargo install
cargo run
```

### **📱 Desarrollo Android**
```bash
cd android
# Abrir en Android Studio
# Sincronizar proyecto
# Ejecutar en dispositivo/emulador
```

---

## 📚 **DOCUMENTACIÓN COMPLETA**

### **📖 Guías de Implementación**
- **[🚛 Logs Generales del Proyecto](./docs/IMPLEMENTATION_LOGS.md)** - Visión completa del proyecto
- **[📋 Índice de Documentación](./docs/PROJECT_LOGS_INDEX.md)** - Navegación por toda la documentación

### **🔧 Logs por Módulo**
- **[🚀 Backend API](./docs/backend/IMPLEMENTATION_LOGS.md)** - Logs específicos del backend
- **[📱 Android App](./docs/android/IMPLEMENTATION_LOGS.md)** - Logs específicos de Android
- **[🍎 iOS App](./docs/ios/IMPLEMENTATION_LOGS.md)** - Logs específicos de iOS

---

## 🎯 **OBJETIVOS INMEDIATOS**

### **📱 MVP para Martes**
- [ ] **App Android básica** funcional
- [ ] **Sincronización** con Colis Privé
- [ ] **Visualización básica** en mapa
- [ ] **Interfaz de usuario** intuitiva

### **🗺️ Semana Siguiente**
- [ ] **Integración Mapbox** completa
- [ ] **Geocodificación** de direcciones
- [ ] **Marcadores** de paquetes en mapa
- [ ] **Interacción táctil** con marcadores

---

## 🤝 **CONTRIBUCIÓN**

### **📋 Cómo Contribuir**
1. Fork del repositorio
2. Crear rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit de tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abrir Pull Request

### **📝 Estándares de Código**
- **Rust:** Seguir las convenciones de Rust
- **Android:** Seguir las guías de Google para Android
- **iOS:** Seguir las guías de Apple para iOS
- **Documentación:** Mantener logs actualizados en `docs/`

---

## 📞 **CONTACTO Y SOPORTE**

### **👥 Equipo de Desarrollo**
- **Backend API:** [backend@proyecto.com](mailto:backend@proyecto.com)
- **Android App:** [android@proyecto.com](mailto:android@proyecto.com)
- **iOS App:** [ios@proyecto.com](mailto:ios@proyecto.com)
- **Proyecto General:** [proyecto@delivery-optimizer.com](mailto:proyecto@delivery-optimizer.com)

### **💬 Canales de Comunicación**
- **Slack:** #delivery-route-optimizer
- **WhatsApp:** +34 XXX XXX XXX
- **Jira:** [Proyecto DRO](https://jira.company.com/projects/DRO)

---

## 📄 **LICENCIA**

Este proyecto está bajo la Licencia MIT. Ver el archivo [LICENSE](LICENSE) para más detalles.

---

<div align="center">

**🚛 Delivery Route Optimizer**  
**📱 Reemplazando scanners lentos con mapas inteligentes**  
**🎯 MVP para martes con visualización geográfica**

---

*Desarrollado con ❤️ por el equipo de Delivery Route Optimizer*

</div>
