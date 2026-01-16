import ArgumentParser
import Foundation

/// Check if a Unicode scalar is a wide character (CJK, fullwidth, etc.)
private func isWideCharacter(_ value: UInt32) -> Bool {
  (0x1100 ... 0x115F).contains(value) || // Hangul Jamo
    (0x2E80 ... 0x9FFF).contains(value) || // CJK
    (0xAC00 ... 0xD7A3).contains(value) || // Hangul Syllables
    (0xF900 ... 0xFAFF).contains(value) || // CJK Compatibility
    (0xFE10 ... 0xFE1F).contains(value) || // Vertical forms
    (0xFF00 ... 0xFF60).contains(value) || // Fullwidth
    (0xFFE0 ... 0xFFE6).contains(value) // Fullwidth symbols
}

/// Calculate display width of a string (CJK characters count as 2)
private func displayWidth(_ str: String) -> Int {
  var width = 0
  for scalar in str.unicodeScalars {
    width += isWideCharacter(scalar.value) ? 2 : 1
  }
  return width
}

/// Pad string to target display width
private func padToWidth(_ str: String, _ targetWidth: Int) -> String {
  let currentWidth = displayWidth(str)
  if currentWidth >= targetWidth {
    return str
  }
  return str + String(repeating: " ", count: targetWidth - currentWidth)
}

/// Format a row with display-width-aware padding
private func formatRow(_ col1: String, _ col2: String, _ col3: String, _ col4: String) -> String {
  padToWidth(col1, 50) + padToWidth(col2, 8) + padToWidth(col3, 30) + col4
}

enum AppExitCode: Int32 {
  case success = 0
  case notFound = 1
  case operationFailed = 2
  case invalidArgument = 3
}

@main
struct DarwinISM: ParsableCommand {
  static let configuration = CommandConfiguration(
    commandName: "darwin-ism",
    abstract: "macOS Input Source Manager CLI",
    // x-release-please-start-version
    version: "0.1.0",
    // x-release-please-end
    subcommands: [ListSources.self, Enable.self, Disable.self],
  )
}

extension DarwinISM {
  struct ListSources: ParsableCommand {
    static let configuration = CommandConfiguration(
      commandName: "list",
      abstract: "List all input sources",
    )

    @Flag(name: [.customShort("e"), .long], help: "Show enabled input sources only")
    var enabled = false

    @Option(name: [.customShort("b"), .customLong("bundle-id")], help: "Filter by bundle ID")
    var bundleID: String?

    func run() throws {
      let sources: [InputSource] = if enabled {
        InputSourceManager.listEnabled()
      } else if let bundleID {
        InputSourceManager.list(bundleID: bundleID)
      } else {
        InputSourceManager.list()
      }

      if sources.isEmpty {
        print("No input sources found.")
        return
      }

      print(formatRow("ID", "Enabled", "Type", "Name"))
      print(String(repeating: "-", count: 110))

      for source in sources {
        let enabledStr = source.isEnabled ? "true" : "false"
        let typeName = source.type.replacingOccurrences(of: "TISType", with: "")
        print(formatRow(source.id, enabledStr, typeName, source.localizedName))
      }

      print("\nTotal: \(sources.count) input source(s)")
    }
  }

  struct Enable: ParsableCommand {
    static let configuration = CommandConfiguration(
      abstract: "Enable an input source",
    )

    @Argument(help: "Input source ID to enable")
    var id: String

    func run() throws {
      guard let source = InputSourceManager.find(byID: id) else {
        print("Error: Input source not found: \(id)")
        throw ExitCode(AppExitCode.notFound.rawValue)
      }

      if source.isEnabled {
        print("Already enabled: \(id)")
        return
      }

      if !source.isEnableCapable {
        print("Error: Input source cannot be enabled: \(id)")
        throw ExitCode(AppExitCode.operationFailed.rawValue)
      }

      let status = source.enable()
      if status == noErr {
        print("Enabled: \(id)")
      } else {
        print("Error: Failed to enable (OSStatus: \(status)): \(id)")
        throw ExitCode(AppExitCode.operationFailed.rawValue)
      }
    }
  }

  struct Disable: ParsableCommand {
    static let configuration = CommandConfiguration(
      abstract: "Disable an input source",
    )

    private static let kotoeriRomajiID = "com.apple.inputmethod.Kotoeri.RomajiTyping"
    private static let kotoeriRomanModeID = "com.apple.inputmethod.Kotoeri.RomajiTyping.Roman"

    @Argument(help: "Input source ID to disable")
    var id: String

