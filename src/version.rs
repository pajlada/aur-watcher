use std::io::{self, Write};
use std::path::{Path, PathBuf};

const VERSION_DIR: &str = ".versions";

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(VERSION_DIR)?;

    Ok(())
}

pub fn read(package: &str) -> Option<String> {
    let path = version_file_path(package);
    if path.exists() {
        std::fs::read_to_string(path)
            .ok()
            .map(|s| s.trim().to_string())
    } else {
        None
    }
}

pub fn write(package: &str, version: &str) -> io::Result<()> {
    let path = version_file_path(package);
    let mut file = std::fs::File::create(path)?;
    file.write_all(version.as_bytes())?;
    Ok(())
}

fn version_file_path(package: &str) -> PathBuf {
    Path::new(VERSION_DIR).join(package)
}
