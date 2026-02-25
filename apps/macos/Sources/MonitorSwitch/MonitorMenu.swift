import SwiftUI

struct MonitorMenu: View {
    private var monitorCore = MonitorCore.shared
    private var loginItemManager = LoginItemManager.shared

    var body: some View {
        let _ = monitorCore.refreshTrigger

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

        Toggle("Launch at Login", isOn: Binding(
            get: { loginItemManager.isEnabled },
            set: { loginItemManager.isEnabled = $0 }
        ))

        Divider()

        Button("Quit") {
            NSApplication.shared.terminate(nil)
        }
        .keyboardShortcut("q", modifiers: .command)
    }
}

struct FavoritesSection: View {
    private var monitorCore = MonitorCore.shared

    var body: some View {
        let _ = monitorCore.refreshTrigger
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
                            monitorCore.setInput(monitorIndex: monitor.index, input: input)
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
    var monitorCore = MonitorCore.shared
    let monitor: MonitorInfo

    var body: some View {
        let _ = monitorCore.refreshTrigger

        Section(monitor.displayName) {
            let currentInput = monitorCore.getCurrentInput(monitorIndex: monitor.index)
            let availableInputs = monitorCore.getAvailableInputs(monitorIndex: monitor.index)

            ForEach(availableInputs, id: \.rawValue) { input in
                let isFavorite = monitorCore.isFavorite(monitorId: monitor.id, input: input)
                let displayName = monitorCore.getInputDisplayName(monitorId: monitor.id, input: input)
                let isSelected = input == currentInput

                Button {
                    monitorCore.setInput(monitorIndex: monitor.index, input: input)
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

