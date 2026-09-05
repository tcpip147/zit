use crate::platform::PlatformProfile;

pub const TITLEBAR_HEIGHT: f32 = 36.0;
pub const WINDOWS_CONTROL_WIDTH: f32 = 46.0;
pub const LINUX_CONTROL_WIDTH: f32 = 46.0;
pub const MACOS_CONTROL_SIZE: f32 = 14.0;

pub const fn controls_on_left(profile: PlatformProfile) -> bool {
    matches!(profile, PlatformProfile::MacOs)
}

pub const fn control_width(profile: PlatformProfile) -> f32 {
    match profile {
        PlatformProfile::Windows => WINDOWS_CONTROL_WIDTH,
        PlatformProfile::Linux => LINUX_CONTROL_WIDTH,
        PlatformProfile::MacOs => MACOS_CONTROL_SIZE + 14.0,
    }
}
