import Cocoa

class PreferencesWindowController: NSWindowController {
    static var shared: PreferencesWindowController?
    
    private var monitors: [MonitorInfo] = []
    private var selectedMonitorIndex: Int = 0
    private var tableView: NSTableView!
    private var inputRows: [InputRow] = []
    
    struct InputRow {
        let input: InputSource
        var alias: String
        var isFavorite: Bool
        var isCurrent: Bool
    }
    
    static func show() {
        if shared == nil {
            shared = PreferencesWindowController()
        }
        NSApp.setActivationPolicy(.regular)
        NSApp.activate(ignoringOtherApps: true)
        shared?.window?.center()
        shared?.window?.makeKeyAndOrderFront(nil)
    }
    
    convenience init() {
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 500, height: 400),
            styleMask: [.titled, .closable, .miniaturizable],
            backing: .buffered,
            defer: false
        )
        window.title = "Monitor Switch Preferences"
        window.center()
        
        self.init(window: window)
        
        monitors = MonitorCore.shared.enumerateMonitors()
        setupUI()
        loadInputs()
    }
    
    private func setupUI() {
        guard let window = window else { return }
        
        let contentView = NSView(frame: window.contentView!.bounds)
        contentView.autoresizingMask = [.width, .height]
        
        let monitorLabel = NSTextField(labelWithString: "Monitor:")
        monitorLabel.frame = NSRect(x: 20, y: 360, width: 60, height: 20)
        contentView.addSubview(monitorLabel)
        
        let monitorPopup = NSPopUpButton(frame: NSRect(x: 85, y: 355, width: 300, height: 30))
        monitorPopup.target = self
        monitorPopup.action = #selector(monitorChanged(_:))
        for monitor in monitors {
            let name = monitor.modelName ?? monitor.manufacturerId ?? "Monitor \(monitor.index + 1)"
            monitorPopup.addItem(withTitle: name)
        }
        monitorPopup.tag = 100
        contentView.addSubview(monitorPopup)
        
        let scrollView = NSScrollView(frame: NSRect(x: 20, y: 60, width: 460, height: 280))
        scrollView.hasVerticalScroller = true
        scrollView.autoresizingMask = [.width, .height]
        
        tableView = NSTableView()
        tableView.delegate = self
        tableView.dataSource = self
        
        let currentCol = NSTableColumn(identifier: NSUserInterfaceItemIdentifier("current"))
        currentCol.title = ""
        currentCol.width = 30
        tableView.addTableColumn(currentCol)

        let inputCol = NSTableColumn(identifier: NSUserInterfaceItemIdentifier("input"))
        inputCol.title = "Input"
        inputCol.width = 100
        tableView.addTableColumn(inputCol)

        let aliasCol = NSTableColumn(identifier: NSUserInterfaceItemIdentifier("alias"))
        aliasCol.title = "Alias"
        aliasCol.width = 180
        tableView.addTableColumn(aliasCol)

        let favCol = NSTableColumn(identifier: NSUserInterfaceItemIdentifier("favorite"))
        favCol.title = "⭐ Favorite"
        favCol.width = 70
        tableView.addTableColumn(favCol)
        
        scrollView.documentView = tableView
        contentView.addSubview(scrollView)
        
        let saveButton = NSButton(title: "Save", target: self, action: #selector(save))
        saveButton.frame = NSRect(x: 380, y: 15, width: 100, height: 32)
        saveButton.bezelStyle = .rounded
        saveButton.keyEquivalent = "\r"
        contentView.addSubview(saveButton)
        
        let cancelButton = NSButton(title: "Cancel", target: self, action: #selector(cancel))
        cancelButton.frame = NSRect(x: 270, y: 15, width: 100, height: 32)
        cancelButton.bezelStyle = .rounded
        cancelButton.keyEquivalent = "\u{1b}"
        contentView.addSubview(cancelButton)
        
        window.contentView = contentView
    }
    
    private func loadInputs() {
        guard selectedMonitorIndex < monitors.count else { return }
        let monitor = monitors[selectedMonitorIndex]
        let availableInputs = MonitorCore.shared.getAvailableInputs(monitorIndex: monitor.index)
        let currentInput = MonitorCore.shared.getCurrentInput(monitorIndex: monitor.index)

        inputRows = availableInputs.map { input in
            InputRow(
                input: input,
                alias: MonitorCore.shared.getAlias(monitorId: monitor.id, input: input) ?? "",
                isFavorite: MonitorCore.shared.isFavorite(monitorId: monitor.id, input: input),
                isCurrent: input == currentInput
            )
        }
        tableView.reloadData()
    }
    
    @objc private func monitorChanged(_ sender: NSPopUpButton) {
        selectedMonitorIndex = sender.indexOfSelectedItem
        loadInputs()
    }
    
    @objc private func save() {
        guard selectedMonitorIndex < monitors.count else { return }
        let monitor = monitors[selectedMonitorIndex]
        
        for row in inputRows {
            if row.alias.isEmpty {
                _ = MonitorCore.shared.removeAlias(monitorId: monitor.id, input: row.input)
            } else {
                _ = MonitorCore.shared.setAlias(monitorId: monitor.id, input: row.input, alias: row.alias)
            }
            
            if row.isFavorite {
                _ = MonitorCore.shared.addFavorite(monitorId: monitor.id, input: row.input)
            } else {
                _ = MonitorCore.shared.removeFavorite(monitorId: monitor.id, input: row.input)
            }
        }
        
        window?.close()
        NSApp.setActivationPolicy(.accessory)
        NotificationCenter.default.post(name: .preferencesChanged, object: nil)
    }

    @objc private func cancel() {
        window?.close()
        NSApp.setActivationPolicy(.accessory)
    }
}

extension PreferencesWindowController: NSTableViewDataSource, NSTableViewDelegate {
    func numberOfRows(in tableView: NSTableView) -> Int {
        return inputRows.count
    }

    func tableView(_ tableView: NSTableView, viewFor tableColumn: NSTableColumn?, row: Int) -> NSView? {
        let inputRow = inputRows[row]

        switch tableColumn?.identifier.rawValue {
        case "current":
            let label = NSTextField(labelWithString: inputRow.isCurrent ? "✓" : "")
            label.alignment = .center
            label.textColor = .systemGreen
            label.font = NSFont.systemFont(ofSize: 14, weight: .bold)
            return label

        case "input":
            let label = NSTextField(labelWithString: inputRow.input.displayName)
            if inputRow.isCurrent {
                label.font = NSFont.systemFont(ofSize: 13, weight: .medium)
            }
            return label

        case "alias":
            let textField = NSTextField()
            textField.stringValue = inputRow.alias
            textField.placeholderString = inputRow.input.displayName
            textField.delegate = self
            textField.tag = row
            textField.isBordered = true
            textField.bezelStyle = .roundedBezel
            return textField

        case "favorite":
            let checkbox = NSButton(checkboxWithTitle: "", target: self, action: #selector(favoriteToggled(_:)))
            checkbox.state = inputRow.isFavorite ? .on : .off
            checkbox.tag = row
            return checkbox

        default:
            return nil
        }
    }

    func tableView(_ tableView: NSTableView, heightOfRow row: Int) -> CGFloat {
        return 28
    }

    @objc private func favoriteToggled(_ sender: NSButton) {
        let row = sender.tag
        inputRows[row].isFavorite = (sender.state == .on)
    }
}

extension PreferencesWindowController: NSTextFieldDelegate {
    func controlTextDidChange(_ obj: Notification) {
        guard let textField = obj.object as? NSTextField else { return }
        let row = textField.tag
        inputRows[row].alias = textField.stringValue
    }
}

extension Notification.Name {
    static let preferencesChanged = Notification.Name("preferencesChanged")
}

