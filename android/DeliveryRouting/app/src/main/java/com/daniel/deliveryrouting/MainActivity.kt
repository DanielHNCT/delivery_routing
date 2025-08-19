package com.daniel.deliveryrouting

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
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
import com.daniel.deliveryrouting.presentation.main.MainViewModel
import com.daniel.deliveryrouting.presentation.colis.ColisAuthViewModel
import com.daniel.deliveryrouting.data.repository.ColisRepository
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
        // ✅ CAMBIO: Ir directamente a lista de paquetes (sin selección)
        val app = DeliveryRoutingApplication.getInstance()
        
        val mainViewModel = remember {
            MainViewModel(app.deliveryRepository, app.locationRepository)
        }
        
        // ✅ CARGAR AUTOMÁTICAMENTE la tournée del día actual
        LaunchedEffect(Unit) {
            mainViewModel.loadCurrentDayTournee()
        }
        
        // ✅ MOSTRAR SOLO la lista de paquetes (sin campos de selección)
        PackageListScreen(
            viewModel = mainViewModel,
            onPackageClick = { packageItem ->
                // TODO: Implementar navegación a detalles del paquete
            }
        )
    } else {
        // 🚀 NUEVA: Pantalla de login personalizada
        val app = DeliveryRoutingApplication.getInstance()
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