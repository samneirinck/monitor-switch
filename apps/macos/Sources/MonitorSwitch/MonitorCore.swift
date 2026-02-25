import Foundation
import CMonitorCore

struct MonitorInfo {
    let id: String
    let modelName: String?
    let manufacturerId: String?
    let index: Int
}

class MonitorCore {
    static let shared = MonitorCore()

    private init() {
        monitor_core_init()
    }

    func enumerateMonitors() -> [MonitorInfo] {
        let list = monitor_enumerate()
        defer { monitor_list_free(list) }

        var monitors: [MonitorInfo] = []
        for i in 0..<Int(list.count) {
            let info = list.monitors[i]
            let id = info.id.map { String(cString: $0) } ?? "unknown"
            let modelName = info.model_name.map { String(cString: $0) }
            let manufacturerId = info.manufacturer_id.map { String(cString: $0) }
            monitors.append(MonitorInfo(id: id, modelName: modelName, manufacturerId: manufacturerId, index: i))
        }
        return monitors
    }

    func getCurrentInput(monitorIndex: Int) -> InputSource {
        return monitor_get_current_input(UInt(monitorIndex))
    }

    func setInput(monitorIndex: Int, input: InputSource) -> Bool {
        return monitor_set_input(UInt(monitorIndex), input)
    }

    func getAvailableInputs(monitorIndex: Int) -> [InputSource] {
        let list = monitor_get_available_inputs(UInt(monitorIndex))
        defer { input_source_list_free(list) }

        var inputs: [InputSource] = []
        guard let inputsPtr = list.inputs else { return inputs }
        for i in 0..<Int(list.count) {
            inputs.append(inputsPtr[i])
        }
        return inputs
    }

    func getAlias(monitorId: String, input: InputSource) -> String? {
        guard let ptr = config_get_alias(monitorId, UInt16(input.rawValue)) else {
            return nil
        }
        let alias = String(cString: ptr)
        string_free(ptr)
        return alias
    }

    func setAlias(monitorId: String, input: InputSource, alias: String) -> Bool {
        return config_set_alias(monitorId, UInt16(input.rawValue), alias)
    }

    func removeAlias(monitorId: String, input: InputSource) -> Bool {
        return config_remove_alias(monitorId, UInt16(input.rawValue))
    }

    func reloadConfig() {
        config_reload()
    }

    func isFavorite(monitorId: String, input: InputSource) -> Bool {
        return config_is_favorite(monitorId, UInt16(input.rawValue))
    }

    func addFavorite(monitorId: String, input: InputSource) -> Bool {
        return config_add_favorite(monitorId, UInt16(input.rawValue))
    }

    func removeFavorite(monitorId: String, input: InputSource) -> Bool {
        return config_remove_favorite(monitorId, UInt16(input.rawValue))
    }

    func toggleFavorite(monitorId: String, input: InputSource) -> Bool {
        if isFavorite(monitorId: monitorId, input: input) {
            return removeFavorite(monitorId: monitorId, input: input)
        } else {
            return addFavorite(monitorId: monitorId, input: input)
        }
    }

    func getFavorites() -> [(monitorId: String, inputValue: UInt16)] {
        let list = config_get_favorites()
        defer { favorite_list_free(list) }

        var favorites: [(String, UInt16)] = []
        guard let ptr = list.favorites else { return favorites }
        for i in 0..<Int(list.count) {
            let info = ptr[i]
            if let monitorIdPtr = info.monitor_id {
                let monitorId = String(cString: monitorIdPtr)
                favorites.append((monitorId, info.input_value))
            }
        }
        return favorites
    }
}

