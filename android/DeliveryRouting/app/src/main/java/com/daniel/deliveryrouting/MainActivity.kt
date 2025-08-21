package com.daniel.deliveryrouting

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import com.daniel.deliveryrouting.presentation.login.LoginScreen
import com.daniel.deliveryrouting.presentation.login.LoginViewModel
import com.daniel.deliveryrouting.presentation.packages.PackageListScreen
import com.daniel.deliveryrouting.presentation.tournee.TourneeScreen
import com.daniel.deliveryrouting.presentation.main.MainViewModel
import com.daniel.deliveryrouting.data.repository.ColisRepository
import com.daniel.deliveryrouting.data.token.ColisTokenManager
import com.daniel.deliveryrouting.ui.theme.DeliveryRoutingTheme

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add Mapbox initialization
// 2. Add location permissions handling
// 3. Add map state management

class MainActivity : ComponentActivity() {
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        setContent {
            DeliveryRoutingTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    DeliveryRoutingApp()
                }
            }
        }
    }
}

@Composable
fun DeliveryRoutingApp() {
    var isLoggedIn by remember { mutableStateOf(false) }
    
    if (isLoggedIn) {
        // 🆕 NUEVA: App principal con tabs
        MainAppWithTabs()
    } else {
        // 🚀 NUEVA: Pantalla de login personalizada
        val context = LocalContext.current
        
        val colisRepository = remember(context) {
            ColisRepository(context)
        }
        
        LoginScreen(
            onLoginSuccess = {
                isLoggedIn = true
            }
        )
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainAppWithTabs() {
    var selectedTabIndex by remember { mutableStateOf(0) }
    
    val tabs = listOf(
        TabItem("Tournée", Icons.Default.Star),
        TabItem("Lista Original", Icons.Default.List)
    )
    
    Column(modifier = Modifier.fillMaxSize()) {
        // Top App Bar
        TopAppBar(
            title = { Text("Delivery Routing") },
            colors = TopAppBarDefaults.topAppBarColors(
                containerColor = MaterialTheme.colorScheme.primaryContainer
            )
        )
        
        // Tab Row
        TabRow(selectedTabIndex = selectedTabIndex) {
            tabs.forEachIndexed { index, tab ->
                Tab(
                    selected = selectedTabIndex == index,
                    onClick = { selectedTabIndex = index },
                    text = { Text(tab.title) },
                    icon = { Icon(tab.icon, contentDescription = null) }
                )
            }
        }
        
        // Content based on selected tab
        when (selectedTabIndex) {
            0 -> {
                // 🆕 NUEVA: Pantalla de Tournée
                TourneeScreen(modifier = Modifier.fillMaxSize())
            }
            1 -> {
                // ✅ Lista original de paquetes
                val context = LocalContext.current
                
                val colisRepository = remember(context) {
                    ColisRepository(context)
                }
                
                val tokenManager = remember(context) {
                    ColisTokenManager(context)
                }
                
                val mainViewModel = remember {
                    MainViewModel(colisRepository, tokenManager)
                }
                
                // ✅ VERIFICAR ESTADO DE AUTENTICACIÓN Y CARGAR TOURNÉE
                LaunchedEffect(Unit) {
                    mainViewModel.checkAuthenticationStatus()
                }
                
                // ✅ MOSTRAR SOLO la lista de paquetes (sin campos de selección)
                PackageListScreen(
                    viewModel = mainViewModel,
                    onPackageClick = { packageItem ->
                        // TODO: Implementar navegación a detalles del paquete
                    }
                )
            }
        }
    }
}

data class TabItem(
    val title: String,
    val icon: androidx.compose.ui.graphics.vector.ImageVector
)