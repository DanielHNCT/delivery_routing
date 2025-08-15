#!/bin/bash

# =====================================================
# SCRIPT DE INSTALACIÓN DEL SCHEMA COMPLETO CORREGIDO
# =====================================================
# Este script elimina las tablas existentes y crea el nuevo schema
# con las convenciones estándar (primary keys = 'id')

set -e

echo "🚀 Instalando schema completo corregido para Delivery Routing..."
echo "================================================================"

# Verificar que PostgreSQL esté corriendo
if ! pg_isready -h localhost -p 5432 > /dev/null 2>&1; then
    echo "❌ Error: PostgreSQL no está corriendo en localhost:5432"
    exit 1
fi

# Obtener credenciales del archivo .env
if [ -f "../.env" ]; then
    source ../.env
    echo "✅ Archivo .env encontrado"
else
    echo "❌ Error: Archivo .env no encontrado"
    exit 1
fi

# Verificar que DATABASE_URL esté configurada
if [ -z "$DATABASE_URL" ]; then
    echo "❌ Error: DATABASE_URL no está configurada en .env"
    exit 1
fi

echo "📊 Conectando a la base de datos..."
echo "Base de datos: $DATABASE_URL"

# Función para ejecutar SQL
execute_sql() {
    local sql_file="$1"
    local description="$2"
    
    echo "🔧 $description..."
    psql "$DATABASE_URL" -f "$sql_file" -v ON_ERROR_STOP=1
    echo "✅ $description completado"
}

# Función para eliminar tablas existentes
drop_existing_tables() {
    echo "🗑️  Eliminando tablas existentes..."
    
    psql "$DATABASE_URL" << 'EOF'
-- Deshabilitar triggers temporalmente
SET session_replication_role = replica;

-- Eliminar tablas en orden inverso (por dependencias)
DROP TABLE IF EXISTS sync_log CASCADE;
DROP TABLE IF EXISTS notifications_log CASCADE;
DROP TABLE IF EXISTS performance_analytics CASCADE;
DROP TABLE IF EXISTS driver_field_data CASCADE;
DROP TABLE IF EXISTS packages CASCADE;
DROP TABLE IF EXISTS tournees CASCADE;
DROP TABLE IF EXISTS vehicle_damages CASCADE;
DROP TABLE IF EXISTS vehicle_documents CASCADE;
DROP TABLE IF EXISTS api_integrations CASCADE;
DROP TABLE IF EXISTS vehicles CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS companies CASCADE;

-- Eliminar tipos ENUM
DROP TYPE IF EXISTS user_type CASCADE;
DROP TYPE IF EXISTS user_status CASCADE;
DROP TYPE IF EXISTS vehicle_status CASCADE;
DROP TYPE IF EXISTS sync_status CASCADE;
DROP TYPE IF EXISTS document_type CASCADE;
DROP TYPE IF EXISTS document_status CASCADE;
DROP TYPE IF EXISTS damage_type CASCADE;
DROP TYPE IF EXISTS damage_status CASCADE;
DROP TYPE IF EXISTS tournee_status CASCADE;
DROP TYPE IF EXISTS delivery_status CASCADE;
DROP TYPE IF EXISTS delivery_failure_reason CASCADE;
DROP TYPE IF EXISTS notification_type CASCADE;
DROP TYPE IF EXISTS notification_priority CASCADE;

-- Eliminar funciones
DROP FUNCTION IF EXISTS update_updated_at_column() CASCADE;
DROP FUNCTION IF EXISTS calculate_tournee_distance() CASCADE;
DROP FUNCTION IF EXISTS update_document_status() CASCADE;

-- Restaurar triggers
SET session_replication_role = DEFAULT;
EOF

    echo "✅ Tablas existentes eliminadas"
}

# Función para crear extensiones
create_extensions() {
    echo "🔌 Creando extensiones..."
    
    psql "$DATABASE_URL" << 'EOF'
-- Crear extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "postgis";
EOF

    echo "✅ Extensiones creadas"
}

# Función para verificar la instalación
verify_installation() {
    echo "🔍 Verificando la instalación..."
    
    local table_count=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" | tr -d ' ')
    local enum_count=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM pg_type WHERE typnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public') AND typtype = 'e';" | tr -d ' ')
    
    echo "📊 Tablas creadas: $table_count"
    echo "📊 Tipos ENUM creados: $enum_count"
    
    if [ "$table_count" -ge 12 ] && [ "$enum_count" -ge 10 ]; then
        echo "✅ Instalación verificada exitosamente"
    else
        echo "❌ Error: La instalación no se completó correctamente"
        exit 1
    fi
}

# Función para mostrar resumen
show_summary() {
    echo ""
    echo "🎉 ¡SCHEMA COMPLETO INSTALADO EXITOSAMENTE!"
    echo "============================================="
    echo ""
    echo "📋 Resumen de la instalación:"
    echo "   • Tablas principales: 12"
    echo "   • Tipos ENUM: 10+"
    echo "   • Índices optimizados: 50+"
    echo "   • Triggers automáticos: 15+"
    echo ""
    echo "🏗️  Arquitectura implementada:"
    echo "   • Nivel 1: Companies (tabla raíz)"
    echo "   • Nivel 2: Users, Vehicles, API Integrations"
    echo "   • Nivel 3: Documents, Damages, Tournees"
    echo "   • Nivel 4: Packages, Driver Field Data"
    echo "   • Nivel 5: Analytics, Notifications, Sync Logs"
    echo ""
    echo "🔒 Características de seguridad:"
    echo "   • Multi-tenancy completo"
    echo "   • Soft deletes en todas las tablas"
    echo "   • Constraints de integridad"
    echo "   • Triggers automáticos"
    echo ""
    echo "📱 Funcionalidades avanzadas:"
    echo "   • Soporte para GPS (PostGIS)"
    echo "   • Integración con APIs externas"
    echo "   • Sistema de notificaciones"
    echo "   • Analytics automáticos"
    echo ""
    echo "🚀 Próximos pasos:"
    echo "   1. Ejecutar 'cargo build' para compilar el proyecto"
    echo "   2. Verificar que no hay errores de SQLx"
    echo "   3. Ejecutar 'cargo run' para iniciar el servidor"
    echo ""
}

# Función principal
main() {
    echo "🔄 Iniciando instalación del schema completo..."
    
    # Eliminar tablas existentes
    drop_existing_tables
    
    # Crear extensiones
    create_extensions
    
    # Instalar schema principal
    execute_sql "complete_schema.sql" "Instalando tablas principales"
    
    # Instalar índices y triggers
    execute_sql "indexes_and_triggers.sql" "Instalando índices y triggers"
    
    # Verificar instalación
    verify_installation
    
    # Mostrar resumen
    show_summary
    
    echo "🎯 ¡Schema listo para usar! Ahora puedes ejecutar 'cargo build'"
}

# Ejecutar función principal
main "$@"
