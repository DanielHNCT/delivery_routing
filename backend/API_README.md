# 🚚 Delivery Route Optimizer - API REST

API REST completa para optimización de rutas de entrega con soporte multi-tenant y integración con PostgreSQL + PostGIS.

## 🏗️ Estructura del Proyecto

```
src/
├── main.rs              # Punto de entrada principal
├── lib.rs               # Módulo principal de la API
├── api/                 # Endpoints de la API
│   ├── mod.rs          # Configuración de rutas
│   ├── auth.rs         # Autenticación (login, registro)
│   ├── companies.rs    # Gestión de empresas
│   ├── users.rs        # Gestión de usuarios
│   ├── vehicles.rs     # Gestión de vehículos
│   ├── tournees.rs     # Gestión de tournees
│   └── packages.rs     # Gestión de paquetes
├── middleware/          # Middleware de la API
│   ├── mod.rs          # Exportaciones de middleware
│   ├── auth.rs         # Autenticación JWT
│   └── cors.rs         # Configuración CORS
├── models/              # Modelos de datos
│   ├── mod.rs          # Exportaciones de modelos
│   ├── auth.rs         # Modelos de autenticación
│   ├── company.rs      # Modelo de empresa
│   ├── user.rs         # Modelo de usuario
│   ├── vehicle.rs      # Modelo de vehículo
│   ├── tournee.rs      # Modelo de tournée
│   └── package.rs      # Modelo de paquete
├── database/            # Configuración de base de datos
│   ├── mod.rs          # Exportaciones de base de datos
│   └── connection.rs   # Pool de conexiones PostgreSQL
└── utils/               # Utilidades
    ├── mod.rs          # Exportaciones de utilidades
    ├── validation.rs   # Funciones de validación
    └── errors.rs       # Manejo de errores
```

## 🚀 Características Principales

- ✅ **API REST completa** con Axum
- ✅ **Autenticación JWT** multi-tenant
- ✅ **Base de datos PostgreSQL** con PostGIS
- ✅ **Row Level Security (RLS)** por empresa
- ✅ **Validación de datos** con validator
- ✅ **Manejo de errores** robusto
- ✅ **Middleware CORS** configurado
- ✅ **Logging estructurado** con tracing
- ✅ **Async/await** para todas las operaciones

## 📋 Endpoints Disponibles

### 🔐 Autenticación (Públicos)
- `POST /api/auth/login` - Login de usuario
- `POST /api/auth/register` - Registro de empresa + admin

### 🏢 Empresas (Protegidos)
- `GET /api/companies` - Listar empresas (solo super admin)
- `POST /api/companies` - Crear empresa
- `GET /api/companies/:id` - Obtener empresa
- `PUT /api/companies/:id` - Actualizar empresa

### 👥 Usuarios (Protegidos)
- `GET /api/users` - Listar usuarios de la empresa
- `POST /api/users` - Crear usuario (admin only)
- `GET /api/users/:id` - Obtener usuario
- `PUT /api/users/:id` - Actualizar usuario
- `DELETE /api/users/:id` - Eliminar usuario (soft delete)

### 🚗 Vehículos (Protegidos)
- `GET /api/vehicles` - Listar vehículos de la empresa
- `POST /api/vehicles` - Crear vehículo
- `GET /api/vehicles/:id` - Obtener vehículo
- `PUT /api/vehicles/:id` - Actualizar vehículo

### 🛣️ Tournees (Protegidos)
- `GET /api/tournees` - Listar tournees de la empresa
- `POST /api/tournees` - Crear tournée
- `GET /api/tournees/:id` - Obtener tournée
- `PUT /api/tournees/:id` - Actualizar tournée
- `GET /api/tournees/:id/packages` - Paquetes de la tournée

### 📦 Paquetes (Protegidos)
- `GET /api/packages` - Listar paquetes de la empresa
- `POST /api/packages` - Crear paquete
- `GET /api/packages/:id` - Obtener paquete
- `PUT /api/packages/:id` - Actualizar paquete

### 🏥 Health Check (Público)
- `GET /health` - Estado del servidor

## 🔧 Configuración

### 1. Variables de Entorno

Crear archivo `.env` basado en `env.example`:

```bash
# Base de datos
DATABASE_URL=postgresql://username:password@localhost/delivery_routing

# JWT
JWT_SECRET=your-super-secret-jwt-key-here

# Servidor
PORT=3000

# Logging
RUST_LOG=debug

# Colis Privé (mantener configuración existente)
COLIS_PRIVE_USERNAME=tu_usuario_aqui
COLIS_PRIVE_PASSWORD=tu_password_aqui
COLIS_PRIVE_SOCIETE=tu_societe_aqui
```

