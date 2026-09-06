use std::rc::Rc;

use crate::Theme;
use gpui::{
    AnyView, Context, EntityId, EventEmitter, MouseButton, Role, ScrollHandle, SharedString,
    Window, div, prelude::*, px, svg,
};

/// Bundled fallback icon path. Hosts must expose this path from their `AssetSource`.
pub const NF: &str = "tab-bar/fallback-project.svg";
const CLOSE_ICON: &str = "tab-bar/close.svg";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TabId(SharedString);

impl TabId {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self(id.into())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&'static str> for TabId {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for TabId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<SharedString> for TabId {
    fn from(value: SharedString) -> Self {
        Self::new(value)
    }
}

#[derive(Clone)]
pub struct TabSpec {
    pub id: TabId,
    pub project_name: SharedString,
    pub project_icon: SharedString,
    pub page: AnyView,
}

impl TabSpec {
    pub fn new(
        id: impl Into<TabId>,
        name: impl Into<SharedString>,
        icon: impl Into<SharedString>,
        page: impl Into<AnyView>,
    ) -> Self {
        Self {
            id: id.into(),
            project_name: name.into(),
            project_icon: icon.into(),
            page: page.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TabBarEvent {
    SelectionChanged(TabId),
    CloseRequested(TabId),
    Closed(TabId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TabBarError {
    DuplicateId(TabId),
}

struct TabEntry {
    spec: TabSpec,
}

pub struct TabBar {
    tabs: Vec<TabEntry>,
    active: Option<TabId>,
    scroll: ScrollHandle,
    can_close: Rc<dyn Fn(&TabId) -> bool>,
    theme: Theme,
}

impl TabBar {
    /// Rejects duplicate stable IDs without constructing UI state.
    pub fn validate(specs: &[TabSpec]) -> Result<(), TabBarError> {
        for (index, spec) in specs.iter().enumerate() {
            if specs[..index].iter().any(|other| other.id == spec.id) {
                return Err(TabBarError::DuplicateId(spec.id.clone()));
            }
        }
        Ok(())
    }

    /// Builds a retained tab/page collection.
    ///
    /// Duplicate IDs are rejected. If `active` is absent or unknown, the first
    /// tab becomes active. Page entities are retained and are never recreated
    /// while switching tabs.
    pub fn new(
        specs: impl IntoIterator<Item = TabSpec>,
        active: Option<TabId>,
        _cx: &mut Context<Self>,
    ) -> Result<Self, TabBarError> {
        let specs: Vec<_> = specs.into_iter().collect();
        Self::validate(&specs)?;
        let mut tabs = Vec::new();
        for spec in specs {
            tabs.push(TabEntry { spec });
        }
        let active = active
            .filter(|id| tabs.iter().any(|tab| &tab.spec.id == id))
            .or_else(|| tabs.first().map(|tab| tab.spec.id.clone()));
        Ok(Self {
            tabs,
            active,
            scroll: ScrollHandle::new(),
            can_close: Rc::new(|_| true),
            theme: Theme::default(),
        })
    }

    pub fn set_theme(&mut self, theme: Theme, cx: &mut Context<Self>) {
        self.theme = theme;
        cx.notify();
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn set_close_filter(&mut self, filter: impl Fn(&TabId) -> bool + 'static) {
        self.can_close = Rc::new(filter);
    }
    pub fn len(&self) -> usize {
        self.tabs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
    pub fn active_id(&self) -> Option<&TabId> {
        self.active.as_ref()
    }
    pub fn ids(&self) -> impl Iterator<Item = &TabId> {
        self.tabs.iter().map(|tab| &tab.spec.id)
    }

    pub fn project_name(&self, id: &TabId) -> Option<&str> {
        self.index(id)
            .map(|index| self.tabs[index].spec.project_name.as_ref())
    }

    pub fn update_metadata(
        &mut self,
        id: &TabId,
        name: impl Into<SharedString>,
        icon: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(index) = self.index(id) else {
            return false;
        };
        self.tabs[index].spec.project_name = name.into();
        self.tabs[index].spec.project_icon = icon.into();
        cx.notify();
        true
    }
    pub fn active_page_entity_id(&self) -> Option<EntityId> {
        self.active_entry().map(|tab| tab.spec.page.entity_id())
    }
    pub fn scroll_offset(&self) -> gpui::Point<gpui::Pixels> {
        self.scroll.offset()
    }

    pub fn insert(
        &mut self,
        spec: TabSpec,
        activate: bool,
        cx: &mut Context<Self>,
    ) -> Result<(), TabBarError> {
        if self.tabs.iter().any(|tab| tab.spec.id == spec.id) {
            return Err(TabBarError::DuplicateId(spec.id));
        }
        let id = spec.id.clone();
        self.tabs.push(TabEntry { spec });
        if self.active.is_none() || activate {
            self.select(&id, cx);
        }
        cx.notify();
        Ok(())
    }

    pub fn select(&mut self, id: &TabId, cx: &mut Context<Self>) -> bool {
        let Some(index) = self.index(id) else {
            return false;
        };
        self.scroll.scroll_to_item(index);
        if self.active.as_ref() == Some(id) {
            cx.notify();
            return true;
        }
        self.active = Some(id.clone());
        cx.emit(TabBarEvent::SelectionChanged(id.clone()));
        cx.notify();
        true
    }

    pub fn request_close(&mut self, id: &TabId, cx: &mut Context<Self>) -> bool {
        if self.index(id).is_none() {
            return false;
        }
        cx.emit(TabBarEvent::CloseRequested(id.clone()));
        if !(self.can_close)(id) {
            return false;
        }
        self.close(id, cx)
    }

    pub fn close(&mut self, id: &TabId, cx: &mut Context<Self>) -> bool {
        let Some(index) = self.index(id) else {
            return false;
        };
        let was_active = self.active.as_ref() == Some(id);
        self.tabs.remove(index);
        let neighbor = self
            .tabs
            .get(index)
            .or_else(|| index.checked_sub(1).and_then(|i| self.tabs.get(i)))
            .map(|tab| tab.spec.id.clone());
        if was_active {
            self.active = neighbor.clone();
            if let Some(replacement) = neighbor {
                self.scroll
                    .scroll_to_item(self.index(&replacement).unwrap_or(0));
                cx.emit(TabBarEvent::SelectionChanged(replacement));
            }
        }
        cx.emit(TabBarEvent::Closed(id.clone()));
        cx.notify();
        true
    }

    fn index(&self, id: &TabId) -> Option<usize> {
        self.tabs.iter().position(|tab| &tab.spec.id == id)
    }
    fn active_entry(&self) -> Option<&TabEntry> {
        self.active
            .as_ref()
            .and_then(|id| self.index(id))
            .and_then(|i| self.tabs.get(i))
    }
}

impl EventEmitter<TabBarEvent> for TabBar {}

impl Render for TabBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.tab_bar.clone();
        let active_page = self.active_entry().map(|tab| tab.spec.page.clone());
        let count = self.tabs.len();
        div()
            .size_full()
            .text_xs()
            .flex()
            .flex_col()
            .child(
                div()
                    .id("tab-list")
                    .role(Role::TabList)
                    .aria_label("Project tabs")
                    .h(px(30.))
                    .w_full()
                    .flex()
                    .flex_row()
                    .items_stretch()
                    .flex_none()
                    .border_b_1()
                    .bg(theme.background.gpui())
                    .text_color(theme.text.gpui())
                    .border_color(theme.border.gpui())
                    .overflow_x_scroll()
                    .track_scroll(&self.scroll)
                    .children(self.tabs.iter().enumerate().map(|(index, tab)| {
                        let id = tab.spec.id.clone();
                        let close_id = id.clone();
                        let selected = self.active.as_ref() == Some(&id);
                        let icon = if tab.spec.project_icon.is_empty() {
                            SharedString::from(NF)
                        } else {
                            tab.spec.project_icon.clone()
                        };
                        div()
                            .id(id.as_str().to_string())
                            .role(Role::Tab)
                            .aria_label(tab.spec.project_name.clone())
                            .aria_selected(selected)
                            .aria_position_in_set(index + 1)
                            .aria_size_of_set(count)
                            .flex()
                            .flex_none()
                            .items_center()
                            .gap_2()
                            .h_full()
                            .w(px(180.))
                            .px_3()
                            .text_xs()
                            .border_r_1()
                            .bg(theme.tab_background.gpui())
                            .border_color(theme.border.gpui())
                            .cursor_pointer()
                            .when(selected, |el| el.bg(theme.tab_active_background.gpui()))
                            .hover({
                                let color = theme.tab_hover_background.gpui();
                                move |el| el.bg(color)
                            })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.select(&id, cx);
                            }))
                            .child(
                                svg()
                                    .path(icon)
                                    .size(px(16.))
                                    .flex_none()
                                    .text_color(theme.icon.gpui()),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .overflow_hidden()
                                    .whitespace_nowrap()
                                    .child(tab.spec.project_name.clone()),
                            )
                            .child(
                                div()
                                    .id(("close", index))
                                    .role(Role::Button)
                                    .aria_label(SharedString::from(format!(
                                        "Close {}",
                                        tab.spec.project_name
                                    )))
                                    .size(px(20.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .flex_none()
                                    .cursor_pointer()
                                    .hover({
                                        let color = theme.close_hover_background.gpui();
                                        move |el| el.bg(color)
                                    })
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        cx.stop_propagation();
                                        this.request_close(&close_id, cx);
                                    }))
                                    .child(
                                        svg()
                                            .path(CLOSE_ICON)
                                            .size(px(14.))
                                            .text_color(theme.icon.gpui()),
                                    ),
                            )
                    })),
            )
            .child(div().flex_1().overflow_hidden().children(active_page))
    }
}
