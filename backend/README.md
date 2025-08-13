# 🚚 Delivery Route Optimizer - MVP

Cliente API mínimo en Rust para conectarse a Colis Privé y obtener datos de tournées.

## 🎯 Funcionalidades

- ✅ Autenticación con API de Colis Privé
- ✅ Obtención de datos de tournée específica
- ✅ Decodificación Base64 de respuestas
- ✅ Extracción de información básica de la hoja de ruta
- ✅ Manejo robusto de errores

## 🚀 Instalación y Uso

### 1. Configurar credenciales

Edita `src/main.rs` y reemplaza las credenciales placeholder:

```rust
let username = "tu_usuario_real";
let password = "tu_password_real";
```

### 2. Compilar el proyecto

```bash
cargo build
```

### 3. Ejecutar

```bash
cargo run
```

## 📁 Estructura del Proyecto

```
src/
├── main.rs      # Función principal y demostración
├── client.rs    # Cliente API de Colis Privé
├── models.rs    # Estructuras de datos para requests/responses
└── utils.rs     # Utilidades (Base64, parsing básico)
```

## 🔧 Dependencias

- **reqwest**: Cliente HTTP con soporte JSON
- **serde**: Serialización/deserialización
- **tokio**: Runtime asíncrono
- **anyhow**: Manejo de errores
- **base64**: Decodificación Base64

## 📊 Output Esperado

```
🚚 Delivery Route Optimizer - MVP
=====================================
🔐 Intentando login con usuario: tu_usuario
✅ Login exitoso!
   📋 Matricule: PCP0010699_A187518
   🏢 Societe: PCP0010699

📅 Obteniendo tournée para la fecha: 2025-08-13
✅ Tournée obtenida exitosamente

🔍 Decodificando datos Base64...
✅ Datos decodificados correctamente

📊 Información de la tournée:
🎯 Tournée encontrada: TOURNEE N°A187518
📦 Total de paquetes: NOMBRE DE COLIS TOTAL :  40
⚖️ Peso total: POIDS TOTAL : 63.74 Kg

📋 Datos completos de la tournée:
[contenido decodificado de la hoja de ruta]

🎉 MVP completado exitosamente!
```

## 🚨 Notas Importantes

- **Credenciales**: Debes tener credenciales válidas de Colis Privé
- **Fechas**: La fecha de tournée debe ser válida y accesible
- **Red**: Se requieren conexiones HTTP a los endpoints de Colis Privé

## 🔮 Próximos Pasos

1. **Parser estructurado**: Extraer direcciones y datos de paquetes
2. **Base de datos**: Almacenar datos en PostgreSQL
3. **Optimización**: Implementar algoritmos de optimización de rutas
4. **API REST**: Crear endpoints para aplicaciones móviles

## 🐛 Troubleshooting

### Error de compilación
```bash
cargo clean && cargo build
```

### Error de autenticación
- Verifica que las credenciales sean correctas
- Asegúrate de tener acceso a la API de Colis Privé

### Error de red
- Verifica tu conexión a internet
- Confirma que los endpoints de Colis Privé sean accesibles
