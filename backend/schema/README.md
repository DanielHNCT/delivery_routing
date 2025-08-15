# 🚚 Schema PostgreSQL - Delivery Route Optimizer

## 📋 Descripción General

Este schema implementa una base de datos completa para una plataforma SaaS multi-tenant de optimización de rutas de delivery. La arquitectura sigue un patrón jerárquico tipo "muñecas rusas" donde cada nivel contiene y gestiona el siguiente.

## 🏗️ Arquitectura Jerárquica

```
Company (Nivel 1)
├── Users + Vehicles (Nivel 2)
│   ├── Vehicle Documents + Vehicle Damages + Tournees (Nivel 3)
│   │   ├── Packages + Driver Field Data (Nivel 4)
│   │   └── Performance Analytics + Notifications (Nivel 5)
```

## 📁 Estructura de Archivos

### 1. `01_companies_and_users.sql`
- **Companies**: Tabla raíz del sistema multi-tenant
- **Users**: Administradores y choferes por empresa
- **Vehicles**: Flota de vehículos por empresa

### 2. `02_vehicle_documents_and_damages.sql`
- **Vehicle Documents**: Gestión de documentos (control técnico, seguro, etc.)
- **Vehicle Damages**: Seguimiento de daños e incidentes
- **Tournees**: Rutas diarias de entrega

### 3. `03_packages_and_analytics.sql`
- **Packages**: Paquetes individuales por tournée
- **Driver Field Data**: Datos crowdsourced por choferes
- **Performance Analytics**: Métricas semanales automáticas
- **Notifications Log**: Sistema de alertas y notificaciones

### 4. `04_functions_triggers_security.sql`
- **Funciones**: Cálculos automáticos y lógica de negocio
- **Triggers**: Automatización de procesos
- **Row Level Security**: Seguridad multi-tenant

### 5. `05_views_examples_and_data.sql`
- **Views**: Dashboards y reportes predefinidos
- **Queries de ejemplo**: Casos de uso comunes
- **Datos de prueba**: Para desarrollo y testing

## 🚀 Instalación

### Prerrequisitos
- PostgreSQL 13+ con extensiones:
  - `uuid-ossp` (para UUIDs)
  - `postgis` (para coordenadas geográficas)

### Pasos de instalación
```bash
# 1. Crear base de datos
createdb delivery_routing

# 2. Ejecutar archivos en orden
psql -d delivery_routing -f 01_companies_and_users.sql
psql -d delivery_routing -f 02_vehicle_documents_and_damages.sql
psql -d delivery_routing -f 03_packages_and_analytics.sql
psql -d delivery_routing -f 04_functions_triggers_security.sql
psql -d delivery_routing -f 05_views_examples_and_data.sql
```

## 🔐 Características de Seguridad

### Row Level Security (RLS)
- Todas las tablas tienen RLS habilitado
- Políticas basadas en `company_id`
- Aislamiento completo entre empresas

### Autenticación
- Sistema de usuarios con roles (admin/driver)
- Máximo 2 admins por empresa
- Contraseñas hasheadas

## ⚡ Funcionalidades Automáticas

### Triggers Implementados
1. **`update_updated_at_column`**: Actualiza `updated_at` automáticamente
2. **`calculate_tournee_distance`**: Calcula distancia total de tournées
3. **`calculate_weekly_performance`**: Genera analytics semanales automáticamente
4. **`update_document_status`**: Actualiza status de documentos según fecha de vencimiento
5. **`create_document_notifications`**: Crea notificaciones automáticas

### Cálculos Automáticos
- Distancia total de tournées
- Métricas de performance semanal
- Status de documentos (valid/expiring_soon/expired)
- Notificaciones de vencimiento (30, 15 días y críticas)

## 📊 Views Disponibles

### 1. `company_dashboard`
Resumen completo de una empresa con métricas agregadas.

### 2. `driver_performance_summary`
Performance consolidada de todos los choferes.

### 3. `expiring_documents`
Documentos próximos a vencer con prioridades.

### 4. `weekly_analytics_summary`
Analytics semanales consolidados por empresa.

## 🔍 Queries de Ejemplo

### Obtener tournée del día para un chofer
```sql
SELECT * FROM tournees 
WHERE driver_id = $1 
AND tournee_date = CURRENT_DATE;
```

### Obtener paquetes con datos de campo
```sql
SELECT p.*, dfd.door_codes, dfd.access_instructions
FROM packages p
LEFT JOIN driver_field_data dfd ON p.delivery_address = dfd.address
WHERE p.tournee_id = $1;
```

### Performance semanal de un chofer
```sql
SELECT * FROM performance_analytics
WHERE driver_id = $1
AND week_start_date >= CURRENT_DATE - INTERVAL '12 weeks';
```

