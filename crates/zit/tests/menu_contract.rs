#[path = "../src/menu.rs"]
mod menu;
#[path = "../src/platform.rs"]
mod platform;

use menu::{FILE_ITEMS, HELP_ITEMS, MenuId, MenuState};
use platform::ShellCommand;

#[test]
fn requested_menu_contract_is_exact_and_safe() {
    assert_eq!(
        FILE_ITEMS.map(|item| item.label),
        ["New Repository", "Open ...", "Clone ...", "Exit"]
    );
    assert_eq!(HELP_ITEMS.map(|item| item.label), ["About"]);
    assert!(
        FILE_ITEMS[..3]
            .iter()
            .all(|item| !item.command.is_enabled())
    );

    let mut state = MenuState::default();
    state.open(MenuId::File);
    state.move_highlight(1);
    assert_eq!(
        state.selected_command(),
        Some(ShellCommand::ExitApplication)
    );
    state.switch(1);
    assert_eq!(state.open, Some(MenuId::Help));
    assert_eq!(state.selected_command(), Some(ShellCommand::ShowAbout));
    state.close();
    assert_eq!(state.open, None);
}
