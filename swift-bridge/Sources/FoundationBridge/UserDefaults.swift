import Foundation

private func userDefaults(_ handle: UnsafeMutableRawPointer) -> UserDefaults {
    Unmanaged<UserDefaults>.fromOpaque(handle).takeUnretainedValue()
}

@_cdecl("foundation_user_defaults_standard")
public func foundation_user_defaults_standard() -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(UserDefaults.standard).toOpaque()
}

@_cdecl("foundation_user_defaults_with_suite_name")
public func foundation_user_defaults_with_suite_name(_ name: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    guard let suite = UserDefaults(suiteName: String(cString: name)) else {
        return nil
    }
    return Unmanaged.passRetained(suite).toOpaque()
}

@_cdecl("foundation_user_defaults_release")
public func foundation_user_defaults_release(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<UserDefaults>.fromOpaque(handle).release()
}

@_cdecl("foundation_user_defaults_register_bool")
public func foundation_user_defaults_register_bool(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>,
    _ value: Bool
) {
    userDefaults(handle).register(defaults: [String(cString: key): value])
}

@_cdecl("foundation_user_defaults_register_integer")
public func foundation_user_defaults_register_integer(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>,
    _ value: Int64
) {
    userDefaults(handle).register(defaults: [String(cString: key): Int(value)])
}

@_cdecl("foundation_user_defaults_register_double")
public func foundation_user_defaults_register_double(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>,
    _ value: Double
) {
    userDefaults(handle).register(defaults: [String(cString: key): value])
}

@_cdecl("foundation_user_defaults_register_string")
public func foundation_user_defaults_register_string(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>,
    _ value: UnsafePointer<CChar>
) {
    userDefaults(handle).register(defaults: [String(cString: key): String(cString: value)])
}

@_cdecl("foundation_user_defaults_set_bool")
public func foundation_user_defaults_set_bool(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>,
    _ value: Bool
) {
    userDefaults(handle).set(value, forKey: String(cString: key))
}

@_cdecl("foundation_user_defaults_set_integer")
public func foundation_user_defaults_set_integer(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>,
    _ value: Int64
) {
    userDefaults(handle).set(Int(value), forKey: String(cString: key))
}

@_cdecl("foundation_user_defaults_set_double")
public func foundation_user_defaults_set_double(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>,
    _ value: Double
) {
    userDefaults(handle).set(value, forKey: String(cString: key))
}

@_cdecl("foundation_user_defaults_set_string")
public func foundation_user_defaults_set_string(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>,
    _ value: UnsafePointer<CChar>
) {
    userDefaults(handle).set(String(cString: value), forKey: String(cString: key))
}

@_cdecl("foundation_user_defaults_bool")
public func foundation_user_defaults_bool(_ handle: UnsafeMutableRawPointer, _ key: UnsafePointer<CChar>) -> Bool {
    userDefaults(handle).bool(forKey: String(cString: key))
}

@_cdecl("foundation_user_defaults_integer")
public func foundation_user_defaults_integer(_ handle: UnsafeMutableRawPointer, _ key: UnsafePointer<CChar>) -> Int64 {
    Int64(userDefaults(handle).integer(forKey: String(cString: key)))
}

@_cdecl("foundation_user_defaults_double")
public func foundation_user_defaults_double(_ handle: UnsafeMutableRawPointer, _ key: UnsafePointer<CChar>) -> Double {
    userDefaults(handle).double(forKey: String(cString: key))
}

@_cdecl("foundation_user_defaults_string")
public func foundation_user_defaults_string(
    _ handle: UnsafeMutableRawPointer,
    _ key: UnsafePointer<CChar>
) -> UnsafeMutablePointer<CChar>? {
    guard let value = userDefaults(handle).string(forKey: String(cString: key)) else {
        return nil
    }
    return strdup(value)
}

@_cdecl("foundation_user_defaults_contains")
public func foundation_user_defaults_contains(_ handle: UnsafeMutableRawPointer, _ key: UnsafePointer<CChar>) -> Bool {
    userDefaults(handle).object(forKey: String(cString: key)) != nil
}

@_cdecl("foundation_user_defaults_remove")
public func foundation_user_defaults_remove(_ handle: UnsafeMutableRawPointer, _ key: UnsafePointer<CChar>) {
    userDefaults(handle).removeObject(forKey: String(cString: key))
}