## 🛠️ Funciones Utilitarias

### `get_company_stats(company_uuid)`
Retorna estadísticas completas de una empresa.

### `cleanup_old_data(months_to_keep)`
Limpia datos antiguos para mantenimiento.

## 📱 Casos de Uso Principales

### Para Choferes (App Móvil)
- Ver tournée del día
- Escanear paquetes
- Registrar entregas/fallos con coordenadas GPS
- Agregar datos de campo (códigos de puerta, etc.)
- Reportar daños de vehículo
- Tracking de ubicación en tiempo real
- Gestión de horarios de turno

### Para Administradores
- Dashboard de empresa
- Gestión de usuarios y vehículos
- Seguimiento de documentos
- Reportes de performance
- Gestión de daños e incidentes

### Para el Sistema
- Sincronización con APIs externas (Colis Privé)
- Cálculos automáticos de performance
- Sistema de notificaciones
- Optimización de rutas
- Análisis de condiciones de tráfico y clima
- Tracking GPS en tiempo real
- Push notifications para choferes

## 🔧 Mantenimiento

### Limpieza Automática
```sql
-- Limpiar datos de más de 24 meses
SELECT cleanup_old_data(24);
```

### Monitoreo de Documentos
```sql
-- Ver documentos próximos a vencer
SELECT * FROM expiring_documents;
```

### Performance Analytics
```sql
-- Ver analytics de la semana actual
SELECT * FROM weekly_analytics_summary 
WHERE week_start_date = DATE_TRUNC('week', CURRENT_DATE)::DATE;
```

## 📈 Escalabilidad y Performance

### Multi-Tenant
- Aislamiento completo por empresa
- Índices compuestos optimizados para consultas multi-tenant
- Políticas RLS eficientes

### Performance
- **Índices compuestos multi-tenant**:
  - `(company_id, tournee_date, driver_id)` en tournees
  - `(company_id, delivery_status, delivery_date)` en packages
  - `(company_id, document_status, expiry_date)` en vehicle_documents

- **Particionamiento por fecha**:
  - Tabla `performance_analytics` particionada por mes
  - Mejora significativa en queries de reportes históricos
  - Gestión automática de particiones

- **Materialized Views**:
  - `monthly_company_summary`: Estadísticas mensuales por empresa
  - `driver_ranking_monthly`: Rankings de performance mensual
  - `company_cost_analysis`: Análisis de costos por empresa

- **Funciones de mantenimiento automático**:
  - Cleanup de datos antiguos (6+ meses)
  - Gestión automática de particiones
  - Refresh automático de materialized views

## 🚨 Sistema de Alertas

### Tipos de Notificaciones
- **30 días**: Advertencia temprana
- **15 días**: Advertencia urgente
- **0 días**: Alerta crítica

### Importante
- Los vehículos siguen operando normalmente independientemente del status de documentos
- Sistema de alertas sin bloqueos operativos

## 📱 Nuevos Campos para Funcionalidades Avanzadas

### Tabla PACKAGES
- **`signature_photo`**: Fotos de firma de entrega para evidencia
- **`delivery_coordinates`**: Ubicación exacta de entrega (PostGIS POINT)
- **`delivery_duration_minutes`**: Tiempo de entrega para análisis de eficiencia

### Tabla TOURNEES
- **`route_coordinates`**: Array de coordenadas de la ruta completa
- **`traffic_conditions`**: Condiciones de tráfico del día (JSONB)
- **`weather_conditions`**: Condiciones meteorológicas (JSONB)

### Tabla USERS (Drivers)
- **`device_token`**: Token para push notifications
- **`last_location`**: Última ubicación conocida del chofer (PostGIS POINT)
- **`shift_start_time` / `shift_end_time`**: Horarios de trabajo

### Índices Optimizados
- Índices GIST para campos geográficos (PostGIS)
- Índices GIN para campos JSONB
- Índices compuestos para consultas de performance

## 🔄 Sincronización

### APIs Externas
- Soporte para tracking numbers de Colis Privé
- Campos `external_tracking_number` para integración
- Sistema preparado para múltiples proveedores

## 📋 Próximos Pasos

1. **Implementar en Rust**: Usar SQLx para queries
2. **Configurar RLS**: Implementar `current_setting('app.current_user_id')`
3. **Testing**: Usar datos de prueba incluidos
4. **Monitoreo**: Implementar alertas de performance
5. **Backup**: Configurar respaldos automáticos

## 🚀 Optimizaciones de Performance Implementadas

