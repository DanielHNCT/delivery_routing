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
- Registrar entregas/fallos
- Agregar datos de campo (códigos de puerta, etc.)
- Reportar daños de vehículo

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

## 📈 Escalabilidad

### Multi-Tenant
- Aislamiento completo por empresa
- Índices optimizados para consultas multi-tenant
- Políticas RLS eficientes

### Performance
- Índices en todas las claves de búsqueda
- Views materializadas para reportes complejos
- Triggers optimizados para cálculos automáticos

## 🚨 Sistema de Alertas

### Tipos de Notificaciones
- **30 días**: Advertencia temprana
- **15 días**: Advertencia urgente
- **0 días**: Alerta crítica

### Importante
- Los vehículos siguen operando normalmente independientemente del status de documentos
- Sistema de alertas sin bloqueos operativos

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

## 🆘 Soporte

Para preguntas o problemas con el schema:
- Revisar los comentarios en el código SQL
- Verificar las constraints y triggers
- Usar las views para debugging
- Consultar los datos de prueba incluidos

---

**Desarrollado para Delivery Route Optimizer**  
*Plataforma SaaS Multi-Tenant para Optimización de Rutas de Delivery*
