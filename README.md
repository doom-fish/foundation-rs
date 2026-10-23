# foundation-rs

Safe Rust bindings for Apple's [Foundation](https://developer.apple.com/documentation/foundation) framework on macOS.
The crate supplies user defaults, process information, notification observers, process activities and the main bundle.

The crate requires macOS 11 or later. Low power mode and its notification need macOS 12. The animation tracking, tracking, and user interactive activity options need macOS 13. On macOS 11, `ProcessInfo::is_low_power_mode_enabled` returns `false`.

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

## Examples

```bash
cargo run --example 01_process_info
cargo run --example 02_user_defaults
cargo run --example 03_notifications_and_activities
```

## License

MIT OR Apache-2.0
