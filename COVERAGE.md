# COVERAGE

- Framework: `Foundation.framework`
- Scope: this release covers the values of `NSUserDefaults`, the system information of `NSProcessInfo`, and autorelease pools. The other Foundation classes are not in this release.
- Legend: `✅` implemented, `⏭️` not implemented

## NSUserDefaults

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `+standardUserDefaults` | ✅ | `UserDefaults::standard` |  |
| `-initWithSuiteName:` | ✅ | `UserDefaults::with_suite_name` | A name that Foundation does not accept gives `FoundationError::InvalidArgument`. |
| `-registerDefaults:` | ✅ | `UserDefaults::register` | Bool, integer, double and string values. |
| `-setBool:forKey:` | ✅ | `UserDefaults::set` with `DefaultValue::Bool` |  |
| `-setInteger:forKey:` | ✅ | `UserDefaults::set` with `DefaultValue::Integer` |  |
| `-setDouble:forKey:` | ✅ | `UserDefaults::set` with `DefaultValue::Double` |  |
| `-setObject:forKey:` with a string | ✅ | `UserDefaults::set` with `DefaultValue::String` |  |
| `-boolForKey:` | ✅ | `UserDefaults::bool` |  |
| `-integerForKey:` | ✅ | `UserDefaults::integer` |  |
| `-doubleForKey:` | ✅ | `UserDefaults::double` |  |
| `-stringForKey:` | ✅ | `UserDefaults::string` |  |
| `-objectForKey:` | ✅ | `UserDefaults::contains` | Tells only if a value exists. |
| `-removeObjectForKey:` | ✅ | `UserDefaults::remove` |  |
| `-floatForKey:`, `-URLForKey:`, `-dataForKey:`, `-arrayForKey:`, `-stringArrayForKey:`, `-dictionaryForKey:` | ⏭️ |  |  |
| `-setFloat:forKey:`, `-setURL:forKey:`, `-setObject:forKey:` with other objects | ⏭️ |  |  |
| `-addSuiteNamed:`, `-removeSuiteNamed:` | ⏭️ |  |  |
| `-persistentDomainForName:`, `-setPersistentDomain:forName:`, `-removePersistentDomainForName:` | ⏭️ |  |  |
| `-volatileDomainForName:`, `-setVolatileDomain:forName:`, `-removeVolatileDomainForName:` | ⏭️ |  |  |
| `-dictionaryRepresentation`, `-objectIsForcedForKey:` | ⏭️ |  |  |
| `NSUserDefaultsDidChangeNotification` | ⏭️ |  |  |

## NSProcessInfo

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `operatingSystemVersionString` | ✅ | `ProcessInfo::operating_system_version_string` |  |
| `operatingSystemVersion` | ✅ | `ProcessInfo::operating_system_version` |  |
| `-isOperatingSystemAtLeastVersion:` | ✅ | `ProcessInfo::is_operating_system_at_least` |  |
| `processName` | ✅ | `ProcessInfo::process_name` |  |
| `processIdentifier` | ✅ | `ProcessInfo::process_identifier` |  |
| `processorCount` | ✅ | `ProcessInfo::processor_count` |  |
| `activeProcessorCount` | ✅ | `ProcessInfo::active_processor_count` |  |
| `physicalMemory` | ✅ | `ProcessInfo::physical_memory` |  |
| `systemUptime` | ✅ | `ProcessInfo::system_uptime` |  |
| `lowPowerModeEnabled` | ✅ | `ProcessInfo::is_low_power_mode_enabled` | Needs macOS 12. On macOS 11 the value is `false`. |
| `thermalState` | ✅ | `ProcessInfo::thermal_state` | A value that this release does not know gives `ThermalState::Unknown`. |
| `environment`, `arguments`, `globallyUniqueString`, `hostName` | ⏭️ |  |  |
| `-beginActivityWithOptions:reason:`, `-endActivity:`, `-performActivityWithOptions:reason:usingBlock:` | ⏭️ |  |  |
| `-disableSuddenTermination`, `-enableSuddenTermination`, `-disableAutomaticTermination:`, `-enableAutomaticTermination:` | ⏭️ |  |  |
| `macCatalystApp`, `iOSAppOnMac` | ⏭️ |  |  |

## Autorelease pools

| Objective-C API | Status | Rust API | Notes |
| --- | --- | --- | --- |
| `@autoreleasepool` | ✅ | `autoreleasepool` | Drains when the closure returns or panics. |
