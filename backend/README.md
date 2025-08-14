# Delivery Route Optimizer - Backend

Cliente API MVP para Colis Privé en Rust.

## 🚀 Configuración

### 1. Configurar credenciales
```bash
# Copiar el archivo de ejemplo
cp src/config.example.rs src/config.rs

# Editar con tus credenciales reales
nano src/config.rs
```

### 2. Configurar variables en `src/config.rs`:
```rust
pub const COLIS_PRIVE_USERNAME: &str = "tu_usuario_real";
pub const COLIS_PRIVE_PASSWORD: &str = "tu_password_real";
pub const COLIS_PRIVE_SOCIETE: &str = "tu_societe_real";
```

## 🔧 Compilar y ejecutar

```bash
# Compilar
cargo build

# Ejecutar
cargo run
```

## 📁 Estructura del proyecto

```
src/
├── main.rs          # Punto de entrada principal
├── client.rs        # Cliente API de Colis Privé
├── models.rs        # Estructuras de datos
├── utils.rs         # Funciones utilitarias
└── config.rs        # Configuración (crear desde config.example.rs)
```

## ⚠️ Seguridad

- **NUNCA** subas `src/config.rs` a Git
- **NUNCA** subas archivos `.env` a Git
- El archivo `.gitignore` ya está configurado para proteger estos archivos

## 🐛 Troubleshooting

Si tienes problemas de compilación con OpenSSL, el proyecto ya está configurado para usar `rustls-tls` en lugar de `native-tls`.
