#!/bin/bash

# 🧪 Test directo de la API de Colis Privé
# Este script prueba directamente los endpoints de Colis Privé

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuración
LOG_FILE="test_colis_prive_direct.log"

# Limpiar log anterior
> "$LOG_FILE"

echo -e "${BLUE}🧪 INICIANDO TEST DIRECTO DE LA API DE COLIS PRIVÉ${NC}" | tee -a "$LOG_FILE"
echo -e "${BLUE}================================================${NC}" | tee -a "$LOG_FILE"
echo "Timestamp: $(date)" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Función para hacer requests con logging detallado
make_request() {
    local url="$1"
    local method="$2"
    local headers="$3"
    local data="$4"
    local description="$5"
    
    echo -e "${YELLOW}📡 $description${NC}" | tee -a "$LOG_FILE"
    echo "URL: $method $url" | tee -a "$LOG_FILE"
    
    if [ -n "$headers" ]; then
        echo "Headers: $headers" | tee -a "$LOG_FILE"
    fi
    
    if [ -n "$data" ]; then
        echo "Payload: $data" | tee -a "$LOG_FILE"
    fi
    
    echo "---" | tee -a "$LOG_FILE"
    
    local response
    local status_code
    
    if [ -n "$data" ]; then
        response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME:%{time_total}s\nSIZE:%{size_download}bytes" \
            -X "$method" \
            -H "$headers" \
            -d "$data" \
            "$url")
    else
        response=$(curl -s -w "\nHTTP_STATUS:%{http_code}\nTIME:%{time_total}s\nSIZE:%{size_download}bytes" \
            -X "$method" \
            -H "$headers" \
            "$url")
    fi
    
    # Extraer status code y tiempo
    status_code=$(echo "$response" | grep "HTTP_STATUS:" | cut -d: -f2)
    response_time=$(echo "$response" | grep "TIME:" | cut -d: -f2)
    response_size=$(echo "$response" | grep "SIZE:" | cut -d: -f2)
    
    # Obtener solo el body de la respuesta
    response_body=$(echo "$response" | sed '/HTTP_STATUS:/d' | sed '/TIME:/d' | sed '/SIZE:/d')
    
    echo "Status: $status_code" | tee -a "$LOG_FILE"
    echo "Tiempo: ${response_time}s" | tee -a "$LOG_FILE"
    echo "Tamaño: ${response_size} bytes" | tee -a "$LOG_FILE"
    
    if [ "$status_code" = "200" ]; then
        echo -e "${GREEN}✅ ÉXITO${NC}" | tee -a "$LOG_FILE"
    else
        echo -e "${RED}❌ ERROR${NC}" | tee -a "$LOG_FILE"
    fi
    
    echo "Respuesta:" | tee -a "$LOG_FILE"
    echo "$response_body" | jq . 2>/dev/null || echo "$response_body" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"
    echo "---" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"
    
    # Retornar la respuesta para poder extraer el token
    echo "$response_body"
    
    # Retornar el status code como exit code
    return "$status_code"
}

# Test 1: Login Membership
echo -e "${BLUE}🔐 TEST 1: Login Membership${NC}" | tee -a "$LOG_FILE"
login_url="https://wsauthentificationexterne.colisprive.com/api/auth/login/Membership"
login_headers="Accept: application/json, text/plain, */*
Accept-Language: fr-FR,fr;q=0.5
Cache-Control: no-cache
Content-Type: application/json
Origin: https://gestiontournee.colisprive.com
Referer: https://gestiontournee.colisprive.com/
User-Agent: DeliveryRouting/1.0"
login_payload='{
    "login": "PCP0010699_A187518",
    "password": "INTI7518",
    "societe": "PCP0010699",
    "commun": {
        "dureeTokenInHour": 24
    }
}'

login_response=$(make_request "$login_url" "POST" "$login_headers" "$login_payload" "Login Membership - Obtener token SsoHopps")

