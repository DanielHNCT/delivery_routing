package com.daniel.deliveryrouting.presentation.main

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.daniel.deliveryrouting.R
import com.daniel.deliveryrouting.data.api.models.Package

import com.daniel.deliveryrouting.utils.LocationUtils

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add MapView composable
// 2. Implement map markers for packages
// 3. Add route display
// 4. Add map controls

@Composable
fun MainScreen(
    viewModel: MainViewModel,
    onPackageClick: (Package) -> Unit
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
    val context = LocalContext.current
    
    Column(
        modifier = Modifier.fillMaxSize()
    ) {
        // Header con controles
        MainHeader(
            tourneeCode = uiState.tourneeCode,
            selectedDate = uiState.selectedDate,
            tourneeCodeError = uiState.tourneeCodeError,
            dateError = uiState.dateError,
            onTourneeCodeChange = viewModel::onTourneeCodeChange,
            onDateChange = viewModel::onDateChange,
            onLoadClick = viewModel::onLoadTourneeClick,
            isLoading = uiState.isLoading
        )
        
        // Contenido principal
        when (uiState.viewMode) {
            ViewMode.LIST -> {
                PackageListView(
                    packages = uiState.packages,
                    isLoading = uiState.isLoading,
                    error = uiState.error,
                    sortOrder = uiState.sortOrder,
                    onPackageClick = onPackageClick,
                    onRefresh = viewModel::onRefresh,
                    onSortByDistance = viewModel::onSortByDistance,
                    onSortByReference = viewModel::onSortByReference,
                    onClearError = viewModel::clearError
                )
            }
            ViewMode.MAP -> {
                // TODO: Implementar vista de mapa cuando se agregue Mapbox
                MapPlaceholderView(
                    onBackToList = { viewModel.onToggleViewMode() }
                )
            }
        }
        
        // FAB para cambiar vista
        FloatingActionButton(
            onClick = viewModel::onToggleViewMode,
            modifier = Modifier
                .padding(16.dp)
                .align(Alignment.End)
        ) {
            Icon(
                imageVector = if (uiState.viewMode == ViewMode.LIST) {
                    Icons.Default.Place
                } else {
                    Icons.Default.List
                },
                contentDescription = "Cambiar vista"
            )
        }
    }
    
    // Dialog de paquete seleccionado
    uiState.selectedPackage?.let { packageItem ->
        PackageDetailDialog(
            packageItem = packageItem,
            onDismiss = viewModel::onPackageDismiss
        )
    }
}

@Composable
private fun MainHeader(
    tourneeCode: String,
    selectedDate: String,
    tourneeCodeError: String?,
    dateError: String?,
    onTourneeCodeChange: (String) -> Unit,
    onDateChange: (String) -> Unit,
    onLoadClick: () -> Unit,
    isLoading: Boolean
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp),
        elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            Text(
                text = "Mi Tournée del Día",
                style = MaterialTheme.typography.headlineSmall,
                fontWeight = FontWeight.Bold,
                modifier = Modifier.padding(16.dp)
            )
            
            Spacer(modifier = Modifier.height(16.dp))
            
            // Campo de código de tournée (solo lectura para livreurs)
            OutlinedTextField(
                value = tourneeCode,
                onValueChange = { }, // Solo lectura
                label = { Text("Mi Código de Tournée") },
                singleLine = true,
                readOnly = true,
                enabled = false,
                modifier = Modifier.fillMaxWidth()
            )
            
            Spacer(modifier = Modifier.height(16.dp))
            
            // Campo de fecha (solo lectura para livreurs)
            OutlinedTextField(
                value = selectedDate,
                onValueChange = { }, // Solo lectura
                label = { Text("Fecha del Tournée") },
                singleLine = true,
                readOnly = true,
                enabled = false,
                modifier = Modifier.fillMaxWidth()
            )
            
            Spacer(modifier = Modifier.height(16.dp))
            
            // Botón de actualizar (para jefes que quieran cambiar tournée)
            Button(
                onClick = onLoadClick,
                enabled = !isLoading,
                modifier = Modifier
                    .fillMaxWidth()
                    .height(48.dp)
            ) {
                if (isLoading) {
                    CircularProgressIndicator(
                        modifier = Modifier.size(20.dp),
                        color = MaterialTheme.colorScheme.onPrimary
                    )
                } else {
                    Text("Actualizar Tournée")
                }
            }
        }
    }
}

