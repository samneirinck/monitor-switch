import CMonitorCore

typealias InputSource = CMonitorCore.InputSource

extension InputSource {
    private static let names: [UInt32: String] = [
        1: "VGA 1", 2: "VGA 2",
        3: "DVI 1", 4: "DVI 2",
        5: "Composite 1", 6: "Composite 2",
        7: "S-Video 1", 8: "S-Video 2",
        9: "Tuner 1", 10: "Tuner 2", 11: "Tuner 3",
        12: "Component 1", 13: "Component 2", 14: "Component 3",
        15: "DisplayPort 1", 16: "DisplayPort 2",
        17: "HDMI 1", 18: "HDMI 2", 19: "HDMI 3", 20: "HDMI 4",
        21: "USB-C 1", 22: "USB-C 2", 23: "USB-C 3"
    ]

    var displayName: String {
        Self.names[rawValue] ?? "Unknown"
    }
}

