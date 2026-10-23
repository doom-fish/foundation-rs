import Foundation

@_cdecl("foundation_process_info_operating_system_version_string")
public func foundation_process_info_operating_system_version_string() -> UnsafeMutablePointer<CChar>? {
    strdup(ProcessInfo.processInfo.operatingSystemVersionString)
}

@_cdecl("foundation_process_info_operating_system_version")
public func foundation_process_info_operating_system_version(
    _ major: UnsafeMutablePointer<Int64>,
    _ minor: UnsafeMutablePointer<Int64>,
    _ patch: UnsafeMutablePointer<Int64>
) {
    let version = ProcessInfo.processInfo.operatingSystemVersion
    major.pointee = Int64(version.majorVersion)
    minor.pointee = Int64(version.minorVersion)
    patch.pointee = Int64(version.patchVersion)
}

@_cdecl("foundation_process_info_is_operating_system_at_least")
public func foundation_process_info_is_operating_system_at_least(_ major: Int64, _ minor: Int64, _ patch: Int64) -> Bool {
    ProcessInfo.processInfo.isOperatingSystemAtLeast(
        OperatingSystemVersion(majorVersion: Int(major), minorVersion: Int(minor), patchVersion: Int(patch))
    )
}

@_cdecl("foundation_process_info_process_name")
public func foundation_process_info_process_name() -> UnsafeMutablePointer<CChar>? {
    strdup(ProcessInfo.processInfo.processName)
}

@_cdecl("foundation_process_info_process_identifier")
public func foundation_process_info_process_identifier() -> Int32 {
    ProcessInfo.processInfo.processIdentifier
}

@_cdecl("foundation_process_info_processor_count")
public func foundation_process_info_processor_count() -> Int64 {
    Int64(ProcessInfo.processInfo.processorCount)
}

@_cdecl("foundation_process_info_active_processor_count")
public func foundation_process_info_active_processor_count() -> Int64 {
    Int64(ProcessInfo.processInfo.activeProcessorCount)
}

@_cdecl("foundation_process_info_physical_memory")
public func foundation_process_info_physical_memory() -> UInt64 {
    ProcessInfo.processInfo.physicalMemory
}

@_cdecl("foundation_process_info_system_uptime")
public func foundation_process_info_system_uptime() -> Double {
    ProcessInfo.processInfo.systemUptime
}

@_cdecl("foundation_process_info_is_low_power_mode_enabled")
public func foundation_process_info_is_low_power_mode_enabled() -> Bool {
    if #available(macOS 12.0, *) {
        return ProcessInfo.processInfo.isLowPowerModeEnabled
    }
    return false
}

@_cdecl("foundation_process_info_thermal_state")
public func foundation_process_info_thermal_state() -> Int64 {
    Int64(ProcessInfo.processInfo.thermalState.rawValue)
}
