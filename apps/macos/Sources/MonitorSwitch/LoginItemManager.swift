import Foundation
import ServiceManagement

class LoginItemManager {
    static let shared = LoginItemManager()
    
    private init() {}
    
    var isEnabled: Bool {
        get {
            if #available(macOS 13.0, *) {
                return SMAppService.mainApp.status == .enabled
            } else {
                return false
            }
        }
        set {
            if #available(macOS 13.0, *) {
                do {
                    if newValue {
                        try SMAppService.mainApp.register()
                    } else {
                        try SMAppService.mainApp.unregister()
                    }
                } catch {
                    print("Failed to update login item: \(error)")
                }
            }
        }
    }
}

