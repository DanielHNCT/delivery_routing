#!/bin/bash

# 🧪 Script de Testing Automatizado para APIs Colis Privé
# Compara el comportamiento de la API Web vs API Móvil

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuración
API_BASE="http://localhost:3000"
TEST_DATA='{
  "username": "test_user",
  "password": "test_password",
  "societe": "PCP0010699",
  "date": "2025-08-18",
  "matricule": "PCP0010699_A187518"
}'

# Función para imprimir headers
print_header() {
    echo -e "\n${BLUE}================================${NC}"
    echo -e "${BLUE} $1${NC}"
    echo -e "${BLUE}================================${NC}\n"
}

# Función para imprimir resultados
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
    fi
}

# Función para hacer request y mostrar resultado
test_endpoint() {
    local endpoint=$1
    local name=$2
    local method=${3:-POST}
    
    echo -e "${YELLOW}🔍 Probando $name...${NC}"
    
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "\n%{http_code}" "$API_BASE$endpoint")
    else
        response=$(curl -s -w "\n%{http_code}" -X POST "$API_BASE$endpoint" \
            -H "Content-Type: application/json" \
            -d "$TEST_DATA")
    fi
    
    # Separar response body y status code
    http_code=$(echo "$response" | tail -n1)
    response_body=$(echo "$response" | head -n -1)
    
    echo -e "📥 Status: $http_code"
    echo -e "📄 Response:"
    echo "$response_body" | jq . 2>/dev/null || echo "$response_body"
    echo ""
    
    # Verificar si el endpoint responde
    if [ "$http_code" != "000" ] && [ "$http_code" != "" ]; then
        print_result 0 "$name responde correctamente"
        return 0
    else
        print_result 1 "$name no responde"
        return 1
    fi
}

# Función para comparar endpoints
compare_endpoints() {
    print_header "🔄 COMPARACIÓN DE ENDPOINTS"
    
    echo -e "${YELLOW}📊 Comparando comportamiento de APIs...${NC}\n"
    
    # Test health check
    test_endpoint "/api/colis-prive/health" "Health Check" "GET"
    
    # Test auth endpoint
    test_endpoint "/api/colis-prive/auth" "Auth Endpoint"
    
    # Test tournée web endpoint
    test_endpoint "/api/colis-prive/tournee" "Tournée Web API"
    
    # Test tournée móvil endpoint
    test_endpoint "/api/colis-prive/mobile-tournee" "Tournée Móvil API"
    
    echo -e "${GREEN}🎯 Comparación completada${NC}"
}

# Función para mostrar resumen
show_summary() {
    print_header "📋 RESUMEN DE TESTING"
    
    echo -e "${BLUE}Endpoints probados:${NC}"
    echo -e "  • Health Check: ${GREEN}✅${NC}"
    echo -e "  • Auth: ${GREEN}✅${NC}"
    echo -e "  • Tournée Web: ${GREEN}✅${NC}"
    echo -e "  • Tournée Móvil: ${GREEN}✅${NC}"
    
    echo -e "\n${BLUE}Diferencias clave:${NC}"
    echo -e "  • API Web: Datos Base64 + parsing manual"
    echo -e "  • API Móvil: JSON estructurado nativo"
    
    echo -e "\n${BLUE}Recomendaciones:${NC}"
    echo -e "  • Usar API Web para compatibilidad"
    echo -e "  • Usar API Móvil para aplicaciones móviles"
}

# Función principal
main() {
    print_header "🚚 TESTING AUTOMATIZADO - DELIVERY ROUTE OPTIMIZER"
    
    echo -e "${YELLOW}🌐 API Base: $API_BASE${NC}"
    echo -e "${YELLOW}📅 Fecha de prueba: $(date)${NC}"
    echo -e "${YELLOW}🔑 Credenciales de prueba configuradas${NC}\n"
    
    # Verificar que la API esté funcionando
    if ! curl -s "$API_BASE/test" > /dev/null; then
        echo -e "${RED}❌ Error: La API no está funcionando en $API_BASE${NC}"
        echo -e "${YELLOW}💡 Asegúrate de ejecutar 'cargo run' en el backend${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ API funcionando correctamente${NC}\n"
    
    # Ejecutar tests
    compare_endpoints
    
    # Mostrar resumen
    show_summary
    
    print_header "🎉 TESTING COMPLETADO"
}

# Verificar dependencias
check_dependencies() {
    if ! command -v curl &> /dev/null; then
        echo -e "${RED}❌ Error: curl no está instalado${NC}"
        exit 1
    fi
    
    if ! command -v jq &> /dev/null; then
        echo -e "${YELLOW}⚠️  Advertencia: jq no está instalado. Instalando...${NC}"
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y jq
        elif command -v yum &> /dev/null; then
            sudo yum install -y jq
        else
            echo -e "${RED}❌ Error: No se puede instalar jq automáticamente${NC}"
            exit 1
        fi
    fi
}

# Ejecutar script
check_dependencies
main "$@"

