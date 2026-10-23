# foundation-rs

Safe Rust bindings for Apple's [Foundation](https://developer.apple.com/documentation/foundation) framework on macOS. This release gives access to user defaults (`UserDefaults`) and to process information (`ProcessInfo`).

The crate requires macOS 11 or later. Low power mode needs macOS 12. On macOS 11, `ProcessInfo::is_low_power_mode_enabled` returns `false`.

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

## Raw functions

The `ffi` module declares the functions of the Swift bridge. A string that a bridge function returns belongs to the caller. Use `ffi::foundation_string_free` to release it.

## Examples

```bash
cargo run --example 01_process_info
cargo run --example 02_user_defaults
```

## License

MIT OR Apache-2.0
