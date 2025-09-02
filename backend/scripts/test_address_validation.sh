#!/bin/bash

# 🧪 Script de prueba para validación inteligente de direcciones
# Este script prueba el endpoint de paquetes con validación automática

echo "🔍 =========================================="
echo "🔍 PRUEBA DE VALIDACIÓN INTELIGENTE"
echo "🔍 =========================================="

# Configuración
BASE_URL="http://192.168.1.9:3000"
USERNAME="A187518"
SOCIETE="PCP0010699"
MATRICULE="A187518"
DATE="2025-09-01"

echo ""
echo "🎯 Configuración:"
echo " Username: $USERNAME"
echo " Societe: $SOCIETE"
echo " Matricule: $MATRICULE"
echo " Date: $DATE"
echo ""

echo "🔐 PASO 1: Autenticación..."
AUTH_RESPONSE=$(curl -s -X POST "$BASE_URL/api/colis-prive/auth" \
  -H "Content-Type: application/json" \
  -d "{
    \"username\": \"$USERNAME\",
    \"password\": \"INTI7518\",
    \"societe\": \"$SOCIETE\"
  }")

echo "📥 Respuesta de autenticación:"
echo "$AUTH_RESPONSE" | jq '.'

# Extraer token si está disponible
TOKEN=$(echo "$AUTH_RESPONSE" | jq -r '.authentication.token // empty')

if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
    echo "❌ No se pudo obtener el token de autenticación"
    exit 1
fi

echo ""
echo "🔑 Token extraído: ${TOKEN:0:50}..."

echo ""
echo "🚚 PASO 2: Obtener paquetes con validación inteligente..."
PACKAGES_RESPONSE=$(curl -s -X POST "$BASE_URL/api/colis-prive/packages" \
  -H "Content-Type: application/json" \
  -d "{
    \"matricule\": \"$MATRICULE\",
    \"date\": \"$DATE\"
  }")

echo "📥 Respuesta de paquetes:"
echo "$PACKAGES_RESPONSE" | jq '.'

echo ""
echo "📊 =========================================="
echo "📊 ANÁLISIS DE VALIDACIÓN"
echo "📊 =========================================="

# Extraer información de validación
VALIDATION_INFO=$(echo "$PACKAGES_RESPONSE" | jq '.address_validation // empty')

if [ -n "$VALIDATION_INFO" ] && [ "$VALIDATION_INFO" != "null" ]; then
    echo "✅ Validación de direcciones disponible:"
    echo "$VALIDATION_INFO" | jq '.'
    
    # Estadísticas
    TOTAL=$(echo "$VALIDATION_INFO" | jq '.total_packages // 0')
    AUTO_VALIDATED=$(echo "$VALIDATION_INFO" | jq '.auto_validated // 0')
    CLEANED_AUTO=$(echo "$VALIDATION_INFO" | jq '.cleaned_auto // 0')
    COMPLETED_AUTO=$(echo "$VALIDATION_INFO" | jq '.completed_auto // 0')
    PARTIAL_FOUND=$(echo "$VALIDATION_INFO" | jq '.partial_found // 0')
    REQUIRES_MANUAL=$(echo "$VALIDATION_INFO" | jq '.requires_manual // 0')
    
    echo ""
    echo "📈 ESTADÍSTICAS DE VALIDACIÓN:"
    echo " Total de paquetes: $TOTAL"
    echo " ✅ Auto-validados (original): $AUTO_VALIDATED"
    echo " 🧹 Limpiados automáticamente: $CLEANED_AUTO"
    echo " 🏢 Completados con sector: $COMPLETED_AUTO"
    echo " 🔍 Encontrados por búsqueda parcial: $PARTIAL_FOUND"
    echo " ❌ Requieren intervención manual: $REQUIRES_MANUAL"
    
    # Calcular porcentaje de éxito
    if [ "$TOTAL" -gt 0 ]; then
        SUCCESS_COUNT=$((AUTO_VALIDATED + CLEANED_AUTO + COMPLETED_AUTO + PARTIAL_FOUND))
        SUCCESS_PERCENTAGE=$((SUCCESS_COUNT * 100 / TOTAL))
        echo ""
        echo "🎯 TASA DE ÉXITO: $SUCCESS_PERCENTAGE% ($SUCCESS_COUNT/$TOTAL)"
    fi
    
    # Mostrar warnings si existen
    WARNINGS=$(echo "$VALIDATION_INFO" | jq '.warnings // []')
    if [ "$(echo "$WARNINGS" | jq 'length')" -gt 0 ]; then
        echo ""
        echo "⚠️ WARNINGS:"
        echo "$WARNINGS" | jq -r '.[]'
    fi
else
    echo "⚠️ No hay información de validación disponible"
fi

echo ""
echo "📦 =========================================="
echo "📦 MUESTRA DE PAQUETES VALIDADOS"
echo "📦 =========================================="

# Mostrar algunos paquetes con sus coordenadas
PACKAGES=$(echo "$PACKAGES_RESPONSE" | jq '.packages // []')
if [ "$(echo "$PACKAGES" | jq 'length')" -gt 0 ]; then
    echo "Mostrando primeros 3 paquetes con información de validación:"
    echo "$PACKAGES" | jq '.[0:3] | .[] | {
        id: .id,
        tracking_number: .tracking_number,
        recipient_name: .recipient_name,
        address: .address,
        latitude: .latitude,
        longitude: .longitude,
        formatted_address: .formatted_address,
        validation_method: .validation_method,
        validation_confidence: .validation_confidence,
        validation_warnings: .validation_warnings
    }'
else
    echo "No hay paquetes disponibles"
fi

echo ""
echo "✅ SCRIPT COMPLETADO"
echo "=========================================="
