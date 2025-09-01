#!/bin/bash

# 🔐 SCRIPT SIMPLE: SOLO AUTENTICACIÓN PARA VER LOGGING DE ALMACENAMIENTO
# Este script solo hace autenticación para verificar que el token se almacene correctamente

set -e

echo "🔐 ========================================="
echo "🔐 TEST SOLO AUTENTICACIÓN - LOGGING DE ALMACENAMIENTO"
echo "🔐 ========================================="
echo ""

# 🎯 CONFIGURACIÓN
BASE_URL="http://192.168.1.9:3000"
USERNAME="A187518"
PASSWORD="INTI7518"
SOCIETE="PCP0010699"

echo "🎯 Configuración:"
echo "   Username: $USERNAME"
echo "   Societe: $SOCIETE"
echo "   Base URL: $BASE_URL"
echo ""

# 🔐 AUTENTICACIÓN
echo "🔐 Haciendo autenticación..."
echo "   Endpoint: POST /api/colis-prive/auth"
echo ""

AUTH_RESPONSE=$(curl -s -X POST "$BASE_URL/api/colis-prive/auth" \
  -H "Content-Type: application/json" \
  -d "{
    \"username\": \"$USERNAME\",
    \"password\": \"$PASSWORD\",
    \"societe\": \"$SOCIETE\"
  }")

echo "📥 Respuesta de autenticación:"
echo "$AUTH_RESPONSE" | jq '.' 2>/dev/null || echo "$AUTH_RESPONSE"
echo ""

# 📊 INSTRUCCIONES PARA REVISAR LOGS
echo "📊 ========================================="
echo "📊 REVISAR LOGS DEL BACKEND"
echo "📊 ========================================="
echo ""

echo "🔍 BUSCAR EN LOS LOGS DEL BACKEND:"
echo "   1. 💾 'Almacenando token con clave:' - Debe mostrar la clave usada"
echo "   2. 💾 'Token almacenado exitosamente con clave:' - Confirmación"
echo "   3. 🔍 'Total de tokens en estado:' - Debe ser > 0"
echo ""

echo "🎯 CLAVE ESPERADA: '$SOCIETE:$USERNAME' = '$SOCIETE:$USERNAME'"
echo ""

echo "✅ TEST COMPLETADO - Revisar logs del backend para verificar almacenamiento"
