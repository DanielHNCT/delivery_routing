#!/bin/bash

echo "🚚 Delivery Route Optimizer - MVP"
echo "=================================="
echo ""

# Verificar que Rust esté instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust no está instalado"
    echo "💡 Instala Rust desde: https://rustup.rs/"
    exit 1
fi

# Verificar que el proyecto compile
echo "🔨 Compilando el proyecto..."
if ! cargo build; then
    echo "❌ Error de compilación"
    exit 1
fi

echo "✅ Compilación exitosa!"
echo ""

# Instrucciones para el usuario
echo "📝 INSTRUCCIONES IMPORTANTES:"
echo "1. Edita src/main.rs y reemplaza las credenciales placeholder:"
echo "   - username = \"tu_usuario_real\""
echo "   - password = \"tu_password_real\""
echo ""
echo "2. Ejecuta el proyecto:"
echo "   cargo run"
echo ""
echo "3. O ejecuta directamente el binario:"
echo "   ./target/debug/delivery-optimizer"
echo ""

# Intentar ejecutar (esto fallará con credenciales placeholder, pero es informativo)
echo "🧪 Probando ejecución (fallará con credenciales placeholder)..."
echo ""

if cargo run 2>&1 | head -20; then
    echo ""
    echo "🎉 ¡Proyecto funcionando correctamente!"
else
    echo ""
    echo "⚠️  Ejecución falló (esperado con credenciales placeholder)"
    echo "💡 Reemplaza las credenciales en src/main.rs y ejecuta nuevamente"
fi
