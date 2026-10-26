// swift-tools-version:5.9
import PackageDescription
import Foundation

let networkSettings: [SwiftSetting] = ProcessInfo.processInfo.environment["FOUNDATION_NETWORK"] == "1"
    ? [.define("FOUNDATION_NETWORK")]
    : []

let package = Package(
    name: "FoundationBridge",
    platforms: [.macOS(.v11)],
    products: [
        .library(name: "FoundationBridge", type: .static, targets: ["FoundationBridge"]),
    ],
    targets: [
        .target(
            name: "FoundationBridge",
            path: "Sources/FoundationBridge",
            publicHeadersPath: "include",
            swiftSettings: networkSettings
        ),
    ]
)
