package com.daniel.deliveryrouting.presentation.login

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.selectable
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
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
    
    val COLIS_PRIVE = Company(
        displayName = "Colis Privé",
        internalCode = "PCP0000001", 
        description = "Colis Privé - Principal"
    )
    
    // Lista de todas las empresas disponibles
    val allCompanies = listOf(INTI, COLIS_PRIVE)
}

/**
 * Selector de empresa con toggle/radio buttons
 */
@Composable
fun CompanySelector(
    selectedCompany: Company,
    onCompanySelected: (Company) -> Unit,
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
            Text(
                text = "🏢 Seleccionar Empresa",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
            
            Spacer(modifier = Modifier.height(12.dp))
            
            Companies.allCompanies.forEach { company ->
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .selectable(
                            selected = (company == selectedCompany),
                            onClick = { onCompanySelected(company) },
                            role = Role.RadioButton
                        )
                        .padding(vertical = 8.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    RadioButton(
                        selected = (company == selectedCompany),
                        onClick = null // Handled by selectable
                    )
                    
                    Spacer(modifier = Modifier.width(12.dp))
                    
                    Column {
                        Text(
                            text = company.displayName,
                            style = MaterialTheme.typography.bodyLarge,
                            color = MaterialTheme.colorScheme.onSurface
                        )
                        Text(
                            text = company.description,
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
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
                        text = "Código: ${selectedCompany.internalCode}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                }
            }
        }
    }
}

