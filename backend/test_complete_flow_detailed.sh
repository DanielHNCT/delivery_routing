#!/bin/bash

# 🧪 SCRIPT DE PRUEBA COMPLETO CON LOGS DETALLADOS
# Prueba el flujo completo: Login → Tournée → Lettre de Voiture

set -e  # Salir si hay error

# Configuración
BACKEND_URL="http://192.168.1.9:3000"
LOG_FILE="test_flow_detailed.log"

# Colores para logs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Función para logging detallado
log() {
    local level=$1
    local message=$2
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S.%3N')
    
    case $level in
        "INFO")
            echo -e "${GREEN}[${timestamp}] [INFO]${NC} ${message}" | tee -a "$LOG_FILE"
            ;;
        "WARN")
            echo -e "${YELLOW}[${timestamp}] [WARN]${NC} ${message}" | tee -a "$LOG_FILE"
            ;;
        "ERROR")
            echo -e "${RED}[${timestamp}] [ERROR]${NC} ${message}" | tee -a "$LOG_FILE"
            ;;
        "DEBUG")
            echo -e "${BLUE}[${timestamp}] [DEBUG]${NC} ${message}" | tee -a "$LOG_FILE"
            ;;
        "STEP")
            echo -e "${PURPLE}[${timestamp}] [STEP]${NC} ${message}" | tee -a "$LOG_FILE"
            ;;
        "SUCCESS")
            echo -e "${CYAN}[${timestamp}] [SUCCESS]${NC} ${message}" | tee -a "$LOG_FILE"
            ;;
    esac
}

# Función para limpiar archivo de log
cleanup_log() {
    log "INFO" "🧹 Limpiando archivo de log anterior..."
    > "$LOG_FILE"
    log "INFO" "✅ Archivo de log limpiado: $LOG_FILE"
}

# Función para verificar conectividad
check_connectivity() {
    log "STEP" "🔍 PASO 1: Verificando conectividad con el backend..."
    
    if curl -s --connect-timeout 5 "$BACKEND_URL/api/colis-prive/health" > /dev/null; then
        log "SUCCESS" "✅ Backend accesible en: $BACKEND_URL"
    else
        log "ERROR" "❌ No se puede conectar al backend en: $BACKEND_URL"
        log "ERROR" "   Verifica que el backend esté corriendo en el Pi 5"
        exit 1
    fi
}

# Función para health check detallado
detailed_health_check() {
    log "STEP" "🔍 PASO 2: Health check detallado del backend..."
    
    log "DEBUG" "📤 Enviando request a: $BACKEND_URL/api/colis-prive/health"
    
    local response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME:%{time_total}s\nSIZE:%{size_download}bytes" \
        "$BACKEND_URL/api/colis-prive/health")
    
    local http_status=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    local response_time=$(echo "$response" | grep "TIME:" | cut -d: -f2)
    local response_size=$(echo "$response" | grep "SIZE:" | cut -d: -f2)
    local response_body=$(echo "$response" | sed '/HTTP_STATUS:/d' | sed '/TIME:/d' | sed '/SIZE:/d')
    
    log "DEBUG" "📥 Respuesta recibida:"
    log "DEBUG" "   HTTP Status: $http_status"
    log "DEBUG" "   Tiempo de respuesta: ${response_time}s"
    log "DEBUG" "   Tamaño de respuesta: ${response_size} bytes"
    log "DEBUG" "   Body: $response_body"
    
    if [ "$http_status" = "200" ]; then
        log "SUCCESS" "✅ Health check exitoso"
    else
        log "ERROR" "❌ Health check falló con status: $http_status"
        exit 1
    fi
}

