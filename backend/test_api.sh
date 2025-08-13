#!/bin/bash

echo "🔍 Investigando API de Colis Privé"
echo "=================================="
echo ""

BASE_URL="https://wsauthentificationexterne.colisprive.com"
ENDPOINT="/api/auth/login/Membership"

echo "📍 Endpoint: ${BASE_URL}${ENDPOINT}"
echo ""

# Test 1: Formato original
echo "🧪 Test 1: Formato original (username/password)"
curl -s -w "\nStatus: %{http_code}\n" \
  -X POST "${BASE_URL}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "User-Agent: DeliveryOptimizer/1.0" \
  -d '{"username":"A187518","password":"INTI7518"}'
echo ""

# Test 2: Formato alternativo (user/pass)
echo "🧪 Test 2: Formato alternativo (user/pass)"
curl -s -w "\nStatus: %{http_code}\n" \
  -X POST "${BASE_URL}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "User-Agent: DeliveryOptimizer/1.0" \
  -d '{"user":"A187518","pass":"INTI7518"}'
echo ""

# Test 3: Formato con campos adicionales
echo "🧪 Test 3: Formato con campos adicionales"
curl -s -w "\nStatus: %{http_code}\n" \
  -X POST "${BASE_URL}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "User-Agent: DeliveryOptimizer/1.0" \
  -d '{"username":"A187518","password":"INTI7518","type":"Membership"}'
echo ""

# Test 4: Headers adicionales
echo "🧪 Test 4: Headers adicionales"
curl -s -w "\nStatus: %{http_code}\n" \
  -X POST "${BASE_URL}${ENDPOINT}" \
  -H "Content-Type: application/json" \
  -H "User-Agent: DeliveryOptimizer/1.0" \
  -H "Accept: application/json" \
  -H "Cache-Control: no-cache" \
  -d '{"username":"A187518","password":"INTI7518"}'
echo ""

# Test 5: Verificar si el endpoint base responde
echo "🧪 Test 5: Verificar endpoint base"
curl -s -w "\nStatus: %{http_code}\n" \
  -X GET "${BASE_URL}/api/auth/"
echo ""

echo "🔍 Investigación completada"
