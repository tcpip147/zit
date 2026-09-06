use std::{rc::Rc, sync::Arc};

use gpui::{
    AccessibleAction, AnyElement, Context, CursorStyle, Entity, EventEmitter, InteractiveElement,
    MouseButton, Render, Role, StatefulInteractiveElement, Task, Window, div, prelude::*, px, rgb,
    svg,
};
use widget::{DataGrid, DataGridCell, DataGridColumn, DataGridRow};

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
    history_grid: Option<Entity<DataGrid>>,
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
            history_grid: None,
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
                page.history_grid = match &page.state {
                    ProjectLoadState::Ready(data) if !data.commits.is_empty() => {
                        let commits = Rc::new(data.commits.clone());
                        Some(cx.new(|_| DataGrid::new(Self::history_columns(), commits.len(), move |index| Self::history_row(&commits[index])).expect("history columns are valid")))
                    }
                    _ => None,
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
            ProjectLoadState::Ready(_) => self
                .history_grid
                .as_ref()
                .expect("ready history owns a grid")
                .clone()
                .into_any_element(),
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

    fn history_columns() -> Vec<DataGridColumn> {
        ["Graph", "Commit", "Author", "Date", "Hash"]
            .into_iter()
            .enumerate()
            .map(|(index, label)| {
                DataGridColumn::new(label.to_ascii_lowercase(), label).divider(index < 4)
            })
            .collect()
    }

    fn history_row(commit: &super::CommitSummary) -> DataGridRow {
        let short_id = commit.object_id[..commit.object_id.len().min(8)].to_owned();
        DataGridRow::new([
            DataGridCell::new(Self::commit_graph(commit.parent_ids.len() > 1)).accessible_label(
                if commit.parent_ids.len() > 1 {
                    "Merge commit graph"
                } else {
                    "Commit graph"
                },
            ),
            DataGridCell::text(commit.summary.clone()),
            DataGridCell::text(commit.author_name.clone()),
            DataGridCell::text(commit.authored_at_seconds.to_string()),
            DataGridCell::text(short_id),
        ])
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
    }
}
