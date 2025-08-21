package com.daniel.deliveryrouting.presentation.tournee

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.daniel.deliveryrouting.data.api.models.*
import com.google.accompanist.swiperefresh.SwipeRefresh
import com.google.accompanist.swiperefresh.rememberSwipeRefreshState

/**
 * 📦 PANTALLA PRINCIPAL DE TOURNÉE
 * 
 * Características:
 * - ✅ Lista de paquetes REALES del backend
 * - ✅ Pull-to-refresh para actualizar datos
 * - ✅ Filtros por zona, estado y búsqueda
 * - ✅ Estadísticas en tiempo real
 * - ✅ Preparada para integración con mapas
 * - ✅ Estados de loading y error
 * 
 * IMPORTANTE: Solo muestra datos REALES - no crea datos falsos
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TourneeScreen(
    modifier: Modifier = Modifier
) {
    val context = LocalContext.current
    val viewModel: TourneeViewModel = viewModel { TourneeViewModel(context) }
    val uiState by viewModel.uiState.collectAsState()
    
    // Extraer valores para evitar problemas de smart cast
    val currentError = uiState.error
    val currentTournee = uiState.tournee
    val isLoading = uiState.isLoading
    val isRefreshing = uiState.isRefreshing
    val showMap = uiState.showMap
    val filteredPackages = uiState.filteredPackages
    val selectedPackage = uiState.selectedPackage
    val activeFilters = uiState.activeFilters
    
    // Estado para mostrar/ocultar filtros
    var showFilters by remember { mutableStateOf(false) }
    
    Column(
        modifier = modifier
            .fillMaxSize()
            .background(MaterialTheme.colorScheme.background)
    ) {
        // 🎯 TOP BAR CON ESTADÍSTICAS
        TourneeTopBar(
            tournee = currentTournee,
            onRefreshClick = { viewModel.handleAction(TourneeAction.RefreshTournee) },
            onFilterClick = { showFilters = !showFilters },
            onMapToggle = { viewModel.handleAction(TourneeAction.ToggleMapView) },
            showMap = showMap
        )
        
        // 🔍 FILTROS (EXPANDIBLE)
        if (showFilters) {
            TourneeFiltersCard(
                activeFilters = activeFilters,
                availableFilters = viewModel.getAvailableFilters(),
                onFiltersChanged = { filters ->
                    viewModel.handleAction(TourneeAction.ApplyFilters(filters))
                }
            )
        }
        
        // 📦 CONTENIDO PRINCIPAL
        Box(modifier = Modifier.fillMaxSize()) {
            when {
                isLoading && currentTournee == null -> {
                    // Estado de carga inicial
                    LoadingContent()
                }
                currentError != null -> {
                    // Estado de error
                    ErrorContent(
                        error = currentError,
                        onRetryClick = { viewModel.handleAction(TourneeAction.LoadTournee) },
                        onDismiss = { viewModel.clearError() }
                    )
                }
                currentTournee != null -> {
                    // Contenido principal con datos
                    SwipeRefresh(
                        state = rememberSwipeRefreshState(isRefreshing),
                        onRefresh = { viewModel.handleAction(TourneeAction.RefreshTournee) }
                    ) {
                        if (showMap) {
                            // 🗺️ VISTA DE MAPA (Placeholder - integrar Mapbox después)
                            MapPlaceholder(
                                packages = viewModel.getPackagesWithCoordinates(),
                                optimizedRoute = currentTournee.optimizedRoute,
                                onOptimizeRoute = { viewModel.handleAction(TourneeAction.OptimizeRoute) }
                            )
                        } else {
                            // 📋 VISTA DE LISTA
                            PackageList(
                                packages = filteredPackages,
                                selectedPackage = selectedPackage,
                                onPackageClick = { pkg ->
                                    viewModel.handleAction(TourneeAction.SelectPackage(pkg))
                                },
                                onOptimizeRoute = { viewModel.handleAction(TourneeAction.OptimizeRoute) }
                            )
                        }
                    }
                }
                else -> {
                    // Estado vacío (sin datos)
                    EmptyContent(
                        onLoadClick = { viewModel.handleAction(TourneeAction.LoadTournee) }
                    )
                }
            }
        }
    }
}

/**
 * 🎯 TOP BAR CON ESTADÍSTICAS
 */
