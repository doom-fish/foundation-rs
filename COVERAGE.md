# COVERAGE

- Framework: `Foundation.framework`
- Scope: This release covers `NSUserDefaults`, `NSProcessInfo`, process activities, `NSNotificationCenter` observers, autorelease pools and the main `NSBundle`. Other Foundation classes are not in this release.
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

## NSNotificationCenter

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `+defaultCenter` | ✅ | `NotificationCenter::default_center` | The process-wide center. |
| `-init` | ✅ | `NotificationCenter::new` | Each call makes a private center. |
| `-addObserverForName:object:queue:usingBlock:` | ✅ | `NotificationCenter::add_observer` | Any object; the callback runs on the posting thread. |
| `-removeObserver:` | ✅ | `NotificationObserver` drop | A callback that already started can finish after drop. |
| `-postNotificationName:object:` | ✅ | `NotificationCenter::post` | The object is nil. |
| `-postNotification:`, `-addObserver:selector:name:object:`, `-removeObserver:name:object:` | ⏭️ |  |  |

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
| `NSProcessInfoThermalStateDidChangeNotification` | ✅ | `ProcessInfo::THERMAL_STATE_DID_CHANGE_NOTIFICATION` | Observe the default center. |
| `NSProcessInfoPowerStateDidChangeNotification` | ✅ | `ProcessInfo::POWER_STATE_DID_CHANGE_NOTIFICATION` | Observe the default center on macOS 12 or later. |
| `environment`, `arguments`, `globallyUniqueString`, `hostName` | ⏭️ |  |  |
| `-disableSuddenTermination`, `-enableSuddenTermination`, `-disableAutomaticTermination:`, `-enableAutomaticTermination:` | ⏭️ |  |  |
| `macCatalystApp`, `iOSAppOnMac` | ⏭️ |  |  |

## Process activities

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `-beginActivityWithOptions:reason:` | ✅ | `ProcessInfo::begin_activity` | The reason must not be empty. |
| `-endActivity:` | ✅ | `Activity` drop | The activity ends once. |
| `NSActivityIdleDisplaySleepDisabled` | ✅ | `ActivityOptions::IDLE_DISPLAY_SLEEP_DISABLED` |  |
| `NSActivityIdleSystemSleepDisabled` | ✅ | `ActivityOptions::IDLE_SYSTEM_SLEEP_DISABLED` |  |
| `NSActivitySuddenTerminationDisabled` | ✅ | `ActivityOptions::SUDDEN_TERMINATION_DISABLED` |  |
| `NSActivityAutomaticTerminationDisabled` | ✅ | `ActivityOptions::AUTOMATIC_TERMINATION_DISABLED` |  |
| `NSActivityAnimationTrackingEnabled` | ✅ | `ActivityOptions::ANIMATION_TRACKING_ENABLED` | Needs macOS 13. |
| `NSActivityTrackingEnabled` | ✅ | `ActivityOptions::TRACKING_ENABLED` | Needs macOS 13. |
| `NSActivityUserInitiated` | ✅ | `ActivityOptions::USER_INITIATED` |  |
| `NSActivityUserInitiatedAllowingIdleSystemSleep` | ✅ | `ActivityOptions::USER_INITIATED_ALLOWING_IDLE_SYSTEM_SLEEP` |  |
| `NSActivityBackground` | ✅ | `ActivityOptions::BACKGROUND` |  |
| `NSActivityLatencyCritical` | ✅ | `ActivityOptions::LATENCY_CRITICAL` |  |
| `NSActivityUserInteractive` | ✅ | `ActivityOptions::USER_INTERACTIVE` | Needs macOS 13. |
| `-performActivityWithOptions:reason:usingBlock:` | ⏭️ |  |  |

## Autorelease pools

| Objective-C API | Status | Rust API | Notes |
| --- | --- | --- | --- |
| `@autoreleasepool` | ✅ | `autoreleasepool` | Drains when the closure returns or panics. |

## NSBundle

| Objective-C API | Status | Rust API | Notes |
| --- | --- | --- | --- |
| `+mainBundle` | ✅ | `Bundle::main` |  |
| `-bundleIdentifier` | ✅ | `Bundle::bundle_identifier` | `None` when the bundle has no identifier. |
| `-bundlePath` | ✅ | `Bundle::bundle_path` | A copy of the file-system bytes. |
| `-URLForAuxiliaryExecutable:` | ✅ | `Bundle::url_for_auxiliary_executable` | `None` when the executable does not exist. |
