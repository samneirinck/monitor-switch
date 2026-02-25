// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "MonitorSwitch",
    platforms: [
        .macOS(.v14)
    ],
    products: [
        .executable(name: "MonitorSwitch", targets: ["MonitorSwitch"])
    ],
    targets: [
        .systemLibrary(
            name: "CMonitorCore",
            pkgConfig: nil,
            providers: []
        ),
        .executableTarget(
            name: "MonitorSwitch",
            dependencies: ["CMonitorCore"],
            path: "Sources/MonitorSwitch",
            linkerSettings: [
                .unsafeFlags(["-L../../target/release"]),
                .linkedLibrary("monitor_core"),
                .linkedFramework("CoreFoundation"),
                .linkedFramework("CoreGraphics"),
                .linkedFramework("IOKit"),
            ]
        )
    ]
)

