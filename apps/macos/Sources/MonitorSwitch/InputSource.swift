import Foundation
import CMonitorCore

typealias InputSource = CMonitorCore.InputSource

extension InputSource {
    var displayName: String {
        switch self.rawValue {
        case 1: return "VGA 1"
        case 2: return "VGA 2"
        case 3: return "DVI 1"
        case 4: return "DVI 2"
        case 5: return "Composite 1"
        case 6: return "Composite 2"
        case 7: return "S-Video 1"
        case 8: return "S-Video 2"
        case 9: return "Tuner 1"
        case 10: return "Tuner 2"
        case 11: return "Tuner 3"
        case 12: return "Component 1"
        case 13: return "Component 2"
        case 14: return "Component 3"
        case 15: return "DisplayPort 1"
        case 16: return "DisplayPort 2"
        case 17: return "HDMI 1"
        case 18: return "HDMI 2"
        case 19: return "HDMI 3"
        case 20: return "HDMI 4"
        case 21: return "USB-C 1"
        case 22: return "USB-C 2"
        case 23: return "USB-C 3"
        default: return "Unknown"
        }
    }
}

