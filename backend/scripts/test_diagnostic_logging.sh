#!/bin/bash

# 🔍 SCRIPT DE DIAGNÓSTICO COMPLETO DEL ESTADO COMPARTIDO
# Este script prueba el flujo completo para identificar dónde falla la autenticación dinámica

set -e

echo "🔍 ========================================="
echo "🔍 DIAGNÓSTICO COMPLETO DEL ESTADO COMPARTIDO"
echo "🔍 ========================================="
echo ""

# 🎯 CONFIGURACIÓN
BASE_URL="http://localhost:8000"
USERNAME="A187518"
PASSWORD="INTI7518"
SOCIETE="PCP0010699"
MATRICULE="A187518"

echo "🎯 Configuración:"
echo "   Username: $USERNAME"
echo "   Societe: $SOCIETE"
echo "   Matricule: $MATRICULE"
echo "   Base URL: $BASE_URL"
echo ""

# 🧹 PASO 1: LIMPIAR ESTADO (OPCIONAL)
echo "🧹 PASO 1: Limpiando estado anterior..."
echo "   (El backend mantiene el estado en memoria, esto es solo para referencia)"
echo ""

# 🔐 PASO 2: AUTENTICACIÓN INICIAL
echo "🔐 PASO 2: Autenticación inicial para obtener token..."
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

# 🔍 PASO 3: VERIFICAR SI EL TOKEN SE ALMACENÓ
echo "🔍 PASO 3: Verificando si el token se almacenó en el estado..."
echo "   (Revisar logs del backend para ver el logging de almacenamiento)"
echo ""

# 📦 PASO 4: INTENTAR OBTENER PAQUETES (DEBE USAR TOKEN ALMACENADO)
echo "📦 PASO 4: Intentando obtener paquetes (debe usar token almacenado)..."
echo "   Endpoint: POST /api/colis-prive/packages"
echo ""

PACKAGES_RESPONSE=$(curl -s -X POST "$BASE_URL/api/colis-prive/packages" \
  -H "Content-Type: application/json" \
  -d "{
    \"matricule\": \"$MATRICULE\",
    \"date\": \"2025-08-28\"
  }")

echo "📥 Respuesta de paquetes:"
echo "$PACKAGES_RESPONSE" | jq '.' 2>/dev/null || echo "$PACKAGES_RESPONSE"
echo ""

# 🚚 PASO 5: INTENTAR OBTENER TOURNÉE (DEBE USAR TOKEN ALMACENADO)
echo "🚚 PASO 5: Intentando obtener tournée (debe usar token almacenado)..."
echo "   Endpoint: POST /api/colis-prive/tournee"
echo ""

TOURNEE_RESPONSE=$(curl -s -X POST "$BASE_URL/api/colis-prive/tournee" \
  -H "Content-Type: application/json" \
  -d "{
    \"username\": \"$USERNAME\",
    \"password\": \"$PASSWORD\",
    \"societe\": \"$SOCIETE\",
    \"matricule\": \"$MATRICULE\",
    \"date\": \"2025-08-28\"
  }")

echo "📥 Respuesta de tournée:"
echo "$TOURNEE_RESPONSE" | jq '.' 2>/dev/null || echo "$TOURNEE_RESPONSE"
echo ""

# 📊 PASO 6: RESUMEN DEL DIAGNÓSTICO
echo "📊 ========================================="
echo "📊 RESUMEN DEL DIAGNÓSTICO"
echo "📊 ========================================="
echo ""

echo "🔍 REVISAR LOGS DEL BACKEND PARA:"
echo "   1. 💾 Logs de almacenamiento de token (store_auth_token)"
echo "   2. 🔍 Logs de búsqueda de token (get_auth_token)"
echo "   3. 🔑 Claves usadas para almacenar vs buscar"
echo "   4. 📊 Total de tokens en el estado"
echo "   5. ❌ Errores de autenticación automática"
echo ""

echo "🎯 PROBLEMAS ESPERADOS:"
echo "   - Si el token se almacena pero no se encuentra: problema de claves"
echo "   - Si la autenticación automática falla: problema de credenciales"
echo "   - Si el estado se pierde: problema de inicialización"
echo ""

echo "✅ SCRIPT COMPLETADO - Revisar logs del backend para diagnóstico completo"