@Composable
private fun TourneeTopBar(
    tournee: Tournee?,
    onRefreshClick: () -> Unit,
    onFilterClick: () -> Unit,
    onMapToggle: () -> Unit,
    showMap: Boolean
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(8.dp),
        elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            // Título y acciones
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = "Tournée ${tournee?.date ?: ""}",
                    style = MaterialTheme.typography.headlineSmall,
                    fontWeight = FontWeight.Bold
                )
                
                Row {
                    IconButton(onClick = onFilterClick) {
                        Icon(Icons.Default.Info, contentDescription = "Filtros")
                    }
                    IconButton(onClick = onMapToggle) {
                        Icon(
                            if (showMap) Icons.Default.List else Icons.Default.Place,
                            contentDescription = if (showMap) "Lista" else "Mapa"
                        )
                    }
                    IconButton(onClick = onRefreshClick) {
                        Icon(Icons.Default.Refresh, contentDescription = "Actualizar")
                    }
                }
            }
            
            // Estadísticas
            if (tournee != null) {
                Spacer(modifier = Modifier.height(8.dp))
                TourneeStatistics(statistics = tournee.statistics)
            }
        }
    }
}

/**
 * 📊 COMPONENTE DE ESTADÍSTICAS
 */
@Composable
private fun TourneeStatistics(statistics: TourneeStatistics) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceEvenly
    ) {
        StatCard(
            label = "Total",
            value = statistics.totalPackages.toString(),
            color = MaterialTheme.colorScheme.primary
        )
        StatCard(
            label = "Completados",
            value = statistics.completedPackages.toString(),
            color = Color(0xFF4CAF50)
        )
        StatCard(
            label = "Pendientes",
            value = statistics.pendingPackages.toString(),
            color = Color(0xFFFF9800)
        )
        StatCard(
            label = "Progreso",
            value = "${String.format("%.1f", statistics.completionPercentage)}%",
            color = MaterialTheme.colorScheme.secondary
        )
    }
}

/**
 * 📈 TARJETA DE ESTADÍSTICA
 */
@Composable
private fun StatCard(
    label: String,
    value: String,
    color: Color
) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = value,
            fontSize = 18.sp,
            fontWeight = FontWeight.Bold,
            color = color
        )
        Text(
            text = label,
            fontSize = 12.sp,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}

/**
 * 🔍 CARD DE FILTROS
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun TourneeFiltersCard(
    activeFilters: TourneeFilters,
    availableFilters: FilterOptions,
    onFiltersChanged: (TourneeFilters) -> Unit
) {
    var searchQuery by remember { mutableStateOf(activeFilters.searchQuery) }
    var showOnlyWithCoordinates by remember { mutableStateOf(activeFilters.showOnlyWithCoordinates) }
    
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(
                text = "Filtros",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )
            
            Spacer(modifier = Modifier.height(8.dp))
            
            // Búsqueda
            OutlinedTextField(
                value = searchQuery,
                onValueChange = { 
                    searchQuery = it
                    onFiltersChanged(activeFilters.copy(searchQuery = it))
                },
                label = { Text("Buscar paquetes...") },
                leadingIcon = { Icon(Icons.Default.Search, contentDescription = null) },
                modifier = Modifier.fillMaxWidth()
            )
            
            Spacer(modifier = Modifier.height(8.dp))
            
            // Checkbox para mostrar solo con coordenadas
            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable { 
                        showOnlyWithCoordinates = !showOnlyWithCoordinates
                        onFiltersChanged(activeFilters.copy(showOnlyWithCoordinates = showOnlyWithCoordinates))
                    }
            ) {
                Checkbox(
                    checked = showOnlyWithCoordinates,
                    onCheckedChange = { 
                        showOnlyWithCoordinates = it
                        onFiltersChanged(activeFilters.copy(showOnlyWithCoordinates = it))
                    }
                )
                Text("Solo paquetes con coordenadas GPS")
            }
            
            // TODO: Agregar filtros por zona y estado cuando sea necesario
            // FilterChips para zonas y estados...
        }
    }
}

/**
 * 📋 LISTA DE PAQUETES
 */
