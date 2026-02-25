import Cocoa

class AppDelegate: NSObject, NSApplicationDelegate {
    private var statusItem: NSStatusItem!
    private var monitors: [MonitorInfo] = []

    func applicationDidFinishLaunching(_ notification: Notification) {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)

        if let button = statusItem.button {
            button.image = NSImage(systemSymbolName: "display", accessibilityDescription: "Monitor Switch")
        }

        NotificationCenter.default.addObserver(
            self,
            selector: #selector(preferencesDidChange),
            name: .preferencesChanged,
            object: nil
        )

        refreshMonitors()
        buildMenu()
    }

    @objc private func preferencesDidChange() {
        MonitorCore.shared.reloadConfig()
        buildMenu()
    }

    private func refreshMonitors() {
        monitors = MonitorCore.shared.enumerateMonitors()
    }

    private func getInputDisplayName(monitor: MonitorInfo, input: InputSource) -> String {
        if let alias = MonitorCore.shared.getAlias(monitorId: monitor.id, input: input) {
            return alias
        }
        return input.displayName
    }

    private func buildMenu() {
        let menu = NSMenu()

        if monitors.isEmpty {
            menu.addItem(NSMenuItem(title: "No monitors found", action: nil, keyEquivalent: ""))
        } else {
            let favorites = buildFavoritesSection()
            if !favorites.isEmpty {
                let favHeader = NSMenuItem(title: "⭐ Quick Switch", action: nil, keyEquivalent: "")
                favHeader.isEnabled = false
                menu.addItem(favHeader)

                for item in favorites {
                    menu.addItem(item)
                }
                menu.addItem(NSMenuItem.separator())
            }

            for monitor in monitors {
                let monitorName = getMonitorDisplayName(monitor: monitor)
                let monitorItem = NSMenuItem(title: monitorName, action: nil, keyEquivalent: "")
                monitorItem.isEnabled = false
                menu.addItem(monitorItem)

                let currentInput = MonitorCore.shared.getCurrentInput(monitorIndex: monitor.index)
                let availableInputs = MonitorCore.shared.getAvailableInputs(monitorIndex: monitor.index)

                for input in availableInputs {
                    let isFav = MonitorCore.shared.isFavorite(monitorId: monitor.id, input: input)
                    let displayName = getInputDisplayName(monitor: monitor, input: input)
                    let starPrefix = isFav ? "⭐ " : ""
                    let inputItem = NSMenuItem(
                        title: "  \(starPrefix)\(displayName)",
                        action: #selector(selectInput(_:)),
                        keyEquivalent: ""
                    )
                    inputItem.target = self
                    inputItem.representedObject = InputSelection(monitorId: monitor.id, monitorIndex: monitor.index, input: input)
                    inputItem.state = (input == currentInput) ? .on : .off
                    menu.addItem(inputItem)
                }

                menu.addItem(NSMenuItem.separator())
            }
        }

        menu.addItem(NSMenuItem(title: "Preferences...", action: #selector(openPreferences), keyEquivalent: ","))
        menu.addItem(NSMenuItem(title: "Refresh", action: #selector(refresh), keyEquivalent: "r"))
        menu.addItem(NSMenuItem.separator())

        let launchAtLoginItem = NSMenuItem(
            title: "Launch at Login",
            action: #selector(toggleLaunchAtLogin(_:)),
            keyEquivalent: ""
        )
        launchAtLoginItem.target = self
        launchAtLoginItem.state = LoginItemManager.shared.isEnabled ? .on : .off
        menu.addItem(launchAtLoginItem)

        menu.addItem(NSMenuItem.separator())
        menu.addItem(NSMenuItem(title: "Quit", action: #selector(quit), keyEquivalent: "q"))

        statusItem.menu = menu
    }

    @objc private func selectInput(_ sender: NSMenuItem) {
        guard let selection = sender.representedObject as? InputSelection else { return }
        _ = MonitorCore.shared.setInput(monitorIndex: selection.monitorIndex, input: selection.input)
        buildMenu()
    }

    @objc private func openPreferences() {
        PreferencesWindowController.show()
    }

    @objc private func refresh() {
        MonitorCore.shared.reloadConfig()
        refreshMonitors()
        buildMenu()
    }

    @objc private func toggleLaunchAtLogin(_ sender: NSMenuItem) {
        LoginItemManager.shared.isEnabled.toggle()
        buildMenu()
    }

    private func buildFavoritesSection() -> [NSMenuItem] {
        var items: [NSMenuItem] = []
        let favorites = MonitorCore.shared.getFavorites()

        for (monitorId, inputValue) in favorites {
            guard let monitor = monitors.first(where: { $0.id == monitorId }) else { continue }
            let input = InputSource(rawValue: UInt32(inputValue))

            let monitorName = getMonitorDisplayName(monitor: monitor)
            let inputName = getInputDisplayName(monitor: monitor, input: input)

            let item = NSMenuItem(
                title: "  \(inputName) → \(monitorName)",
                action: #selector(selectInput(_:)),
                keyEquivalent: ""
            )
            item.target = self
            item.representedObject = InputSelection(monitorId: monitorId, monitorIndex: monitor.index, input: input)

            let currentInput = MonitorCore.shared.getCurrentInput(monitorIndex: monitor.index)
            item.state = (input == currentInput) ? .on : .off

            items.append(item)
        }

        return items
    }

    private func getMonitorDisplayName(monitor: MonitorInfo) -> String {
        if let name = monitor.modelName {
            return "\(name) (\(monitor.index + 1))"
        } else if let mfr = monitor.manufacturerId {
            return "\(mfr) (\(monitor.index + 1))"
        } else {
            return "Monitor \(monitor.index + 1)"
        }
    }

    @objc private func quit() {
        NSApplication.shared.terminate(nil)
    }
}

struct InputSelection {
    let monitorId: String
    let monitorIndex: Int
    let input: InputSource
}

