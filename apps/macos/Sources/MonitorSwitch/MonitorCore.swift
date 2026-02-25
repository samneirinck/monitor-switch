import Foundation
import CMonitorCore

struct MonitorInfo: Identifiable {
    let id: String
    let modelName: String?
    let manufacturerId: String?
    let index: Int

    var displayName: String {
        if let name = modelName {
            "\(name) (\(index + 1))"
        } else if let mfr = manufacturerId {
            "\(mfr) (\(index + 1))"
        } else {
            "Monitor \(index + 1)"
        }
    }
}

@Observable
final class MonitorCore {
    static let shared = MonitorCore()

    private(set) var monitors: [MonitorInfo] = []
    private(set) var refreshTrigger = false

    private init() {
        monitor_core_init()
        refreshMonitors()
    }

    func refreshMonitors() {
        monitors = enumerateMonitors()
        refreshTrigger.toggle()
    }

    private func enumerateMonitors() -> [MonitorInfo] {
        let list = monitor_enumerate()
        defer { monitor_list_free(list) }

        var result: [MonitorInfo] = []
        for i in 0..<Int(list.count) {
            let info = list.monitors[i]
            let id = info.id.map { String(cString: $0) } ?? "unknown"
            let modelName = info.model_name.map { String(cString: $0) }
            let manufacturerId = info.manufacturer_id.map { String(cString: $0) }
            result.append(MonitorInfo(id: id, modelName: modelName, manufacturerId: manufacturerId, index: i))
        }
        return result
    }

    func getInputDisplayName(monitorId: String, input: InputSource) -> String {
        getAlias(monitorId: monitorId, input: input) ?? input.displayName
    }

    func getCurrentInput(monitorIndex: Int) -> InputSource {
        monitor_get_current_input(UInt(monitorIndex))
    }

    func setInput(monitorIndex: Int, input: InputSource) {
        _ = monitor_set_input(UInt(monitorIndex), input)
        refreshTrigger.toggle()
    }

    func getAvailableInputs(monitorIndex: Int) -> [InputSource] {
        let list = monitor_get_available_inputs(UInt(monitorIndex))
        defer { input_source_list_free(list) }

        guard let inputsPtr = list.inputs else { return [] }
        return (0..<Int(list.count)).map { inputsPtr[$0] }
    }

    func getAlias(monitorId: String, input: InputSource) -> String? {
        guard let ptr = config_get_alias(monitorId, UInt16(input.rawValue)) else {
            return nil
        }
        let alias = String(cString: ptr)
        string_free(ptr)
        return alias
    }

    func setAlias(monitorId: String, input: InputSource, alias: String) {
        _ = config_set_alias(monitorId, UInt16(input.rawValue), alias)
    }

    func removeAlias(monitorId: String, input: InputSource) {
        _ = config_remove_alias(monitorId, UInt16(input.rawValue))
    }

    func reloadConfig() {
        config_reload()
        refreshTrigger.toggle()
    }

    func isFavorite(monitorId: String, input: InputSource) -> Bool {
        config_is_favorite(monitorId, UInt16(input.rawValue))
    }

    func addFavorite(monitorId: String, input: InputSource) {
        _ = config_add_favorite(monitorId, UInt16(input.rawValue))
    }

    func removeFavorite(monitorId: String, input: InputSource) {
        _ = config_remove_favorite(monitorId, UInt16(input.rawValue))
    }

    func getFavorites() -> [(monitorId: String, inputValue: UInt16)] {
        let list = config_get_favorites()
        defer { favorite_list_free(list) }

        guard let ptr = list.favorites else { return [] }
        return (0..<Int(list.count)).compactMap { i in
            guard let monitorIdPtr = ptr[i].monitor_id else { return nil }
            return (String(cString: monitorIdPtr), ptr[i].input_value)
        }
    }
}