@Composable
private fun PackageListView(
    packages: List<Package>,
    isLoading: Boolean,
    error: String?,
    sortOrder: SortOrder,
    onPackageClick: (Package) -> Unit,
    onRefresh: () -> Unit,
    onSortByDistance: () -> Unit,
    onSortByReference: () -> Unit,
    onClearError: () -> Unit
) {
    Column(
        modifier = Modifier.fillMaxSize()
    ) {
        // Controles de ordenamiento
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp, vertical = 8.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "Paquetes: ${packages.size}",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Medium
            )
            
            Row {
                OutlinedButton(
                    onClick = onSortByReference,
                    colors = ButtonDefaults.outlinedButtonColors(
                        containerColor = if (sortOrder == SortOrder.REFERENCE) {
                            MaterialTheme.colorScheme.primaryContainer
                        } else {
                            MaterialTheme.colorScheme.surface
                        }
                    )
                ) {
                    Text("Por Referencia")
                }
                
                Spacer(modifier = Modifier.width(8.dp))
                
                OutlinedButton(
                    onClick = onSortByDistance,
                    colors = ButtonDefaults.outlinedButtonColors(
                        containerColor = if (sortOrder == SortOrder.DISTANCE) {
                            MaterialTheme.colorScheme.primaryContainer
                        } else {
                            MaterialTheme.colorScheme.surface
                        }
                    )
                ) {
                    Text("Por Distancia")
                }
            }
        }
        
        // Lista de paquetes
        if (packages.isNotEmpty()) {
            LazyColumn(
                modifier = Modifier.fillMaxSize(),
                contentPadding = PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(packages) { packageItem ->
                    PackageCard(
                        packageItem = packageItem,
                        onClick = { onPackageClick(packageItem) }
                    )
                }
            }
        } else if (!isLoading) {
            // Estado vacío
            Box(
                modifier = Modifier.fillMaxSize(),
                contentAlignment = Alignment.Center
            ) {
                Column(
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                                    Icon(
                    imageVector = Icons.Default.Info,
                    contentDescription = "Sin paquetes",
                    modifier = Modifier.size(64.dp),
                    tint = MaterialTheme.colorScheme.onSurfaceVariant
                )
                    
                    Spacer(modifier = Modifier.height(16.dp))
                    
                    Text(
                        text = "No hay paquetes para mostrar",
                        style = MaterialTheme.typography.titleMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    
                    Text(
                        text = "Carga un tournée para comenzar",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        textAlign = TextAlign.Center
                    )
                }
            }
        }
        
        // Mostrar error
        if (error != null) {
            ErrorCard(
                error = error,
                onDismiss = onClearError
            )
        }
    }
}

@Composable
private fun PackageCard(
    packageItem: Package,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth(),
        elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            // Header del paquete
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
                
                // Chip de acción
                AssistChip(
                    onClick = { },
                    label = { Text(packageItem.action.label) },
                    colors = AssistChipDefaults.assistChipColors(
                        containerColor = packageItem.action.color?.let { color ->
                            try {
                                Color(android.graphics.Color.parseColor(color))
                            } catch (e: Exception) {
                                MaterialTheme.colorScheme.primaryContainer
                            }
                        } ?: MaterialTheme.colorScheme.primaryContainer
                    )
                )
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            // Información del paquete
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Column(modifier = Modifier.weight(1f)) {
                    Text(
                        text = "Barcode: ${packageItem.barcode}",
                        style = MaterialTheme.typography.bodyMedium
                    )
                    
                    Text(
                        text = "Estado: ${packageItem.status.label}",
                        style = MaterialTheme.typography.bodyMedium,
                        color = if (packageItem.status.isCompleted) {
                            MaterialTheme.colorScheme.primary
                        } else {
                            MaterialTheme.colorScheme.onSurface
                        }
                    )
                }
                
                // Información de ubicación
                if (packageItem.location.hasCoordinates) {
                    Column(
                        horizontalAlignment = Alignment.End
                    ) {
                        AssistChip(
                            onClick = { },
                            label = { 
                                Text("GPS: ${packageItem.location.gpsQualityMeters ?: "Preciso"}m") 
                            },
                            leadingIcon = {
                                Icon(
                                    imageVector = Icons.Default.LocationOn,
                                    contentDescription = "GPS"
                                )
                            },
                            colors = AssistChipDefaults.assistChipColors(
                                containerColor = MaterialTheme.colorScheme.secondaryContainer
                            )
                        )
                        
                        Spacer(modifier = Modifier.height(4.dp))
                        
                        Text(
                            text = "Lat: ${packageItem.location.latitude?.let { "%.6f".format(it) } ?: "N/A"}",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        
                        Text(
                            text = "Lng: ${packageItem.location.longitude?.let { "%.6f".format(it) } ?: "N/A"}",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                }
            }
            
            // Botón para mostrar en mapa (preparado para Mapbox)
            if (packageItem.location.hasCoordinates) {
                Spacer(modifier = Modifier.height(12.dp))
                
                OutlinedButton(
                    onClick = { /* TODO: Implementar navegación a mapa */ },
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Icon(
                        imageVector = Icons.Default.Place,
                        contentDescription = "Mostrar en mapa"
                    )
                    
                    Spacer(modifier = Modifier.width(8.dp))
                    
                    Text("Mostrar en Mapa")
                }
            }
        }
    }
}

