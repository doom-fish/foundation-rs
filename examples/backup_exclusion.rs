use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use foundation::FileManager;

struct ExampleFolder(PathBuf);

impl Drop for ExampleFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir(&self.0);
    }
}

fn tmutil_excludes(path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new("tmutil").arg("isexcluded").arg(path).output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    print!("{text}");
    if !output.status.success() {
        return Err("The tmutil command failed.".into());
    }
    if text.contains("[Excluded]") {
        Ok(true)
    } else if text.contains("[Included]") {
        Ok(false)
    } else {
        Err("The tmutil command returned an unknown result.".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = format!(
        "backup-exclusion-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    );
    let directory = std::env::current_dir()?.join("target");
    fs::create_dir_all(&directory)?;
    let path = directory.join(name);
    fs::create_dir(&path)?;
    let folder = ExampleFolder(path);
    let inherited_exclusion = tmutil_excludes(&directory)?;
    println!("Parent folder excluded: {inherited_exclusion}");
    for excluded in [true, false] {
        FileManager::set_excluded_from_backup(&folder.0, excluded)?;
        let actual = FileManager::is_excluded_from_backup(&folder.0)?;
        println!("Excluded from backup: {actual}");
        if actual != excluded {
            return Err("The backup exclusion flag does not match.".into());
        }
        if tmutil_excludes(&folder.0)? != (excluded || inherited_exclusion) {
            return Err("The tmutil result does not match the backup exclusion flag.".into());
        }
    }
    fs::remove_dir(&folder.0)?;
    Ok(())
}
