// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "FoundationBridge",
    platforms: [.macOS(.v11)],
    products: [
        .library(name: "FoundationBridge", type: .static, targets: ["FoundationBridge"]),
    ],
    targets: [
        .target(name: "FoundationBridge", path: "Sources/FoundationBridge", publicHeadersPath: "include"),
    ]
)
