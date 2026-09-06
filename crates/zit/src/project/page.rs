use std::{rc::Rc, sync::Arc};

use gpui::{
    AccessibleAction, AnyElement, Bounds, Context, CursorStyle, Entity, EventEmitter,
    InteractiveElement, MouseButton, Path, PathBuilder, Pixels, Point, Render, Role,
    StatefulInteractiveElement, Task, Window, canvas, deferred, div, point, prelude::*, px, quad,
    rgb, size, svg,
};
use widget::{DataGrid, DataGridCell, DataGridColumn, DataGridRow};

use super::{ProjectId, ProjectLoadState, ProjectMetadata, RepositoryLoader, build_commit_graph};

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum HistoryBranchFilter {
    #[default]
    All,
    Current,
}

#[derive(Clone)]
struct CachedGraphPath {
    path: Path<Pixels>,
    color: gpui::Rgba,
}

type CommitGraphPathCache = Rc<Vec<CachedGraphPath>>;

pub struct ProjectPage {
    metadata: ProjectMetadata,
    state: ProjectLoadState,
    generation: u64,
    loader: Arc<dyn RepositoryLoader>,
    load_task: Option<Task<()>>,
    selected_section: ProjectSection,
    history_grid: Option<Entity<DataGrid>>,
    history_branch_filter: HistoryBranchFilter,
    include_remote_branches: bool,
    branch_filter_open: bool,
    selected_history_commit: Option<super::CommitSummary>,
    selected_commit_files: Option<Result<Vec<String>, String>>,
    selected_commit_file: Option<String>,
    selected_commit_file_diff: Option<Result<String, String>>,
    file_load_generation: u64,
    file_load_task: Option<Task<()>>,
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
            history_branch_filter: HistoryBranchFilter::default(),
            include_remote_branches: true,
            branch_filter_open: false,
            selected_history_commit: None,
            selected_commit_files: None,
            selected_commit_file: None,
            selected_commit_file_diff: None,
            file_load_generation: 0,
            file_load_task: None,
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
                page.rebuild_history_grid(cx);
                page.load_task = None;
                cx.notify();
            });
        }));
    }

    fn rebuild_history_grid(&mut self, cx: &mut Context<Self>) {
        self.selected_history_commit = None;
        self.selected_commit_files = None;
        self.selected_commit_file = None;
        self.selected_commit_file_diff = None;
        self.file_load_generation = self.file_load_generation.wrapping_add(1);
        self.file_load_task = None;
        let ProjectLoadState::Ready(data) = &self.state else {
            self.history_grid = None;
            return;
        };
        let mut commits = data
            .commits
            .iter()
            .filter(|commit| match self.history_branch_filter {
                HistoryBranchFilter::Current => commit.reachable_from_current,
                HistoryBranchFilter::All if self.include_remote_branches => {
                    commit.reachable_from_local || commit.reachable_from_remote
                }
                HistoryBranchFilter::All => commit.reachable_from_local,
            })
            .cloned()
            .collect::<Vec<_>>();
        if !self.include_remote_branches {
            for commit in &mut commits {
                commit
                    .branches
                    .retain(|branch| !commit.remote_branches.contains(branch));
            }
        }
        if commits.is_empty() {
            self.history_grid = None;
            return;
        }
        build_commit_graph(&mut commits);
        let commits = Rc::new(commits);
        let graph_paths = Rc::new(
            commits
                .iter()
                .map(Self::build_commit_graph_paths)
                .collect::<Vec<_>>(),
        );
        let commits_for_selection = commits.clone();
        let page = cx.entity();
        self.history_grid = Some(cx.new(|_| {
            DataGrid::new(Self::history_columns(), commits.len(), move |index| {
                Self::history_row(&commits[index], graph_paths[index].clone())
            })
            .expect("history columns are valid")
            .on_row_mouse_down(move |index, _, _, cx| {
                let commit = commits_for_selection[index].clone();
                let _ = page.update(cx, |page, cx| {
                    page.select_history_commit(commit, cx);
                });
            })
        }));
    }

    fn select_history_commit(&mut self, commit: super::CommitSummary, cx: &mut Context<Self>) {
        self.file_load_generation = self.file_load_generation.wrapping_add(1);
        let generation = self.file_load_generation;
        let commit_id = commit.object_id.clone();
        self.selected_history_commit = Some(commit);
        self.selected_commit_files = None;
        self.selected_commit_file = None;
        self.selected_commit_file_diff = None;
        self.file_load_task = None;
        cx.notify();

        let loader = self.loader.clone();
        let metadata = self.metadata.clone();
        let background = cx
            .background_executor()
            .spawn(async move { loader.load_commit_files(&metadata, &commit_id) });
        self.file_load_task = Some(cx.spawn(async move |this, cx| {
            let result = background.await;
            let _ = this.update(cx, |page, cx| {
                if page.file_load_generation != generation {
                    return;
                }
                page.selected_commit_files = Some(result.map_err(|error| error.to_string()));
                page.file_load_task = None;
                cx.notify();
            });
        }));
    }

    fn set_history_branch_filter(&mut self, filter: HistoryBranchFilter, cx: &mut Context<Self>) {
        self.history_branch_filter = filter;
        self.branch_filter_open = false;
        self.rebuild_history_grid(cx);
        cx.notify();
    }

    fn history_filter_option(
        &self,
        label: &'static str,
        filter: HistoryBranchFilter,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .id(("history-branch-option", filter as usize))
            .role(Role::Button)
            .aria_label(label)
            .w_full()
            .h(px(24.))
            .px_2()
            .flex()
            .items_center()
            .cursor(CursorStyle::PointingHand)
            .bg(if self.history_branch_filter == filter {
                rgb(0xdbeafe)
            } else {
                rgb(0xffffff)
            })
            .hover(|style| style.bg(rgb(0xe2e8f0)))
            .on_click(cx.listener(move |page, _, _, cx| {
                page.set_history_branch_filter(filter, cx);
            }))
            .child(label)
            .into_any_element()
    }

    fn history_toolbar(&self, cx: &mut Context<Self>) -> AnyElement {
        let selected_label = match self.history_branch_filter {
            HistoryBranchFilter::All => "All branches",
            HistoryBranchFilter::Current => "Current branch",
        };
        div()
            .id("history-filter-toolbar")
            .h(px(36.))
            .w_full()
            .px_2()
            .flex_none()
            .flex()
            .items_center()
            .gap_3()
            .border_b_1()
            .border_color(rgb(0xcbd5e1))
            .bg(rgb(0xf8fafc))
            .child(
                div()
                    .relative()
                    .child(
                        div()
                            .id("history-branch-select")
                            .role(Role::Button)
                            .aria_label("Branch filter")
                            .w(px(140.))
                            .h(px(24.))
                            .px_2()
                            .flex()
                            .items_center()
                            .justify_between()
                            .border_1()
                            .border_color(rgb(0xcbd5e1))
                            .rounded(px(3.))
                            .bg(rgb(0xffffff))
                            .cursor(CursorStyle::PointingHand)
                            .on_click(cx.listener(|page, _, _, cx| {
                                page.branch_filter_open = !page.branch_filter_open;
                                cx.notify();
                            }))
                            .child(selected_label)
                            .child("▾"),
                    )
                    .when(self.branch_filter_open, |select| {
                        select.child(
                            deferred(
                                div()
                                    .absolute()
                                    .top(px(26.))
                                    .left_0()
                                    .w(px(140.))
                                    .border_1()
                                    .border_color(rgb(0xcbd5e1))
                                    .rounded(px(3.))
                                    .bg(rgb(0xffffff))
                                    .child(self.history_filter_option(
                                        "All branches",
                                        HistoryBranchFilter::All,
                                        cx,
                                    ))
                                    .child(self.history_filter_option(
                                        "Current branch",
                                        HistoryBranchFilter::Current,
                                        cx,
                                    )),
                            )
                            .with_priority(1),
                        )
                    }),
            )
            .child(
                div()
                    .id("history-remote-branches")
                    .role(Role::CheckBox)
                    .aria_label("Remote branches")
                    .aria_selected(self.include_remote_branches)
                    .flex()
                    .items_center()
                    .gap_1()
                    .cursor(CursorStyle::PointingHand)
                    .on_click(cx.listener(|page, _, _, cx| {
                        page.include_remote_branches = !page.include_remote_branches;
                        page.rebuild_history_grid(cx);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .size(px(14.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .border_1()
                            .border_color(rgb(0x94a3b8))
                            .rounded(px(2.))
                            .bg(rgb(0xffffff))
                            .when(self.include_remote_branches, |box_| box_.child("✓")),
                    )
                    .child("Remote branches"),
            )
            .into_any_element()
    }

    fn commit_detail_field(label: &'static str, value: String) -> AnyElement {
        div()
            .w_full()
            .flex()
            .child(
                div()
                    .w(px(82.))
                    .flex_none()
                    .text_color(rgb(0x64748b))
                    .child(label),
            )
            .child(div().min_w_0().flex_1().child(value))
            .into_any_element()
    }

    fn selected_commit_details(&self) -> AnyElement {
        let Some(commit) = &self.selected_history_commit else {
            return div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(rgb(0x64748b))
                .child("Select a commit to view its details.")
                .into_any_element();
        };
        let parents = if commit.parent_ids.is_empty() {
            String::new()
        } else {
            commit.parent_ids.join("\n")
        };
        let message = if commit.message_body.is_empty() {
            commit.summary.clone()
        } else {
            format!("{}\n\n{}", commit.summary, commit.message_body)
        };
        div()
            .w_full()
            .p_3()
            .overflow_hidden()
            .flex()
            .flex_col()
            .gap_1()
            .child(Self::commit_detail_field("Hash", commit.object_id.clone()))
            .when(!parents.is_empty(), |details| {
                details.child(Self::commit_detail_field("Parent Hash", parents))
            })
            .child(Self::commit_detail_field(
                "Author",
                format!("{} <{}>", commit.author_name, commit.author_email),
            ))
            .child(Self::commit_detail_field(
                "Date",
                Self::format_commit_time(
                    commit.commit_time_seconds,
                    commit.commit_time_offset_minutes,
                ),
            ))
            .child(Self::commit_detail_field(
                "Committer",
                format!("{} <{}>", commit.committer_name, commit.committer_email),
            ))
            .child(div().h(px(8.)).flex_none())
            .child(div().text_color(rgb(0x64748b)).child("Message"))
            .child(
                div()
                    .min_h_0()
                    .overflow_hidden()
                    .whitespace_normal()
                    .child(message),
            )
            .into_any_element()
    }

    fn select_history_file(&mut self, path: String, cx: &mut Context<Self>) {
        let Some(commit) = &self.selected_history_commit else { return };
        self.file_load_generation = self.file_load_generation.wrapping_add(1);
        let generation = self.file_load_generation;
        self.selected_commit_file = Some(path.clone());
        self.selected_commit_file_diff = None;
        let loader = self.loader.clone();
        let metadata = self.metadata.clone();
        let commit_id = commit.object_id.clone();
        self.file_load_task = Some(cx.spawn(async move |this, cx| {
            let result = cx.background_executor().spawn(async move {
                loader.load_commit_file_diff(&metadata, &commit_id, &path)
            }).await;
            let _ = this.update(cx, |page, cx| {
                if page.file_load_generation != generation { return; }
                page.selected_commit_file_diff = Some(result.map_err(|error| error.to_string()));
                page.file_load_task = None;
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn selected_commit_file_list(&self, cx: &mut Context<Self>) -> AnyElement {
        let content = match &self.selected_history_commit {
            None => div()
                .text_color(rgb(0x64748b))
                .child("Select a commit to view its files.")
                .into_any_element(),
            Some(_) => match &self.selected_commit_files {
                None => div()
                    .id("history-files-loading")
                    .role(Role::Status)
                    .child("Loading changed files…")
                    .into_any_element(),
                Some(Ok(files)) if files.is_empty() => div()
                    .text_color(rgb(0x64748b))
                    .child("No changed files.")
                    .into_any_element(),
                Some(Ok(files)) => div()
                    .w_full()
                    .children(files.iter().cloned().map(|path| {
                        let selected = self.selected_commit_file.as_deref() == Some(path.as_str());
                        let selected_path = path.clone();
                        div()
                            .id(format!("history-file-{path}"))
                            .w_full()
                            .h(px(22.))
                            .px_1()
                            .flex()
                            .items_center()
                            .cursor(CursorStyle::PointingHand)
                            .when(selected, |style| style.bg(rgb(0xdbeafe)))
                            .hover(|style| style.bg(rgb(0xe2e8f0)))
                            .on_click(cx.listener(move |page, _, _, cx| {
                                page.select_history_file(selected_path.clone(), cx);
                            }))
                            .child(path)
                    }))
                    .into_any_element(),
                Some(Err(message)) => div()
                    .id("history-files-error")
                    .role(Role::Alert)
                    .text_color(rgb(0xb91c1c))
                    .child(message.clone())
                    .into_any_element(),
            },
        };
        div()
            .size_full()
            .min_h_0()
            .p_2()
            .flex()
            .flex_col()
            .child(
                div()
                    .id("history-files-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(content),
            )
            .into_any_element()
    }

    fn selected_commit_file_diff(&self) -> AnyElement {
        let Some(path) = &self.selected_commit_file else {
            return div().size_full().p_3().text_color(rgb(0x64748b)).child("Select a file to view its changes.").into_any_element();
        };
        let body = match &self.selected_commit_file_diff {
            None => "Loading changes…".to_owned(),
            Some(Ok(diff)) if diff.is_empty() => "No textual changes available.".to_owned(),
            Some(Ok(diff)) => diff.clone(),
            Some(Err(error)) => format!("Could not load changes: {error}"),
        };
        div().size_full().p_3().overflow_hidden().child(div().text_color(rgb(0x334155)).child(path.clone())).child(div().mt_2().whitespace_normal().child(body)).into_any_element()
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
            ProjectLoadState::Ready(_) => div()
                .id("history-split-view")
                .size_full()
                .min_h_0()
                .flex()
                .flex_col()
                .child(
                    div()
                        .id("history-upper-pane")
                        .flex_1()
                        .min_h_0()
                        .overflow_hidden()
                        .flex()
                        .flex_col()
                        .child(self.history_toolbar(cx))
                        .child(match &self.history_grid {
                            Some(grid) => div()
                                .flex_1()
                                .min_h_0()
                                .child(grid.clone())
                                .into_any_element(),
                            None => div()
                                .flex_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child("No commits match the selected filters.")
                                .into_any_element(),
                        }),
                )
                .child(
                    div()
                        .id("history-lower-pane")
                        .flex_1()
                        .min_h_0()
                        .flex()
                        .border_t_1()
                        .border_color(rgb(0xcbd5e1))
                        .bg(rgb(0xffffff))
                        .child(
                            div()
                                .id("history-commit-details")
                                .w_1_2()
                                .h_full()
                                .min_w_0()
                                .overflow_hidden()
                                .flex()
                                .flex_col()
                                .border_r_1()
                                .border_color(rgb(0xcbd5e1))
                                .child(
                                    div()
                                        .id("history-commit-metadata")
                                        .flex_1()
                                        .min_h_0()
                                        .overflow_y_scroll()
                                        .child(self.selected_commit_details()),
                                )
                                .child(
                                    div()
                                        .id("history-commit-files")
                                        .flex_1()
                                        .min_h_0()
                                        .overflow_hidden()
                                        .border_t_1()
                                        .border_color(rgb(0xcbd5e1))
                                        .child(self.selected_commit_file_list(cx)),
                                ),
                        )
                        .child(
                            div()
                                .id("history-lower-right-pane")
                                .w_1_2()
                                .h_full()
                                .min_w_0()
                                .child(self.selected_commit_file_diff()),
                        ),
                )
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
        vec![
            DataGridColumn::new("graph", "Graph").fixed(px(200.)),
            DataGridColumn::new("commit", "Description").fraction(1.),
            DataGridColumn::new("author", "Author").fixed(px(200.)),
            DataGridColumn::new("date", "Date").fixed(px(120.)),
            DataGridColumn::new("hash", "Hash")
                .fixed(px(100.))
                .divider(false),
        ]
    }

    fn history_row(
        commit: &super::CommitSummary,
        graph_paths: CommitGraphPathCache,
    ) -> DataGridRow {
        let graph_label = if commit.branches.is_empty() {
            if commit.is_head {
                "HEAD".to_owned()
            } else {
                format!("Graph lane {}", commit.commit_lane.lane)
            }
        } else {
            format!(
                "{}{}",
                if commit.is_head { "HEAD, " } else { "" },
                commit.branches.join(", ")
            )
        };
        let commit_content = div()
            .min_w_0()
            .flex()
            .items_center()
            .gap_1()
            .when(commit.is_head, |content| {
                content.child(
                    div()
                        .flex_none()
                        .size(px(9.))
                        .rounded_full()
                        .border_2()
                        .border_color(Self::graph_color(commit.commit_lane.color))
                        .bg(rgb(0xffffff)),
                )
            })
            .children(commit.branches.iter().cloned().map(|label| {
                div()
                    .flex_none()
                    .px_1()
                    .rounded(px(3.))
                    .bg(rgb(0xfef3c7))
                    .child(label)
            }))
            .child(
                div()
                    .min_w_0()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(commit.summary.clone()),
            );
        DataGridRow::new([
            DataGridCell::new(Self::commit_graph(commit, graph_paths))
                .accessible_label(graph_label),
            DataGridCell::new(commit_content).accessible_label(commit.summary.clone()),
            DataGridCell::text(format!("{} <{}>", commit.author_name, commit.author_email)),
            DataGridCell::text(Self::format_commit_time(
                commit.commit_time_seconds,
                commit.commit_time_offset_minutes,
            )),
            DataGridCell::text(commit.object_id[..commit.object_id.len().min(8)].to_owned()),
        ])
    }

    fn format_commit_time(seconds: i64, offset_minutes: i32) -> String {
        let local_seconds = seconds.saturating_add(i64::from(offset_minutes) * 60);
        let days = local_seconds.div_euclid(86_400);
        let seconds_of_day = local_seconds.rem_euclid(86_400);
        let hour = seconds_of_day / 3_600;
        let minute = seconds_of_day % 3_600 / 60;

        // Convert days since 1970-01-01 to the proleptic Gregorian calendar.
        let shifted = days + 719_468;
        let era = shifted.div_euclid(146_097);
        let day_of_era = shifted - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let mut year = year_of_era + era * 400;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_prime = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
        let month = month_prime + if month_prime < 10 { 3 } else { -9 };
        year += i64::from(month <= 2);

        format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}")
    }

    fn graph_color(index: usize) -> gpui::Rgba {
        const COLORS: [u32; 8] = [
            0x2563eb, 0x8b5cf6, 0x059669, 0xdc2626, 0xd97706, 0x0891b2, 0xdb2777, 0x4f46e5,
        ];
        rgb(COLORS[index % COLORS.len()])
    }

    fn build_commit_graph_paths(commit: &super::CommitSummary) -> CommitGraphPathCache {
        const LANE_SPACING: f32 = 12.;
        const LANE_INSET: f32 = 8.;
        const ROW_HEIGHT: f32 = 24.;
        let lane_x = |lane: usize| px(LANE_INSET + lane as f32 * LANE_SPACING);
        let center_y = px(ROW_HEIGHT / 2.);
        let mut paths = Vec::with_capacity(commit.input_edges.len() + commit.output_edges.len());

        for edge in &commit.input_edges {
            let mut path = PathBuilder::stroke(px(2.));
            let start = point(lane_x(edge.from_lane), px(0.));
            let end = point(lane_x(edge.to_lane), center_y);
            path.move_to(start);
            if edge.from_lane == edge.to_lane {
                path.line_to(end);
            } else {
                let control_offset = (end.y - start.y) * 0.6;
                path.cubic_bezier_to(
                    end,
                    point(start.x, start.y + control_offset),
                    point(end.x, end.y - control_offset),
                );
            }
            if let Ok(path) = path.build() {
                paths.push(CachedGraphPath {
                    path,
                    color: Self::graph_color(edge.color),
                });
            }
        }
        for edge in &commit.output_edges {
            let mut path = PathBuilder::stroke(px(2.));
            let start = point(lane_x(edge.from_lane), center_y);
            let end = point(lane_x(edge.to_lane), px(ROW_HEIGHT));
            path.move_to(start);
            if edge.from_lane == edge.to_lane {
                path.line_to(end);
            } else {
                let control_offset = (end.y - start.y) * 0.6;
                path.cubic_bezier_to(
                    end,
                    point(start.x, start.y + control_offset),
                    point(end.x, end.y - control_offset),
                );
            }
            if let Ok(path) = path.build() {
                paths.push(CachedGraphPath {
                    path,
                    color: Self::graph_color(edge.color),
                });
            }
        }
        Rc::new(paths)
    }

    fn translate_graph_path(path: &Path<Pixels>, origin: Point<Pixels>) -> Path<Pixels> {
        let mut translated = path.clone();
        translated.bounds.origin = point(
            translated.bounds.origin.x + origin.x,
            translated.bounds.origin.y + origin.y,
        );
        for vertex in &mut translated.vertices {
            vertex.xy_position = point(
                vertex.xy_position.x + origin.x,
                vertex.xy_position.y + origin.y,
            );
        }
        translated
    }

    fn commit_graph(
        commit: &super::CommitSummary,
        graph_paths: CommitGraphPathCache,
    ) -> AnyElement {
        const LANE_SPACING: f32 = 12.;
        const LANE_INSET: f32 = 8.;
        let marker = commit.commit_lane.clone();
        let is_head = commit.is_head;
        canvas(
            |_, _, _| (),
            move |bounds, _, window, _| {
                let lane_x =
                    |lane: usize| bounds.origin.x + px(LANE_INSET + lane as f32 * LANE_SPACING);
                let center_y = bounds.origin.y + bounds.size.height / 2.;
                for cached in graph_paths.iter() {
                    window.paint_path(
                        Self::translate_graph_path(&cached.path, bounds.origin),
                        cached.color,
                    );
                }
                let center = point(lane_x(marker.lane), center_y);
                window.paint_quad(quad(
                    Bounds::new(center - point(px(4.5), px(4.5)), size(px(9.), px(9.))),
                    px(4.5),
                    if is_head {
                        rgb(0xffffff)
                    } else {
                        Self::graph_color(marker.color)
                    },
                    if is_head { px(2.) } else { px(1.) },
                    if is_head {
                        Self::graph_color(marker.color)
                    } else {
                        rgb(0xffffff)
                    },
                    /*
                    if is_head {λισ px(2.) } else { px(1.) },
                    if is_head {
                        Self::graph_color(marker.color)
                    } else {
                       .website rgb(0xffffff)
                    },
                    */
                    Default::default(),
                ));
            },
        )
        .size_full()
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

#[cfg(test)]
mod tests {
    use super::ProjectPage;

    #[test]
    fn formats_commit_time_with_repository_offset() {
        assert_eq!(ProjectPage::format_commit_time(0, 0), "1970-01-01 00:00");
        assert_eq!(
            ProjectPage::format_commit_time(946_684_800, 540),
            "2000-01-01 09:00"
        );
        assert_eq!(ProjectPage::format_commit_time(0, -60), "1969-12-31 23:00");
    }
}
