import SwiftUI

@main
struct MonitorSwitchApp: App {
    @StateObject private var monitorCore = MonitorCore.shared
    @StateObject private var loginItemManager = LoginItemManager.shared
    
    var body: some Scene {
        MenuBarExtra("Monitor Switch", systemImage: "display") {
            MonitorMenu()
                .environmentObject(monitorCore)
                .environmentObject(loginItemManager)
        }
        
        Settings {
            PreferencesView()
                .environmentObject(monitorCore)
        }
    }
}

