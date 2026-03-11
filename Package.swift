// swift-tools-version: 5.10
import PackageDescription

let package = Package(
  name: "darwin-ism",
  platforms: [.macOS(.v14)],
  dependencies: [
    .package(url: "https://github.com/apple/swift-argument-parser.git", exact: "1.6.2"),
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
      ]
    ),
  ]
)
