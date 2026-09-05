use gpui::{
    AccessibleAction, AnyElement, Context, Entity, FocusHandle, Focusable, MouseButton, Render,
    Role, Subscription, Window, WindowControlArea, actions, deferred, div, prelude::*, px, rgb,
    svg,
};
use widget::{NF, TabBar, TabSpec, Theme};
use zit::project::{ProjectId, ProjectStore, ProjectStoreEvent};

use crate::{
    about::{APPLICATION_NAME, APPLICATION_VERSION},
    assets::icon_path,
    menu::{MenuId, MenuState, items},
    platform::{PlatformProfile, ShellCommand, WindowSnapshot, dispatch_window_command},
    titlebar::{TITLEBAR_HEIGHT, control_width, controls_on_left},
};

actions!(
    zit_menu,
    [MenuLeft, MenuRight, MenuUp, MenuDown, MenuEnter, MenuEscape]
);

pub struct ZitShell {
    profile: PlatformProfile,
    menus: MenuState,
    about_open: bool,
    dark: bool,
    active: bool,
    focus_handle: FocusHandle,
    tab_bar: Entity<TabBar>,
    project_store: Entity<ProjectStore>,
    _subscriptions: Vec<Subscription>,
}

impl Focusable for ZitShell {
    fn focus_handle(&self, _: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ZitShell {
    pub fn new(window: &mut Window, theme: Theme, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle().tab_stop(true);
        window.focus(&focus_handle, cx);
        let subscriptions = vec![
            cx.observe_window_bounds(window, |_, _, cx| cx.notify()),
            cx.observe_window_appearance(window, |_, _, cx| cx.notify()),
            cx.observe_window_activation(window, |this, window, cx| {
                if !window.is_window_active() {
                    this.menus.close();
                }
                cx.notify();
            }),
        ];
        let project_store = ProjectStore::global(cx);
        let (tabs, active_project): (Vec<_>, _) = project_store.read_with(cx, |store, _| {
            let tabs = store
                .projects()
                .iter()
                .filter_map(|project| {
                    store.page(&project.id).map(|page| {
                        TabSpec::new(
                            project.id.as_str().to_owned(),
                            project.display_name.clone(),
                            NF,
                            page,
                        )
                    })
                })
                .collect();
            (tabs, store.active_project().cloned())
        });
        let active_tab = active_project
            .as_ref()
            .map(|id| widget::TabId::new(id.as_str().to_owned()));
        let tab_bar = cx.new(|cx| {
            let mut tab_bar =
                TabBar::new(tabs, active_tab, cx).expect("project tab IDs are unique");
            tab_bar.set_theme(theme, cx);
            tab_bar
        });
        let mut subscriptions = subscriptions;
        subscriptions.push(cx.subscribe(&tab_bar, Self::on_tab_event));
        subscriptions.push(cx.subscribe(&project_store, Self::on_project_store_event));
        if let Some(id) = &active_project {
            project_store.update(cx, |store, cx| {
                store.activate(id, cx);
            });
        }
        Self {
            profile: PlatformProfile::current(),
            menus: MenuState::default(),
            about_open: false,
            dark: true,
            active: true,
            focus_handle,
            tab_bar,
            project_store,
            _subscriptions: subscriptions,
        }
    }

    fn on_tab_event(
        &mut self,
        _: Entity<TabBar>,
        event: &widget::TabBarEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            widget::TabBarEvent::SelectionChanged(id) => {
                if let Ok(id) = ProjectId::new(id.as_str().to_owned()) {
                    self.project_store.update(cx, |store, cx| {
                        store.activate(&id, cx);
                    });
                }
            }
            widget::TabBarEvent::Closed(id) => {
                if let Ok(id) = ProjectId::new(id.as_str().to_owned()) {
                    self.project_store.update(cx, |store, cx| {
                        store.remove(&id, cx);
                    });
                }
            }
            widget::TabBarEvent::CloseRequested(_) => {}
        }
    }

    fn on_project_store_event(
        &mut self,
        _: Entity<ProjectStore>,
        event: &ProjectStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            ProjectStoreEvent::MetadataChanged(id) => {
                if let Some(metadata) = self.project_store.read_with(cx, |store, _| {
                    store
                        .projects()
                        .iter()
                        .find(|project| &project.id == id)
                        .cloned()
                }) {
                    let tab_id = widget::TabId::new(id.as_str().to_owned());
                    self.tab_bar.update(cx, |bar, cx| {
                        bar.update_metadata(&tab_id, metadata.display_name, NF, cx);
                    });
                }
            }
            ProjectStoreEvent::ActiveChanged(active) => {
                if let Some(active) = active {
                    let tab_id = widget::TabId::new(active.as_str().to_owned());
                    self.tab_bar.update(cx, |bar, cx| {
                        bar.select(&tab_id, cx);
                    });
                }
            }
            ProjectStoreEvent::ProjectRemoved { project_id, .. } => {
                let tab_id = widget::TabId::new(project_id.as_str().to_owned());
                self.tab_bar.update(cx, |bar, cx| {
                    bar.close(&tab_id, cx);
                });
            }
            ProjectStoreEvent::Restored { .. } => {}
        }
    }

