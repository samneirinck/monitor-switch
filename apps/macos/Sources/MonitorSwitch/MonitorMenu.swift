import SwiftUI

struct MonitorMenu: View {
    @EnvironmentObject var monitorCore: MonitorCore
    @EnvironmentObject var loginItemManager: LoginItemManager
    
    var body: some View {
        if monitorCore.monitors.isEmpty {
            Text("No monitors found")
        } else {
            FavoritesSection()
            
            ForEach(monitorCore.monitors) { monitor in
                MonitorSection(monitor: monitor)
            }
        }
        
        Divider()
        
        SettingsLink {
            Text("Preferences...")
        }
        .keyboardShortcut(",", modifiers: .command)
        
        Button("Refresh") {
            monitorCore.reloadConfig()
            monitorCore.refreshMonitors()
        }
        .keyboardShortcut("r", modifiers: .command)
        
        Divider()
        
        Toggle("Launch at Login", isOn: $loginItemManager.isEnabled)
        
        Divider()
        
        Button("Quit") {
            NSApplication.shared.terminate(nil)
        }
        .keyboardShortcut("q", modifiers: .command)
    }
}

struct FavoritesSection: View {
    @EnvironmentObject var monitorCore: MonitorCore
    
    var body: some View {
        let favorites = monitorCore.getFavorites()
        
        if !favorites.isEmpty {
            Section("⭐ Quick Switch") {
                ForEach(favorites, id: \.inputValue) { favorite in
                    if let monitor = monitorCore.monitors.first(where: { $0.id == favorite.monitorId }) {
                        let input = InputSource(rawValue: UInt32(favorite.inputValue))
                        let inputName = monitorCore.getInputDisplayName(monitorId: monitor.id, input: input)
                        let currentInput = monitorCore.getCurrentInput(monitorIndex: monitor.index)
                        let isSelected = input == currentInput
                        
                        Button {
                            _ = monitorCore.setInput(monitorIndex: monitor.index, input: input)
                            monitorCore.objectWillChange.send()
                        } label: {
                            HStack {
                                Text("\(inputName) → \(monitor.displayName)")
                                Spacer()
                                if isSelected {
                                    Image(systemName: "checkmark")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct MonitorSection: View {
    @EnvironmentObject var monitorCore: MonitorCore
    let monitor: MonitorInfo
    
    var body: some View {
        Section(monitor.displayName) {
            let currentInput = monitorCore.getCurrentInput(monitorIndex: monitor.index)
            let availableInputs = monitorCore.getAvailableInputs(monitorIndex: monitor.index)
            
            ForEach(availableInputs, id: \.rawValue) { input in
                let isFavorite = monitorCore.isFavorite(monitorId: monitor.id, input: input)
                let displayName = monitorCore.getInputDisplayName(monitorId: monitor.id, input: input)
                let isSelected = input == currentInput
                
                Button {
                    _ = monitorCore.setInput(monitorIndex: monitor.index, input: input)
                    monitorCore.objectWillChange.send()
                } label: {
                    HStack {
                        if isFavorite {
                            Text("⭐ \(displayName)")
                        } else {
                            Text(displayName)
                        }
                        Spacer()
                        if isSelected {
                            Image(systemName: "checkmark")
                        }
                    }
                }
            }
        }
    }
}

