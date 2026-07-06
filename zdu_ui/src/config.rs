use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("zdu").join("zdu_rom_path.txt"))
}

pub fn load_rom_path() -> Option<PathBuf> {
    let path = get_config_path()?;
    if path.exists() {
        fs::read_to_string(path)
            .ok()
            .map(|s| PathBuf::from(s.trim()))
    } else {
        None
    }
}

pub fn save_rom_path(rom_path: &Path) -> io::Result<()> {
    if let Some(path) = get_config_path() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, rom_path.to_string_lossy().as_ref())?;
    }
    Ok(())
}

pub fn get_port_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("zdu").join("zdu_port.txt"))
}

pub fn load_port() -> u16 {
    if let Some(path) = get_port_config_path() {
        if path.exists() {
            if let Ok(s) = fs::read_to_string(path) {
                if let Ok(port) = s.trim().parse::<u16>() {
                    return port;
                }
            }
        }
    }
    42069
}

pub fn save_port(port: u16) -> io::Result<()> {
    if let Some(path) = get_port_config_path() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, port.to_string())?;
    }
    Ok(())
}
