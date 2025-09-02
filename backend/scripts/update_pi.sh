#!/bin/bash

# 🚀 SCRIPT PARA ACTUALIZAR CÓDIGO EN RASPBERRY PI 5
# ===================================================

echo "🔍 =========================================="
echo "🔍 ACTUALIZACIÓN DE CÓDIGO EN RASPBERRY PI 5"
echo "🔍 =========================================="

# Configuración
PI_HOST="192.168.1.100"  # ⚠️ CAMBIAR POR LA IP REAL DEL PI
PI_USER="pi"             # ⚠️ CAMBIAR POR EL USUARIO DEL PI
PROJECT_PATH="/home/pi/delivery_routing"

echo ""
echo "🎯 Configuración:"
echo " Host: $PI_HOST"
echo " Usuario: $PI_USER"
echo " Proyecto: $PROJECT_PATH"
echo ""

# Verificar conectividad SSH
echo "🔌 PASO 1: Verificar conectividad SSH..."
if ssh -o ConnectTimeout=5 -o BatchMode=yes "$PI_USER@$PI_HOST" exit 2>/dev/null; then
    echo "✅ Conexión SSH exitosa"
else
    echo "❌ No se puede conectar por SSH al Raspberry Pi"
    echo "   Verifica que:"
    echo "   1. El Pi esté encendido y conectado a la red"
    echo "   2. SSH esté habilitado en el Pi"
    echo "   3. Tengas las claves SSH configuradas"
    echo "   4. La IP $PI_HOST sea correcta"
    exit 1
fi

echo ""
echo "📥 PASO 2: Actualizar código desde Git..."
ssh "$PI_USER@$PI_HOST" << 'EOF'
    cd /home/pi/delivery_routing
    echo "🔄 Actualizando desde Git..."
    git pull origin main
    
    if [ $? -eq 0 ]; then
        echo "✅ Código actualizado exitosamente"
    else
        echo "❌ Error actualizando código"
        exit 1
    fi
EOF

if [ $? -ne 0 ]; then
    echo "❌ Error en la actualización del código"
    exit 1
fi

echo ""
echo "🔧 PASO 3: Compilar backend en Raspberry Pi..."
ssh "$PI_USER@$PI_HOST" << 'EOF'
    cd /home/pi/delivery_routing/backend
    echo "🔄 Compilando backend..."
    
    # Configurar variables de entorno si es necesario
    export MAPBOX_TOKEN="pk.test_token_for_validation"
    
    # Compilar
    cargo build --release
    
    if [ $? -eq 0 ]; then
        echo "✅ Backend compilado exitosamente"
    else
        echo "❌ Error compilando backend"
        exit 1
    fi
EOF

if [ $? -ne 0 ]; then
    echo "❌ Error en la compilación"
    exit 1
fi

echo ""
echo "🔄 PASO 4: Reiniciar servicio del backend..."
ssh "$PI_USER@$PI_HOST" << 'EOF'
    echo "🔄 Reiniciando servicio del backend..."
    
    # Detener proceso existente si está corriendo
    pkill -f "delivery-optimizer" || true
    sleep 2
    
    # Iniciar nuevo proceso
    cd /home/pi/delivery_routing/backend
    export MAPBOX_TOKEN="pk.test_token_for_validation"
    nohup ./target/release/delivery-optimizer > backend.log 2>&1 &
    
    sleep 3
    
    # Verificar que esté corriendo
    if pgrep -f "delivery-optimizer" > /dev/null; then
        echo "✅ Backend reiniciado exitosamente"
    else
        echo "❌ Error reiniciando backend"
        exit 1
    fi
EOF

if [ $? -ne 0 ]; then
    echo "❌ Error reiniciando el servicio"
    exit 1
fi

echo ""
echo "✅ ACTUALIZACIÓN COMPLETADA"
echo "=========================================="
echo ""
echo "🎯 PRÓXIMOS PASOS:"
echo "1. Ejecutar: ./scripts/test_validation_on_pi.sh"
echo "2. Verificar logs: ssh pi@$PI_HOST 'tail -f /home/pi/delivery_routing/backend/backend.log'"
echo "3. Configurar MAPBOX_TOKEN real si es necesario"
echo ""
echo "🚀 ¡El Raspberry Pi está listo para probar la validación inteligente!"
