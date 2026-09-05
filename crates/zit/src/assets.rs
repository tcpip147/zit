use std::{borrow::Cow, fs, path::PathBuf};

use gpui::{AssetSource, Result, SharedString};

use crate::platform::PlatformProfile;

pub struct ZitAssets {
    root: PathBuf,
}

impl ZitAssets {
    pub fn bundled() -> Self {
        Self {
            root: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"),
        }
    }
}

impl AssetSource for ZitAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Some(Cow::Owned(fs::read(self.root.join(path))?)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(fs::read_dir(self.root.join(path))?
            .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
            .map(SharedString::from)
            .collect())
    }
}

pub fn icon_path(profile: PlatformProfile, name: &str) -> &'static str {
    match (profile, name) {
        (PlatformProfile::Windows, "minimize") => "window-controls/windows/minimize.svg",
        (PlatformProfile::Windows, "maximize") => "window-controls/windows/maximize.svg",
        (PlatformProfile::Windows, "restore") => "window-controls/windows/restore.svg",
        (PlatformProfile::Windows, "close") => "window-controls/windows/close.svg",
        (PlatformProfile::Linux, "minimize") => "window-controls/linux/minimize.svg",
        (PlatformProfile::Linux, "maximize") => "window-controls/linux/maximize.svg",
        (PlatformProfile::Linux, "restore") => "window-controls/linux/restore.svg",
        (PlatformProfile::Linux, "close") => "window-controls/linux/close.svg",
        (PlatformProfile::MacOs, "minimize") => "window-controls/macos/minimize.svg",
        (PlatformProfile::MacOs, "fullscreen") => "window-controls/macos/fullscreen.svg",
        (PlatformProfile::MacOs, "close") => "window-controls/macos/close.svg",
        _ => "window-controls/windows/close.svg",
    }
}
