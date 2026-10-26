# foundation-rs

Safe Rust bindings for Apple's [Foundation](https://developer.apple.com/documentation/foundation) framework on macOS.
The crate supplies user defaults, process information, notification observers, process activities and the main bundle.
The optional `network` feature adds HTTP and WebSocket clients.

The crate requires macOS 11 or later. Low power mode and its notification need macOS 12. The animation tracking, tracking, and user interactive activity options need macOS 13. On macOS 11, `ProcessInfo::is_low_power_mode_enabled` returns `false`.
On other targets, the library is empty.

## Quick start

```rust
use foundation::prelude::*;

fn main() -> Result<(), FoundationError> {
    let version = ProcessInfo::operating_system_version();
    println!("macOS {version}: {}", ProcessInfo::operating_system_version_string());

    let defaults = UserDefaults::standard();
    defaults.register(&[("RecentFileLimit", DefaultValue::Integer(10))])?;
    println!("recent files: {}", defaults.integer("RecentFileLimit")?);
    Ok(())
}
```

## User defaults

- `UserDefaults::standard` gives the defaults of the process. `UserDefaults::with_suite_name` gives the defaults of a suite.
- `register` puts values in the registration domain. These values do not persist, and all other domains override them.
- `set` writes a value to the domain of the application or of the suite. The value persists.
- `bool`, `integer`, `double` and `string` read a value. `contains` tells if a value exists, and `remove` removes a value.
- A key or a string value that contains a NUL byte gives `FoundationError::InvalidArgument`. In this condition, `register` registers no values.
- `UserDefaults` is `Send` and `Sync`, because `NSUserDefaults` is thread-safe.

## Process information

`ProcessInfo` gives the version of the operating system, the name and the identifier of the process, the number of processors, the physical memory, the system uptime, the low power mode and the thermal state.

```rust
use foundation::{OperatingSystemVersion, ProcessInfo};

assert!(ProcessInfo::is_operating_system_at_least(OperatingSystemVersion::new(11, 0, 0)));
assert_eq!(ProcessInfo::process_identifier() as u32, std::process::id());
```

## Notifications

`NotificationCenter::default_center` gives the process-wide center. `NotificationCenter::new` makes a private center. Clone a center to retain it.

`add_observer` registers a callback for a name from any object. The callback runs on the posting thread. Drop the `NotificationObserver` to remove it. A callback that starts before the drop can finish after the drop returns.

`post` sends a notification with no object. `ProcessInfo` supplies the thermal and power notification names.

## Process activities

`ProcessInfo::begin_activity` starts an activity with `ActivityOptions` and a nonempty reason. Combine options with `|` or `|=`. The `from_bits` method keeps unknown bits. Drop the `Activity` to end it.

## Main bundle

`Bundle::main` gives access to the main bundle. `bundle_identifier` returns `None` if the bundle has no identifier.
`bundle_path` and `url_for_auxiliary_executable` return `PathBuf` values with copies of the file-system bytes.
If the name is invalid or the executable does not exist, `url_for_auxiliary_executable` returns `None`.

```rust
use foundation::Bundle;

let bundle = Bundle::main();
println!("bundle path: {}", bundle.bundle_path().display());
println!("bundle identifier: {:?}", bundle.bundle_identifier());
if let Some(path) = bundle.url_for_auxiliary_executable("Tool") {
    println!("auxiliary executable: {}", path.display());
}
```

## Raw functions

The `ffi` module declares the functions of the Swift bridge. A string that a bridge function returns belongs to the caller. Use `ffi::foundation_string_free` to release it.

## HTTP and WebSocket clients

Enable the native network API:

```toml
[dependencies]
foundation-rs = { version = "0.2", features = ["network"] }
```

`foundation::network` supplies `DelegateQueue`, `Session`, `Request`, `DataTask`, `UploadWriter`, `WebSocketTask` and owned `Event` values.
It needs no additional dependencies or asynchronous runtime.
Use one `foundation-rs` version per link target.
The native bridge uses one fixed symbol namespace.

### Sessions and callback ownership

Create a serial delegate queue with `DelegateQueue::new`.
Pass that queue to `Session::new`.
The host owns the queue handle and can share it between sessions.
All event callbacks run on that queue, not on the main thread.
`DelegateQueue::is_current` identifies its callbacks.

Callbacks must return without a wait for another callback or a UI thread.
Send owned events to the host's event queue.
The host must apply its request generation checks before it calls a UI listener.
Do not retain a task in its own callback.
A callback that already starts can finish after cancellation or drop.

The delegate queue uses default `QoS`.
The peer received no close frame with utility `QoS` on the tested OS.

Each `Session` owns ephemeral native sessions for HTTP and WebSocket connections, with separate system and direct proxy routes.
All native sessions disable the URL cache, automatic cookies, cookie storage and credential storage.
Explicit `Cookie` and `Authorization` headers remain available.
The default connection limit is sixteen per host in each native session.
TLS starts at version 1.2 and uses strict system trust.
The OS can select HTTP/2 and HTTP/3.

`ProxyPolicy::SystemWithLocalBypass` is the default.
Public destinations use the system proxy policy.
Localhost, local names, loopback, private, shared and link-local addresses use the direct route.
Host names that resolve to a local address also use the direct route.
Address resolution runs outside the delegate queue.
Cancellation prevents a resolver result from starting a late request.
`ProxyPolicy::Direct` disables proxies for every destination.
Each followed redirect selects its route again.