### Índices Compuestos Multi-Tenant
```sql
-- Tournees
CREATE INDEX idx_tournees_company_date_driver ON tournees(company_id, tournee_date, driver_id);
CREATE INDEX idx_tournees_company_status_date ON tournees(company_id, tournee_status, tournee_date);

-- Packages
CREATE INDEX idx_packages_company_status_date ON packages(company_id, delivery_status, delivery_date);
CREATE INDEX idx_packages_company_tournee_date ON packages(company_id, tournee_id, delivery_date);

-- Vehicle Documents
CREATE INDEX idx_vehicle_documents_company_status_expiry ON vehicle_documents(company_id, document_status, expiry_date);
```

### Particionamiento por Mes
```sql
-- Tabla performance_analytics particionada por mes
CREATE TABLE performance_analytics (
    -- ... campos ...
) PARTITION BY RANGE (week_start_date);

-- Particiones automáticas para 2024-2025
CREATE TABLE performance_analytics_2024_01 PARTITION OF performance_analytics
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
-- ... más particiones ...
```

### Materialized Views para Reportes
```sql
-- Resumen mensual por empresa
SELECT * FROM monthly_company_summary WHERE company_id = $1;

-- Ranking mensual de choferes
SELECT * FROM driver_ranking_monthly WHERE company_id = $1;

-- Análisis de costos mensual
SELECT * FROM company_cost_analysis WHERE company_id = $1;
```

### Funciones de Mantenimiento Automático
```sql
-- Refrescar todas las materialized views
SELECT refresh_all_materialized_views();

-- Gestionar particiones automáticamente
SELECT manage_performance_partitions();

-- Cleanup agresivo de datos antiguos
SELECT * FROM aggressive_cleanup_old_data(6);
```

### Configuración de Cron Jobs Recomendada
```bash
# Diario a las 2:00 AM - Refresh de materialized views
0 2 * * * psql -d delivery_routing -c "SELECT refresh_all_materialized_views();"

# Mensual - Gestión de particiones
0 3 1 * * psql -d delivery_routing -c "SELECT manage_performance_partitions();"

# Semanal - Cleanup de datos antiguos
0 4 * * 0 psql -d delivery_routing -c "SELECT * FROM aggressive_cleanup_old_data(6);"
```

## 🆘 Soporte

Para preguntas o problemas con el schema:

## 🔌 Integraciones con APIs Externas

### Tabla API_INTEGRATIONS
```sql
CREATE TABLE api_integrations (
    integration_id UUID PRIMARY KEY,
    company_id UUID NOT NULL,
    provider_name VARCHAR(100) NOT NULL, -- 'colis_prive', 'chronopost'
    api_credentials JSONB NOT NULL, -- Encriptado
    sync_status ENUM('active', 'error', 'disabled', 'syncing'),
    daily_sync_limit INTEGER DEFAULT 1000,
    sync_frequency_hours INTEGER DEFAULT 24
);
```

### Tabla SYNC_LOG
```sql
CREATE TABLE sync_log (
    sync_id UUID PRIMARY KEY,
    integration_id UUID NOT NULL,
    sync_type VARCHAR(50), -- 'full_sync', 'incremental', 'webhook'
    records_processed INTEGER,
    errors_count INTEGER,
    sync_duration_seconds INTEGER,
    error_details JSONB
);
```

### Funciones Principales
```sql
-- Marcar tournée como sincronizada desde API
SELECT mark_tournee_as_api_synced(
    tournee_uuid, 
    api_integration_uuid, 
    'external_id_123'
);

-- Marcar paquete como sincronizado desde API
SELECT mark_package_as_api_synced(
    package_uuid, 
    api_integration_uuid, 
    'external_pkg_456'
);

-- Crear nueva integración
SELECT create_api_integration(
    company_uuid,
    'colis_prive',
    'Colis Privé',
    '{"api_key": "your_key", "secret": "your_secret"}'
);
```

### Casos de Uso
1. **Sincronización automática** con Colis Privé, Chronopost, etc.
2. **Tracking de origen** para auditoría y debugging
3. **Monitoreo de performance** de APIs externas
4. **Gestión de credenciales** por empresa y proveedor
5. **Webhooks** para sincronización en tiempo real

### Configuración Recomendada
- **Colis Privé**: Sincronización cada 6 horas
- **Chronopost**: Sincronización cada 12 horas
- **Webhooks**: Para actualizaciones en tiempo real
- **Límites diarios**: 1000-2000 registros por proveedor
- Revisar los comentarios en el código SQL
- Verificar las constraints y triggers
- Usar las views para debugging
- Consultar los datos de prueba incluidos

---

**Desarrollado para Delivery Route Optimizer**  
*Plataforma SaaS Multi-Tenant para Optimización de Rutas de Delivery*
