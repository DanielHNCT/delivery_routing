# 🧪 Delivery Route Optimizer - Testing Tool

## **DESCRIPCIÓN**
CLI interactivo en Rust para testear la API de Delivery Route Optimizer. Esta herramienta te permite probar todos los endpoints de forma fácil y visual.

## **🚀 INSTALACIÓN Y USO**

### Compilar
```bash
cargo build --release
```

### Ejecutar
```bash
cargo run
```

### Ejecutar binario compilado
```bash
./target/release/testing-tool
```

## **🎮 FUNCIONALIDADES**

### 1. 🔐 Test Colis Privé Authentication
- **Input seguro** de credenciales (password oculto)
- **Validación** de autenticación
- **Visualización** de tokens y matricules
- **Manejo de errores** con colores

### 2. 📦 Get Tournée Data
- **Credenciales dinámicas** para cada request
- **Selección de fecha** con formato YYYY-MM-DD
- **Matricule específico** para cada chofer
- **Decodificación automática** de respuestas Base64

### 3. 🧪 Test All Endpoints
- **Health checks** automáticos
- **Testing de endpoints** con datos de muestra
- **Validación** de respuestas
- **Reporte completo** de estado

## **🔧 CONFIGURACIÓN**

### URL del Backend
Por defecto se conecta a `http://localhost:3000`

Para cambiar la URL, modifica en `src/api_client.rs`:
```rust
base_url: "http://tu-servidor:puerto".to_string(),
```

### Timeouts
El cliente HTTP tiene timeouts configurados para evitar bloqueos.

## **📋 EJEMPLOS DE USO**

### Testing de Autenticación
```
🚀 Delivery Optimizer API Testing Tool
=====================================
Select an option
  🔐 Test Colis Privé Authentication
  📦 Get Tournée Data
  🧪 Test All Endpoints
  ❌ Exit

> 🔐 Test Colis Privé Authentication

🔐 Testing Colis Privé Authentication
=====================================
Username: PCP0010699_A187518
Password: ********
Societe: PCP0010699

🔄 Testing authentication...
✅ Authentication successful!
🔑 Token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
📋 Matricule: A187518
💬 Message: Autenticación exitosa
```

### Testing de Tournée
```
📦 Testing Tournée Data Retrieval
=================================
Username: PCP0010699_A187518
Password: ********
Societe: PCP0010699
Date (YYYY-MM-DD): 2025-01-20
Matricule: A187518

🔄 Retrieving tournée data...
✅ Tournée data retrieved successfully!
📅 Date: 2025-01-20
📋 Matricule: A187518
📊 Decoded Data:
[Contenido de la tournée...]
```

## **🎨 CARACTERÍSTICAS VISUALES**

- **Colores** para diferentes tipos de mensajes:
  - 🟢 **Verde**: Éxito
  - 🔴 **Rojo**: Errores
  - 🟡 **Amarillo**: Advertencias/Procesando
  - 🔵 **Azul**: Información
  - 🟣 **Magenta**: Títulos

- **Emojis** para mejor UX:
  - 🚀 Inicio
  - 🔐 Autenticación
  - 📦 Datos
  - 🧪 Testing
  - ❌ Salida

## **🔍 TROUBLESHOOTING**

### Error de Conexión
```
❌ Error: HTTP 500: Internal Server Error
```
**Solución**: Verifica que el backend esté corriendo en `localhost:3000`

### Error de Timeout
```
❌ Error: operation timed out
```
**Solución**: Verifica conectividad de red y que el backend responda

### Error de Credenciales
```
❌ Authentication failed!
💬 Message: Credenciales inválidas
```
**Solución**: Verifica username, password y societe

## **📊 LOGS Y DEBUGGING**

El tool muestra información detallada:
- **Status codes** HTTP
- **Respuestas completas** de la API
- **Errores detallados** con contexto
- **Tiempos de respuesta**

## **🚀 DESARROLLO**

### Estructura del Código
```
src/
├── main.rs          # Punto de entrada
├── cli.rs           # Lógica del CLI interactivo
└── api_client.rs    # Cliente HTTP para la API
```

### Agregar Nuevos Endpoints
1. Agrega método en `ApiClient`
2. Agrega opción en el menú principal
3. Implementa la función de testing

### Personalizar Colores
Modifica en `cli.rs` usando la librería `colored`:
```rust
println!("{}", "Tu mensaje".bright_green().bold());
```

## **📦 DEPENDENCIAS**

- **tokio**: Runtime async
- **reqwest**: Cliente HTTP
- **serde**: Serialización JSON
- **dialoguer**: CLI interactivo
- **colored**: Output colorizado
- **anyhow**: Manejo de errores

## **🎯 ROADMAP**

- [ ] **Configuración por archivo** (config.toml)
- [ ] **Testing de múltiples endpoints** en paralelo
- [ ] **Exportación de resultados** a JSON/CSV
- [ ] **Modo batch** para testing automatizado
- [ **Soporte para múltiples backends**

---

**🧪 ¡Comienza a testear tu API de delivery de forma fácil y visual!**
