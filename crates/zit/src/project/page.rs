use std::{sync::Arc, time::Duration};

use gpui::{
    AccessibleAction, AnyElement, Context, CursorStyle, EventEmitter, InteractiveElement,
    ListHorizontalSizingBehavior, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Render, Role, ScrollWheelEvent, StatefulInteractiveElement, Task, UniformListScrollHandle,
    Window, canvas, div, point, prelude::*, px, rgb, svg, uniform_list,
};

use super::{ProjectId, ProjectLoadState, ProjectMetadata, RepositoryLoader};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectToolbarAction {
    Commit,
    Pull,
    Push,
    Fetch,
    Branch,
    Merge,
    Stash,
    Discard,
    Tag,
    GitFlow,
    Terminal,
    Explorer,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectPageEvent {
    ToolbarAction(ProjectToolbarAction),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProjectSection {
    #[default]
    FileStatus,
    History,
    Search,
}

pub struct ProjectPage {
    metadata: ProjectMetadata,
    state: ProjectLoadState,
    generation: u64,
    loader: Arc<dyn RepositoryLoader>,
    load_task: Option<Task<()>>,
    selected_section: ProjectSection,
    history_scroll: UniformListScrollHandle,
    history_scroll_drag: Option<(f32, f32)>,
    history_scrollbar_visible: bool,
    history_scrollbar_hide_task: Option<Task<()>>,
}

impl ProjectPage {
    pub fn new(metadata: ProjectMetadata, loader: Arc<dyn RepositoryLoader>) -> Self {
        Self {
            metadata,
            state: ProjectLoadState::NotLoaded,
            generation: 0,
            loader,
            load_task: None,
            selected_section: ProjectSection::default(),
            history_scroll: UniformListScrollHandle::new(),
            history_scroll_drag: None,
            history_scrollbar_visible: false,
            history_scrollbar_hide_task: None,
        }
    }

    pub fn project_id(&self) -> &ProjectId {
        &self.metadata.id
    }
    pub fn metadata(&self) -> &ProjectMetadata {
        &self.metadata
    }
    pub fn load_state(&self) -> &ProjectLoadState {
        &self.state
    }
    pub fn selected_section(&self) -> ProjectSection {
        self.selected_section
    }

    pub fn select_section(&mut self, section: ProjectSection, cx: &mut Context<Self>) {
        if self.selected_section != section {
            self.selected_section = section;
            cx.notify();
        }
        if section == ProjectSection::History && matches!(self.state, ProjectLoadState::NotLoaded) {
            self.start_load(cx);
        }
    }

    pub fn update_metadata(&mut self, metadata: ProjectMetadata, cx: &mut Context<Self>) {
        self.metadata = metadata;
        cx.notify();
    }

    pub fn activate(&mut self, cx: &mut Context<Self>) {
        if matches!(self.state, ProjectLoadState::NotLoaded) {
            self.start_load(cx);
        }
    }

    pub fn retry(&mut self, cx: &mut Context<Self>) {
        if matches!(self.state, ProjectLoadState::Failed { .. }) {
            let mut metadata = self.loader.inspect(
                &self.metadata.repository_path,
                Some(&self.metadata.display_name),
            );
            metadata.id = self.metadata.id.clone();
            self.metadata = metadata;
            self.start_load(cx);
        }
    }

    /// Invalidates a pending result before this page is removed.
    pub fn cancel_pending_load(&mut self, cx: &mut Context<Self>) {
        if matches!(self.state, ProjectLoadState::Loading { .. }) {
            self.generation = self.generation.wrapping_add(1);
            self.load_task = None;
            self.state = ProjectLoadState::NotLoaded;
            cx.notify();
        }
    }

    fn start_load(&mut self, cx: &mut Context<Self>) {
        if matches!(
            self.state,
            ProjectLoadState::Loading { .. } | ProjectLoadState::Ready(_)
        ) {
            return;
        }
        self.generation = self.generation.wrapping_add(1);
        let generation = self.generation;
        self.state = ProjectLoadState::Loading { generation };
        cx.notify();
        let loader = self.loader.clone();
        let metadata = self.metadata.clone();
        let background = cx
            .background_executor()
            .spawn(async move { loader.load_history(&metadata) });
        self.load_task = Some(cx.spawn(async move |this, cx| {
            let result = background.await;
            let _ = this.update(cx, |page, cx| {
                if !matches!(page.state, ProjectLoadState::Loading { generation: current } if current == generation) { return; }
                page.state = match result {
                    Ok(data) if data.project_id == page.metadata.id => ProjectLoadState::Ready(data),
                    Ok(_) => ProjectLoadState::Failed { generation, message: "Repository data did not match this project.".into() },
                    Err(error) => ProjectLoadState::Failed { generation, message: error.user_message().to_owned() },
                };
                page.load_task = None;
                cx.notify();
            });
        }));
    }

    fn state_content(&self, cx: &mut Context<Self>) -> AnyElement {
        match &self.state {
            ProjectLoadState::NotLoaded => div()
                .child("Select this project to load its history.")
                .into_any_element(),
            ProjectLoadState::Loading { .. } => div()
                .id("project-loading")
                .role(Role::Status)
                .aria_label("Loading repository history")
                .child("Loading history…")
                .into_any_element(),
            ProjectLoadState::Ready(data) if data.commits.is_empty() => div()
                .id("project-empty-history")
                .role(Role::Status)
                .aria_label("Repository history is empty")
                .child("This repository has no commits yet.")
                .into_any_element(),
            ProjectLoadState::Ready(data) => {
                let commit_count = data.commits.len();
                div()
                    .id("history-grid")
                    .w_full()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .child(Self::history_header())
                    .child(
                        div()
                            .relative()
                            .w_full()
                            .flex_1()
                            .min_h_0()
                            .overflow_hidden()
                            .on_scroll_wheel(cx.listener(Self::wheel_history_scrollbar))
                            .child(
                                uniform_list(
                                    "history-rows",
                                    commit_count,
                                    cx.processor(|page, range: std::ops::Range<usize>, _, _| {
                                        let ProjectLoadState::Ready(data) = &page.state else {
                                            return Vec::new();
                                        };
                                        range
                                            .filter_map(|index| data.commits.get(index))
                                            .map(Self::history_row)
                                            .collect()
                                    }),
                                )
                                .size_full()
                                .with_horizontal_sizing_behavior(
                                    ListHorizontalSizingBehavior::FitList,
                                )
                                .track_scroll(&self.history_scroll),
                            )
                            .child(self.history_scrollbar(cx)),
                    )
                    .into_any_element()
            }
            ProjectLoadState::Failed { message, .. } => {
                let label = format!("Retry loading {}", self.metadata.display_name);
                let page = cx.entity();
                div()
                    .id("project-load-error")
                    .role(Role::Alert)
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(message.clone())
                    .child(
                        div()
                            .id("retry-project")
                            .role(Role::Button)
                            .aria_label(label)
                            .focusable()
                            .tab_stop(true)
                            .cursor(CursorStyle::PointingHand)
                            .px_3()
                            .py_1()
                            .bg(rgb(0xe2e8f0))
                            .hover(|style| style.bg(rgb(0xcbd5e1)))
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .on_click(cx.listener(|page, _, _, cx| page.retry(cx)))
                            .on_a11y_action(AccessibleAction::Click, move |_, _, cx| {
                                page.update(cx, |page, cx| page.retry(cx));
                            })
                            .child("Retry"),
                    )
                    .into_any_element()
            }
        }
    }

    fn history_header() -> AnyElement {
        div()
            .grid()
            .grid_cols(5)
            .w_full()
            .pr_2()
            .h(px(30.))
            .items_center()
            .bg(rgb(0xf1f5f9))
            .text_color(rgb(0x475569))
            .child(Self::history_cell("Graph", true))
            .child(Self::history_cell("Commit", true))
            .child(Self::history_cell("Author", true))
            .child(Self::history_cell("Date", true))
            .child(Self::history_cell("Hash", false))
            .into_any_element()
    }

    fn history_cell(content: impl IntoElement, divider: bool) -> AnyElement {
        div()
            .h_full()
            .min_w_0()
            .px_2()
            .flex()
            .items_center()
            .overflow_hidden()
            .whitespace_nowrap()
            .when(divider, |cell| {
                cell.border_r_1().border_color(rgb(0xcbd5e1))
            })
            .child(content)
            .into_any_element()
    }

    fn history_row(commit: &super::CommitSummary) -> AnyElement {
        let short_id = commit.object_id[..commit.object_id.len().min(8)].to_owned();
        div()
            .h(px(34.))
            .w_full()
            .min_w_0()
            .pr_2()
            .grid()
            .grid_cols(5)
            .child(Self::history_cell(
                Self::commit_graph(commit.parent_ids.len() > 1),
                true,
            ))
            .child(Self::history_cell(commit.summary.clone(), true))
            .child(Self::history_cell(commit.author_name.clone(), true))
            .child(Self::history_cell(
                commit.authored_at_seconds.to_string(),
                true,
            ))
            .child(Self::history_cell(short_id, false))
            .into_any_element()
    }

    fn history_scrollbar(&self, cx: &mut Context<Self>) -> AnyElement {
        let state = self.history_scroll.0.borrow();
        let viewport = state.base_handle.bounds().size.height.as_f32().max(1.);
        let max_offset = state.base_handle.max_offset().y.as_f32().max(0.);
        let scrollable = max_offset > 0.;
        let offset = (-state.base_handle.offset().y.as_f32()).clamp(0., max_offset);
        let thumb_height = if max_offset > 0. {
            (viewport * viewport / (viewport + max_offset)).max(24.)
        } else {
            viewport
        };
        let thumb_top = if max_offset > 0. {
            offset / max_offset * (viewport - thumb_height).max(0.)
        } else {
            0.
        };

        div()
            .id("history-scroll-track")
            .absolute()
            .right(px(2.))
            .top(px(2.))
            .bottom(px(2.))
            .w(px(6.))
            .rounded(px(3.))
            .bg(rgb(0xe2e8f0))
            .on_hover(cx.listener(Self::hover_history_scrollbar))
            .when(!self.history_scrollbar_visible, |bar| {
                bar.opacity(0.).hover(|style| style.opacity(1.))
            })
            .when(scrollable, |bar| {
                bar.child(
                    div()
                        .id("history-scroll-thumb")
                        .absolute()
                        .top(px(thumb_top))
                        .w_full()
                        .h(px(thumb_height))
                        .rounded(px(3.))
                        .bg(rgb(0x94a3b8))
                        .cursor(CursorStyle::PointingHand)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(Self::start_history_scrollbar_drag),
                        ),
                )
            })
            .into_any_element()
    }

    fn start_history_scrollbar_drag(
        &mut self,
        event: &MouseDownEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let offset = self
            .history_scroll
            .0
            .borrow()
            .base_handle
            .offset()
            .y
            .as_f32();
        self.history_scroll_drag = Some((event.position.y.as_f32(), offset));
        self.show_history_scrollbar(cx);
        cx.stop_propagation();
    }

    fn drag_history_scrollbar(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((start_y, start_offset)) = self.history_scroll_drag else {
            return;
        };
        let state = self.history_scroll.0.borrow();
        let viewport = state.base_handle.bounds().size.height.as_f32().max(1.);
        let max_offset = state.base_handle.max_offset().y.as_f32().max(0.);
        let thumb_height = (viewport * viewport / (viewport + max_offset)).max(24.);
        let travel = (viewport - thumb_height).max(1.);
        let delta = event.position.y.as_f32() - start_y;
        let new_offset = (start_offset - delta * max_offset / travel).clamp(-max_offset, 0.);
        let x = state.base_handle.offset().x;
        state.base_handle.set_offset(point(x, px(new_offset)));
        drop(state);
        cx.notify();
    }

    fn stop_history_scrollbar_drag(
        &mut self,
        _: &MouseUpEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.history_scroll_drag.take().is_some() {
            self.show_history_scrollbar_temporarily(cx);
        }
    }

    fn wheel_history_scrollbar(
        &mut self,
        _: &ScrollWheelEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.history_scroll.is_scrollable() {
            self.show_history_scrollbar_temporarily(cx);
        }
    }

    fn hover_history_scrollbar(&mut self, hovered: &bool, _: &mut Window, cx: &mut Context<Self>) {
        if *hovered {
            self.show_history_scrollbar(cx);
        } else if self.history_scroll_drag.is_none() {
            self.show_history_scrollbar_temporarily(cx);
        }
    }

    fn show_history_scrollbar(&mut self, cx: &mut Context<Self>) {
        self.history_scrollbar_hide_task = None;
        if !self.history_scrollbar_visible {
            self.history_scrollbar_visible = true;
            cx.notify();
        }
    }

    fn show_history_scrollbar_temporarily(&mut self, cx: &mut Context<Self>) {
        self.show_history_scrollbar(cx);
        let timer = cx.background_executor().timer(Duration::from_secs(1));
        self.history_scrollbar_hide_task = Some(cx.spawn(async move |this, cx| {
            timer.await;
            let _ = this.update(cx, |page, cx| {
                if page.history_scroll_drag.is_none() {
                    page.history_scrollbar_visible = false;
                    page.history_scrollbar_hide_task = None;
                    cx.notify();
                }
            });
        }));
    }

    fn history_drag_event_capture(&self, cx: &mut Context<Self>) -> AnyElement {
        let page = cx.entity().downgrade();
        canvas(
            |_, _, _| (),
            move |_, _, window, _| {
                let page_for_move = page.clone();
                window.on_mouse_event(move |event: &MouseMoveEvent, _, window, cx| {
                    let _ = page_for_move.update(cx, |page, cx| {
                        page.drag_history_scrollbar(event, window, cx)
                    });
                });
                let page_for_up = page.clone();
                window.on_mouse_event(move |event: &MouseUpEvent, _, window, cx| {
                    if event.button == MouseButton::Left {
                        let _ = page_for_up.update(cx, |page, cx| {
                            page.stop_history_scrollbar_drag(event, window, cx)
                        });
                    }
                });
            },
        )
        .absolute()
        .inset_0()
        .into_any_element()
    }

    fn commit_graph(merge: bool) -> AnyElement {
        div()
            .relative()
            .w_full()
            .h_full()
            .child(
                div()
                    .absolute()
                    .left(px(17.))
                    .top(px(0.))
                    .w(px(2.))
                    .h_full()
                    .bg(rgb(0x94a3b8)),
            )
            .when(merge, |graph| {
                graph.child(
                    div()
                        .absolute()
                        .left(px(18.))
                        .top(px(16.))
                        .w(px(18.))
                        .h(px(2.))
                        .bg(rgb(0x8b5cf6)),
                )
            })
            .child(
                div()
                    .absolute()
                    .left(px(12.))
                    .top(px(11.))
                    .size(px(12.))
                    .rounded_full()
                    .border_1()
                    .border_color(rgb(0x2563eb))
                    .bg(rgb(0xffffff)),
            )
            .into_any_element()
    }

    fn toolbar_button(
        &self,
        label: &'static str,
        icon: &'static str,
        action: ProjectToolbarAction,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .id(label)
            .role(Role::Button)
            .aria_label(label)
            .focusable()
            .tab_stop(true)
            .w(px(52.))
            .h(px(50.))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(3.))
            .rounded(px(4.))
            .text_xs()
            .cursor(CursorStyle::PointingHand)
            .hover(|style| style.bg(rgb(0xe2e8f0)))
            .active(|style| style.opacity(0.8))
            .on_click(cx.listener(move |_, _, _, cx| {
                cx.emit(ProjectPageEvent::ToolbarAction(action));
            }))
            .child(svg().path(icon).size(px(18.)).text_color(rgb(0x475569)))
            .child(label)
            .into_any_element()
    }

    fn toolbar(&self, cx: &mut Context<Self>) -> AnyElement {
        let spacer = || div().w(px(14.)).h_full().flex_none();
        div()
            .id("project-toolbar")
            .w_full()
            .h(px(58.))
            .px_2()
            .flex()
            .items_center()
            .flex_none()
            .border_b_1()
            .border_color(rgb(0xcbd5e1))
            .bg(rgb(0xf8fafc))
            .child(self.toolbar_button(
                "Commit",
                "project-toolbar/commit.svg",
                ProjectToolbarAction::Commit,
                cx,
            ))
            .child(spacer())
            .child(self.toolbar_button(
                "Pull",
                "project-toolbar/pull.svg",
                ProjectToolbarAction::Pull,
                cx,
            ))
            .child(self.toolbar_button(
                "Push",
                "project-toolbar/push.svg",
                ProjectToolbarAction::Push,
                cx,
            ))
            .child(self.toolbar_button(
                "Fetch",
                "project-toolbar/fetch.svg",
                ProjectToolbarAction::Fetch,
                cx,
            ))
            .child(spacer())
            .child(self.toolbar_button(
                "Branch",
                "project-toolbar/branch.svg",
                ProjectToolbarAction::Branch,
                cx,
            ))
            .child(self.toolbar_button(
                "Merge",
                "project-toolbar/merge.svg",
                ProjectToolbarAction::Merge,
                cx,
            ))
            .child(spacer())
            .child(self.toolbar_button(
                "Stash",
                "project-toolbar/stash.svg",
                ProjectToolbarAction::Stash,
                cx,
            ))
            .child(spacer())
            .child(self.toolbar_button(
                "Discard",
                "project-toolbar/discard.svg",
                ProjectToolbarAction::Discard,
                cx,
            ))
            .child(self.toolbar_button(
                "Tag",
                "project-toolbar/tag.svg",
                ProjectToolbarAction::Tag,
                cx,
            ))
            .child(div().flex_1())
            .child(self.toolbar_button(
                "Git Flow",
                "project-toolbar/git-flow.svg",
                ProjectToolbarAction::GitFlow,
                cx,
            ))
            .child(self.toolbar_button(
                "Terminal",
                "project-toolbar/terminal.svg",
                ProjectToolbarAction::Terminal,
                cx,
            ))
            .child(self.toolbar_button(
                "Explorer",
                "project-toolbar/explorer.svg",
                ProjectToolbarAction::Explorer,
                cx,
            ))
            .child(self.toolbar_button(
                "Settings",
                "project-toolbar/settings.svg",
                ProjectToolbarAction::Settings,
                cx,
            ))
            .into_any_element()
    }

    fn sidebar_item(
        &self,
        label: &'static str,
        section: ProjectSection,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let selected = self.selected_section == section;
        div()
            .id(("project-section", section as usize))
            .role(Role::Button)
            .aria_label(label)
            .aria_selected(selected)
            .focusable()
            .tab_stop(true)
            .w_full()
            .h(px(30.))
            .px_3()
            .flex()
            .items_center()
            .text_xs()
            .cursor(CursorStyle::PointingHand)
            .when(selected, |style| style.bg(rgb(0xdbeafe)))
            .hover(|style| style.bg(rgb(0xe2e8f0)))
            .on_click(cx.listener(move |page, _, _, cx| {
                page.select_section(section, cx);
            }))
            .child(label)
            .into_any_element()
    }

    fn sidebar_heading(label: &'static str) -> AnyElement {
        div()
            .w_full()
            .h(px(28.))
            .px_3()
            .mt_2()
            .flex()
            .items_center()
            .text_xs()
            .text_color(rgb(0x64748b))
            .child(label)
            .into_any_element()
    }

    fn sidebar(&self, cx: &mut Context<Self>) -> AnyElement {
        div()
            .id("project-sidebar")
            .role(Role::Navigation)
            .aria_label("Project navigation")
            .w(px(180.))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .border_r_1()
            .border_color(rgb(0xcbd5e1))
            .bg(rgb(0xf8fafc))
            .child(Self::sidebar_heading("PROJECT"))
            .child(self.sidebar_item("File Status", ProjectSection::FileStatus, cx))
            .child(self.sidebar_item("History", ProjectSection::History, cx))
            .child(self.sidebar_item("Search", ProjectSection::Search, cx))
            .child(Self::sidebar_heading("LOCAL"))
            .child(Self::sidebar_heading("REMOTE"))
            .child(Self::sidebar_heading("ISSUES"))
            .child(Self::sidebar_heading("TAGS"))
            .into_any_element()
    }

    fn selected_content(&self, cx: &mut Context<Self>) -> AnyElement {
        let body = match self.selected_section {
            ProjectSection::FileStatus => div()
                .id("file-status-content")
                .child("Working tree changes will appear here.")
                .into_any_element(),
            ProjectSection::History => self.state_content(cx),
            ProjectSection::Search => div()
                .id("search-content")
                .child("Search this repository's history and files.")
                .into_any_element(),
        };
        div()
            .id("project-content")
            .h_full()
            .flex_1()
            .min_h_0()
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(body)
            .into_any_element()
    }
}

impl EventEmitter<ProjectPageEvent> for ProjectPage {}

impl Render for ProjectPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .text_xs()
            .flex()
            .flex_col()
            .child(div().flex_none().child(self.toolbar(cx)))
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .flex()
                    .child(self.sidebar(cx))
                    .child(self.selected_content(cx)),
            )
            .child(self.history_drag_event_capture(cx))
    }
}
