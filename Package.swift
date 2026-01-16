// swift-tools-version: 6.2
import PackageDescription

let package = Package(
  name: "darwin-ism",
  platforms: [.macOS(.v15)],
  dependencies: [
    .package(url: "https://github.com/apple/swift-argument-parser.git", from: "1.7.0"),
  ],
  targets: [
    .executableTarget(
      name: "darwin-ism",
      dependencies: [
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ],
      linkerSettings: [
        .linkedFramework("Carbon"),
        .linkedFramework("Foundation"),
      ],
    ),
  ],
)