# Extraer token SsoHopps dinámicamente
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Login exitoso - Extrayendo token SsoHopps...${NC}" | tee -a "$LOG_FILE"
    
    # Extraer el token SsoHopps de la respuesta
    sso_hopps_token=$(echo "$login_response" | jq -r '.tokens.SsoHopps' 2>/dev/null)
    
    if [ -n "$sso_hopps_token" ] && [ "$sso_hopps_token" != "null" ]; then
        echo -e "${GREEN}✅ Token SsoHopps extraído: ${sso_hopps_token:0:50}...${NC}" | tee -a "$LOG_FILE"
    else
        echo -e "${RED}❌ No se pudo extraer el token SsoHopps${NC}" | tee -a "$LOG_FILE"
        echo "Respuesta completa del login:" | tee -a "$LOG_FILE"
        echo "$login_response" | tee -a "$LOG_FILE"
        exit 1
    fi
else
    echo -e "${RED}❌ Login falló - Abortando tests${NC}" | tee -a "$LOG_FILE"
    exit 1
fi

# Test 2: Pilot Access
echo -e "${BLUE}👨‍✈️ TEST 2: Pilot Access${NC}" | tee -a "$LOG_FILE"
pilot_url="https://ws-gestiontournee-inter.colisprive.com/WS_PilotManagement/api/Pilot/access/PCP0010699_A187518/PCP0010699/FRONT_MOP"
pilot_headers="Accept: application/json, text/plain, */*
Accept-Language: fr-FR,fr;q=0.5
Cache-Control: no-cache
SsoHopps: $sso_hopps_token
User-Agent: DeliveryRouting/1.0"

make_request "$pilot_url" "GET" "$pilot_headers" "" "Pilot Access - Verificar acceso del piloto"

# Test 3: Dashboard Info
echo -e "${BLUE}📊 TEST 3: Dashboard Info${NC}" | tee -a "$LOG_FILE"
dashboard_url="https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getBeanInfoDashBoardBySocieteMatriculev2/"
dashboard_headers="Accept: application/json, text/plain, */*
Accept-Language: fr-FR,fr;q=0.5
Cache-Control: no-cache
Content-Type: application/json
SsoHopps: $sso_hopps_token
User-Agent: DeliveryRouting/1.0"
dashboard_payload='{
    "Societe": "PCP0010699",
    "Matricule": "PCP0010699_A187518",
    "DateDebut": "2025-08-23T00:00:00.000Z",
    "Agence": null,
    "Concentrateur": null
}'

make_request "$dashboard_url" "POST" "$dashboard_headers" "$dashboard_payload" "Dashboard Info - Obtener información del dashboard"

# Test 4: Lettre de Voiture
echo -e "${BLUE}📄 TEST 4: Lettre de Voiture${NC}" | tee -a "$LOG_FILE"
lettre_url="https://wstournee-v2.colisprive.com/WS-TourneeColis/api/getLettreVoitureEco_POST"
lettre_headers="Accept: application/json, text/plain, */*
Accept-Language: fr-FR,fr;q=0.5
Cache-Control: no-cache
Content-Type: application/json
SsoHopps: $sso_hopps_token
User-Agent: DeliveryRouting/1.0"
lettre_payload='{
    "Societe": "PCP0010699",
    "Matricule": "PCP0010699_A187518",
    "DateDebut": "2025-08-23T00:00:00.000Z",
    "Agence": null,
    "Concentrateur": null
}'

make_request "$lettre_url" "POST" "$lettre_headers" "$lettre_payload" "Lettre de Voiture - Obtener lettre de voiture"

echo -e "${BLUE}🏁 TEST COMPLETADO${NC}" | tee -a "$LOG_FILE"
echo "Timestamp: $(date)" | tee -a "$LOG_FILE"
echo "Log guardado en: $LOG_FILE" | tee -a "$LOG_FILE"
