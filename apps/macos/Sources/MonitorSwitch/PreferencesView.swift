import SwiftUI

struct PreferencesView: View {
    private var monitorCore = MonitorCore.shared
    @State private var selectedMonitorIndex = 0
    @State private var inputRows: [InputRow] = []

    struct InputRow: Identifiable {
        let id: UInt32
        let input: InputSource
        var alias: String
        var isFavorite: Bool
        let isCurrent: Bool
    }

    private var selectedMonitor: MonitorInfo? {
        guard selectedMonitorIndex < monitorCore.monitors.count else { return nil }
        return monitorCore.monitors[selectedMonitorIndex]
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            HStack {
                Text("Monitor:")
                Picker("", selection: $selectedMonitorIndex) {
                    ForEach(Array(monitorCore.monitors.enumerated()), id: \.offset) { index, monitor in
                        Text(monitor.displayName).tag(index)
                    }
                }
                .labelsHidden()
                .frame(width: 250)
            }

            Table(inputRows) {
                TableColumn("") { row in
                    if row.isCurrent {
                        Image(systemName: "checkmark")
                            .foregroundColor(.green)
                            .fontWeight(.bold)
                    }
                }
                .width(30)

                TableColumn("Input") { row in
                    Text(row.input.displayName)
                        .fontWeight(row.isCurrent ? .medium : .regular)
                }
                .width(100)

                TableColumn("Alias") { row in
                    AliasTextField(row: binding(for: row))
                }
                .width(180)

                TableColumn("⭐ Favorite") { row in
                    FavoriteToggle(row: binding(for: row))
                }
                .width(70)
            }
            .tableStyle(.bordered)

            HStack {
                Spacer()
                Button("Save") {
                    save()
                }
                .keyboardShortcut(.return)
            }
        }
        .padding()
        .frame(width: 500, height: 400)
        .onAppear { loadInputs() }
        .onChange(of: selectedMonitorIndex) { loadInputs() }
    }

    private func binding(for row: InputRow) -> Binding<InputRow> {
        guard let index = inputRows.firstIndex(where: { $0.id == row.id }) else {
            return .constant(row)
        }
        return $inputRows[index]
    }

    private func loadInputs() {
        guard let monitor = selectedMonitor else {
            inputRows = []
            return
        }

        let availableInputs = monitorCore.getAvailableInputs(monitorIndex: monitor.index)
        let currentInput = monitorCore.getCurrentInput(monitorIndex: monitor.index)

        inputRows = availableInputs.map { input in
            InputRow(
                id: input.rawValue,
                input: input,
                alias: monitorCore.getAlias(monitorId: monitor.id, input: input) ?? "",
                isFavorite: monitorCore.isFavorite(monitorId: monitor.id, input: input),
                isCurrent: input == currentInput
            )
        }
    }

    private func save() {
        guard let monitor = selectedMonitor else { return }

        for row in inputRows {
            if row.alias.isEmpty {
                monitorCore.removeAlias(monitorId: monitor.id, input: row.input)
            } else {
                monitorCore.setAlias(monitorId: monitor.id, input: row.input, alias: row.alias)
            }

            if row.isFavorite {
                monitorCore.addFavorite(monitorId: monitor.id, input: row.input)
            } else {
                monitorCore.removeFavorite(monitorId: monitor.id, input: row.input)
            }
        }

        monitorCore.reloadConfig()
    }
}

struct AliasTextField: View {
    @Binding var row: PreferencesView.InputRow

    var body: some View {
        TextField(row.input.displayName, text: $row.alias)
            .textFieldStyle(.roundedBorder)
    }
}

struct FavoriteToggle: View {
    @Binding var row: PreferencesView.InputRow

    var body: some View {
        Toggle("", isOn: $row.isFavorite)
            .labelsHidden()
    }
}