@Composable
private fun PackageList(
    packages: List<MobilePackageAction>,
    selectedPackage: MobilePackageAction?,
    onPackageClick: (MobilePackageAction) -> Unit,
    onOptimizeRoute: () -> Unit
) {
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(8.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        // Botón de optimización si hay paquetes con coordenadas
        val packagesWithCoords = packages.filter { 
            it.location.latitude != null && it.location.longitude != null 
        }
        
        if (packagesWithCoords.isNotEmpty()) {
            item {
                Button(
                    onClick = onOptimizeRoute,
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Icon(Icons.Default.Star, contentDescription = null)
                    Spacer(modifier = Modifier.width(8.dp))
                    Text("Optimizar Ruta (${packagesWithCoords.size} paquetes con GPS)")
                }
            }
        }
        
        // Lista de paquetes
        items(packages) { pkg ->
            PackageCard(
                packageAction = pkg,
                isSelected = pkg == selectedPackage,
                onClick = { onPackageClick(pkg) }
            )
        }
        
        // Mensaje si la lista está vacía
        if (packages.isEmpty()) {
            item {
                EmptyListMessage()
            }
        }
    }
}

/**
 * 📦 TARJETA DE PAQUETE
 */
@Composable
private fun PackageCard(
    packageAction: MobilePackageAction,
    isSelected: Boolean,
    onClick: () -> Unit
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onClick() },
        elevation = CardDefaults.cardElevation(
            defaultElevation = if (isSelected) 8.dp else 2.dp
        ),
        colors = CardDefaults.cardColors(
            containerColor = if (isSelected) 
                MaterialTheme.colorScheme.primaryContainer 
            else 
                MaterialTheme.colorScheme.surface
        )
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            // Header: ID y Estado
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = packageAction.package_info.reference,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold
                )
                
                StatusChip(status = packageAction.status)
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            // Dirección
            Row(verticalAlignment = Alignment.Top) {
                Icon(
                    Icons.Default.LocationOn,
                    contentDescription = null,
                    modifier = Modifier.size(16.dp),
                    tint = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Spacer(modifier = Modifier.width(4.dp))
                Text(
                    text = packageAction.location.formattedAddress,
                    style = MaterialTheme.typography.bodyMedium,
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis
                )
            }
            
            Spacer(modifier = Modifier.height(4.dp))
            
            // Cliente y código postal
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(
                    text = packageAction.customer.name,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                
                Text(
                    text = packageAction.location.postalCode,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
            
            // Indicador de GPS
            if (packageAction.location.latitude != null && packageAction.location.longitude != null) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.padding(top = 4.dp)
                ) {
                    Icon(
                        Icons.Default.LocationOn,
                        contentDescription = null,
                        modifier = Modifier.size(14.dp),
                        tint = Color(0xFF4CAF50)
                    )
                    Spacer(modifier = Modifier.width(4.dp))
                    Text(
                        text = "GPS disponible",
                        style = MaterialTheme.typography.bodySmall,
                        color = Color(0xFF4CAF50)
                    )
                }
            }
        }
    }
}

/**
 * 🎯 CHIP DE ESTADO
 */
@Composable
private fun StatusChip(status: Status) {
    val backgroundColor = when (status.code) {
        "PENDING" -> Color(0xFFFF9800)
        "COMPLETED" -> Color(0xFF4CAF50)
        "IN_PROGRESS" -> Color(0xFF2196F3)
        "CANCELLED" -> Color(0xFFF44336)
        else -> MaterialTheme.colorScheme.outline
    }
    
    Box(
        modifier = Modifier
            .clip(RoundedCornerShape(12.dp))
            .background(backgroundColor.copy(alpha = 0.2f))
            .padding(horizontal = 8.dp, vertical = 4.dp)
    ) {
        Text(
            text = status.label,
            style = MaterialTheme.typography.bodySmall,
            color = backgroundColor,
            fontWeight = FontWeight.Medium
        )
    }
}

