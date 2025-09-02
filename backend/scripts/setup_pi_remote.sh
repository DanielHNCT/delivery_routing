#!/bin/bash

# 🚀 CONFIGURACIÓN REMOTA DEL BACKEND EN RASPBERRY PI
# Este script configura todo el backend en el Pi via SSH

echo "🚀 Configurando backend en Raspberry Pi..."

# Configuración del Pi (ajusta según tu setup)
PI_HOST="${PI_HOST:-192.168.1.100}"  # Cambia por la IP de tu Pi
PI_USER="${PI_USER:-pi}"             # Cambia por tu usuario

# Token de Mapbox extraído del código Android
MAPBOX_TOKEN="pk.eyJ1IjoiZGFuaWVsaG5jdCIsImEiOiJjbThuY2w2b3kwYnVwMmxxemIzbmMxZG8wIn0.SAaCMIDxHKjlK_avG-i6og"

echo "📡 Conectando a $PI_USER@$PI_HOST..."

# Ejecutar configuración en el Pi
ssh $PI_USER@$PI_HOST << EOF
echo "🔧 Configurando backend automáticamente..."

# Configurar variables de entorno
echo "📝 Configurando variables de entorno..."
export MAPBOX_TOKEN="$MAPBOX_TOKEN"
export RUST_LOG=info
export PORT=3000

# Ir al directorio del backend
cd /home/pi/delivery_routing/backend

# Crear archivo .env si no existe
if [ ! -f .env ]; then
    echo "📄 Creando archivo .env..."
    cat > .env << 'ENV_EOF'
# 🗺️ CONFIGURACIÓN DEL BACKEND
MAPBOX_TOKEN=$MAPBOX_TOKEN
DATABASE_URL=postgresql://postgres:password@localhost:5432/delivery_optimizer
RUST_LOG=info
PORT=3000
ENV_EOF
    echo "✅ Archivo .env creado"
else
    echo "📄 Archivo .env ya existe"
fi

# Configurar para systemd service
echo "📝 Configurando systemd service..."
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
Environment=PORT=3000

[Install]
WantedBy=multi-user.target
SYSTEMD_EOF

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
EOF

echo "🧪 Ahora puedes probar la validación con:"
echo "   ./scripts/test_validation_on_pi.sh"