### HTTP data and uploads

`Session::data_task` returns a suspended, clonable task.
Call `resume` to start it.
`Event::Response` supplies the real status, URL, headers and expected native content length.
HTTP error statuses remain responses.
`Event::Data` supplies incremental bytes before the transfer completes.
`Event::Complete` supplies the terminal result.
Use `suspend` and `resume` for host backpressure.

`Request` accepts an ordered header vector, including repeated names, and an optional owned body.
The bridge copies the body before task creation returns.
Header names and methods must use valid ASCII field characters.
Header values must use visible ASCII.
Requests and responses permit at most 100 fields and 100 KiB per name/value pair.
Foundation can combine repeated response fields, including `Set-Cookie`.
The native client can change wire header order and case.
The native client supplies transport and WebSocket handshake headers.

Call `DataTask::upload_body(length)` before the task starts to obtain an `UploadWriter`.
The writer implements `std::io::Write` with a bounded 256 KiB native stream.
Call `resume` before a worker copies a reader into the writer.
Call `UploadWriter::finish` after exactly the declared byte count.
`Event::UploadProgress` reports native transport progress, not bytes accepted by the writer.
Task cancellation releases a writer that waits for stream space.
An incomplete writer drop aborts the body stream.
Native retries cannot replay a consumed upload stream.

The OS decodes chunked transfers and compressed responses.
Response headers retain native values, which can describe the encoded body.
If the OS decoded the body, remove stale `Content-Encoding` and `Content-Length` before delivery.
Use `Accept-Encoding: identity` when the host requires uncompressed transfer.
The host owns resource limits, its application cache and body transforms.

### Redirects and timeouts

`Event::Redirect` supplies the response, a proposed request and an owned `RedirectDecision`.
Use `follow(&Request)` to select the next method, headers and optional body.
Use `refuse`, or drop the decision, to return the redirect response without a follow.
The decision can cross threads and never blocks the delegate queue.
The host owns hop limits, body replay, credential removal and downgrade policy.
Cancellation resolves a pending decision and rejects a late follow.

`SessionConfig` defaults to a thirty-second idle timeout and a sixty-second HTTP resource timeout.
`Request::idle_timeout` overrides the idle limit for one request.
`Request::deadline` adds a precise total deadline, which includes route resolution, redirects and suspension.
The deadline starts at the first `resume`.
An independent timer preserves the HTTP resource limit across redirects.
An explicit deadline can shorten that limit.
For WebSocket connections, the idle and deadline values limit only the opening handshake.
The smaller value controls that handshake.
HTTP resource limits do not close established WebSocket connections.
Timeouts must be positive and at most 4294967295 seconds.
`URLSession` has no exact connect-only timeout.

`cancel` cancels all clones of a task.
Explicit cancellation produces one terminal `NSURLErrorDomain` error with code `-999`.
A deadline produces code `-1001`.
No data or operation event follows a terminal result.
Drop the last task handle to cancel it and suppress further callbacks.
Task handles retain their session.
Use `Session::invalidate_and_cancel` to cancel every task and reject new tasks.

### WebSocket messages and close

`Session::web_socket_task` accepts a WS or WSS request and a `WebSocketConfig`.
Set `Origin` on the request and offered subprotocols in the configuration.
The native client supplies handshake fields, masking, continuation-frame assembly and automatic pong.
The bridge rejects an absent or unsolicited selected subprotocol when the offer requires a selection.
`Event::WebSocketOpen` supplies the selected protocol before any message.
The default message limit is 64 MiB and can increase.

Call `receive` after open and after each message.
Keep a receive pending so that the native client can process control frames.
`Event::Message` preserves text versus binary and owns its bytes.
Each task permits one pending operation of each type: receive, send and ping.
`send` and `ping` use host operation IDs.
Their completion events are `Sent` and `Pong`.
A send completion does not mean that the remote application received the message.

`close` accepts standard or private close codes and a UTF-8 reason.
The reason stops at 123 bytes without a split character.
`Event::WebSocketClose` retains the native code and reason.
It precedes the terminal result after native completion.
An absent native close code becomes 1005.
The host must report an abnormal close as 1006 when a transport error has no close frame.

### Native errors and ATS

`NetworkError` retains the native domain, code, description, user information, underlying error, socket error and available trust details.
`category` classifies DNS, connection refusal, connection loss, timeout, cancellation, ATS, certificate and other secure-connection failures.
Certificate codes `-1201` through `-1204` have the certificate category.
Code `-1200` alone does not establish a certificate failure.
`Event::Metrics` reports the negotiated HTTP and TLS protocols, connection reuse, the proxy connection flag and the selected local route.

Apps that permit arbitrary plain HTTP need `NSAllowsArbitraryLoads` in their ATS dictionary.
Do not add `NSAllowsLocalNetworking` beside that broad exception.
A command-line executable can embed the same plist in its Mach-O `__TEXT,__info_plist` section.
An ATS exception does not make an invalid HTTPS certificate trusted.

## Examples

```bash
cargo run --example 01_process_info
cargo run --example 02_user_defaults
cargo run --example 03_notifications_and_activities
cargo run --features network --example 04_http -- https://example.com/
```

## License

MIT OR Apache-2.0
