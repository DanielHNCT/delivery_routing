# 🚚 Instrucciones de Uso - Delivery Route Optimizer MVP

## 🎯 ¿Qué hace este proyecto?

Este es un cliente API mínimo en Rust que se conecta a la API de Colis Privé para:
1. **Autenticarse** con credenciales de usuario
2. **Obtener datos** de una tournée específica
3. **Decodificar** la respuesta Base64
4. **Mostrar** la información de la hoja de ruta

## 🚀 Cómo usar el proyecto

### Paso 1: Configurar credenciales

Edita el archivo `src/config.rs` y reemplaza las credenciales placeholder:

```rust
pub const COLIS_PRIVE_USERNAME: &str = "tu_usuario_real_de_colis_prive";
pub const COLIS_PRIVE_PASSWORD: &str = "tu_password_real_de_colis_prive";
pub const TOURNEE_DATE: &str = "2025-08-13"; // Cambia la fecha si es necesario
```

### Paso 2: Compilar el proyecto

```bash
cargo build
```

### Paso 3: Ejecutar

```bash
cargo run
```

## 📁 Estructura del proyecto

```
backend/
├── Cargo.toml              # Dependencias del proyecto
├── src/
│   ├── main.rs             # Función principal
│   ├── config.rs           # Configuración (credenciales)
│   ├── client.rs           # Cliente API de Colis Privé
│   ├── models.rs           # Estructuras de datos
│   └── utils.rs            # Utilidades (Base64, parsing)
├── README.md               # Documentación general
├── INSTRUCCIONES.md        # Este archivo
├── config.example.rs       # Ejemplo de configuración
└── run_example.sh          # Script de prueba
```

## 🔧 Dependencias técnicas

- **Rust**: Lenguaje de programación
- **reqwest**: Cliente HTTP con soporte JSON y rustls
- **serde**: Serialización/deserialización de datos
- **tokio**: Runtime asíncrono
- **anyhow**: Manejo de errores
- **base64**: Decodificación Base64

## 📊 Output esperado

Cuando ejecutes con credenciales válidas, verás:

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
[contenido completo de la hoja de ruta]

🎉 MVP completado exitosamente!
```

## 🚨 Solución de problemas

### Error: "Credenciales no configuradas"
- Edita `src/config.rs` y reemplaza las credenciales placeholder

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

## 🔮 Próximos pasos del proyecto

Una vez que este MVP funcione, implementaremos:

1. **Parser estructurado**: Extraer direcciones y datos de paquetes en estructuras
2. **Base de datos**: Almacenar datos en PostgreSQL
3. **Optimización**: Implementar algoritmos de optimización de rutas
4. **API REST**: Crear endpoints para aplicaciones móviles

## 📞 Soporte

Si tienes problemas:
1. Verifica que Rust esté instalado: `rustc --version`
2. Verifica que las credenciales sean correctas
3. Revisa los mensajes de error para más detalles

## 🎉 ¡Listo para usar!

Con estas instrucciones deberías poder ejecutar el MVP exitosamente. ¡Buena suerte con tu optimizador de rutas de delivery!
