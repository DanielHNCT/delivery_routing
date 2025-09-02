package com.daniel.deliveryrouting.data.demo

import com.daniel.deliveryrouting.data.api.models.PackageData

object DemoData {
    
    fun getDemoPackages(): List<PackageData> {
        return listOf(
            PackageData(
                id = "demo-001",
                trackingNumber = "PU0000867901",
                recipientName = "Marie Dubois",
                address = "15 Rue de la Paix, 75001 Paris",
                status = "Pendiente",
                instructions = "Entregar en recepción",
                phone = "06 12 34 56 78",
                priority = "1",
                latitude = 48.8667,
                longitude = 2.3333
            ),
            PackageData(
                id = "demo-002", 
                trackingNumber = "2E0000153827",
                recipientName = "Jean Martin",
                address = "42 Avenue des Champs-Élysées, 75008 Paris",
                status = "Pendiente",
                instructions = "Código puerta: 1234",
                phone = "06 87 65 43 21",
                priority = "2",
                latitude = 48.8698,
                longitude = 2.3076
            ),
            PackageData(
                id = "demo-003",
                trackingNumber = "S79401757791", 
                recipientName = "Sophie Leroy",
                address = "8 Rue de Rivoli, 75004 Paris",
                status = "Pendiente",
                instructions = "Boîte aux lettres disponible",
                phone = "06 98 76 54 32",
                priority = "1",
                latitude = 48.8566,
                longitude = 2.3522
            ),
            PackageData(
                id = "demo-004",
                trackingNumber = "PU0000867902",
                recipientName = "Pierre Moreau", 
                address = "25 Boulevard Saint-Germain, 75005 Paris",
                status = "Pendiente",
                instructions = "Llamar antes de entregar",
                phone = "06 11 22 33 44",
                priority = "3",
                latitude = 48.8500,
                longitude = 2.3400
            ),
            PackageData(
                id = "demo-005",
                trackingNumber = "2E0000153828",
                recipientName = "Claire Bernard",
                address = "12 Place de la Bastille, 75011 Paris", 
                status = "Pendiente",
                instructions = "Entregar a vecino si no está",
                phone = "06 55 66 77 88",
                priority = "2",
                latitude = 48.8532,
                longitude = 2.3694
            ),
            PackageData(
                id = "demo-006",
                trackingNumber = "S79401757792",
                recipientName = "Antoine Petit",
                address = "33 Rue de la République, 75011 Paris",
                status = "Pendiente", 
                instructions = "Código puerta: 5678",
                phone = "06 99 88 77 66",
                priority = "1",
                latitude = 48.8630,
                longitude = 2.3650
            ),
            PackageData(
                id = "demo-007",
                trackingNumber = "PU0000867903",
                recipientName = "Isabelle Roux",
                address = "7 Avenue de la Grande Armée, 75017 Paris",
                status = "Pendiente",
                instructions = "Boîte aux lettres en el patio",
                phone = "06 44 33 22 11", 
                priority = "2",
                latitude = 48.8756,
                longitude = 2.2847
            ),
            PackageData(
                id = "demo-008",
                trackingNumber = "2E0000153829",
                recipientName = "François Blanc",
                address = "19 Rue de Belleville, 75019 Paris",
                status = "Pendiente",
                instructions = "Entregar en horario de oficina",
                phone = "06 77 88 99 00",
                priority = "3",
                latitude = 48.8722,
                longitude = 2.3764
            ),
            PackageData(
                id = "demo-009", 
                trackingNumber = "S79401757793",
                recipientName = "Nathalie Durand",
                address = "5 Place du Tertre, 75018 Paris",
                status = "Pendiente",
                instructions = "Código puerta: 9999",
                phone = "06 12 34 56 78",
                priority = "1",
                latitude = 48.8867,
                longitude = 2.3431
            )
        )
    }
    
    fun getDemoMessage(): String {
        return "🗺️ MODO DEMO ACTIVADO\n\n" +
               "Usando datos de prueba mientras Colis Privé arregla su API.\n\n" +
               "✅ 9 paquetes de prueba cargados\n" +
               "✅ Mapa funcional\n" +
               "✅ Navegación implementada\n\n" +
               "¡Prueba todas las funcionalidades!"
    }
}
