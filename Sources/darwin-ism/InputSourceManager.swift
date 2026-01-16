import Carbon
import Foundation

enum InputSourceManager {
  static func list(includeAllInstalled: Bool = true, bundleID: String? = nil) -> [InputSource] {
    var filter: CFDictionary?
    if let bundleID {
      filter = [kTISPropertyBundleID as String: bundleID] as CFDictionary
    }

    guard let unmanagedArray = TISCreateInputSourceList(filter, includeAllInstalled) else {
      return []
    }

    let cfArray = unmanagedArray.takeRetainedValue()
    guard let inputSources = cfArray as? [TISInputSource] else {
      return []
    }

    return inputSources.map { InputSource(tisInputSource: $0) }
  }

  static func find(byID id: String) -> InputSource? {
    list(includeAllInstalled: true).first { $0.id == id }
  }

  static func listEnabled() -> [InputSource] {
    list(includeAllInstalled: false).filter(\.isEnabled)
  }
}
