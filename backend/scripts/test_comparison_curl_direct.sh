#!/bin/bash

# 🔍 SCRIPT DE COMPARACIÓN: BACKEND vs CURL DIRECTO
# Este script compara exactamente qué envía nuestro backend vs curl directo

set -e

echo "🔍 ========================================="
echo "🔍 COMPARACIÓN: BACKEND vs CURL DIRECTO"
echo "🔍 ========================================="
echo ""

# 🎯 CONFIGURACIÓN
BASE_URL="http://192.168.1.9:3000"
USERNAME="A187518"
PASSWORD="INTI7518"
SOCIETE="PCP0010699"
MATRICULE="A187518"
DATE="2025-08-28"

echo "🎯 Configuración:"
echo "   Username: $USERNAME"
echo "   Societe: $SOCIETE"
echo "   Matricule: $MATRICULE"
echo "   Date: $DATE"
echo ""

# 🔐 PASO 1: AUTENTICACIÓN PARA OBTENER TOKEN
echo "🔐 PASO 1: Autenticación para obtener token..."
echo "   Endpoint: POST $BASE_URL/api/colis-prive/auth"
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

# 🔑 EXTRAER TOKEN DE LA RESPUESTA
TOKEN=$(echo "$AUTH_RESPONSE" | jq -r '.authentication.token' 2>/dev/null)

if [ "$TOKEN" = "null" ] || [ -z "$TOKEN" ]; then
    echo "❌ No se pudo extraer el token de la respuesta"
    exit 1
fi

echo "🔑 Token extraído: ${TOKEN:0:50}..."
echo ""

# 🚚 PASO 2: TEST CON NUESTRO BACKEND
echo "🚚 PASO 2: Test con nuestro backend..."
echo "   Endpoint: POST $BASE_URL/api/colis-prive/tournee"
echo ""

BACKEND_RESPONSE=$(curl -s -X POST "$BASE_URL/api/colis-prive/tournee" \
  -H "Content-Type: application/json" \
  -d "{
    \"username\": \"$USERNAME\",
    \"password\": \"$PASSWORD\",
    \"societe\": \"$SOCIETE\",
    \"matricule\": \"$MATRICULE\",
    \"date\": \"$DATE\"
  }")

echo "📥 Respuesta del backend:"
echo "$BACKEND_RESPONSE" | jq '.' 2>/dev/null || echo "$BACKEND_RESPONSE"
echo ""

# 🌐 PASO 3: TEST CON CURL DIRECTO A COLIS PRIVÉ
echo "🌐 PASO 3: Test con curl directo a Colis Privé..."
echo "   URL: https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getLettreVoitureEco_POST"
echo ""

DIRECT_RESPONSE=$(curl -s -X POST "https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getLettreVoitureEco_POST" \
  -H "Accept: application/json, text/plain, */*" \
  -H "Accept-Encoding: gzip, deflate, br, zstd" \
  -H "Accept-Language: fr-FR,fr;q=0.5" \
  -H "Cache-Control: no-cache" \
  -H "Connection: keep-alive" \
  -H "Content-Type: application/json" \
  -H "Origin: https://gestiontournee.colisprive.com" \
  -H "Referer: https://gestiontournee.colisprive.com/" \
  -H "SsoHopps: $TOKEN" \
  -H "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36" \
  -H "Sec-Fetch-Dest: empty" \
  -H "Sec-Fetch-Mode: cors" \
  -H "Sec-Fetch-Site: same-site" \
  -H "Sec-GPC: 1" \
  -H "sec-ch-ua: \"Not;A=Brand\";v=\"99\", \"Brave\";v=\"139\", \"Chromium\";v=\"139\"" \
  -H "sec-ch-ua-mobile: ?0" \
  -H "sec-ch-ua-platform: \"macOS\"" \
  -d "{
    \"enumTypeLettreVoiture\": \"ordreScan\",
    \"beanParamsMatriculeDateDebut\": {
      \"Societe\": \"$SOCIETE\",
      \"Matricule\": \"$MATRICULE\",
      \"DateDebut\": \"$DATE\"
    }
  }")

echo "📥 Respuesta directa de Colis Privé:"
echo "$DIRECT_RESPONSE" | jq '.' 2>/dev/null || echo "$DIRECT_RESPONSE"
echo ""

# 📊 PASO 4: COMPARACIÓN
echo "📊 ========================================="
echo "📊 COMPARACIÓN DE RESULTADOS"
echo "📊 ========================================="
echo ""

echo "🔍 BACKEND RESPONSE:"
if [ -z "$BACKEND_RESPONSE" ]; then
    echo "   ❌ BACKEND: Sin respuesta (posible timeout o error)"
elif echo "$BACKEND_RESPONSE" | grep -q "401\|Unauthorized\|Authorization has been denied"; then
    echo "   ❌ BACKEND: 401 Unauthorized"
else
    echo "   ✅ BACKEND: Éxito"
fi

echo ""

echo "🔍 CURL DIRECTO RESPONSE:"
if echo "$DIRECT_RESPONSE" | grep -q "401\|Unauthorized\|Authorization has been denied"; then
    echo "   ❌ CURL DIRECTO: 401 Unauthorized"
elif echo "$DIRECT_RESPONSE" | grep -q "Index was outside the bounds"; then
    echo "   ⚠️ CURL DIRECTO: Error de array bounds (pero autenticación OK)"
else
    echo "   ✅ CURL DIRECTO: Éxito completo"
fi

echo ""

echo "🎯 CONCLUSIÓN:"
if echo "$BACKEND_RESPONSE" | grep -q "401\|Unauthorized\|Authorization has been denied" && ! echo "$DIRECT_RESPONSE" | grep -q "401\|Unauthorized\|Authorization has been denied"; then
    echo "   🔍 PROBLEMA: Nuestro backend está enviando algo diferente que causa 401"
    echo "   📋 REVISAR: Logs del backend para ver exactamente qué headers/payload envía"
elif ! echo "$BACKEND_RESPONSE" | grep -q "401\|Unauthorized\|Authorization has been denied" && echo "$DIRECT_RESPONSE" | grep -q "401\|Unauthorized\|Authorization has been denied"; then
    echo "   🔍 PROBLEMA: CURL directo falla pero nuestro backend funciona"
elif echo "$BACKEND_RESPONSE" | grep -q "401\|Unauthorized\|Authorization has been denied" && echo "$DIRECT_RESPONSE" | grep -q "401\|Unauthorized\|Authorization has been denied"; then
    echo "   🔍 PROBLEMA: Ambos fallan - posible problema con el token o la API"
else
    echo "   ✅ AMBOS FUNCIONAN: No hay problema aparente"
fi

echo ""
echo "✅ SCRIPT COMPLETADO - Revisar logs del backend para comparación detallada"
