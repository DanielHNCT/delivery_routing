#!/bin/bash

echo "🧪 PROBANDO NUEVO ENDPOINT DE LOGIN DIRECTO A COLIS PRIVE"
echo "=================================================="

# URL del backend local
BACKEND_URL="http://localhost:3000"

echo "📡 Probando endpoint: $BACKEND_URL/api/colis-prive/login"
echo ""

# Datos de prueba (credenciales reales de Colis Prive)
curl -X POST "$BACKEND_URL/api/colis-prive/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "A187518",
    "password": "INTI7518",
    "societe": "PCP0010699",
    "api_choice": "web"
  }' \
  -w "\n\nHTTP Status: %{http_code}\nTiempo total: %{time_total}s\n" \
  -s

echo ""
echo "✅ Prueba completada"
