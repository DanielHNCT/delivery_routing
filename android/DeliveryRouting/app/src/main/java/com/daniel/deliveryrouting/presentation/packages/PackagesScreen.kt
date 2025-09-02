package com.daniel.deliveryrouting.presentation.packages

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.LocationOn
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.daniel.deliveryrouting.data.api.models.PackageData
import android.util.Log

private const val TAG_PACKAGES = "PackagesScreen"

@Composable
fun PackagesScreen(
    packages: List<PackageData>,
    isLoading: Boolean,
    onRefresh: () -> Unit,
    onLogout: () -> Unit,
    onMapClick: () -> Unit = {}
) {
    Log.d(TAG_PACKAGES, "📦 PackagesScreen iniciado con ${packages.size} paquetes, isLoading: $isLoading")
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        // Header con botón de sincronizar
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "Paquetes de Entrega",
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold
            )
            
            Row {
                // 🗺️ Botón del mapa
                IconButton(
                    onClick = {
                        Log.d(TAG_PACKAGES, "🗺️ Botón de mapa presionado - paquetes: ${packages.size}")
                        onMapClick()
                    },
                    enabled = packages.isNotEmpty()
                ) {
                    Icon(
                        imageVector = Icons.Default.LocationOn,
                        contentDescription = "Ver Mapa"
                    )
                }
                
                // Botón de sincronizar
                IconButton(
                    onClick = {
                        Log.d(TAG_PACKAGES, "🔄 Botón de sincronizar presionado")
                        onRefresh()
                    },
                    enabled = !isLoading
                ) {
                    if (isLoading) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(24.dp),
                            strokeWidth = 2.dp
                        )
                    } else {
                        Icon(
                            imageVector = Icons.Default.Refresh,
                            contentDescription = "Sincronizar"
                        )
                    }
                }
                
                // Botón de logout
                TextButton(onClick = {
                    Log.d(TAG_PACKAGES, "🚪 Botón de logout presionado")
                    onLogout()
                }) {
                    Text("Cerrar Sesión")
                }
            }
        }
        
        Spacer(modifier = Modifier.height(16.dp))
        
        // Lista de paquetes
        if (packages.isEmpty() && !isLoading) {
            // Estado vacío
            Box(
                modifier = Modifier.fillMaxSize(),
                contentAlignment = Alignment.Center
            ) {
                Column(
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Text(
                        text = "No hay paquetes disponibles",
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(
                        text = "Usa el botón de sincronizar para actualizar",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }
        } else {
            LazyColumn(
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(packages) { packageItem ->
                    PackageCard(packageItem = packageItem)
                }
            }
        }
    }
}

@Composable
fun PackageCard(packageItem: PackageData) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(
                    text = packageItem.trackingNumber,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold
                )
                Text(
                    text = packageItem.status,
                    style = MaterialTheme.typography.bodyMedium,
                    color = when (packageItem.status) {
                        "Pendiente" -> MaterialTheme.colorScheme.primary
                        "Entregado" -> MaterialTheme.colorScheme.tertiary
                        "Fallido" -> MaterialTheme.colorScheme.error
                        else -> MaterialTheme.colorScheme.onSurface
                    }
                )
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            Text(
                text = "Destinatario: ${packageItem.recipientName}",
                style = MaterialTheme.typography.bodyMedium
            )
            
            Text(
                text = "Dirección: ${packageItem.address}",
                style = MaterialTheme.typography.bodyMedium
            )
            
            if (packageItem.instructions.isNotEmpty()) {
                Text(
                    text = "Instrucciones: ${packageItem.instructions}",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }
    }
}
