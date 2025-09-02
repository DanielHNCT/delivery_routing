#!/bin/bash

# 🚀 SCRIPT PARA PROBAR VALIDACIÓN INTELIGENTE EN RASPBERRY PI 5
# ================================================================

echo "🔍 =========================================="
echo "🔍 PRUEBA DE VALIDACIÓN EN RASPBERRY PI 5"
echo "🔍 =========================================="

# Configuración
PI_HOST="192.168.1.100"  # ⚠️ CAMBIAR POR LA IP REAL DEL PI
PI_PORT="3000"
ENDPOINT="http://${PI_HOST}:${PI_PORT}"

echo ""
echo "🎯 Configuración:"
echo " Host: $PI_HOST"
echo " Puerto: $PI_PORT"
echo " Endpoint: $ENDPOINT"
echo ""

# Verificar conectividad
echo "🔌 PASO 1: Verificar conectividad con Raspberry Pi..."
if curl -s --connect-timeout 5 "$ENDPOINT/health" > /dev/null; then
    echo "✅ Raspberry Pi conectado correctamente"
else
    echo "❌ No se puede conectar al Raspberry Pi en $ENDPOINT"
    echo "   Verifica que:"
    echo "   1. El Pi esté encendido y conectado a la red"
    echo "   2. El backend esté corriendo en el puerto $PI_PORT"
    echo "   3. La IP $PI_HOST sea correcta"
    exit 1
fi

echo ""
echo "🔐 PASO 2: Autenticación con Colis Privé..."
AUTH_RESPONSE=$(curl -s -X POST "$ENDPOINT/api/colis-prive/auth" \
  -H "Content-Type: application/json" \
  -d '{
    "societe": "PCP0010699",
    "username": "A187518"
  }')

if echo "$AUTH_RESPONSE" | jq -e '.success' > /dev/null; then
    echo "✅ Autenticación exitosa"
    TOKEN=$(echo "$AUTH_RESPONSE" | jq -r '.authentication.token')
    echo "🔑 Token obtenido: ${TOKEN:0:20}..."
else
    echo "❌ Error en autenticación:"
    echo "$AUTH_RESPONSE" | jq '.'
    exit 1
fi

echo ""
echo "🚚 PASO 3: Obtener paquetes con validación inteligente..."
PACKAGES_RESPONSE=$(curl -s -X POST "$ENDPOINT/api/colis-prive/packages" \
  -H "Content-Type: application/json" \
  -d '{
    "matricule": "A187518",
    "date": "2025-09-01"
  }')

if echo "$PACKAGES_RESPONSE" | jq -e '.success' > /dev/null; then
    echo "✅ Paquetes obtenidos exitosamente"
    
    # Extraer información de validación
    VALIDATION_INFO=$(echo "$PACKAGES_RESPONSE" | jq '.address_validation')
    
    if [ "$VALIDATION_INFO" != "null" ]; then
        echo ""
        echo "📊 =========================================="
        echo "📊 RESULTADOS DE VALIDACIÓN INTELIGENTE"
        echo "📊 =========================================="
        
        TOTAL=$(echo "$VALIDATION_INFO" | jq '.total_packages')
        AUTO_VALIDATED=$(echo "$VALIDATION_INFO" | jq '.auto_validated')
        CLEANED_AUTO=$(echo "$VALIDATION_INFO" | jq '.cleaned_auto')
        COMPLETED_AUTO=$(echo "$VALIDATION_INFO" | jq '.completed_auto')
        PARTIAL_FOUND=$(echo "$VALIDATION_INFO" | jq '.partial_found')
        REQUIRES_MANUAL=$(echo "$VALIDATION_INFO" | jq '.requires_manual')
        
        echo "📦 Total de paquetes: $TOTAL"
        echo "✅ Auto-validados (original): $AUTO_VALIDATED"
        echo "🧹 Limpiados automáticamente: $CLEANED_AUTO"
        echo "🔧 Completados con sector: $COMPLETED_AUTO"
        echo "🔍 Encontrados parcialmente: $PARTIAL_FOUND"
        echo "⚠️ Requieren intervención manual: $REQUIRES_MANUAL"
        
        # Calcular porcentajes
        if [ "$TOTAL" -gt 0 ]; then
            AUTO_PERCENT=$(( (AUTO_VALIDATED + CLEANED_AUTO + COMPLETED_AUTO) * 100 / TOTAL ))
            MANUAL_PERCENT=$(( REQUIRES_MANUAL * 100 / TOTAL ))
            
            echo ""
            echo "📈 =========================================="
            echo "📈 ESTADÍSTICAS DE EFICIENCIA"
            echo "📈 =========================================="
            echo "🎯 Validación automática: $AUTO_PERCENT%"
            echo "⚠️ Intervención manual: $MANUAL_PERCENT%"
            
            if [ "$AUTO_PERCENT" -ge 80 ]; then
                echo "🚀 ¡EXCELENTE! Más del 80% de validación automática"
            elif [ "$AUTO_PERCENT" -ge 60 ]; then
                echo "✅ BUENO: Más del 60% de validación automática"
            elif [ "$AUTO_PERCENT" -ge 40 ]; then
                echo "⚠️ REGULAR: Menos del 60% de validación automática"
            else
                echo "❌ NECESITA MEJORAS: Menos del 40% de validación automática"
            fi
        fi
        
        # Mostrar warnings si los hay
        WARNINGS=$(echo "$VALIDATION_INFO" | jq -r '.warnings[]?' 2>/dev/null)
        if [ -n "$WARNINGS" ]; then
            echo ""
            echo "⚠️ =========================================="
            echo "⚠️ WARNINGS DE VALIDACIÓN"
            echo "⚠️ =========================================="
            echo "$WARNINGS"
        fi
        
    else
        echo "⚠️ No hay información de validación disponible"
        echo "   Esto puede indicar que:"
        echo "   1. El token de Mapbox no está configurado"
        echo "   2. La validación no se está ejecutando"
        echo "   3. Hay un error en la integración"
    fi
    
    echo ""
    echo "📦 =========================================="
    echo "📦 MUESTRA DE PAQUETES VALIDADOS"
    echo "📦 =========================================="
    echo "Mostrando primeros 3 paquetes con información de validación:"
    
    echo "$PACKAGES_RESPONSE" | jq '.packages[0:3][] | {
        id: .id,
        tracking_number: .tracking_number,
        address: .address,
        latitude: .latitude,
        longitude: .longitude,
        formatted_address: .formatted_address,
        validation_method: .validation_method,
        validation_confidence: .validation_confidence,
        validation_warnings: .validation_warnings
    }'
    
else
    echo "❌ Error obteniendo paquetes:"
    echo "$PACKAGES_RESPONSE" | jq '.'
    exit 1
fi

echo ""
echo "✅ SCRIPT COMPLETADO"
echo "=========================================="
echo ""
echo "🎯 PRÓXIMOS PASOS:"
echo "1. Si la validación no funciona, verificar MAPBOX_TOKEN en el Pi"
echo "2. Revisar logs del backend en el Raspberry Pi"
echo "3. Optimizar regex basado en patrones reales encontrados"
echo "4. Implementar cache de coordenadas para mejorar rendimiento"
