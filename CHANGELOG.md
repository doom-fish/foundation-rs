# Changelog

This file records the important changes to `foundation-rs`.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). The versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2026-10-26

### Fixed

- Build the Swift bridge with the default SwiftPM build engine and use its reported product directory.

## [0.1.2] - 2026-10-24

### Added

- `Bundle` reads the main bundle: its identifier, its path and the paths of its auxiliary executables. A path is a copy of the file-system bytes.

## [0.1.1] - 2026-10-23

### Added

- `autoreleasepool` runs a closure inside an Objective-C autorelease pool. The pool drains when the closure returns or panics.
- `NotificationCenter` observes and posts named notifications. An observer removes itself when it drops.
- `ProcessInfo::begin_activity` starts a process activity. The `Activity` ends the activity when it drops.
- `ActivityOptions` supplies the activity options, and `ProcessInfo` supplies the thermal and power notification names.

## [0.1.0] - 2026-10-23

### Added

- `UserDefaults` for `NSUserDefaults`: the standard defaults, suites, registered values, and bool, integer, double and string values.
- `ProcessInfo` for `NSProcessInfo`: the operating system version and its description, a version check, the process name and identifier, the processor counts, the physical memory, the system uptime, the low power mode and the thermal state.
