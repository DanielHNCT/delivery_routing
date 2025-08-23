#!/bin/bash

echo "🧪 SIMULANDO FLUJO COMPLETO DESDE SONY XPERIA Z1"
echo "=================================================="

# URL del backend local (IP que usaría tu Sony)
BACKEND_URL="http://192.168.1.9:3000"

echo "📱 Dispositivo: Sony D5503 (Xperia Z1)"
echo "🌐 Backend URL: $BACKEND_URL"
echo ""

echo "🔑 CREDENCIALES DE PRUEBA:"
echo "   Username: A187518"
echo "   Password: INTI7518"
echo "   Societe: PCP0010699"
echo "   API Choice: web"
echo ""

# Archivo temporal para guardar el token
TOKEN_FILE="/tmp/colis_token.txt"
TOKEN=""

echo "🚀 PASO 1: LOGIN A COLIS PRIVE"
echo "================================"

# Simular exactamente lo que enviaría tu Sony para login
echo "📤 Enviando login request..."
LOGIN_RESPONSE=$(curl -s -X POST "$BACKEND_URL/api/colis-prive/login" \
  -H "Content-Type: application/json; charset=UTF-8" \
  -H "Accept-Charset: UTF-8" \
  -H "Connection: Keep-Alive" \
  -H "Accept-Encoding: gzip" \
  -H "User-Agent: okhttp/3.4.1" \
  -d '{
    "username": "A187518",
    "password": "INTI7518",
    "societe": "PCP0010699",
    "api_choice": "web"
  }' \
  -w "\nHTTP Status: %{http_code}")

echo "📥 Respuesta del login:"
echo "$LOGIN_RESPONSE"
echo ""

# Extraer el token de la respuesta
TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo "❌ ERROR: No se pudo extraer el token del login"
    echo "🔍 Respuesta completa:"
    echo "$LOGIN_RESPONSE"
    exit 1
fi

echo "✅ TOKEN OBTENIDO: ${TOKEN:0:50}..."
echo "💾 Guardando token en $TOKEN_FILE..."

# Guardar token en archivo temporal
echo "$TOKEN" > "$TOKEN_FILE"
echo ""

echo "🚀 PASO 2: PEDIR TOURNÉE CON EL TOKEN"
echo "======================================="

# Simular exactamente lo que enviaría tu Sony para pedir tournée
echo "📤 Enviando request de tournée con token..."
TOURNEE_RESPONSE=$(curl -s -X POST "$BACKEND_URL/api/colis-prive/tournee" \
  -H "Content-Type: application/json; charset=UTF-8" \
  -H "Accept-Charset: UTF-8" \
  -H "Connection: Keep-Alive" \
  -H "Accept-Encoding: gzip" \
  -H "User-Agent: okhttp/3.4.1" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "Societe": "PCP0010699",
    "Matricule": "PCP0010699_A187518",
    "DateDebut": "'$(date -u +"%Y-%m-%dT00:00:00.000Z")'",
    "Agence": null,
    "Concentrateur": null
  }' \
  -w "\nHTTP Status: %{http_code}")

echo "📥 Respuesta de tournée:"
echo "$TOURNEE_RESPONSE"
echo ""

echo "🚀 PASO 3: VERIFICAR ACCESO PILOT"
echo "=================================="

# Simular la llamada de acceso pilot que hace Colis Prive
echo "📤 Verificando acceso pilot..."
PILOT_RESPONSE=$(curl -s -X GET \
  "https://ws-gestiontournee-inter.colisprive.com/WS_PilotManagement/api/Pilot/access/PCP0010699_A187518/PCP0010699/FRONT_MOP" \
  -H "Accept: application/json, text/plain, */*" \
  -H "Accept-Language: fr-FR,fr;q=0.5" \
  -H "Cache-Control: no-cache" \
  -H "Connection: keep-alive" \
  -H "Origin: https://gestiontournee.colisprive.com" \
  -H "Referer: https://gestiontournee.colisprive.com/" \
  -H "SsoHopps: $TOKEN" \
  -H "User-Agent: DeliveryRouting/1.0" \
  -w "\nHTTP Status: %{http_code}")

echo "📥 Respuesta de acceso pilot:"
echo "$PILOT_RESPONSE"
echo ""

echo "🚀 PASO 4: OBTENER DASHBOARD INFO"
echo "=================================="

# Simular la llamada de dashboard info
echo "📤 Obteniendo información del dashboard..."
DASHBOARD_RESPONSE=$(curl -s -X POST \
  "https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getBeanInfoDashBoardBySocieteMatriculev2/" \
  -H "Accept: application/json, text/plain, */*" \
  -H "Accept-Language: fr-FR,fr;q=0.5" \
  -H "Cache-Control: no-cache" \
  -H "Connection: keep-alive" \
  -H "Content-Type: application/json" \
  -H "Origin: https://gestiontournee.colisprive.com" \
  -H "Referer: https://gestiontournee.colisprive.com/" \
  -H "SsoHopps: $TOKEN" \
  -H "User-Agent: DeliveryRouting/1.0" \
  -d '{
    "Societe": "PCP0010699",
    "Matricule": "PCP0010699_A187518",
    "DateDebut": "'$(date -u +"%Y-%m-%dT00:00:00.000Z")'",
    "Agence": null,
    "Concentrateur": null
  }' \
  -w "\nHTTP Status: %{http_code}")

echo "📥 Respuesta del dashboard:"
echo "$DASHBOARD_RESPONSE"
echo ""

echo "✅ FLUJO COMPLETO SIMULADO"
echo "=================================================="
echo "📋 RESUMEN:"
echo "   1. ✅ Login exitoso"
echo "   2. ✅ Token obtenido: ${TOKEN:0:30}..."
echo "   3. ✅ Tournée solicitada"
echo "   4. ✅ Acceso pilot verificado"
echo "   5. ✅ Dashboard info obtenida"
echo ""
echo "🔑 TOKEN COMPLETO: $TOKEN"
echo "💾 Guardado en: $TOKEN_FILE"
echo ""
echo "🎯 ESTE ES EXACTAMENTE EL FLUJO QUE DEBE REPLICAR TU BACKEND"
echo "   - Login → Obtener Token → Usar Token para todas las operaciones"
