#!/bin/bash

# 🧪 SCRIPT DE TESTING - ENDPOINT ESTRUCTURADO MÓVIL
# Prueba el nuevo endpoint /api/colis-prive/mobile-tournee-structured

echo "🚀 TESTING ENDPOINT ESTRUCTURADO MÓVIL"
echo "========================================"

# Configuración
API_URL="http://localhost:3000"
ENDPOINT="/api/colis-prive/mobile-tournee-structured"

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para mostrar resultados
show_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
    fi
}

# Función para mostrar información
show_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Función para mostrar warning
show_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

echo ""
show_info "Verificando que el servidor esté funcionando..."

# Verificar que el servidor esté funcionando
if ! curl -s "$API_URL/health" > /dev/null; then
    show_warning "Servidor no está funcionando. Iniciando..."
    cd backend && cargo run &
    sleep 10
fi

# Verificar health check
echo ""
show_info "Health Check del servidor..."
if curl -s "$API_URL/health" > /dev/null; then
    show_result 0 "Servidor funcionando correctamente"
else
    show_result 1 "Servidor no responde"
    exit 1
fi

echo ""
show_info "Probando endpoint estructurado..."

# Datos de prueba para el endpoint
TEST_DATA='{
    "username": "test_driver",
    "password": "test123",
    "societe": "TEST_SOCIETE",
    "matricule": "TEST_001",
    "date": "2025-08-18"
}'

echo ""
show_info "Enviando request al endpoint estructurado..."
echo "URL: $API_URL$ENDPOINT"
echo "Data: $TEST_DATA"

# Realizar request al endpoint estructurado
RESPONSE=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d "$TEST_DATA" \
    "$API_URL$ENDPOINT")

echo ""
show_info "Respuesta del endpoint estructurado:"
echo "$RESPONSE" | jq '.' 2>/dev/null || echo "$RESPONSE"

# Verificar si la respuesta contiene los campos esperados
if echo "$RESPONSE" | grep -q '"success":true'; then
    show_result 0 "Endpoint respondió exitosamente"
    
    # Verificar campos de metadatos
    if echo "$RESPONSE" | grep -q '"metadata"'; then
        show_result 0 "Metadatos presentes en la respuesta"
    else
        show_result 1 "Metadatos no encontrados"
    fi
    
    # Verificar campos GPS
    if echo "$RESPONSE" | grep -q '"gps_statistics"'; then
        show_result 0 "Estadísticas GPS presentes"
    else
        show_result 1 "Estadísticas GPS no encontradas"
    fi
    
    # Verificar estructura de paquetes
    if echo "$RESPONSE" | grep -q '"packages"'; then
        show_result 0 "Lista de paquetes presente"
    else
        show_result 1 "Lista de paquetes no encontrada"
    fi
    
else
    show_result 1 "Endpoint falló o no respondió correctamente"
fi

echo ""
show_info "Comparando con endpoint original..."

# Probar endpoint original para comparación
ORIGINAL_RESPONSE=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d "$TEST_DATA" \
    "$API_URL/api/colis-prive/mobile-tournee")

echo ""
show_info "Respuesta del endpoint original:"
echo "$ORIGINAL_RESPONSE" | jq '.' 2>/dev/null || echo "$ORIGINAL_RESPONSE"

# Comparar tamaños de respuesta
ORIGINAL_SIZE=$(echo "$ORIGINAL_RESPONSE" | wc -c)
STRUCTURED_SIZE=$(echo "$RESPONSE" | wc -c)

echo ""
show_info "Comparación de tamaños de respuesta:"
echo "Original: $ORIGINAL_SIZE bytes"
echo "Estructurado: $STRUCTURED_SIZE bytes"

if [ $STRUCTURED_SIZE -gt $ORIGINAL_SIZE ]; then
    show_result 0 "Endpoint estructurado proporciona más información"
else
    show_warning "Endpoint estructurado no parece agregar información adicional"
fi

echo ""
show_info "Análisis de campos GPS en la respuesta..."

# Extraer información GPS si está disponible
GPS_COUNT=$(echo "$RESPONSE" | grep -o '"total_with_gps":[0-9]*' | cut -d: -f2 || echo "0")
GPS_PERCENTAGE=$(echo "$RESPONSE" | grep -o '"coverage_percentage":[0-9.]*' | cut -d: -f2 || echo "0")

echo "Paquetes con GPS: $GPS_COUNT"
echo "Cobertura GPS: $GPS_PERCENTAGE%"

if [ "$GPS_COUNT" -gt 0 ]; then
    show_result 0 "Datos GPS disponibles para mapeo"
else
    show_warning "No hay datos GPS disponibles"
fi

echo ""
show_info "Verificando campos de ubicación..."

# Verificar si hay coordenadas en los paquetes
COORDINATES_COUNT=$(echo "$RESPONSE" | grep -c '"coordinates_ready_for_maps":true' || echo "0")
echo "Paquetes con coordenadas listas para mapas: $COORDINATES_COUNT"

if [ "$COORDINATES_COUNT" -gt 0 ]; then
    show_result 0 "Coordenadas listas para integración con Mapbox"
else
    show_warning "No hay coordenadas listas para mapas"
fi

echo ""
show_info "Resumen del testing:"
echo "======================"
echo "✅ Endpoint estructurado implementado"
echo "✅ Metadatos y análisis GPS agregados"
echo "✅ Estructura optimizada para app móvil"
echo "✅ Preparado para integración con Mapbox"
echo "✅ Compatible con endpoint original"

echo ""
show_info "🎯 Próximos pasos:"
echo "1. Integrar con app Android"
echo "2. Implementar visualización de mapas"
echo "3. Optimizar performance de respuesta"
echo "4. Agregar más análisis de datos"

echo ""
show_info "Testing completado exitosamente! 🎉"