    fn execute(&mut self, command: ShellCommand, window: &mut Window, cx: &mut Context<Self>) {
        if !command.is_enabled() {
            return;
        }
        self.menus.close();
        if command == ShellCommand::ShowAbout {
            self.about_open = true;
        } else {
            dispatch_window_command(command, window, cx);
        }
        cx.notify();
    }

    fn menu_left(&mut self, _: &MenuLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.menus.switch(-1);
        cx.notify();
    }
    fn menu_right(&mut self, _: &MenuRight, _: &mut Window, cx: &mut Context<Self>) {
        self.menus.switch(1);
        cx.notify();
    }
    fn menu_up(&mut self, _: &MenuUp, _: &mut Window, cx: &mut Context<Self>) {
        self.menus.move_highlight(-1);
        cx.notify();
    }
    fn menu_down(&mut self, _: &MenuDown, _: &mut Window, cx: &mut Context<Self>) {
        if self.menus.open.is_none() {
            self.menus.open(MenuId::File);
        } else {
            self.menus.move_highlight(1);
        }
        cx.notify();
    }
    fn menu_enter(&mut self, _: &MenuEnter, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(command) = self.menus.selected_command() {
            self.execute(command, window, cx);
        } else {
            self.menus.open(MenuId::File);
            cx.notify();
        }
    }
    fn menu_escape(&mut self, _: &MenuEscape, window: &mut Window, cx: &mut Context<Self>) {
        if self.about_open {
            self.about_open = false;
        } else {
            self.menus.close();
        }
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }

