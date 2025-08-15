#!/bin/bash

# =====================================================
# INSTALADOR DEL SCHEMA - DELIVERY ROUTE OPTIMIZER
# =====================================================

set -e  # Salir si hay algún error

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para imprimir con colores
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Función para verificar si PostgreSQL está instalado
check_postgresql() {
    if ! command -v psql &> /dev/null; then
        print_error "PostgreSQL no está instalado o no está en el PATH"
        print_error "Por favor instala PostgreSQL 13+ antes de continuar"
        exit 1
    fi
    
    PG_VERSION=$(psql --version | grep -oE '[0-9]+\.[0-9]+' | head -1)
    REQUIRED_VERSION="13.0"
    
    if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$PG_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
        print_warning "PostgreSQL $PG_VERSION detectado. Se recomienda versión 13+"
    else
        print_success "PostgreSQL $PG_VERSION detectado ✓"
    fi
}

# Función para verificar extensiones
check_extensions() {
    print_status "Verificando extensiones PostgreSQL..."
    
    # Verificar uuid-ossp
    if ! psql -d "$DB_NAME" -c "SELECT uuid_generate_v4();" &> /dev/null; then
        print_error "Extensión uuid-ossp no disponible"
        print_status "Instalando extensión uuid-ossp..."
        psql -d "$DB_NAME" -c "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";"
    fi
    
    # Verificar postgis
    if ! psql -d "$DB_NAME" -c "SELECT PostGIS_Version();" &> /dev/null; then
        print_warning "Extensión postgis no disponible"
        print_status "Instalando extensión postgis..."
        # Intentar con postgis primero (PostgreSQL 17+)
        if psql -d "$DB_NAME" -c "CREATE EXTENSION IF NOT EXISTS postgis;" &> /dev/null; then
            print_success "Extensión postgis instalada ✓"
        else
            # Fallback a postgis-3 si postgis falla
            print_status "Intentando con postgis-3..."
            psql -d "$DB_NAME" -c "CREATE EXTENSION IF NOT EXISTS \"postgis-3\";"
        fi
    fi
    
    print_success "Extensiones verificadas ✓"
}

# Función para crear base de datos
create_database() {
    if [ -z "$DB_NAME" ]; then
        DB_NAME="delivery_routing"
    fi
    
    print_status "Creando base de datos '$DB_NAME'..."
    
    if psql -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
        print_warning "La base de datos '$DB_NAME' ya existe"
        read -p "¿Deseas continuar con la instalación? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_status "Instalación cancelada"
            exit 0
        fi
    else
        createdb "$DB_NAME"
        print_success "Base de datos '$DB_NAME' creada ✓"
    fi
}

# Función para ejecutar archivo SQL
execute_sql_file() {
    local file="$1"
    local description="$2"
    
    if [ ! -f "$file" ]; then
        print_error "Archivo no encontrado: $file"
        return 1
    fi
    
    print_status "Ejecutando: $description"
    print_status "Archivo: $file"
    
    if psql -d "$DB_NAME" -f "$file"; then
        print_success "$description completado ✓"
    else
        print_error "Error ejecutando $description"
        return 1
    fi
}

# Función para verificar instalación
verify_installation() {
    print_status "Verificando instalación..."
    
    # Verificar tablas principales
    local tables=("companies" "users" "vehicles" "tournees" "packages" "api_integrations" "sync_log")
    local missing_tables=()
    
    for table in "${tables[@]}"; do
        if ! psql -d "$DB_NAME" -c "SELECT 1 FROM $table LIMIT 1;" &> /dev/null; then
            missing_tables+=("$table")
        fi
    done
    
    if [ ${#missing_tables[@]} -eq 0 ]; then
        print_success "Todas las tablas principales están presentes ✓"
    else
        print_error "Faltan las siguientes tablas: ${missing_tables[*]}"
        return 1
    fi
    
    # Verificar views
    local views=("company_dashboard" "driver_performance_summary" "expiring_documents")
    local missing_views=()
    
    for view in "${views[@]}"; do
        if ! psql -d "$DB_NAME" -c "SELECT 1 FROM $view LIMIT 1;" &> /dev/null; then
            missing_views+=("$view")
        fi
    done
    
    if [ ${#missing_views[@]} -eq 0 ]; then
        print_success "Todas las views están presentes ✓"
    else
        print_error "Faltan las siguientes views: ${missing_views[*]}"
        return 1
    fi
    
    # Verificar datos de prueba
    local company_count=$(psql -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM companies;" | xargs)
    if [ "$company_count" -gt 0 ]; then
        print_success "Datos de prueba cargados ✓ ($company_count empresas)"
    else
        print_warning "No se encontraron datos de prueba"
    fi
}

# Función para mostrar información de conexión
show_connection_info() {
    print_success "Instalación completada exitosamente! 🎉"
    echo
    echo "📊 Base de datos: $DB_NAME"
    echo "🔗 Conectar con: psql -d $DB_NAME"
    echo "📁 Schema ubicado en: $(pwd)"
    echo
    echo "🚀 Próximos pasos:"
    echo "1. Configurar variables de entorno en tu aplicación Rust"
    echo "2. Implementar autenticación JWT"
    echo "3. Configurar Row Level Security (RLS)"
    echo "4. Probar las queries de ejemplo incluidas"
    echo "5. Configurar integraciones con APIs externas (Colis Privé, Chronopost)"
    echo "6. Probar las funciones de sincronización de APIs"
    echo
    echo "📚 Documentación: README.md"
    echo "🧪 Datos de prueba incluidos para testing"
}

# Función principal
main() {
    echo "🚚 DELIVERY ROUTE OPTIMIZER - INSTALADOR DE SCHEMA"
    echo "=================================================="
    echo
    
    # Verificar argumentos
    if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        echo "Uso: $0 [NOMBRE_BASE_DATOS]"
        echo "  NOMBRE_BASE_DATOS: Nombre de la base de datos (default: delivery_routing)"
        echo
        echo "Ejemplos:"
        echo "  $0                    # Usa 'delivery_routing'"
        echo "  $0 mi_empresa         # Usa 'mi_empresa'"
        exit 0
    fi
    
    # Configurar nombre de base de datos
    DB_NAME="${1:-delivery_routing}"
    
    print_status "Iniciando instalación del schema..."
    print_status "Base de datos objetivo: $DB_NAME"
    echo
    
    # Verificaciones previas
    check_postgresql
    create_database
    check_extensions
    
    echo
    print_status "Instalando schema en orden..."
    echo
    
    # Ejecutar archivos en orden
    execute_sql_file "01_companies_and_users.sql" "Companies, Users y Vehicles"
    execute_sql_file "02_vehicle_documents_and_damages.sql" "Vehicle Documents, Damages y Tournees"
    execute_sql_file "03_packages_and_analytics.sql" "Packages, Field Data y Analytics"
    execute_sql_file "04_functions_triggers_security.sql" "Functions, Triggers y Security"
    execute_sql_file "05_views_examples_and_data.sql" "Views, Examples y Data de prueba"
    execute_sql_file "06_api_integration_functions.sql" "API Integration Functions"
    
    echo
    verify_installation
    echo
    show_connection_info
}

# Ejecutar función principal
main "$@"
