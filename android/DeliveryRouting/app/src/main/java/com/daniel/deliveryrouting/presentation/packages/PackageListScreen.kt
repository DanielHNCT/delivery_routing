package com.daniel.deliveryrouting.presentation.packages

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.daniel.deliveryrouting.data.api.models.Package
import com.daniel.deliveryrouting.presentation.main.MainViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PackageListScreen(
    viewModel: MainViewModel,
    onPackageClick: (Package) -> Unit
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    
    Scaffold(
        topBar = {
            TopAppBar(
                title = { 
                    Column {
                        Text("Delivery Routing")
                        Text(
                            text = "Tournée del día: ${uiState.selectedDate}",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                },
                actions = {
                    IconButton(onClick = { viewModel.loadCurrentDayTournee() }) {
                        if (uiState.isLoading) {
                            CircularProgressIndicator(
                                modifier = Modifier.size(24.dp),
                                strokeWidth = 2.dp
                            )
                        } else {
                            Icon(Icons.Default.Refresh, contentDescription = "Actualizar")
                        }
                    }
                }
            )
        }
    ) { paddingValues ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
        ) {
            // Mostrar error si existe
            uiState.error?.let { error ->
                Card(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(16.dp),
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.errorContainer
                    )
                ) {
                    Row(
                        modifier = Modifier.padding(16.dp),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Icon(
                            Icons.Default.Warning,
                            contentDescription = null,
                            tint = MaterialTheme.colorScheme.onErrorContainer
                        )
                        Spacer(modifier = Modifier.width(8.dp))
                        Text(
                            text = error,
                            color = MaterialTheme.colorScheme.onErrorContainer,
                            modifier = Modifier.weight(1f)
                        )
                        IconButton(onClick = viewModel::clearError) {
                            Icon(Icons.Default.Close, contentDescription = "Cerrar")
                        }
                    }
                }
            }
            
            // Mostrar paquetes
            if (uiState.packages.isNotEmpty()) {
                LazyColumn(
                    modifier = Modifier.fillMaxSize(),
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    items(uiState.packages) { packageItem ->
                        PackageCard(
                            packageItem = packageItem,
                            onClick = { onPackageClick(packageItem) }
                        )
                    }
                }
            } else if (!uiState.isLoading) {
                // Estado vacío
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Icon(
                            Icons.Default.Info,
                            contentDescription = null,
                            modifier = Modifier.size(64.dp),
                            tint = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        Spacer(modifier = Modifier.height(16.dp))
                        Text(
                            text = "No hay paquetes para mostrar",
                            style = MaterialTheme.typography.bodyLarge,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                        Text(
                            text = "Toca el botón de actualizar para cargar la tournée",
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                }
            }
            
            // Loading state
            if (uiState.isLoading) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    CircularProgressIndicator()
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun PackageCard(
    packageItem: Package,
    onClick: () -> Unit
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        onClick = onClick,
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surface
        )
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = packageItem.reference,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold
                )
                StatusChip(status = packageItem.status)
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            Text(
                text = packageItem.location.formattedAddress ?: "Dirección no disponible",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
            
            if (packageItem.location.city != null) {
                Text(
                    text = "${packageItem.location.city} ${packageItem.location.postalCode ?: ""}",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(
                    text = "Tournée: ${packageItem.tourneeCode}",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.primary
                )
                
                if (packageItem.sender.name.isNotEmpty()) {
                    Text(
                        text = packageItem.sender.name,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }
        }
    }
}

@Composable
private fun StatusChip(status: com.daniel.deliveryrouting.data.api.models.PackageStatus) {
    val backgroundColor = when {
        status.isCompleted -> MaterialTheme.colorScheme.primaryContainer
        else -> MaterialTheme.colorScheme.secondaryContainer
    }
    
    val textColor = when {
        status.isCompleted -> MaterialTheme.colorScheme.onPrimaryContainer
        else -> MaterialTheme.colorScheme.onSecondaryContainer
    }
    
    Surface(
        color = backgroundColor,
        shape = MaterialTheme.shapes.small
    ) {
        Text(
            text = status.label,
            style = MaterialTheme.typography.bodySmall,
            color = textColor,
            modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
        )
    }
}
