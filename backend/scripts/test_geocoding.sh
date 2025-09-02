#!/bin/bash

# 🗺️ Script de prueba para el servicio de geocoding
# Este script prueba los endpoints de geocoding del backend

set -e

# Configuración
BASE_URL="http://localhost:3000"
GEOCODING_ENDPOINT="$BASE_URL/api/geocoding"
BATCH_GEOCODING_ENDPOINT="$BASE_URL/api/geocoding/batch"

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🗺️ PRUEBA DEL SERVICIO DE GEOCODING${NC}"
echo "=================================="
echo ""

# Función para hacer requests con curl
make_request() {
    local url="$1"
    local data="$2"
    local description="$3"
    
    echo -e "${YELLOW}📡 Probando: $description${NC}"
    echo "URL: $url"
    echo "Data: $data"
    echo ""
    
    response=$(curl -s -w "\nHTTP_CODE:%{http_code}" \
        -X POST \
        -H "Content-Type: application/json" \
        -d "$data" \
        "$url")
    
    http_code=$(echo "$response" | grep "HTTP_CODE:" | cut -d: -f2)
    body=$(echo "$response" | sed '/HTTP_CODE:/d')
    
    echo -e "${BLUE}📄 Respuesta:${NC}"
    echo "$body" | jq . 2>/dev/null || echo "$body"
    echo ""
    echo -e "${BLUE}📊 Status Code: $http_code${NC}"
    echo ""
    
    if [ "$http_code" -eq 200 ]; then
        echo -e "${GREEN}✅ ÉXITO${NC}"
    else
        echo -e "${RED}❌ ERROR${NC}"
    fi
    echo "----------------------------------"
    echo ""
}

# Verificar que el servidor esté corriendo
echo -e "${YELLOW}🔍 Verificando que el servidor esté corriendo...${NC}"
if ! curl -s "$BASE_URL" > /dev/null; then
    echo -e "${RED}❌ Error: El servidor no está corriendo en $BASE_URL${NC}"
    echo "Por favor, inicia el servidor con: cargo run"
    exit 1
fi
echo -e "${GREEN}✅ Servidor corriendo${NC}"
echo ""

# Test 1: Geocoding de una sola dirección
echo -e "${BLUE}🧪 TEST 1: Geocoding de una sola dirección${NC}"
make_request "$GEOCODING_ENDPOINT" \
    '{"address": "15 Rue de la Paix, 75001 Paris"}' \
    "Geocoding de Rue de la Paix, París"

# Test 2: Geocoding de dirección con caracteres especiales
echo -e "${BLUE}🧪 TEST 2: Geocoding con caracteres especiales${NC}"
make_request "$GEOCODING_ENDPOINT" \
    '{"address": "42 Avenue des Champs-Élysées, 75008 Paris"}' \
    "Geocoding de Champs-Élysées, París"

# Test 3: Geocoding de dirección inexistente
echo -e "${BLUE}🧪 TEST 3: Geocoding de dirección inexistente${NC}"
make_request "$GEOCODING_ENDPOINT" \
    '{"address": "Dirección que no existe 12345"}' \
    "Geocoding de dirección inexistente"

# Test 4: Geocoding con dirección vacía
echo -e "${BLUE}🧪 TEST 4: Geocoding con dirección vacía${NC}"
make_request "$GEOCODING_ENDPOINT" \
    '{"address": ""}' \
    "Geocoding con dirección vacía"

# Test 5: Batch geocoding
echo -e "${BLUE}🧪 TEST 5: Batch geocoding${NC}"
make_request "$BATCH_GEOCODING_ENDPOINT" \
    '{
        "addresses": [
            "15 Rue de la Paix, 75001 Paris",
            "42 Avenue des Champs-Élysées, 75008 Paris",
            "8 Rue de Rivoli, 75004 Paris"
        ]
    }' \
    "Batch geocoding de 3 direcciones de París"

# Test 6: Batch geocoding con muchas direcciones (límite)
echo -e "${BLUE}🧪 TEST 6: Batch geocoding con límite${NC}"
addresses_json='{"addresses": ['
for i in {1..51}; do
    if [ $i -gt 1 ]; then
        addresses_json+=","
    fi
    addresses_json+="\"Dirección de prueba $i, París\""
done
addresses_json+="]}"

make_request "$BATCH_GEOCODING_ENDPOINT" \
    "$addresses_json" \
    "Batch geocoding con 51 direcciones (debería fallar por límite)"

# Test 7: Batch geocoding vacío
echo -e "${BLUE}🧪 TEST 7: Batch geocoding vacío${NC}"
make_request "$BATCH_GEOCODING_ENDPOINT" \
    '{"addresses": []}' \
    "Batch geocoding con lista vacía"

echo -e "${GREEN}🎉 PRUEBAS COMPLETADAS${NC}"
echo ""
echo -e "${BLUE}📋 RESUMEN:${NC}"
echo "- Se probaron 7 casos diferentes"
echo "- Geocoding individual y en lote"
echo "- Manejo de errores y casos límite"
echo "- Validación de límites de la API"
echo ""
echo -e "${YELLOW}💡 NOTA:${NC}"
echo "Asegúrate de tener configurado el token de Mapbox:"
echo "export MAPBOX_TOKEN=tu_token_aqui"
echo ""
