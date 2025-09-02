#!/bin/bash

# 🗺️ CONFIGURAR TOKEN DE MAPBOX EN RASPBERRY PI
# Este script configura el token de Mapbox extraído del código Android

echo "🔧 Configurando token de Mapbox en Raspberry Pi..."

# ⚠️ IMPORTANTE: Reemplaza con tu token real
# El token se encuentra en: android/DeliveryRouting/app/src/main/res/values/mapbox.xml
MAPBOX_TOKEN="${MAPBOX_TOKEN:-}"

if [ -z "$MAPBOX_TOKEN" ]; then
    echo "❌ Error: MAPBOX_TOKEN no está configurado"
    echo "📝 Configura la variable de entorno:"
    echo "   export MAPBOX_TOKEN='tu_token_aqui'"
    echo "   o edita este script y reemplaza la variable MAPBOX_TOKEN"
    exit 1
fi

# Configurar variable de entorno
echo "📝 Configurando MAPBOX_TOKEN en /etc/environment..."
echo "MAPBOX_TOKEN=$MAPBOX_TOKEN" | sudo tee -a /etc/environment

# Configurar para el usuario actual
echo "📝 Configurando MAPBOX_TOKEN en ~/.bashrc..."
echo "export MAPBOX_TOKEN=$MAPBOX_TOKEN" >> ~/.bashrc

# Configurar para systemd service
echo "📝 Configurando MAPBOX_TOKEN para systemd..."
sudo systemctl edit delivery-optimizer --full << EOF
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
EOF

echo "✅ Token de Mapbox configurado correctamente!"
echo "🔄 Reiniciando servicio..."
sudo systemctl daemon-reload
sudo systemctl restart delivery-optimizer

echo "📊 Verificando estado del servicio..."
sudo systemctl status delivery-optimizer --no-pager

echo "🎯 ¡Listo! El backend ahora tiene acceso al token de Mapbox"
echo "🧪 Puedes probar la validación con: ./scripts/test_address_validation.sh"
