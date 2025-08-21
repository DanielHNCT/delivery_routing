package com.daniel.deliveryrouting.presentation.login

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/**
 * Datos de empresas disponibles
 */
data class Company(
    val displayName: String,    // Lo que ve el usuario
    val internalCode: String,   // Código interno para API
    val description: String     // Descripción adicional
)

/**
 * Lista de empresas disponibles
 */
object Companies {
    val INTI = Company(
        displayName = "INTI",
        internalCode = "PCP0010699",
        description = "Empresa INTI - Distribución"
    )
    
    // Solo INTI por ahora
    val allCompanies = listOf(INTI)
}

/**
 * Datos de APIs disponibles
 */
data class ApiType(
    val displayName: String,    // Lo que ve el usuario
    val internalCode: String,   // Código interno para API
    val description: String     // Descripción adicional
)

/**
 * Lista de APIs disponibles
 */
object ApiTypes {
    val WEB = ApiType(
        displayName = "Web",
        internalCode = "web",
        description = "API Web - Más simple y estable"
    )
    
    val MOBILE = ApiType(
        displayName = "Mobile",
        internalCode = "mobile",
        description = "API Mobile - Completa pero compleja"
    )
    
    // Lista de todas las APIs disponibles
    val allApiTypes = listOf(WEB, MOBILE)
}

/**
 * Selector de API y empresa con dropdowns desplegables
 */
@Composable
fun CompanySelector(
    selectedCompany: Company,
    onCompanySelected: (Company) -> Unit,
    selectedApiType: ApiType = ApiTypes.WEB, // Por defecto Web
    onApiTypeSelected: (ApiType) -> Unit = {}, // Callback opcional
    modifier: Modifier = Modifier
) {
    Card(
        modifier = modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceVariant
        )
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            // 🚀 SELECTOR DE API (DROPDOWN)
            Text(
                text = "🌐 Seleccionar API",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
            
            Spacer(modifier = Modifier.height(8.dp))
            
            var apiExpanded by remember { mutableStateOf(false) }
            
            Box {
                Button(
                    onClick = { apiExpanded = true },
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = MaterialTheme.colorScheme.surface
                    )
                ) {
                    Text(
                        text = selectedApiType.displayName,
                        color = MaterialTheme.colorScheme.onSurface
                    )
                }
                
                DropdownMenu(
                    expanded = apiExpanded,
                    onDismissRequest = { apiExpanded = false }
                ) {
                    ApiTypes.allApiTypes.forEach { apiType ->
                        DropdownMenuItem(
                            text = {
                                Column {
                                    Text(text = apiType.displayName)
                                    Text(
                                        text = apiType.description,
                                        style = MaterialTheme.typography.bodySmall,
                                        color = MaterialTheme.colorScheme.onSurfaceVariant
                                    )
                                }
                            },
                            onClick = {
                                onApiTypeSelected(apiType)
                                apiExpanded = false
                            }
                        )
                    }
                }
            }
            
            Spacer(modifier = Modifier.height(16.dp))
            
            // 🏢 SELECTOR DE EMPRESA (DROPDOWN)
            Text(
                text = "🏢 Seleccionar Empresa",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
            
            Spacer(modifier = Modifier.height(8.dp))
            
            var companyExpanded by remember { mutableStateOf(false) }
            
            Box {
                Button(
                    onClick = { companyExpanded = true },
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = MaterialTheme.colorScheme.surface
                    )
                ) {
                    Text(
                        text = selectedCompany.displayName,
                        color = MaterialTheme.colorScheme.onSurface
                    )
                }
                
                DropdownMenu(
                    expanded = companyExpanded,
                    onDismissRequest = { companyExpanded = false }
                ) {
                    Companies.allCompanies.forEach { company ->
                        DropdownMenuItem(
                            text = {
                                Column {
                                    Text(text = company.displayName)
                                    Text(
                                        text = company.description,
                                        style = MaterialTheme.typography.bodySmall,
                                        color = MaterialTheme.colorScheme.onSurfaceVariant
                                    )
                                }
                            },
                            onClick = {
                                onCompanySelected(company)
                                companyExpanded = false
                            }
                        )
                    }
                }
            }
            
            // Mostrar información del código interno seleccionado
            Spacer(modifier = Modifier.height(12.dp))
            
            Card(
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer
                )
            ) {
                Column(
                    modifier = Modifier.padding(12.dp)
                ) {
                    Text(
                        text = "📋 Configuración Interna",
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        text = "API: ${selectedApiType.displayName}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                    Text(
                        text = "Código: ${selectedCompany.internalCode}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                }
            }
        }
    }
}

