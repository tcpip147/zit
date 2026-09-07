#[path = "../src/platform.rs"]
mod platform;

use platform::ShellCommand;

#[test]
fn unavailable_repository_commands_are_rejected_at_dispatch_boundary() {
    assert!(ShellCommand::NewRepository.is_enabled());
    assert!(!ShellCommand::OpenRepository.is_enabled());
    assert!(!ShellCommand::CloneRepository.is_enabled());
}

#[test]
fn shell_and_window_commands_are_available() {
    for command in [
        ShellCommand::ExitApplication,
        ShellCommand::ShowAbout,
        ShellCommand::CloseWindow,
        ShellCommand::MinimizeWindow,
        ShellCommand::ToggleMaximize,
        ShellCommand::ToggleFullscreen,
    ] {
        assert!(command.is_enabled());
    }
}