    func run() throws {
      guard let source = InputSourceManager.find(byID: id) else {
        print("Error: Input source not found: \(id)")
        throw ExitCode(AppExitCode.notFound.rawValue)
      }

      if !source.isEnabled {
        print("Already disabled: \(id)")
        return
      }

      let status = source.disable()

      // Check if actually disabled (TISDisableInputSource may return noErr but not actually disable)
      let actuallyDisabled =
        if let updatedSource = InputSourceManager.find(byID: id) {
          !updatedSource.isEnabled
        } else {
          true
        }

      if status == noErr, actuallyDisabled {
        print("Disabled: \(id)")
        return
      }

      // If direct disable failed and target is ABC keyboard layout, try workaround
      if id.hasPrefix("com.apple.keylayout.ABC") {
        print("Direct disable failed. Attempting workaround for ABC keyboard...")
        try disableABCWithWorkaround(source: source)
      } else {
        print("Error: Failed to disable (OSStatus: \(status)): \(id)")
        throw ExitCode(AppExitCode.operationFailed.rawValue)
      }
    }

    /// Enable Kotoeri (Japanese Romaji input) if not already enabled
    /// - Returns: Tuple of (kotoeri source, was it already enabled)
    private func enableKotoeriIfNeeded() throws -> (InputSource, Bool) {
      guard let kotoeri = InputSourceManager.find(byID: Self.kotoeriRomajiID) else {
        print("Error: Japanese Romaji input not found. Cannot apply workaround.")
        throw ExitCode(AppExitCode.operationFailed.rawValue)
      }

      let wasEnabled = kotoeri.isEnabled

      if !wasEnabled {
        print("Enabling Japanese Romaji input temporarily...")
        let status = kotoeri.enable()
        if status != noErr {
          print("Error: Failed to enable Japanese Romaji input (OSStatus: \(status))")
          throw ExitCode(AppExitCode.operationFailed.rawValue)
        }
      }

      return (kotoeri, wasEnabled)
    }

    /// Enable Roman mode (English input) in Kotoeri if not already enabled
    private func enableRomanModeIfNeeded() throws {
      guard let kotoeriRoman = InputSourceManager.find(byID: Self.kotoeriRomanModeID) else {
        print("Error: Japanese Romaji Roman mode not found. Cannot apply workaround.")
        throw ExitCode(AppExitCode.operationFailed.rawValue)
      }

      if !kotoeriRoman.isEnabled {
        print("Enabling Roman mode in Japanese Romaji input...")
        let status = kotoeriRoman.enable()
        if status != noErr {
          print("Error: Failed to enable Roman mode (OSStatus: \(status))")
          throw ExitCode(AppExitCode.operationFailed.rawValue)
        }
      }
    }

    /// Disable Kotoeri if it was temporarily enabled and other input methods exist
    private func disableKotoeriIfTemporary(_ kotoeri: InputSource, wasEnabled: Bool) {
      guard !wasEnabled else { return }

      let enabledSources = InputSourceManager.listEnabled()
      let hasOtherInputMethod = enabledSources.contains { src in
        src.id != Self.kotoeriRomajiID
          && !src.id.hasPrefix("com.apple.inputmethod.Kotoeri.")
          && (src.type.contains("KeyboardInputMethod") || src.type.contains("KeyboardLayout"))
      }

      if hasOtherInputMethod {
        print("Disabling Japanese Romaji input (temporary)...")
        let status = kotoeri.disable()
        if status == noErr {
          print("Disabled: \(Self.kotoeriRomajiID)")
        } else {
          print("Note: Could not disable Japanese Romaji input automatically.")
        }
      } else {
        print("Note: Keeping Japanese Romaji input enabled as fallback.")
      }
    }

    private func disableABCWithWorkaround(source: InputSource) throws {
      // Step 1: Enable Japanese Romaji input (Kotoeri) as alternative
      let (kotoeri, kotoeriWasEnabled) = try enableKotoeriIfNeeded()

      // Step 2: Enable Roman mode (English input) in Kotoeri
      try enableRomanModeIfNeeded()

      // Step 3: Try to disable ABC again
      let disableStatus = source.disable()

      // Verify actually disabled
      let actuallyDisabled =
        if let updatedSource = InputSourceManager.find(byID: id) {
          !updatedSource.isEnabled
        } else {
          true
        }

      if disableStatus != noErr || !actuallyDisabled {
        print("Error: Failed to disable ABC even with workaround (OSStatus: \(disableStatus))")
        throw ExitCode(AppExitCode.operationFailed.rawValue)
      }
      print("Disabled: \(id)")

      // Step 4: Disable Kotoeri if it was temporarily enabled
      disableKotoeriIfTemporary(kotoeri, wasEnabled: kotoeriWasEnabled)
    }
  }
}