### 2. Base de Datos

Asegurarse de que PostgreSQL esté ejecutándose y la base de datos `delivery_routing` exista:

```sql
CREATE DATABASE delivery_routing;
```

### 3. Dependencias

Instalar dependencias:

```bash
cargo build
```

## 🚀 Ejecución

### Desarrollo

```bash
# Cargar variables de entorno
source .env

# Ejecutar en modo desarrollo
cargo run
```

### Producción

```bash
# Compilar release
cargo build --release

# Ejecutar
./target/release/delivery-optimizer
```

## 🔐 Autenticación

### Login

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@empresa.com",
    "password": "password123"
  }'
```

### Uso del Token

```bash
curl -X GET http://localhost:3000/api/users \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## 📊 Modelos de Datos

### Usuario
- `id`: UUID único
- `company_id`: ID de la empresa
- `username`: Nombre de usuario
- `email`: Email del usuario
- `user_type`: Admin o Driver
- `is_active`: Estado activo/inactivo

### Empresa
- `id`: UUID único
- `name`: Nombre de la empresa
- `address`: Dirección
- `city`: Ciudad
- `postal_code`: Código postal
- `country`: País

### Vehículo
- `id`: UUID único
- `company_id`: ID de la empresa
- `name`: Nombre del vehículo
- `license_plate`: Matrícula
- `vehicle_type`: Tipo de vehículo
- `status`: Estado (Active, Maintenance, OutOfService, Retired)

### Tournée
- `id`: UUID único
- `company_id`: ID de la empresa
- `name`: Nombre de la tournée
- `driver_id`: ID del conductor
- `vehicle_id`: ID del vehículo
- `planned_date`: Fecha planificada
- `status`: Estado (Planned, InProgress, Completed, Cancelled)

### Paquete
- `id`: UUID único
- `company_id`: ID de la empresa
- `tournee_id`: ID de la tournée
- `tracking_number`: Número de seguimiento
- `recipient_name`: Nombre del destinatario
- `status`: Estado (Pending, InTransit, Delivered, Failed, Returned)

## 🛡️ Seguridad

### Row Level Security (RLS)
- Todos los endpoints filtran por `company_id`
- Usuarios solo pueden acceder a datos de su empresa
- Middleware de autenticación valida JWT en cada request

### Validación
- Validación de entrada con `validator`
- Sanitización de datos
- Validación de UUIDs y formatos

### Autenticación
- JWT tokens con expiración de 24 horas
- Hash de passwords con bcrypt
- Verificación de usuario activo

## 🧪 Testing

### Tests Unitarios

```bash
# Ejecutar todos los tests
cargo test

# Tests específicos
cargo test test_database_connection
cargo test test_authentication
```

### Tests de Integración

```bash
# Tests de endpoints
cargo test --test api_tests
```

## 📝 Logs

La API utiliza `tracing` para logging estructurado:

```bash
# Nivel de log configurable
RUST_LOG=debug cargo run
RUST_LOG=info cargo run
RUST_LOG=warn cargo run
```

## 🔄 Migraciones

Para ejecutar migraciones de la base de datos:

```bash
# Crear migración
sqlx migrate add create_initial_tables

# Ejecutar migraciones
sqlx migrate run

# Revertir migración
sqlx migrate revert
```

## 🚨 Manejo de Errores

La API utiliza `thiserror` para manejo de errores tipado:

- **400 Bad Request**: Validación fallida
- **401 Unauthorized**: Token inválido o expirado
- **403 Forbidden**: Sin permisos para el recurso
- **404 Not Found**: Recurso no encontrado
- **409 Conflict**: Conflicto de datos
- **500 Internal Server Error**: Error interno del servidor

## 🔗 Integración con Colis Privé

La API mantiene la funcionalidad existente de integración con Colis Privé:

- Login y autenticación
- Obtención de tournees
- Dashboard info
- Pilot access

## 📈 Próximos Pasos

- [ ] Implementar tests completos
- [ ] Agregar documentación OpenAPI/Swagger
- [ ] Implementar rate limiting
- [ ] Agregar métricas y monitoreo
- [ ] Implementar cache con Redis
- [ ] Agregar WebSocket para updates en tiempo real
- [ ] Implementar optimización de rutas con algoritmos genéticos

## 🤝 Contribución

1. Fork el proyecto
2. Crear feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push al branch (`git push origin feature/AmazingFeature`)
5. Abrir Pull Request

## 📄 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles.

## 🆘 Soporte

Para soporte técnico o preguntas:

- Crear un issue en GitHub
- Contactar al equipo de desarrollo
- Revisar la documentación de la API

---

**🚚 Delivery Route Optimizer API v1.0.0**
