import json
from datetime import datetime
import re
import os

# 🎯 RUTA ABSOLUTA PARA EL ARCHIVO DE LOG
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
LOG_FILE = os.path.join(SCRIPT_DIR, 'sg_j3_complete_capture.log')

# 🚀 NUESTRO BACKEND - MÁXIMA PRIORIDAD
OUR_BACKEND_DOMAINS = [
    '10.0.2.2:3000',  # Android Emulator -> PC
    '192.168.1.',     # Red local típica
    'localhost:3000', # Local
    '127.0.0.1:3000' # Loopback
]

def should_capture(host):
    """Captura ABSOLUTAMENTE TODO - sin filtros"""
    # 🎯 NUESTRO BACKEND - MÁXIMA PRIORIDAD
    if any(our_backend in host for our_backend in OUR_BACKEND_DOMAINS):
        return True, "🚀 OUR_BACKEND"
    
    # 📡 Capturar TODO lo demás - SIN FILTROS
    return True, "📡 CAPTURED"

def write_to_log(content):
    """Escribe al archivo de log con manejo de errores"""
    try:
        with open(LOG_FILE, 'a', encoding='utf-8') as f:
            f.write(content)
        print(f"📄 Log escrito en: {LOG_FILE}")
    except Exception as e:
        print(f"❌ Error escribiendo log: {e}")
        print(f"📁 Intentando escribir en directorio actual...")
        try:
            with open('sg_j3_complete_capture.log', 'a', encoding='utf-8') as f:
                f.write(content)
            print(f"📄 Log escrito en directorio actual: sg_j3_complete_capture.log")
        except Exception as e2:
            print(f"❌ Error crítico escribiendo log: {e2}")

def request(flow):
    should_log, log_type = should_capture(flow.request.pretty_host)
    
    if should_log:
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        
        # 🎯 Emojis especiales para nuestro backend
        if log_type == "🚀 OUR_BACKEND":
            prefix = "🚀 NUESTRO_BACKEND"
            emoji = "🚀"
        else:
            prefix = "📡 CAPTURED"
            emoji = "📡"
        
        print(f"\n{emoji} {prefix} [{timestamp}] REQUEST: {flow.request.method} {flow.request.pretty_url}")
        print(f"🏠 Host: {flow.request.pretty_host}")
        print(f"🌐 URL Completa: {flow.request.pretty_url}")
        print(f"📋 TODOS LOS HEADERS: {json.dumps(dict(flow.request.headers), indent=2)}")
        
        if flow.request.content:
            try:
                # Intentar mostrar como JSON si es posible
                if 'json' in flow.request.headers.get('content-type', '').lower():
                    request_json = json.loads(flow.request.text)
                    print(f"📤 JSON Body: {json.dumps(request_json, indent=2)}")
                else:
                    print(f"📤 Body: {flow.request.text}")
            except:
                print(f"📤 Binary Body: {len(flow.request.content)} bytes")
        
        print("-" * 100)
        
        # Guardar en archivo con categorización especial
        log_content = f"\n{'='*80}\n"
        log_content += f"TIMESTAMP: {timestamp}\n"
        log_content += f"TYPE: {log_type}\n"
        log_content += f"REQUEST: {flow.request.method} {flow.request.pretty_url}\n"
        log_content += f"HOST: {flow.request.pretty_host}\n"
        log_content += f"FULL_URL: {flow.request.pretty_url}\n"
        log_content += f"TODOS LOS HEADERS: {json.dumps(dict(flow.request.headers), indent=2)}\n"
        
        if flow.request.content:
            try:
                if 'json' in flow.request.headers.get('content-type', '').lower():
                    request_json = json.loads(flow.request.text)
                    log_content += f"BODY: {json.dumps(request_json, indent=2)}\n"
                else:
                    log_content += f"BODY: {flow.request.text}\n"
            except:
                log_content += f"BINARY BODY: {len(flow.request.content)} bytes\n"
        
        write_to_log(log_content)

