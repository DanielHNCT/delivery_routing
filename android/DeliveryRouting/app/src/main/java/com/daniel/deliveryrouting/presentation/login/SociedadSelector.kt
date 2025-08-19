package com.daniel.deliveryrouting.presentation.login

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

// TODO: MAPBOX INTEGRATION
// When adding Mapbox:
// 1. Add location-based society selection if needed
// 2. Add map-based society validation

@Composable
fun SociedadSelector(
    selectedSociedad: String,
    onSociedadSelected: (String) -> Unit
) {
    val sociedades = listOf(
        "INTI" to "PCP0010699",
        // Futuras sociedades se pueden agregar aquí
    )
    
    Column {
        Text(
            text = "Sociedad",
            style = MaterialTheme.typography.bodyMedium,
            modifier = Modifier.padding(bottom = 4.dp)
        )
        
        // Toggle Button para sociedades
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            sociedades.forEach { (displayName, code) ->
                FilterChip(
                    selected = selectedSociedad == code,
                    onClick = { onSociedadSelected(code) },
                    label = { Text(displayName) },
                    modifier = Modifier.weight(1f),
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = MaterialTheme.colorScheme.primaryContainer,
                        selectedLabelColor = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                )
            }
        }
        
        // Mostrar código para debug/testing
        Text(
            text = "Código: $selectedSociedad",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            modifier = Modifier.padding(top = 4.dp)
        )
    }
}
