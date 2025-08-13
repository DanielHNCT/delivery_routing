//
//  DeliveryRoutingApp.swift
//  DeliveryRouting
//
//  Created by dvniel on 11/08/2025.
//

import SwiftUI

@main
struct DeliveryRoutingApp: App {
    let persistenceController = PersistenceController.shared

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(\.managedObjectContext, persistenceController.container.viewContext)
        }
    }
}