def response(flow):
    should_log, log_type = should_capture(flow.request.pretty_host)
    
    if should_log:
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        
        # 🚀 Emojis especiales para nuestro backend
        if log_type == "🚀 OUR_BACKEND":
            emoji = "🚀"
            prefix = "NUESTRO_BACKEND"
        else:
            # Color coding por status
            if 200 <= flow.response.status_code < 300:
                status_emoji = "✅"
            elif 400 <= flow.response.status_code < 500:
                status_emoji = "❌"
            elif 500 <= flow.response.status_code:
                status_emoji = "💥"
            else:
                status_emoji = "ℹ️"
            emoji = status_emoji
            prefix = "RESPONSE"
        
        print(f"\n{emoji} [{timestamp}] {prefix} {flow.response.status_code}: {flow.request.pretty_host}")
        print(f"📊 Size: {len(flow.response.content)} bytes")
        print(f"🌐 URL: {flow.request.pretty_url}")
        print(f"📋 TODOS LOS RESPONSE HEADERS: {json.dumps(dict(flow.response.headers), indent=2)}")
        
        # Mostrar contenido de response
        if flow.response.content:
            try:
                if 'json' in flow.response.headers.get('content-type', '').lower():
                    response_json = json.loads(flow.response.text)
                    print(f"📦 JSON Response: {json.dumps(response_json, indent=2)[:2000]}...")
                else:
                    response_text = flow.response.text[:1000]
                    print(f"📦 Response Text: {response_text}...")
            except:
                print(f"📦 Binary Response: {len(flow.response.content)} bytes")
        
        print("=" * 100)
        
        # Guardar response en archivo
        log_content = f"RESPONSE STATUS: {flow.response.status_code}\n"
        log_content += f"RESPONSE URL: {flow.request.pretty_url}\n"
        log_content += f"TODOS LOS RESPONSE HEADERS: {json.dumps(dict(flow.response.headers), indent=2)}\n"
        
        if flow.response.content:
            try:
                if 'json' in flow.response.headers.get('content-type', '').lower():
                    response_json = json.loads(flow.response.text)
                    log_content += f"RESPONSE JSON: {json.dumps(response_json, indent=2)}\n"
                else:
                    log_content += f"RESPONSE TEXT: {flow.response.text}\n"
            except:
                log_content += f"BINARY RESPONSE: {len(flow.response.content)} bytes\n"
        
        log_content += f"{'='*80}\n\n"
        write_to_log(log_content)

def load(loader):
    """Inicializar archivo de log"""
    try:
        # Intentar escribir en directorio del script
        with open(LOG_FILE, 'w', encoding='utf-8') as f:
            f.write(f"🔍 SG J3 COMPLETE TRAFFIC CAPTURE - Started at {datetime.now()}\n")
            f.write(f"📡 Capturing ABSOLUTELY EVERYTHING - NO FILTERS\n")
            f.write(f"🚀 NUESTRO BACKEND (MÁXIMA PRIORIDAD): {', '.join(OUR_BACKEND_DOMAINS)}\n")
            f.write(f"📋 Capturando TODOS los headers y TODO el contenido\n")
            f.write(f"📁 Log file: {LOG_FILE}\n")
            f.write("="*80 + "\n\n")
        
        print(f"🔍 SG J3 COMPLETE TRAFFIC CAPTURE INICIADO")
        print(f"📁 Archivo de log: {LOG_FILE}")
        print("📡 Capturando ABSOLUTAMENTE TODO - SIN FILTROS")
        print("🚀 NUESTRO BACKEND - MÁXIMA PRIORIDAD")
        print("📋 TODOS los headers y TODO el contenido")
        print("🎯 Para reverse engineering completo de la API")
        print("="*80)
        
    except Exception as e:
        print(f"❌ Error inicializando log en {LOG_FILE}: {e}")
        print(f"📁 Intentando escribir en directorio actual...")
        
        try:
            with open('sg_j3_complete_capture.log', 'w', encoding='utf-8') as f:
                f.write(f"🔍 SG J3 COMPLETE TRAFFIC CAPTURE - Started at {datetime.now()}\n")
                f.write(f"📡 Capturing ABSOLUTELY EVERYTHING - NO FILTROS\n")
                f.write(f"🚀 NUESTRO BACKEND (MÁXIMA PRIORIDAD): {', '.join(OUR_BACKEND_DOMAINS)}\n")
                f.write(f"📋 Capturando TODOS los headers y TODO el contenido\n")
                f.write("="*80 + "\n\n")
            
            print("🔍 SG J3 COMPLETE TRAFFIC CAPTURE INICIADO (directorio actual)")
            print("📁 Archivo de log: sg_j3_complete_capture.log")
            print("="*80)
            
        except Exception as e2:
            print(f"❌ Error crítico inicializando log: {e2}")
            print("⚠️ El script funcionará pero NO guardará logs en archivo")
