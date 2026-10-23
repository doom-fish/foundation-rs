use foundation::prelude::*;

fn main() {
    let version = ProcessInfo::operating_system_version();
    println!("operating system:  {}", ProcessInfo::operating_system_version_string());
    println!("version:           {version}");
    println!(
        "at least macOS 12: {}",
        ProcessInfo::is_operating_system_at_least(OperatingSystemVersion::new(12, 0, 0))
    );
    println!("process:           {} ({})", ProcessInfo::process_name(), ProcessInfo::process_identifier());
    println!(
        "processors:        {} ({} active)",
        ProcessInfo::processor_count(),
        ProcessInfo::active_processor_count()
    );
    println!("physical memory:   {} bytes", ProcessInfo::physical_memory());
    println!("system uptime:     {:?}", ProcessInfo::system_uptime());
    println!("low power mode:    {}", ProcessInfo::is_low_power_mode_enabled());
    println!("thermal state:     {:?}", ProcessInfo::thermal_state());
}