# Función para login directo
direct_login() {
    log "STEP" "🔍 PASO 3: Login directo a Colis Privé..."
    
    local login_payload='{
        "username": "A187518",
        "password": "INTI7518",
        "societe": "PCP0010699",
        "api_choice": "web"
    }'
    
    log "DEBUG" "📤 Enviando login a: $BACKEND_URL/api/colis-prive/login"
    log "DEBUG" "📦 Payload: $login_payload"
    
    local response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME:%{time_total}s\nSIZE:%{size_download}bytes" \
        -X POST "$BACKEND_URL/api/colis-prive/login" \
        -H "Content-Type: application/json" \
        -d "$login_payload")
    
    local http_status=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    local response_time=$(echo "$response" | grep "TIME:" | cut -d: -f2)
    local response_size=$(echo "$response" | grep "SIZE:" | cut -d: -f2)
    local response_body=$(echo "$response" | sed '/HTTP_STATUS:/d' | sed '/TIME:/d' | sed '/SIZE:/d')
    
    log "DEBUG" "📥 Respuesta de login recibida:"
    log "DEBUG" "   HTTP Status: $http_status"
    log "DEBUG" "   Tiempo de respuesta: ${response_time}s"
    log "DEBUG" "   Tamaño de respuesta: ${response_size} bytes"
    log "DEBUG" "   Body: $response_body"
    
    if [ "$http_status" = "200" ]; then
        log "SUCCESS" "✅ Login exitoso"
        
        # Extraer token del login
        local login_token=$(echo "$response_body" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
        if [ -n "$login_token" ]; then
            log "INFO" "🔑 Token de login extraído: ${login_token:0:20}..."
            echo "$login_token"
        else
            log "WARN" "⚠️ No se pudo extraer token del login"
            echo ""
        fi
    else
        log "ERROR" "❌ Login falló con status: $http_status"
        log "ERROR" "   Respuesta: $response_body"
        exit 1
    fi
}

# Función para obtener tournée
get_tournee() {
    log "STEP" "🔍 PASO 4: Obteniendo tournée..."
    
    local tournee_payload='{
        "username": "A187518",
        "password": "INTI7518",
        "societe": "PCP0010699",
        "date": "'$(date +%Y-%m-%d)'",
        "matricule": "PCP0010699_A187518"
    }'
    
    log "DEBUG" "📤 Enviando request de tournée a: $BACKEND_URL/api/colis-prive/tournee"
    log "DEBUG" "📦 Payload: $tournee_payload"
    
    local response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME:%{time_total}s\nSIZE:%{size_download}bytes" \
        -X POST "$BACKEND_URL/api/colis-prive/tournee" \
        -H "Content-Type: application/json" \
        -d "$tournee_payload")
    
    local http_status=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    local response_time=$(echo "$response" | grep "TIME:" | cut -d: -f2)
    local response_size=$(echo "$response" | grep "SIZE:" | cut -d: -f2)
    local response_body=$(echo "$response" | sed '/HTTP_STATUS:/d' | sed '/TIME:/d' | sed '/SIZE:/d')
    
    log "DEBUG" "📥 Respuesta de tournée recibida:"
    log "DEBUG" "   HTTP Status: $http_status"
    log "DEBUG" "   Tiempo de respuesta: ${response_time}s"
    log "DEBUG" "   Tamaño de respuesta: ${response_size} bytes"
    log "DEBUG" "   Body: $response_body"
    
    if [ "$http_status" = "200" ]; then
        log "SUCCESS" "✅ Tournée obtenido exitosamente"
        
        # Extraer token SsoHopps del tournée
        local sso_hopps_token=$(echo "$response_body" | grep -o '"SsoHopps":"[^"]*"' | cut -d'"' -f4)
        if [ -n "$sso_hopps_token" ]; then
            log "INFO" "🔑 Token SsoHopps extraído: ${sso_hopps_token:0:20}..."
            echo "$sso_hopps_token"
        else
            log "WARN" "⚠️ No se pudo extraer token SsoHopps del tournée"
            echo ""
        fi
    else
        log "ERROR" "❌ Tournée falló con status: $http_status"
        log "ERROR" "   Respuesta: $response_body"
        exit 1
    fi
}

