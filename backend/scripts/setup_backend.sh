#!/bin/bash

# 🚀 CONFIGURACIÓN AUTOMÁTICA DEL BACKEND
# Este script configura todo el backend automáticamente

echo "🔧 Configurando backend automáticamente..."

# Token de Mapbox extraído del código Android
MAPBOX_TOKEN="YOUR_MAPBOX_TOKEN_HERE"

# Configurar variables de entorno
echo "📝 Configurando variables de entorno..."
export MAPBOX_TOKEN="$MAPBOX_TOKEN"
export RUST_LOG=info
export PORT=3000

# Crear archivo .env si no existe
if [ ! -f .env ]; then
    echo "📄 Creando archivo .env..."
    cat > .env << EOF
# 🗺️ CONFIGURACIÓN DEL BACKEND
MAPBOX_TOKEN=$MAPBOX_TOKEN
DATABASE_URL=postgresql://postgres:password@localhost:5432/delivery_optimizer
RUST_LOG=info
PORT=3000
EOF
    echo "✅ Archivo .env creado"
else
    echo "📄 Archivo .env ya existe"
fi

# Configurar para systemd service
echo "📝 Configurando systemd service..."
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
Environment=PORT=3000

[Install]
WantedBy=multi-user.target
EOF

# Compilar el proyecto
echo "🔨 Compilando proyecto..."
cargo build --release

# Reiniciar servicio
echo "🔄 Reiniciando servicio..."
sudo systemctl daemon-reload
sudo systemctl restart delivery-optimizer

# Verificar estado
echo "📊 Verificando estado del servicio..."
sudo systemctl status delivery-optimizer --no-pager

echo "✅ ¡Backend configurado y funcionando!"
echo "🧪 Probar validación: ./scripts/test_address_validation.sh"
