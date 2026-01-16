import Carbon
import Foundation

struct InputSource {
  let tisInputSource: TISInputSource

  var id: String {
    guard let ptr = TISGetInputSourceProperty(tisInputSource, kTISPropertyInputSourceID) else {
      return "Unknown"
    }
    return Unmanaged<CFString>.fromOpaque(ptr).takeUnretainedValue() as String
  }

  var bundleID: String {
    guard let ptr = TISGetInputSourceProperty(tisInputSource, kTISPropertyBundleID) else {
      return "Unknown"
    }
    return Unmanaged<CFString>.fromOpaque(ptr).takeUnretainedValue() as String
  }

  var localizedName: String {
    guard let ptr = TISGetInputSourceProperty(tisInputSource, kTISPropertyLocalizedName) else {
      return "Unknown"
    }
    return Unmanaged<CFString>.fromOpaque(ptr).takeUnretainedValue() as String
  }

  var type: String {
    guard let ptr = TISGetInputSourceProperty(tisInputSource, kTISPropertyInputSourceType) else {
      return "Unknown"
    }
    return Unmanaged<CFString>.fromOpaque(ptr).takeUnretainedValue() as String
  }

  var isEnabled: Bool {
    guard let ptr = TISGetInputSourceProperty(tisInputSource, kTISPropertyInputSourceIsEnabled)
    else {
      return false
    }
    return Unmanaged<CFBoolean>.fromOpaque(ptr).takeUnretainedValue() == kCFBooleanTrue
  }

  var isEnableCapable: Bool {
    guard
      let ptr = TISGetInputSourceProperty(tisInputSource, kTISPropertyInputSourceIsEnableCapable)
    else {
      return false
    }
    return Unmanaged<CFBoolean>.fromOpaque(ptr).takeUnretainedValue() == kCFBooleanTrue
  }

  func enable() -> OSStatus {
    TISEnableInputSource(tisInputSource)
  }

  func disable() -> OSStatus {
    TISDisableInputSource(tisInputSource)
  }
}
