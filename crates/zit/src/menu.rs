use crate::platform::ShellCommand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuId {
    File,
    Help,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuItem {
    pub label: &'static str,
    pub command: ShellCommand,
}

pub const FILE_ITEMS: [MenuItem; 4] = [
    MenuItem {
        label: "New Repository",
        command: ShellCommand::NewRepository,
    },
    MenuItem {
        label: "Open ...",
        command: ShellCommand::OpenRepository,
    },
    MenuItem {
        label: "Clone ...",
        command: ShellCommand::CloneRepository,
    },
    MenuItem {
        label: "Exit",
        command: ShellCommand::ExitApplication,
    },
];

pub const HELP_ITEMS: [MenuItem; 1] = [MenuItem {
    label: "About",
    command: ShellCommand::ShowAbout,
}];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MenuState {
    pub open: Option<MenuId>,
    pub highlighted: Option<usize>,
}

impl MenuState {
    pub fn open(&mut self, menu: MenuId) {
        self.open = Some(menu);
        self.highlighted = Some(first_enabled(items(menu)));
    }

    pub fn toggle(&mut self, menu: MenuId) {
        self.open = (self.open != Some(menu)).then_some(menu);
        self.highlighted = self.open.map(|id| first_enabled(items(id)));
    }

    pub fn close(&mut self) {
        self.open = None;
        self.highlighted = None;
    }

    pub fn switch(&mut self, direction: i32) {
        let next = match (self.open.unwrap_or(MenuId::File), direction.signum()) {
            (MenuId::File, 1) | (MenuId::Help, -1) => MenuId::Help,
            _ => MenuId::File,
        };
        self.open(next);
    }

    pub fn move_highlight(&mut self, direction: i32) {
        let Some(menu) = self.open else { return };
        let entries = items(menu);
        if entries.is_empty() {
            return;
        }
        let start = self.highlighted.unwrap_or_else(|| first_enabled(entries));
        for offset in 1..=entries.len() {
            let index = if direction < 0 {
                (start + entries.len() - (offset % entries.len())) % entries.len()
            } else {
                (start + offset) % entries.len()
            };
            if entries[index].command.is_enabled() {
                self.highlighted = Some(index);
                return;
            }
        }
    }

    pub fn selected_command(&self) -> Option<ShellCommand> {
        let command = items(self.open?).get(self.highlighted?)?.command;
        command.is_enabled().then_some(command)
    }
}

pub const fn items(menu: MenuId) -> &'static [MenuItem] {
    match menu {
        MenuId::File => &FILE_ITEMS,
        MenuId::Help => &HELP_ITEMS,
    }
}

fn first_enabled(items: &[MenuItem]) -> usize {
    items
        .iter()
        .position(|item| item.command.is_enabled())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_contract_has_exact_labels_and_order() {
        assert_eq!(
            FILE_ITEMS.map(|item| item.label),
            ["New Repository", "Open ...", "Clone ...", "Exit"]
        );
        assert_eq!(HELP_ITEMS.map(|item| item.label), ["About"]);
    }

    #[test]
    fn only_one_menu_can_be_open() {
        let mut state = MenuState::default();
        state.toggle(MenuId::File);
        state.toggle(MenuId::Help);
        assert_eq!(state.open, Some(MenuId::Help));
    }

    #[test]
    fn keyboard_navigation_skips_disabled_commands() {
        let mut state = MenuState::default();
        state.open(MenuId::File);
        assert_eq!(state.highlighted, Some(3));
        assert_eq!(
            state.selected_command(),
            Some(ShellCommand::ExitApplication)
        );
        state.switch(1);
        assert_eq!(state.selected_command(), Some(ShellCommand::ShowAbout));
    }
}
