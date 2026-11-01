use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use foundation::FileManager;

struct ExampleFile(PathBuf);

impl Drop for ExampleFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = format!(
        "trash-item-{}-{}.txt",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    );
    let directory = std::env::current_dir()?.join("target");
    fs::create_dir_all(&directory)?;
    let path = directory.join(name);
    let mut handle = OpenOptions::new().write(true).create_new(true).open(&path)?;
    let mut file = ExampleFile(path);
    handle.write_all(b"Foundation Trash example\n")?;
    drop(handle);
    file.0 = FileManager::trash_item(&file.0)?;
    println!("Trash path: {}", file.0.display());
    fs::remove_file(&file.0)?;
    println!("The example file no longer exists in the Trash.");
    Ok(())
}
