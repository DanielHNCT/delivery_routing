package com.daniel.deliveryrouting.presentation.map

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.TextButton
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.core.content.ContextCompat
import com.daniel.deliveryrouting.data.api.models.PackageData
import com.daniel.deliveryrouting.ui.theme.DeliveryRoutingTheme
import com.mapbox.maps.MapView
import com.mapbox.maps.Style
import com.mapbox.maps.CameraOptions
import com.mapbox.geojson.Point
import androidx.compose.ui.viewinterop.AndroidView
import com.mapbox.maps.MapboxMap
import com.mapbox.maps.extension.style.sources.generated.geoJsonSource
import com.mapbox.maps.extension.style.layers.generated.circleLayer
import com.mapbox.maps.extension.style.sources.addSource
import com.mapbox.maps.extension.style.layers.addLayer

private const val TAG_MAP = "PackageMapScreen"

@Composable
fun PackageMapScreen(
    packages: List<PackageData>,
    onBackClick: () -> Unit
) {
    Log.d(TAG_MAP, "🗺️ PackageMapScreen iniciado con ${packages.size} paquetes")
    val context = LocalContext.current
    var hasLocationPermission by remember { mutableStateOf(false) }
    var showPermissionDialog by remember { mutableStateOf(false) }
    
    // Registrar para solicitar permisos
    val requestPermissionLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestPermission()
    ) { isGranted: Boolean ->
        hasLocationPermission = isGranted
        showPermissionDialog = false
        Log.d(TAG_MAP, "📍 Resultado de permisos: ${if (isGranted) "✅ Concedidos" else "❌ Denegados"}")
    }
    
    // Verificar permisos de ubicación
    LaunchedEffect(Unit) {
        hasLocationPermission = ContextCompat.checkSelfPermission(
            context,
            Manifest.permission.ACCESS_FINE_LOCATION
        ) == PackageManager.PERMISSION_GRANTED
        
        Log.d(TAG_MAP, "📍 Permisos de ubicación: ${if (hasLocationPermission) "✅ Concedidos" else "❌ No concedidos"}")
        
        // Si no tiene permisos, mostrar diálogo automáticamente
        if (!hasLocationPermission) {
            showPermissionDialog = true
        }
    }
    
    Column(
        modifier = Modifier.fillMaxSize()
    ) {
        // Header con botón de regreso
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "Mapa de Paquetes",
                style = MaterialTheme.typography.headlineMedium
            )
            TextButton(onClick = {
                Log.d(TAG_MAP, "⬅️ Botón regresar presionado")
                onBackClick()
            }) {
                Text("← Regresar")
            }
        }
        
        // Mapa
        Box(
            modifier = Modifier
                .fillMaxSize()
                .weight(1f)
        ) {
            if (hasLocationPermission) {
                // 🗺️ MAPA REAL DE MAPBOX
                Log.d(TAG_MAP, "🗺️ Mostrando mapa real de Mapbox con ${packages.size} paquetes")
                
                // 🗺️ MAPA REAL DE MAPBOX CON MARCADORES
                AndroidView(
                    modifier = Modifier.fillMaxSize(),
                    factory = { context ->
                        MapView(context).apply {
                            Log.d(TAG_MAP, "🗺️ Creando MapView con ${packages.size} paquetes...")
                            
                            // Configurar el mapa según la documentación oficial
                            getMapboxMap().loadStyleUri(Style.MAPBOX_STREETS) { style ->
                                Log.d(TAG_MAP, "🗺️ Estilo MAPBOX_STREETS cargado exitosamente")
                                
                                // Configurar cámara centrada en Paris
                                val cameraOptions = CameraOptions.Builder()
                                    .center(Point.fromLngLat(2.3522, 48.8566)) // Paris
                                    .zoom(12.0)
                                    .build()
                                
                                getMapboxMap().setCamera(cameraOptions)
                                Log.d(TAG_MAP, "🗺️ Cámara configurada en Paris (2.3522, 48.8566)")
                                
                                // 📍 AGREGAR MARCADORES DE PAQUETES
                                addPackageMarkers(style, packages)
                            }
                        }
                    }
                )
            } else {
                // Mensaje si no hay permisos
                Log.d(TAG_MAP, "❌ Sin permisos de ubicación - mostrando mensaje")
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(16.dp),
                    verticalArrangement = Arrangement.Center,
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Text(
                        text = "🗺️ Mapa de Paquetes",
                        style = MaterialTheme.typography.headlineMedium
                    )
                    Spacer(modifier = Modifier.height(16.dp))
                    Text(
                        text = "Se necesitan permisos de ubicación para mostrar el mapa",
                        style = MaterialTheme.typography.bodyMedium,
                        textAlign = androidx.compose.ui.text.style.TextAlign.Center
                    )
                    Spacer(modifier = Modifier.height(16.dp))
                    Text(
                        text = "Paquetes encontrados: ${packages.size}",
                        style = MaterialTheme.typography.bodyLarge
                    )
                    Spacer(modifier = Modifier.height(24.dp))
                    
                    // Botón para solicitar permisos manualmente
                    Button(
                        onClick = {
                            Log.d(TAG_MAP, "🔄 Usuario solicitó permisos manualmente")
                            showPermissionDialog = true
                        }
                    ) {
                        Text("📍 Solicitar Permisos de Ubicación")
                    }
                }
            }
        }
        
        // Footer con información
        Card(
            modifier = Modifier
                .fillMaxWidth()
                .padding(8.dp)
        ) {
            Column(
                modifier = Modifier.padding(16.dp)
            ) {
                Text(
                    text = "📦 Paquetes: ${packages.size}",
                    style = MaterialTheme.typography.bodyMedium
                )
                if (packages.isNotEmpty()) {
                    Text(
                        text = "Primer paquete: ${packages.first().trackingNumber}",
                        style = MaterialTheme.typography.bodySmall
                    )
                }
            }
        }
    }
    
    // 🗺️ DIÁLOGO DE PERMISOS DE UBICACIÓN
    if (showPermissionDialog) {
        AlertDialog(
            onDismissRequest = { 
                showPermissionDialog = false
                Log.d(TAG_MAP, "❌ Usuario canceló solicitud de permisos")
            },
            title = {
                Text("📍 Permisos de Ubicación")
            },
            text = {
                Text(
                    "Para mostrar el mapa con la ubicación de los paquetes, " +
                    "necesitamos acceso a tu ubicación.\n\n" +
                    "Esto nos permitirá:\n" +
                    "• Mostrar tu posición en el mapa\n" +
                    "• Calcular rutas optimizadas\n" +
                    "• Navegar entre paquetes\n\n" +
                    "¿Permitir acceso a la ubicación?"
                )
            },
            confirmButton = {
                TextButton(
                    onClick = {
                        Log.d(TAG_MAP, "✅ Usuario aceptó solicitar permisos")
                        requestPermissionLauncher.launch(Manifest.permission.ACCESS_FINE_LOCATION)
                    }
                ) {
                    Text("✅ Permitir")
                }
            },
            dismissButton = {
                TextButton(
                    onClick = { 
                        showPermissionDialog = false
                        Log.d(TAG_MAP, "❌ Usuario rechazó permisos")
                    }
                ) {
                    Text("❌ Cancelar")
                }
            }
        )
    }
}