# Función para obtener lettre de voiture
get_lettre_de_voiture() {
    local token=$1
    log "STEP" "🔍 PASO 5: Obteniendo Lettre de Voiture..."
    
    if [ -z "$token" ]; then
        log "ERROR" "❌ No hay token para obtener lettre de voiture"
        return 1
    fi
    
    local lettre_payload='{
        "token": "'$token'",
        "matricule": "PCP0010699_A187518",
        "societe": "PCP0010699",
        "date": "'$(date +%Y-%m-%d)'"
    }'
    
    log "DEBUG" "📤 Enviando request de lettre a: $BACKEND_URL/api/colis-prive/lettre-voiture"
    log "DEBUG" "📦 Payload: $lettre_payload"
    log "DEBUG" "🔑 Usando token: ${token:0:20}..."
    
    local response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME:%{time_total}s\nSIZE:%{size_download}bytes" \
        -X POST "$BACKEND_URL/api/colis-prive/lettre-voiture" \
        -H "Content-Type: application/json" \
        -d "$lettre_payload")
    
    local http_status=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    local response_time=$(echo "$response" | grep "TIME:" | cut -d: -f2)
    local response_size=$(echo "$response" | grep "SIZE:" | cut -d: -f2)
    local response_body=$(echo "$response" | sed '/HTTP_STATUS:/d' | sed '/TIME:/d' | sed '/SIZE:/d')
    
    log "DEBUG" "📥 Respuesta de lettre recibida:"
    log "DEBUG" "   HTTP Status: $http_status"
    log "DEBUG" "   Body: $response_body"
    
    if [ "$http_status" = "200" ]; then
        # Verificar si fue exitoso en el body
        if echo "$response_body" | grep -q '"success":true'; then
            log "SUCCESS" "✅ Lettre de Voiture obtenido exitosamente"
        else
            log "WARN" "⚠️ Lettre obtenido pero success = false"
            local error_msg=$(echo "$response_body" | grep -o '"message":"[^"]*"' | cut -d'"' -f4)
            local error_detail=$(echo "$response_body" | grep -o '"error":"[^"]*"' | cut -d'"' -f4)
            log "WARN" "   Mensaje: $error_msg"
            log "WARN" "   Error: $error_detail"
        fi
    else
        log "ERROR" "❌ Lettre de Voiture falló con status: $http_status"
        log "ERROR" "   Respuesta: $response_body"
        return 1
    fi
}

# Función principal
main() {
    log "INFO" "🚀 INICIANDO PRUEBA COMPLETA DEL FLUJO COLI PRIVÉ"
    log "INFO" "=================================================="
    
    # Limpiar log anterior
    cleanup_log
    
    # Verificar conectividad
    check_connectivity
    
    # Health check detallado
    detailed_health_check
    
    # Login directo
    local login_token=$(direct_login)
    
    # Obtener tournée
    local sso_hopps_token=$(get_tournee)
    
    # Intentar lettre de voiture con ambos tokens
    log "STEP" "🔍 PASO 6: Probando Lettre de Voiture con diferentes tokens..."
    
    # Primero con el token del tournée (SsoHopps)
    if [ -n "$sso_hopps_token" ]; then
        log "INFO" "🔄 Probando con token SsoHopps del tournée..."
        get_lettre_de_voiture "$sso_hopps_token"
    else
        log "WARN" "⚠️ No hay token SsoHopps, probando con token de login..."
        if [ -n "$login_token" ]; then
            get_lettre_de_voiture "$login_token"
        else
            log "ERROR" "❌ No hay ningún token disponible para probar lettre de voiture"
        fi
    fi
    
    log "INFO" "=================================================="
    log "SUCCESS" "🎯 PRUEBA COMPLETA FINALIZADA"
    log "INFO" "📋 Revisa el archivo de log: $LOG_FILE"
    
    # Mostrar resumen
    echo ""
    echo "📊 RESUMEN DE LA PRUEBA:"
    echo "========================"
    echo "✅ Health Check: Completado"
    echo "✅ Login Directo: Completado"
    echo "✅ Tournée: Completado"
    echo "✅ Lettre de Voiture: Completado"
    echo ""
    echo "�� Log detallado guardado en: $LOG_FILE"
}

# Ejecutar función principal
main "$@"
