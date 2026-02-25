use std::fs;
use std::path::PathBuf;

const DESKTOP_ENTRY: &str = r#"[Desktop Entry]
Type=Application
Name=Monitor Switch
Exec=monitor-switch
Icon=video-display
Comment=Switch monitor inputs
Categories=Utility;
StartupNotify=false
"#;

pub struct Autostart;

impl Autostart {
    fn autostart_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("autostart").join("monitor-switch.desktop"))
    }

    pub fn is_enabled() -> bool {
        Self::autostart_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }

    pub fn enable() {
        if let Some(path) = Self::autostart_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&path, DESKTOP_ENTRY);
        }
    }

    pub fn disable() {
        if let Some(path) = Self::autostart_path() {
            let _ = fs::remove_file(path);
        }
    }
}