// 📍 FUNCIÓN PARA AGREGAR MARCADORES DE PAQUETES USANDO CÍRCULOS SIMPLES
private fun addPackageMarkers(style: Style, packages: List<PackageData>) {
    Log.d(TAG_MAP, "📍 Agregando ${packages.size} marcadores de paquetes usando círculos...")
    
    // Filtrar paquetes que tienen coordenadas
    val packagesWithCoords = packages.filter { 
        it.latitude != null && it.longitude != null 
    }
    
    Log.d(TAG_MAP, "📍 ${packagesWithCoords.size} paquetes tienen coordenadas válidas")
    
    if (packagesWithCoords.isEmpty()) {
        Log.w(TAG_MAP, "⚠️ No hay paquetes con coordenadas para mostrar")
        return
    }
    
    try {
        // Crear FeatureCollection con todos los puntos
        val features = packagesWithCoords.map { packageData ->
            val point = Point.fromLngLat(packageData.longitude!!, packageData.latitude!!)
            com.mapbox.geojson.Feature.fromGeometry(point).apply {
                addStringProperty("id", packageData.id)
                addStringProperty("trackingNumber", packageData.trackingNumber)
                addStringProperty("recipientName", packageData.recipientName)
                addStringProperty("address", packageData.address)
                addStringProperty("status", packageData.status)
                addStringProperty("priority", packageData.priority)
            }
        }
        
        val featureCollection = com.mapbox.geojson.FeatureCollection.fromFeatures(features)
        
        // Agregar fuente de datos
        style.addSource(
            geoJsonSource("packages-source") {
                featureCollection(featureCollection)
            }
        )
        
        // Agregar capa de círculos para los marcadores
        style.addLayer(
            circleLayer("packages-layer", "packages-source") {
                circleRadius(8.0)
                circleColor("#FF0000") // Rojo
                circleStrokeColor("#FFFFFF") // Borde blanco
                circleStrokeWidth(2.0)
            }
        )
        
        Log.d(TAG_MAP, "✅ ${packagesWithCoords.size} marcadores de paquetes agregados exitosamente")
        
        // Log de cada paquete agregado
        packagesWithCoords.forEach { packageData ->
            Log.d(TAG_MAP, "📍 Paquete ${packageData.trackingNumber}: ${packageData.latitude}, ${packageData.longitude}")
        }
        
    } catch (e: Exception) {
        Log.e(TAG_MAP, "❌ Error agregando marcadores: ${e.message}", e)
    }
}
