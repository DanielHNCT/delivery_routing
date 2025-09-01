package com.daniel.deliveryrouting

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.ui.platform.LocalContext
import com.daniel.deliveryrouting.data.repository.BackendRepository
import com.daniel.deliveryrouting.ui.theme.DeliveryRoutingTheme
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        setContent {
            DeliveryRoutingTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    LoginApp()
                }
            }
        }
    }
}

@Composable
fun LoginApp() {
    var isLoggedIn by remember { mutableStateOf(false) }
    var username by remember { mutableStateOf("A187518") }
    var password by remember { mutableStateOf("INTI7518") }
    var societe by remember { mutableStateOf("PCP0010699") }
    var isLoading by remember { mutableStateOf(false) }
    var errorMessage by remember { mutableStateOf("") }
    var loginData by remember { mutableStateOf<LoginSuccessData?>(null) }
    
    val context = LocalContext.current
    val repository = remember { BackendRepository(context) }
    val scope = rememberCoroutineScope()
    
    if (isLoggedIn && loginData != null) {
        // ✅ PANTALLA DE ÉXITO
        SuccessScreen(
            username = loginData!!.username,
            matricule = loginData!!.matricule,
            onLogout = {
                isLoggedIn = false
                loginData = null
                errorMessage = ""
            }
        )
    } else {
        // ✅ PANTALLA DE LOGIN
        LoginScreen(
            username = username,
            password = password,
            societe = societe,
            isLoading = isLoading,
            errorMessage = errorMessage,
            onUsernameChange = { username = it },
            onPasswordChange = { password = it },
            onLoginClick = {
                scope.launch {
                    isLoading = true
                    errorMessage = ""
                    
                    try {
                        // ✅ LLAMADA REAL AL BACKEND
                        val result = repository.login(username, password, societe)
                        
                        result.fold(
                            onSuccess = { loginResponse ->
                                if (loginResponse.success) {
                                    val fullUsername = "${societe}_${username}"
                                    val matricule = loginResponse.authentication?.matricule ?: fullUsername
                                    loginData = LoginSuccessData(fullUsername, matricule)
                                    isLoggedIn = true
                                } else {
                                    errorMessage = loginResponse.error?.message ?: "Error en el login"
                                }
                            },
                            onFailure = { error ->
                                errorMessage = "Error de conexión: ${error.message}"
                            }
                        )
                    } catch (e: Exception) {
                        errorMessage = "Error inesperado: ${e.message}"
                    } finally {
                        isLoading = false
                    }
                }
            }
        )
    }
}

@Composable
fun LoginScreen(
    username: String,
    password: String,
    societe: String,
    isLoading: Boolean,
    errorMessage: String,
    onUsernameChange: (String) -> Unit,
    onPasswordChange: (String) -> Unit,
    onLoginClick: () -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Text(
            text = "🔐 Login",
            style = MaterialTheme.typography.headlineLarge,
            color = MaterialTheme.colorScheme.primary
        )
        
        Spacer(modifier = Modifier.height(32.dp))
        
        OutlinedTextField(
            value = username,
            onValueChange = onUsernameChange,
            label = { Text("Usuario") },
            modifier = Modifier.fillMaxWidth(),
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Text)
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        OutlinedTextField(
            value = password,
            onValueChange = onPasswordChange,
            label = { Text("Contraseña") },
            modifier = Modifier.fillMaxWidth(),
            visualTransformation = PasswordVisualTransformation(),
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password)
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        OutlinedTextField(
            value = societe,
            onValueChange = { },
            label = { Text("Sociedad") },
            modifier = Modifier.fillMaxWidth(),
            enabled = false  // Hardcodeada por ahora
        )
        
        Spacer(modifier = Modifier.height(32.dp))
        
        if (isLoading) {
            CircularProgressIndicator()
        } else if (errorMessage.isNotEmpty()) {
            Text(
                text = "❌ $errorMessage",
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodyMedium
            )
            Spacer(modifier = Modifier.height(16.dp))
        }
        
        Button(
            onClick = onLoginClick,
            modifier = Modifier.fillMaxWidth(),
            enabled = !isLoading
        ) {
            Text("Iniciar Sesión")
        }
    }
}

@Composable
fun SuccessScreen(
    username: String,
    matricule: String,
    onLogout: () -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Text(
            text = "✅ Login Exitoso",
            style = MaterialTheme.typography.headlineLarge,
            color = MaterialTheme.colorScheme.primary
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Card(
            modifier = Modifier.fillMaxWidth(),
            elevation = CardDefaults.cardElevation(defaultElevation = 4.dp)
        ) {
            Column(
                modifier = Modifier.padding(16.dp)
            ) {
                Text(
                    text = "Usuario: $username",
                    style = MaterialTheme.typography.bodyLarge
                )
                
                Spacer(modifier = Modifier.height(8.dp))
                
                Text(
                    text = "Matrícula: $matricule",
                    style = MaterialTheme.typography.bodyLarge
                )
                
                Spacer(modifier = Modifier.height(8.dp))
                
                Text(
                    text = "Estado: Autenticado",
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.primary
                )
            }
        }
        
        Spacer(modifier = Modifier.height(32.dp))
        
        Button(
            onClick = onLogout,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("Cerrar Sesión")
        }
    }
}

data class LoginSuccessData(
    val username: String,
    val matricule: String
)