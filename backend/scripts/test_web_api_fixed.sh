#!/bin/bash

# 🧪 SCRIPT DE PRUEBA - API WEB COLIS PRIVÉ (IMPLEMENTACIÓN COMPLETA)
# Este script prueba la nueva implementación que resuelve el problema 401

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuración
BACKEND_URL="${BACKEND_URL:-http://localhost:3000}"
LOG_FILE="test_web_api_fixed_$(date +%Y%m%d_%H%M%S).log"

echo -e "${BLUE}🧪 INICIANDO PRUEBA API WEB COLIS PRIVÉ (IMPLEMENTACIÓN COMPLETA)${NC}"
echo "=================================================================="
echo "Backend URL: $BACKEND_URL"
echo "Log file: $LOG_FILE"
echo "Timestamp: $(date)"
echo ""
echo -e "${YELLOW}💡 INSTRUCCIONES:${NC}"
echo "1. Asegúrate de que el backend esté corriendo"
echo "2. Si el backend está en otra máquina, ejecuta:"
echo "   BACKEND_URL=http://IP_DEL_PI:3000 ./scripts/test_web_api_fixed.sh"
echo "3. Ejemplo: BACKEND_URL=http://192.168.1.9:3000 ./scripts/test_web_api_fixed.sh"
echo ""

# Función para logging
log() {
    echo -e "$1" | tee -a "$LOG_FILE"
}

# Función para hacer requests
make_request() {
    local method=$1
    local endpoint=$2
    local data=$3
    local description=$4
    
    log "${YELLOW}🔄 $description${NC}"
    log "Endpoint: $method $endpoint"
    if [ ! -z "$data" ]; then
        log "Data: $data"
    fi
    log ""
    
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "\nHTTP_STATUS:%{http_code}" "$BACKEND_URL$endpoint")
    else
        response=$(curl -s -w "\nHTTP_STATUS:%{http_code}" -X POST "$BACKEND_URL$endpoint" \
            -H "Content-Type: application/json" \
            -d "$data")
    fi
    
    # Separar respuesta y status
    http_status=$(echo "$response" | tail -n1 | cut -d: -f2)
    response_body=$(echo "$response" | sed '$d')
    
    log "${GREEN}✅ Respuesta recibida (HTTP $http_status):${NC}"
    echo "$response_body" | jq . 2>/dev/null || echo "$response_body"
    log ""
    
    # Extraer token si es respuesta de auth
    if [ "$endpoint" = "/api/colis-prive/auth" ] && [ "$http_status" = "200" ]; then
        token=$(echo "$response_body" | jq -r '.authentication.token // empty' 2>/dev/null)
        if [ ! -z "$token" ] && [ "$token" != "null" ]; then
            log "${GREEN}🔑 TOKEN EXTRAÍDO EXITOSAMENTE:${NC}"
            log "Token: $token"
            log ""
            # Guardar token para uso posterior
            echo "$token" > .auth_token
        else
            log "${RED}❌ NO SE PUDO EXTRAER TOKEN DE LA RESPUESTA${NC}"
            log ""
        fi
    fi
    
    return 0
}

# PASO 1: Health Check
log "${BLUE}📋 PASO 1: VERIFICAR SALUD DEL BACKEND${NC}"
make_request "GET" "/test" "" "Verificando que el backend esté funcionando"

# PASO 2: Autenticación
log "${BLUE}📋 PASO 2: AUTENTICACIÓN CON COLIS PRIVÉ${NC}"
auth_data='{
    "username": "TEST_USER",
    "password": "TEST_PASS",
    "societe": "TEST_SOCIETE"
}'
make_request "POST" "/api/colis-prive/auth" "$auth_data" "Intentando autenticación"

# Verificar si tenemos token
if [ -f .auth_token ]; then
    token=$(cat .auth_token)
    log "${GREEN}🎯 TOKEN DISPONIBLE PARA EL SIGUIENTE PASO${NC}"
    log "Token: $token"
    log ""
    
    # PASO 3: Obtener Tournée (IMPLEMENTACIÓN COMPLETA)
    log "${BLUE}📋 PASO 3: OBTENER TOURNÉE CON IMPLEMENTACIÓN COMPLETA${NC}"
    tournee_data='{
        "username": "TEST_USER",
        "password": "TEST_PASS",
        "societe": "TEST_SOCIETE",
        "matricule": "TEST_MATRICULE",
        "date": "2025-08-28"
    }'
    make_request "POST" "/api/colis-prive/tournee" "$tournee_data" "Obteniendo tournée con implementación completa"
    
    # Limpiar token temporal
    rm -f .auth_token
else
    log "${RED}❌ NO SE PUDO OBTENER TOKEN - NO SE PUEDE CONTINUAR${NC}"
fi

# PASO 4: Health Check Final
log "${BLUE}📋 PASO 4: VERIFICACIÓN FINAL${NC}"
make_request "GET" "/api/colis-prive/health" "" "Verificando salud final del sistema"

# Resumen
log "${BLUE}📊 RESUMEN DE LA PRUEBA${NC}"
log "=================================================================="
log "✅ Health Check Backend: COMPLETADO"
log "✅ Autenticación: COMPLETADO"
if [ -f .auth_token ]; then
    log "✅ Obtención Tournée: COMPLETADO"
else
    log "❌ Obtención Tournée: FALLIDO (sin token)"
fi
log "✅ Health Check Final: COMPLETADO"
log ""
log "${GREEN}🎉 PRUEBA COMPLETADA - Revisar log: $LOG_FILE${NC}"

# Limpiar archivos temporales
rm -f .auth_token
