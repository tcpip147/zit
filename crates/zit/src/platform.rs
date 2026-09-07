#![allow(dead_code)]

use gpui::{App, Window, WindowAppearance};

#[cfg(target_os = "windows")]
fn restore_window(window: &Window) -> bool {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{SW_RESTORE, ShowWindowAsync},
    };

    let Ok(handle) = HasWindowHandle::window_handle(window) else {
        return false;
    };
    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        return false;
    };
    let hwnd = HWND(handle.hwnd.get() as *mut core::ffi::c_void);
    unsafe { ShowWindowAsync(hwnd, SW_RESTORE).as_bool() }
}

#[cfg(not(target_os = "windows"))]
fn restore_window(window: &Window) -> bool {
    window.zoom_window();
    true
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatformProfile {
    Windows,
    Linux,
    MacOs,
}

impl PlatformProfile {
    #[cfg(target_os = "windows")]
    pub const fn current() -> Self {
        Self::Windows
    }

    #[cfg(target_os = "linux")]
    pub const fn current() -> Self {
        Self::Linux
    }

    #[cfg(target_os = "macos")]
    pub const fn current() -> Self {
        Self::MacOs
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellCommand {
    NewRepository,
    OpenRepository,
    CloneRepository,
    ExitApplication,
    ShowAbout,
    CloseWindow,
    MinimizeWindow,
    ToggleMaximize,
    ToggleFullscreen,
}

impl ShellCommand {
    pub const fn is_enabled(self) -> bool {
        !matches!(self, Self::NewRepository | Self::CloneRepository)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSnapshot {
    pub active: bool,
    pub maximized: bool,
    pub fullscreen: bool,
    pub scale_factor: f32,
    pub appearance: WindowAppearance,
    pub can_minimize: bool,
    pub can_maximize: bool,
    pub can_fullscreen: bool,
}

impl WindowSnapshot {
    pub fn read(window: &Window) -> Self {
        let controls = window.window_controls();
        Self {
            active: window.is_window_active(),
            maximized: window.is_maximized(),
            fullscreen: window.is_fullscreen(),
            scale_factor: window.scale_factor(),
            appearance: window.appearance(),
            can_minimize: window.is_minimizable(),
            can_maximize: window.is_resizable(),
            can_fullscreen: controls.fullscreen,
        }
    }
}

pub fn dispatch_window_command(command: ShellCommand, window: &mut Window, cx: &mut App) -> bool {
    if !command.is_enabled() {
        return false;
    }

    let controls = window.window_controls();
    match command {
        ShellCommand::NewRepository
        | ShellCommand::OpenRepository
        | ShellCommand::CloneRepository => return false,
        ShellCommand::ExitApplication => cx.quit(),
        ShellCommand::ShowAbout => return false,
        ShellCommand::CloseWindow => window.remove_window(),
        ShellCommand::MinimizeWindow if window.is_minimizable() => window.minimize_window(),
        ShellCommand::ToggleMaximize if window.is_resizable() => {
            if window.is_maximized() {
                return restore_window(window);
            }
            window.zoom_window();
        }
        ShellCommand::ToggleFullscreen if controls.fullscreen => window.toggle_fullscreen(),
        ShellCommand::MinimizeWindow
        | ShellCommand::ToggleMaximize
        | ShellCommand::ToggleFullscreen => return false,
    }
    true
}

#[cfg(test)]
mod tests {
    use super::ShellCommand;

    #[test]
    fn repository_commands_are_disabled() {
        assert!(!ShellCommand::NewRepository.is_enabled());
        assert!(ShellCommand::OpenRepository.is_enabled());
        assert!(!ShellCommand::CloneRepository.is_enabled());
        assert!(ShellCommand::ExitApplication.is_enabled());
    }
}