@Composable
private fun MapPlaceholderView(
    onBackToList: () -> Unit
) {
    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Icon(
                imageVector = Icons.Default.Place,
                contentDescription = "Mapa",
                modifier = Modifier.size(80.dp),
                tint = MaterialTheme.colorScheme.primary
            )
            
            Spacer(modifier = Modifier.height(16.dp))
            
            Text(
                text = "Vista de Mapa",
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold
            )
            
            Text(
                text = "Próximamente con Mapbox",
                style = MaterialTheme.typography.bodyLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                textAlign = TextAlign.Center
            )
            
            Spacer(modifier = Modifier.height(24.dp))
            
            Button(
                onClick = onBackToList
            ) {
                Text("Volver a Lista")
            }
        }
    }
}

@Composable
private fun PackageDetailDialog(
    packageItem: Package,
    onDismiss: () -> Unit
) {
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Detalles del Paquete") },
        text = {
            Column {
                Text("Referencia: ${packageItem.reference}")
                Text("Barcode: ${packageItem.barcode}")
                Text("Acción: ${packageItem.action.label}")
                Text("Estado: ${packageItem.status.label}")
                
                if (packageItem.location.hasCoordinates) {
                    Spacer(modifier = Modifier.height(8.dp))
                    Text("Ubicación GPS:")
                    Text("Lat: ${packageItem.location.latitude?.let { "%.6f".format(it) } ?: "N/A"}")
                    Text("Lng: ${packageItem.location.longitude?.let { "%.6f".format(it) } ?: "N/A"}")
                    Text("Precisión: ${packageItem.location.gpsQualityMeters ?: "N/A"}m")
                }
                
                if (packageItem.sender.name.isNotEmpty()) {
                    Spacer(modifier = Modifier.height(8.dp))
                    Text("Remitente: ${packageItem.sender.name}")
                    packageItem.sender.phone?.let { Text("Tel: $it") }
                    packageItem.sender.email?.let { Text("Email: $it") }
                }
            }
        },
        confirmButton = {
            TextButton(onClick = onDismiss) {
                Text("Cerrar")
            }
        }
    )
}

@Composable
private fun ErrorCard(
    error: String,
    onDismiss: () -> Unit
) {
    Card(
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.errorContainer
        ),
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp)
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = Icons.Default.Warning,
                contentDescription = "Error",
                tint = MaterialTheme.colorScheme.onErrorContainer
            )
            
            Spacer(modifier = Modifier.width(12.dp))
            
            Text(
                text = error,
                color = MaterialTheme.colorScheme.onErrorContainer,
                modifier = Modifier.weight(1f)
            )
            
            IconButton(onClick = onDismiss) {
                Icon(
                    imageVector = Icons.Default.Close,
                    contentDescription = "Cerrar",
                    tint = MaterialTheme.colorScheme.onErrorContainer
                )
            }
        }
    }
}
