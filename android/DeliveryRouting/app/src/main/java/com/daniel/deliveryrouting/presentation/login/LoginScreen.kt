package com.daniel.deliveryrouting.presentation.login

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.compose.ui.platform.LocalContext
import com.daniel.deliveryrouting.presentation.login.LoginViewModel.LoginState

@Composable
fun LoginScreen(
    onLoginSuccess: () -> Unit,
    viewModel: LoginViewModel = viewModel(
        factory = LoginViewModelFactory(LocalContext.current)
    )
) {
    var tourneeNumber by remember { mutableStateOf("A187518") }  // Valor por defecto para testing
    var password by remember { mutableStateOf("INTI7518") }      // Valor por defecto para testing
    var selectedCompany by remember { mutableStateOf(Companies.INTI) }  // Empresa por defecto
    
    val loginState by viewModel.loginState.collectAsState()
    
    LaunchedEffect(loginState) {
        if (loginState is LoginState.Success) {
            onLoginSuccess()
        }
    }
    
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        // 🚀 TÍTULO
        Text(
            text = "Delivery Routing",
            style = MaterialTheme.typography.headlineLarge,
            color = MaterialTheme.colorScheme.primary
        )
        
        Spacer(modifier = Modifier.height(8.dp))
        
        Text(
            text = "Conectarse al Backend Local",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Spacer(modifier = Modifier.height(32.dp))
        
        // 📝 CAMPOS DE LOGIN
        OutlinedTextField(
            value = tourneeNumber,
            onValueChange = { tourneeNumber = it },
            label = { Text("Número de Tournée") },
            placeholder = { Text("Ej: A187518") },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Text)
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        OutlinedTextField(
            value = password,
            onValueChange = { password = it },
            label = { Text("Contraseña") },
            placeholder = { Text("Tu contraseña") },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
            visualTransformation = PasswordVisualTransformation(),
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password)
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        // 🏢 SELECTOR DE EMPRESA
        CompanySelector(
            selectedCompany = selectedCompany,
            onCompanySelected = { selectedCompany = it }
        )
        
        Spacer(modifier = Modifier.height(32.dp))
        
        // 📋 MOSTRAR REQUEST QUE SE CONSTRUIRÁ
        if (tourneeNumber.isNotBlank()) {
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.tertiaryContainer
                )
            ) {
                Column(
                    modifier = Modifier.padding(12.dp)
                ) {
                    Text(
                        text = "🔧 Request que se enviará:",
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onTertiaryContainer
                    )
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        text = "username: ${selectedCompany.internalCode}_${tourneeNumber}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onTertiaryContainer
                    )
                    Text(
                        text = "societe: ${selectedCompany.internalCode}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onTertiaryContainer
                    )
                }
            }
            
            Spacer(modifier = Modifier.height(16.dp))
        }
        
        // 🔐 BOTÓN DE LOGIN
        Button(
            onClick = {
                if (tourneeNumber.isNotBlank() && password.isNotBlank()) {
                    // 🚀 CONSTRUIR USERNAME DINÁMICAMENTE
                    val fullUsername = "${selectedCompany.internalCode}_${tourneeNumber}"
                    val societe = selectedCompany.internalCode
                    
                    viewModel.login(fullUsername, password, societe)
                }
            },
            modifier = Modifier.fillMaxWidth(),
            enabled = tourneeNumber.isNotBlank() && password.isNotBlank() &&
                     loginState !is LoginState.Loading
        ) {
            if (loginState is LoginState.Loading) {
                CircularProgressIndicator(
                    modifier = Modifier.size(20.dp),
                    color = MaterialTheme.colorScheme.onPrimary
                )
                Spacer(modifier = Modifier.width(8.dp))
                Text("Conectando...")
            } else {
                Text("🔐 Conectar al Backend")
            }
        }
        
        Spacer(modifier = Modifier.height(16.dp))
        
        // 📊 ESTADO DE CONEXIÓN
        when (loginState) {
            is LoginState.Loading -> {
                Text(
                    text = "🔄 Conectando a tu backend local...",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.primary
                )
            }
            is LoginState.Success -> {
                Text(
                    text = "✅ ¡Conectado exitosamente!",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.primary
                )
            }
            is LoginState.Error -> {
                Text(
                    text = "❌ Error: ${(loginState as LoginState.Error).message}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.error
                )
            }
            else -> {
                Text(
                    text = "🌐 Backend: http://10.0.2.2:3000",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }
        
        Spacer(modifier = Modifier.height(16.dp))
        
        // 📱 INFORMACIÓN DEL DISPOSITIVO
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
                    text = "📱 Información del Dispositivo",
                    style = MaterialTheme.typography.titleSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Modelo: ${android.os.Build.MODEL}",
                    style = MaterialTheme.typography.bodySmall
                )
                Text(
                    text = "Android: ${android.os.Build.VERSION.RELEASE}",
                    style = MaterialTheme.typography.bodySmall
                )
            }
        }
    }
}
