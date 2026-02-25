import SwiftUI

@main
struct MonitorSwitchApp: App {
    var body: some Scene {
        MenuBarExtra("Monitor Switch", systemImage: "display") {
            MonitorMenu()
        }

        Settings {
            PreferencesView()
        }
    }
}

