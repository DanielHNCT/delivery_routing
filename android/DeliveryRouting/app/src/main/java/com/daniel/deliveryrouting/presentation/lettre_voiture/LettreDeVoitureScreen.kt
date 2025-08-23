package com.daniel.deliveryrouting.presentation.lettre_voiture

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.daniel.deliveryrouting.data.repository.ColisRepository

/**
 * 🆕 NUEVO: Pantalla para mostrar Lettre de Voiture
 * 
 * Muestra el documento completo generado por el backend
 */
@Composable
fun LettreDeVoitureScreen(
    matricule: String,
    societe: String,
    date: String? = null,
    repository: ColisRepository,
    modifier: Modifier = Modifier,
    onBackPressed: () -> Unit = {}
) {
    
    val viewModel: LettreDeVoitureViewModel = viewModel {
        LettreDeVoitureViewModel(repository)
    }
    
    val uiState by viewModel.uiState.collectAsState()
    
    // Cargar lettre de voiture al entrar a la pantalla
    LaunchedEffect(Unit) {
        viewModel.getLettreDeVoiture(
            matricule = matricule,
            societe = societe,
            date = date
        )
    }
    
    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        // Header
        Text(
            text = "📋 Lettre de Voiture",
            fontSize = 24.sp,
            fontWeight = FontWeight.Bold,
            modifier = Modifier.fillMaxWidth(),
            textAlign = TextAlign.Center
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        // Información del usuario
        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceVariant
            )
        ) {
            Column(
                modifier = Modifier.padding(16.dp)
            ) {
                Text(
                    text = "👤 Usuario: $matricule",
                    fontSize = 16.sp,
                    fontWeight = FontWeight.Medium
                )
                Text(
                    text = "🏢 Sociedad: $societe",
                    fontSize = 16.sp,
                    fontWeight = FontWeight.Medium
                )
                if (date != null) {
                    Text(
                        text = "📅 Fecha: $date",
                        fontSize = 16.sp,
                        fontWeight = FontWeight.Medium
                    )
                }
            }
        }
        
        Spacer(modifier = Modifier.height(16.dp))
        
        // Contenido del lettre
        when (uiState) {
            is LettreDeVoitureUiState.Idle -> {
                Text(
                    text = "Presiona el botón para cargar el lettre de voiture",
                    textAlign = TextAlign.Center,
                    modifier = Modifier.fillMaxWidth()
                )
            }
            
            is LettreDeVoitureUiState.Loading -> {
                Box(
                    modifier = Modifier.fillMaxWidth(),
                    contentAlignment = Alignment.Center
                ) {
                    CircularProgressIndicator()
                }
            }
            
            is LettreDeVoitureUiState.Success -> {
                val data = uiState.data
                val lettreData = data.data
                
                if (lettreData != null) {
                    // Información de la tournée
                    lettreData.tourneeInfo?.let { tournee ->
                        Card(
                            modifier = Modifier.fillMaxWidth(),
                            colors = CardDefaults.cardColors(
                                containerColor = MaterialTheme.colorScheme.primaryContainer
                            )
                        ) {
                            Column(
                                modifier = Modifier.padding(16.dp)
                            ) {
                                Text(
                                    text = "🚚 Información de Tournée",
                                    fontSize = 18.sp,
                                    fontWeight = FontWeight.Bold
                                )
                                Spacer(modifier = Modifier.height(8.dp))
                                Text("Code: ${tournee.codeTournee}")
                                Text("Estado: ${tournee.statut}")
                                Text("Distributeur: ${tournee.distributeur}")
                                Text("Centre: ${tournee.centre}")
                                Text("Point de Concentration: ${tournee.pointConcentration}")
                            }
                        }
                    }
                    
                    Spacer(modifier = Modifier.height(16.dp))
                    
                    // Resumen de colis
                    Card(
                        modifier = Modifier.fillMaxWidth(),
                        colors = CardDefaults.cardColors(
                            containerColor = MaterialTheme.colorScheme.secondaryContainer
                        )
                    ) {
                        Column(
                            modifier = Modifier.padding(16.dp)
                        ) {
                            Text(
                                text = "📦 Resumen de Colis",
                                fontSize = 18.sp,
                                fontWeight = FontWeight.Bold
                            )
                            Spacer(modifier = Modifier.height(8.dp))
                            Text("Total: ${lettreData.colisSummary.totalColis}")
                            Text("Distribuidos: ${lettreData.colisSummary.colisDistribue}")
                            Text("Restantes: ${lettreData.colisSummary.colisRestant}")
                            Text("Premium: ${lettreData.colisSummary.colisPremium}")
                            Text("Relais: ${lettreData.colisSummary.colisRelais}")
                            Text("Casier: ${lettreData.colisSummary.colisCasier}")
                        }
                    }
                    
                    Spacer(modifier = Modifier.height(16.dp))
                    
                    // Contenido del lettre
                    Card(
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Column(
                            modifier = Modifier.padding(16.dp)
                        ) {
                            Text(
                                text = "📄 Contenido del Lettre",
                                fontSize = 18.sp,
                                fontWeight = FontWeight.Bold
                            )
                            Spacer(modifier = Modifier.height(8.dp))
                            Text(
                                text = lettreData.lettreContent,
                                fontSize = 14.sp,
                                modifier = Modifier.verticalScroll(rememberScrollState())
                            )
                        }
                    }
                    
                    Spacer(modifier = Modifier.height(16.dp))
                    
                    // Botón de reintento
                    Button(
                        onClick = { viewModel.retry() },
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Text("🔄 Actualizar")
                    }
                } else {
                    Text(
                        text = "No se pudo obtener el contenido del lettre",
                        textAlign = TextAlign.Center,
                        modifier = Modifier.fillMaxWidth()
                    )
                }
            }
            
            is LettreDeVoitureUiState.Error -> {
                Card(
                    modifier = Modifier.fillMaxWidth(),
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.errorContainer
                    )
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Text(
                            text = "❌ Error",
                            fontSize = 18.sp,
                            fontWeight = FontWeight.Bold,
                            color = MaterialTheme.colorScheme.onErrorContainer
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                        Text(
                            text = uiState.message,
                            textAlign = TextAlign.Center,
                            color = MaterialTheme.colorScheme.onErrorContainer
                        )
                        Spacer(modifier = Modifier.height(16.dp))
                        Button(
                            onClick = { viewModel.retry() }
                        ) {
                            Text("🔄 Reintentar")
                        }
                    }
                }
            }
        }
        
        Spacer(modifier = Modifier.height(16.dp))
        
        // Botón de regreso
        Button(
            onClick = onBackPressed,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("← Volver")
        }
    }
}
