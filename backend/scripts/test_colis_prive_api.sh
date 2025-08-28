#!/bin/bash

# 🧪 Test completo de la API de Colis Privé
# Este script prueba todos los endpoints de la API

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuración
BASE_URL="http://localhost:3000"
LOG_FILE="test_colis_prive_api.log"

# Limpiar log anterior
> "$LOG_FILE"

echo -e "${BLUE}🧪 INICIANDO TEST COMPLETO DE LA API DE COLIS PRIVÉ${NC}" | tee -a "$LOG_FILE"
echo -e "${BLUE}================================================${NC}" | tee -a "$LOG_FILE"
echo "Timestamp: $(date)" | tee -a "$LOG_FILE"
echo "Base URL: $BASE_URL" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Función para hacer requests con logging detallado
make_request() {
    local endpoint="$1"
    local method="$2"
    local data="$3"
    local description="$4"
    
    echo -e "${YELLOW}📡 $description${NC}" | tee -a "$LOG_FILE"
    echo "Endpoint: $method $endpoint" | tee -a "$LOG_FILE"
    
    if [ -n "$data" ]; then
        echo "Payload: $data" | tee -a "$LOG_FILE"
    fi
    
    echo "---" | tee -a "$LOG_FILE"
    
    local response
    local status_code
    
    if [ -n "$data" ]; then
        response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME:%{time_total}s\nSIZE:%{size_download}bytes" \
            -X "$method" \
            -H "Content-Type: application/json" \
            -d "$data" \
            "$BASE_URL$endpoint")
    else
        response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME:%{time_total}s\nSIZE:%{size_download}bytes" \
            -X "$method" \
            "$BASE_URL$endpoint")
    fi
    
    # Extraer status code y tiempo
    status_code=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    response_time=$(echo "$response" | grep "TIME:" | cut -d: -f2)
    response_size=$(echo "$response" | grep "SIZE:" | cut -d: -f2)
    
    # Obtener solo el body de la respuesta
    response_body=$(echo "$response" | sed '/HTTP_STATUS:/d' | sed '/TIME:/d' | sed '/SIZE:/d')
    
    echo "Status: $status_code" | tee -a "$LOG_FILE"
    echo "Tiempo: ${response_time}s" | tee -a "$LOG_FILE"
    echo "Tamaño: ${response_size} bytes" | tee -a "$LOG_FILE"
    
    if [ "$status_code" = "200" ]; then
        echo -e "${GREEN}✅ ÉXITO${NC}" | tee -a "$LOG_FILE"
    else
        echo -e "${RED}❌ ERROR${NC}" | tee -a "$LOG_FILE"
    fi
    
    echo "Respuesta:" | tee -a "$LOG_FILE"
    echo "$response_body" | jq . 2>/dev/null || echo "$response_body" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"
    echo "---" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"
    
    return "$status_code"
}

# Test 1: Health Check
echo -e "${BLUE}🔍 TEST 1: Health Check${NC}" | tee -a "$LOG_FILE"
make_request "/health" "GET" "" "Verificar que el backend esté funcionando"

# Test 2: Login directo a Colis Privé
echo -e "${BLUE}🔐 TEST 2: Login directo a Colis Privé${NC}" | tee -a "$LOG_FILE"
login_payload='{
    "username": "A187518",
    "password": "INTI7518",
    "societe": "PCP0010699"
}'

make_request "/api/colis-prive/login" "POST" "$login_payload" "Autenticación directa con Colis Privé"

# Extraer token del login si fue exitoso
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Login exitoso - Continuando con tests...${NC}" | tee -a "$LOG_FILE"
else
    echo -e "${RED}❌ Login falló - Abortando tests${NC}" | tee -a "$LOG_FILE"
    exit 1
fi

# Test 3: Obtener tournée
echo -e "${BLUE}📦 TEST 3: Obtener tournée${NC}" | tee -a "$LOG_FILE"
tournee_payload='{
    "username": "A187518",
    "password": "INTI7518",
    "societe": "PCP0010699",
    "matricule": "PCP0010699_A187518"
}'

make_request "/api/colis-prive/tournee" "POST" "$tournee_payload" "Obtener datos de tournée"

# Test 4: Obtener lettre de voiture
echo -e "${BLUE}📄 TEST 4: Obtener lettre de voiture${NC}" | tee -a "$LOG_FILE"
lettre_payload='{
    "username": "A187518",
    "password": "INTI7518",
    "societe": "PCP0010699",
    "matricule": "PCP0010699_A187518"
}'

make_request "/api/colis-prive/lettre-voiture" "POST" "$lettre_payload" "Obtener lettre de voiture"

echo -e "${BLUE}🏁 TEST COMPLETADO${NC}" | tee -a "$LOG_FILE"
echo "Timestamp: $(date)" | tee -a "$LOG_FILE"
echo "Log guardado en: $LOG_FILE" | tee -a "$LOG_FILE"