    fn heading(&self, id: MenuId, label: &'static str, cx: &mut Context<Self>) -> AnyElement {
        let selected = self.menus.open == Some(id);
        let hover_background = if self.dark { 0x4a4d52 } else { 0xdfe3e8 };
        let selected_background = if self.dark { 0x55585e } else { 0xd4d9df };
        div()
            .id(match id {
                MenuId::File => "menu-file",
                MenuId::Help => "menu-help",
            })
            .relative()
            .role(Role::MenuItem)
            .aria_label(label)
            .aria_expanded(selected)
            .focusable()
            .tab_stop(true)
            .h_full()
            .px_3()
            .flex()
            .items_center()
            .cursor_pointer()
            .when(selected, |style| style.bg(rgb(selected_background)))
            .hover(move |style| style.bg(rgb(hover_background)))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(move |this, _, _, cx| {
                this.menus.toggle(id);
                cx.notify();
            }))
            .child(label)
            .when(selected, |heading| {
                heading.child(deferred(self.popup(id, cx)).with_priority(1))
            })
            .into_any_element()
    }

    fn popup(&self, id: MenuId, cx: &mut Context<Self>) -> AnyElement {
        let popup_background = if self.dark { 0x3b3e43 } else { 0xf7f8fa };
        let hover_background = if self.dark { 0x55585e } else { 0xe1e5ea };
        let border_color = if self.dark { 0x686c72 } else { 0xc7ccd2 };
        div()
            .id(match id {
                MenuId::File => "file-popup",
                MenuId::Help => "help-popup",
            })
            .role(Role::Menu)
            .aria_label(match id {
                MenuId::File => "File menu",
                MenuId::Help => "Help menu",
            })
            .absolute()
            .top(px(TITLEBAR_HEIGHT))
            .left(px(0.0))
            .w(px(210.0))
            .py_1()
            .text_xs()
            .bg(rgb(popup_background))
            .border_1()
            .border_color(rgb(border_color))
            .shadow_lg()
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .children(items(id).iter().enumerate().map(|(index, item)| {
                let command = item.command;
                let enabled = command.is_enabled();
                div()
                    .id(item.label)
                    .role(Role::MenuItem)
                    .aria_label(item.label)
                    .aria_description(if enabled { "Available" } else { "Unavailable" })
                    .when(self.menus.highlighted == Some(index), |style| {
                        style.bg(rgb(hover_background)).aria_active_descendant()
                    })
                    .h(px(30.0))
                    .px_3()
                    .flex()
                    .items_center()
                    .when(!enabled, |style| style.text_color(rgb(0x80868b)))
                    .cursor_pointer()
                    .hover(move |style| style.bg(rgb(hover_background)))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        if !enabled {
                            return;
                        }
                        this.execute(command, window, cx);
                    }))
                    .child(item.label)
            }))
            .into_any_element()
    }

    #[allow(clippy::too_many_arguments)]
    fn control(
        &self,
        id: &'static str,
        icon: &'static str,
        area: WindowControlArea,
        command: ShellCommand,
        enabled: bool,
        close: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let profile = self.profile;
        let icon_color = if self.dark { 0xf1f3f4 } else { 0x202124 };
        div()
            .id(id)
            .role(Role::Button)
            .aria_label(match command {
                ShellCommand::CloseWindow => "Close window",
                ShellCommand::MinimizeWindow => "Minimize window",
                ShellCommand::ToggleMaximize if icon == "restore" => "Restore window",
                ShellCommand::ToggleMaximize => "Maximize window",
                ShellCommand::ToggleFullscreen => "Toggle full screen",
                _ => "Window control",
            })
            .focusable()
            .tab_stop(enabled)
            .h_full()
            .w(px(control_width(profile)))
            .flex()
            .items_center()
            .justify_center()
            .when(!enabled, |style| style.opacity(0.4))
            .when(enabled && !self.active, |style| style.opacity(0.65))
            .when(enabled, |style| {
                style
                    .cursor_pointer()
                    .hover(move |style| style.bg(rgb(if close { 0xe81123 } else { 0x3c4043 })))
                    .active(|style| style.opacity(0.8))
            })
            .when(profile != PlatformProfile::Windows, |element| {
                element.window_control_area(area)
            })
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .when(enabled, |element| {
                element.on_click(cx.listener(move |_, _, window, cx| {
                    cx.stop_propagation();
                    dispatch_window_command(command, window, cx);
                    cx.notify();
                }))
            })
            .when(enabled, |element| {
                element.on_a11y_action(AccessibleAction::Click, {
                    move |_, _, cx| {
                        if let Some(handle) = cx.active_window() {
                            handle
                                .update(cx, |_, window, cx| {
                                    dispatch_window_command(command, window, cx);
                                })
                                .ok();
                        }
                    }
                })
            })
            .child(
                svg()
                    .path(icon_path(profile, icon))
                    .size(px(16.0))
                    .text_color(rgb(icon_color)),
            )
            .into_any_element()
    }

    fn controls(&self, snapshot: WindowSnapshot, cx: &mut Context<Self>) -> Vec<AnyElement> {
        if self.profile == PlatformProfile::MacOs {
            vec![
                self.control(
                    "close",
                    "close",
                    WindowControlArea::Close,
                    ShellCommand::CloseWindow,
                    true,
                    true,
                    cx,
                ),
                self.control(
                    "minimize",
                    "minimize",
                    WindowControlArea::Min,
                    ShellCommand::MinimizeWindow,
                    snapshot.can_minimize,
                    false,
                    cx,
                ),
                self.control(
                    "fullscreen",
                    "fullscreen",
                    WindowControlArea::Max,
                    ShellCommand::ToggleFullscreen,
                    snapshot.can_fullscreen,
                    false,
                    cx,
                ),
            ]
        } else {
            vec![
                self.control(
                    "minimize",
                    "minimize",
                    WindowControlArea::Min,
                    ShellCommand::MinimizeWindow,
                    snapshot.can_minimize,
                    false,
                    cx,
                ),
                self.control(
                    "maximize",
                    if snapshot.maximized {
                        "restore"
                    } else {
                        "maximize"
                    },
                    WindowControlArea::Max,
                    ShellCommand::ToggleMaximize,
                    snapshot.can_maximize,
                    false,
                    cx,
                ),
                self.control(
                    "close",
                    "close",
                    WindowControlArea::Close,
                    ShellCommand::CloseWindow,
                    true,
                    true,
                    cx,
                ),
            ]
        }
    }

    fn about(&self, cx: &mut Context<Self>) -> AnyElement {
        div()
            .id("about-overlay")
            .role(Role::Dialog)
            .aria_label("About Zit")
            .absolute()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::rgba(0x00000088))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .w(px(320.0))
                    .p_6()
                    .bg(rgb(0x292a2d))
                    .border_1()
                    .border_color(rgb(0x5f6368))
                    .rounded_md()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(APPLICATION_NAME)
                    .child(format!("Version {APPLICATION_VERSION}"))
                    .child(
                        div()
                            .id("about-close")
                            .role(Role::Button)
                            .aria_label("Close About dialog")
                            .focusable()
                            .tab_stop(true)
                            .px_3()
                            .py_1()
                            .self_end()
                            .cursor_pointer()
                            .bg(rgb(0x3c4043))
                            .hover(|style| style.bg(rgb(0x5f6368)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.about_open = false;
                                cx.notify();
                            }))
                            .child("Close"),
                    ),
            )
            .into_any_element()
    }
}

