# COVERAGE

- Framework: `Foundation.framework`
- Scope: This release covers defaults, process information, activities, notification observers, autorelease pools, the main bundle and optional native HTTP and WebSocket clients.
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

## URLSession (`network` feature)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `NSOperationQueue`, `maxConcurrentOperationCount`, `currentQueue` | Implemented | `DelegateQueue` | Host-owned serial queue with default QoS. |
| `ephemeralSessionConfiguration`, `sessionWithConfiguration:delegate:delegateQueue:` | Implemented | `Session::new`, `SessionConfig` | Separate HTTP/WebSocket and direct/system routes. |
| `URLCache`, `HTTPCookieStorage`, `HTTPShouldSetCookies`, `URLCredentialStorage`, `requestCachePolicy` | Implemented | Session policy | Native cache, automatic cookies and credential storage are disabled. |
| `connectionProxyDictionary` | Implemented | `ProxyPolicy` | System proxy selection with direct local routes, or direct-only mode. |
| `HTTPMaximumConnectionsPerHost`, `TLSMinimumSupportedProtocolVersion` | Implemented | `SessionConfig`, session policy | Sixteen connections by default. Strict trust and TLS 1.2 minimum. |
| `NSMutableURLRequest`, method, headers, body, timeout | Implemented | `Request` | Repeated input names and horizontal tabs remain available. Foundation can combine values. |
| `dataTaskWithRequest:` | Implemented | `Session::data_task` | Suspended creation with an owned task handle. |
| `uploadTaskWithStreamedRequest:`, `needNewBodyStream` | Implemented | `DataTask::upload_body`, `UploadWriter` | Bounded native stream with a known length. No replay after consumption. |
| `didReceiveResponse`, `didReceiveData`, `didSendBodyData` | Implemented | `Response`, `Data`, `UploadProgress` events | No full-response buffer in the binding. |
| `willPerformHTTPRedirection` | Implemented | `Redirect`, `RedirectDecision` | Asynchronous host decision and route selection for every follow. |
| `timeoutIntervalForRequest`, `timeoutIntervalForResource` | Implemented | `SessionConfig`, `Request::idle_timeout` | HTTP idle and resource limits. WebSocket limits apply only before open. |
| `resume`, `suspend`, `cancel`, `invalidateAndCancel` | Implemented | Task and session methods | Native cancellation and callback-context retention through late completions. |
| `didCompleteWithError`, `NSError`, peer trust | Implemented | `Complete`, `NetworkError`, `ErrorCategory` | Preserve domain, code, details, underlying errors and available trust results. |
| `didFinishCollectingMetrics` | Implemented | `Metrics` | HTTP/TLS protocol, connection reuse and proxy flags. |
| Download tasks, background sessions, authentication credentials, custom trust decisions | Not implemented | | Use data events for streamed downloads. |

## URLSessionWebSocketTask (`network` feature)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `webSocketTaskWithRequest:` | Implemented | `Session::web_socket_task` | Origin and application headers use `Request`. |
| `maximumMessageSize` | Implemented | `WebSocketConfig` | Default 64 MiB. |
| `didOpenWithProtocol` | Implemented | `WebSocketOpen` | The bridge validates offered protocols. |
| `receiveMessageWithCompletionHandler:` | Implemented | `receive`, `Message` | One explicit receive at a time. Text and binary remain distinct. |
| `sendMessage:completionHandler:` | Implemented | `send`, `Sent` | One pending send with an owned native buffer. |
| `sendPingWithPongReceiveHandler:` | Implemented | `ping`, `Pong` | One pending ping. Native automatic pong remains active with a pending receive. |
| `cancelWithCloseCode:reason:`, `didCloseWithCode:reason:` | Implemented | `close`, `WebSocketClose` | Standard/private codes, empty close frames through 1005 and a UTF-8 reason of at most 123 bytes. |
| Continuation frames, masking, HTTP upgrade, control frames | Native | Native task | No frame parser or TLS implementation in the crate. |
| Extension policy, compression control, custom WebSocket handshake validation | Not implemented | | The native client owns these protocol details. |

## Manual network verification

The release probes ran on macOS 27.0.1 with Xcode 27.1.
They used the Rust API, local Python fixtures and public endpoints.
No new unit or visual tests form part of this release.

| Area | Verified result |
| --- | --- |
| HTTP | Origin, authorization, manual cookies, repeated request values, chunk decoding, real error status, IPv4 and IPv6. |
| Storage | Repeated requests reached the fixture. Server cookies did not return. The native client reused connections. |
| Response streams | Eight delayed chunks reached the host before EOF. Response field-count and field-size limits rejected oversized metadata. |
| Uploads | A 4,194,323-byte streamed body matched its echo. Progress reached the declared length. Cancellation and early HTTP 413 released blocked writers. |
| Redirects | Host refusal returned 302. Host follow selected the method and headers. A proxy-to-direct redirect changed its native route. |
| Timeouts | Session idle/resource limits, per-request idle limits and a total deadline returned `-1001`. |
| Cancellation | The probes cancelled HTTP waits, upload waits, redirects, WebSocket open and WebSocket receive. No late data followed completion. |
| Ownership | Session cancellation completed all waiters. A 128-task cancellation probe released every Rust callback context without a host autorelease pool. |
| Protocols | `tls13.1d.pw` negotiated TLS 1.3. Public requests negotiated both `h2` and `h3`. |
| Certificates | Expired, wrong-host, self-signed and untrusted-root endpoints all returned `NSURLErrorDomain -1202`, with trust and underlying details. |
| WebSocket messages | Origin and subprotocol reached the fixture. Text/binary continuation messages matched. Exactly 64 MiB succeeded, and one extra byte failed. |
| WebSocket control | Automatic pong, explicit ping, peer close frames 1000/4001 and a UTF-8 reason limit succeeded. |
| WebSocket lifetime | An idle connection survived the HTTP resource and opening limits. A later native ping completed. |
| Proxy routes | An isolated default proxy fixture received HTTP and WebSocket traffic. Localhost and an actual LAN endpoint bypassed it. |
| ATS | Twenty-five cases covered bare, embedded-plist and headless-app executables against localhost, IPv4, IPv6, LAN and public HTTP. |

The proxy probe replaced the ephemeral configuration factory only in its own process.
It did not change a system proxy setting.
The native HTTP proxy metric remained false despite confirmed proxy traffic.
The probes do not cover proxy authentication or PAC.

Default ATS rejected public HTTP in embedded and app executables with code `-1022`.
`NSAllowsArbitraryLoads` alone permitted every tested HTTP address.
The probes did not run on the minimum supported OS.