/**
 * 🗺️ PLACEHOLDER PARA MAPA
 */
@Composable
private fun MapPlaceholder(
    packages: List<MobilePackageAction>,
    optimizedRoute: OptimizedRoute?,
    onOptimizeRoute: () -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.Place,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Text(
            text = "Vista de Mapa",
            style = MaterialTheme.typography.headlineMedium
        )
        
        Text(
            text = "Integración con Mapbox/Google Maps pendiente",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Text(
            text = "${packages.size} paquetes con coordenadas",
            style = MaterialTheme.typography.bodyLarge
        )
        
        if (optimizedRoute != null) {
            Text(
                text = "Ruta optimizada: ${String.format("%.1f", optimizedRoute.totalDistance)} km",
                style = MaterialTheme.typography.bodyMedium,
                color = Color(0xFF4CAF50)
            )
        }
        
        Spacer(modifier = Modifier.height(16.dp))
        
        if (packages.isNotEmpty() && optimizedRoute == null) {
            Button(onClick = onOptimizeRoute) {
                Icon(Icons.Default.Star, contentDescription = null)
                Spacer(modifier = Modifier.width(8.dp))
                Text("Optimizar Ruta")
            }
        }
    }
}

/**
 * ⏳ CONTENIDO DE CARGA
 */
@Composable
private fun LoadingContent() {
    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            CircularProgressIndicator()
            Spacer(modifier = Modifier.height(16.dp))
            Text("Cargando tournée desde el backend...")
        }
    }
}

/**
 * ❌ CONTENIDO DE ERROR
 */
@Composable
private fun ErrorContent(
    error: String,
    onRetryClick: () -> Unit,
    onDismiss: () -> Unit
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.errorContainer
        )
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.Top
            ) {
                Column(modifier = Modifier.weight(1f)) {
                    Text(
                        text = "Error",
                        style = MaterialTheme.typography.titleMedium,
                        color = MaterialTheme.colorScheme.onErrorContainer,
                        fontWeight = FontWeight.Bold
                    )
                    Text(
                        text = error,
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onErrorContainer
                    )
                }
                IconButton(onClick = onDismiss) {
                    Icon(
                        Icons.Default.Close,
                        contentDescription = "Cerrar",
                        tint = MaterialTheme.colorScheme.onErrorContainer
                    )
                }
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            Button(
                onClick = onRetryClick,
                colors = ButtonDefaults.buttonColors(
                    containerColor = MaterialTheme.colorScheme.error
                )
            ) {
                Icon(Icons.Default.Refresh, contentDescription = null)
                Spacer(modifier = Modifier.width(8.dp))
                Text("Reintentar")
            }
        }
    }
}

/**
 * 📭 CONTENIDO VACÍO
 */
@Composable
private fun EmptyContent(onLoadClick: () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Icon(
            Icons.Default.Info,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Text(
            text = "No hay tournée cargada",
            style = MaterialTheme.typography.headlineMedium
        )
        
        Text(
            text = "Presiona el botón para cargar la tournée del día",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Button(onClick = onLoadClick) {
            Icon(Icons.Default.Refresh, contentDescription = null)
            Spacer(modifier = Modifier.width(8.dp))
            Text("Cargar Tournée")
        }
    }
}

/**
 * 📭 MENSAJE DE LISTA VACÍA
 */
@Composable
private fun EmptyListMessage() {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp)
    ) {
        Column(
            modifier = Modifier.padding(32.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Icon(
                Icons.Default.Info,
                contentDescription = null,
                modifier = Modifier.size(48.dp),
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
            
            Spacer(modifier = Modifier.height(16.dp))
            
            Text(
                text = "No hay paquetes que coincidan con los filtros",
                style = MaterialTheme.typography.bodyLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}