impl Render for ZitShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = WindowSnapshot::read(window);
        self.dark = matches!(
            snapshot.appearance,
            gpui::WindowAppearance::Dark | gpui::WindowAppearance::VibrantDark
        );
        self.active = snapshot.active;
        let content_background = if self.dark { 0x202124 } else { 0xf5f5f5 };
        let titlebar_background = if self.dark { 0x383b40 } else { 0xf1f3f5 };
        let foreground = if self.dark { 0xf1f3f4 } else { 0x202124 };
        let controls = self.controls(snapshot, cx);
        let left_controls = controls_on_left(self.profile);
        let (left, right) = if left_controls {
            (controls, Vec::new())
        } else {
            (Vec::new(), controls)
        };
        let projects_empty = self
            .project_store
            .read_with(cx, |store, _| store.projects().is_empty());

        div()
            .id("zit-shell")
            .role(Role::Application)
            .aria_label("Zit")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::menu_left))
            .on_action(cx.listener(Self::menu_right))
            .on_action(cx.listener(Self::menu_up))
            .on_action(cx.listener(Self::menu_down))
            .on_action(cx.listener(Self::menu_enter))
            .on_action(cx.listener(Self::menu_escape))
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(content_background))
            .text_color(rgb(foreground))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    if this.menus.open.is_some() {
                        this.menus.close();
                        cx.notify();
                    }
                }),
            )
            .child(
                div()
                    .id("menu-bar")
                    .role(Role::MenuBar)
                    .aria_label("Application menu")
                    .text_xs()
                    .h(px(TITLEBAR_HEIGHT))
                    .w_full()
                    .flex()
                    .items_center()
                    .bg(rgb(titlebar_background))
                    .window_control_area(WindowControlArea::Drag)
                    .children(left)
                    .child(self.heading(MenuId::File, "File", cx))
                    .child(self.heading(MenuId::Help, "Help", cx))
                    .child(div().flex_1().h_full())
                    .children(right),
            )
            .child(
                div()
                    .relative()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(self.tab_bar.clone()),
            )
            .when(projects_empty, |root| {
                root.child(
                    div()
                        .absolute()
                        .top(px(TITLEBAR_HEIGHT))
                        .left(px(0.))
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child("No projects are open."),
                )
            })
            .when(self.about_open, |root| root.child(self.about(cx)))
    }
}
