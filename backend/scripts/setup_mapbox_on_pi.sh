#!/bin/bash

# 🗺️ CONFIGURAR TOKEN DE MAPBOX EN RASPBERRY PI REMOTAMENTE
# Este script ejecuta la configuración del token en el Pi via SSH

echo "🚀 Configurando token de Mapbox en Raspberry Pi..."

# Configuración del Pi (ajusta según tu setup)
PI_HOST="${PI_HOST:-192.168.1.100}"  # Cambia por la IP de tu Pi
PI_USER="${PI_USER:-pi}"             # Cambia por tu usuario

# ⚠️ IMPORTANTE: Configura el token antes de ejecutar
MAPBOX_TOKEN="${MAPBOX_TOKEN:-}"

if [ -z "$MAPBOX_TOKEN" ]; then
    echo "❌ Error: MAPBOX_TOKEN no está configurado"
    echo "📝 Configura la variable de entorno:"
    echo "   export MAPBOX_TOKEN='tu_token_aqui'"
    echo "   El token se encuentra en: android/DeliveryRouting/app/src/main/res/values/mapbox.xml"
    exit 1
fi

echo "📡 Conectando a $PI_USER@$PI_HOST..."

# Ejecutar configuración en el Pi
ssh $PI_USER@$PI_HOST << EOF
echo "🔧 Configurando token de Mapbox en Raspberry Pi..."

# Configurar variable de entorno
echo "📝 Configurando MAPBOX_TOKEN en /etc/environment..."
echo "MAPBOX_TOKEN=$MAPBOX_TOKEN" | sudo tee -a /etc/environment

# Configurar para el usuario actual
echo "📝 Configurando MAPBOX_TOKEN en ~/.bashrc..."
echo "export MAPBOX_TOKEN=$MAPBOX_TOKEN" >> ~/.bashrc

# Configurar para systemd service
echo "📝 Configurando MAPBOX_TOKEN para systemd..."
sudo systemctl edit delivery-optimizer --full << 'SYSTEMD_EOF'
[Unit]
Description=Delivery Optimizer Backend
After=network.target

[Service]
Type=simple
User=pi
WorkingDirectory=/home/pi/delivery_routing/backend
ExecStart=/home/pi/delivery_routing/backend/target/release/delivery-optimizer
Restart=always
RestartSec=5
Environment=MAPBOX_TOKEN=$MAPBOX_TOKEN
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
SYSTEMD_EOF

echo "✅ Token de Mapbox configurado correctamente!"
echo "🔄 Reiniciando servicio..."
sudo systemctl daemon-reload
sudo systemctl restart delivery-optimizer

echo "📊 Verificando estado del servicio..."
sudo systemctl status delivery-optimizer --no-pager

echo "🎯 ¡Listo! El backend ahora tiene acceso al token de Mapbox"
EOF

echo "🧪 Ahora puedes probar la validación con:"
echo "   ./scripts/test_validation_on_pi.sh"
