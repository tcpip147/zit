use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use gpui::{AppContext, TestAppContext};
use zit::project::{
    ProjectAvailability, ProjectData, ProjectId, ProjectLoadState, ProjectMetadata, ProjectPage,
    ProjectSection, RepositoryError, RepositoryLoader,
};

struct CountingLoader {
    calls: Arc<AtomicUsize>,
    fail: bool,
}

impl RepositoryLoader for CountingLoader {
    fn inspect(&self, path: &Path, fallback: Option<&str>) -> ProjectMetadata {
        let value = path.to_string_lossy().into_owned();
        ProjectMetadata {
            id: ProjectId::new(value).unwrap(),
            repository_path: path.into(),
            display_name: fallback.unwrap_or("project").into(),
            availability: ProjectAvailability::Available,
        }
    }
    fn load_history(&self, metadata: &ProjectMetadata) -> Result<ProjectData, RepositoryError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            Err(RepositoryError::new("history unavailable"))
        } else {
            Ok(ProjectData {
                project_id: metadata.id.clone(),
                head: None,
                commits: Vec::new(),
            })
        }
    }
}

fn metadata(name: &str) -> ProjectMetadata {
    ProjectMetadata {
        id: ProjectId::new(name).unwrap(),
        repository_path: name.into(),
        display_name: name.into(),
        availability: ProjectAvailability::Available,
    }
}

#[gpui::test]
fn loads_only_after_activation_and_reuses_ready_data(cx: &mut TestAppContext) {
    let calls = Arc::new(AtomicUsize::new(0));
    let page = cx.new(|_| {
        ProjectPage::new(
            metadata("a"),
            Arc::new(CountingLoader {
                calls: calls.clone(),
                fail: false,
            }),
        )
    });
    assert!(matches!(
        page.read_with(cx, |page, _| page.load_state().clone()),
        ProjectLoadState::NotLoaded
    ));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    page.update(cx, |page, cx| {
        page.activate(cx);
        page.activate(cx);
    });
    cx.run_until_parked();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(matches!(
        page.read_with(cx, |page, _| page.load_state().clone()),
        ProjectLoadState::Ready(_)
    ));
    page.update(cx, |page, cx| page.activate(cx));
    cx.run_until_parked();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[gpui::test]
fn selecting_history_starts_lazy_load(cx: &mut TestAppContext) {
    let calls = Arc::new(AtomicUsize::new(0));
    let page = cx.new(|_| {
        ProjectPage::new(
            metadata("history-selection"),
            Arc::new(CountingLoader {
                calls: calls.clone(),
                fail: false,
            }),
        )
    });

    page.update(cx, |page, cx| {
        page.select_section(ProjectSection::History, cx)
    });
    cx.run_until_parked();

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(matches!(
        page.read_with(cx, |page, _| page.load_state().clone()),
        ProjectLoadState::Ready(_)
    ));
}

#[gpui::test]
fn failed_load_can_retry_without_parallel_duplicates(cx: &mut TestAppContext) {
    let calls = Arc::new(AtomicUsize::new(0));
    let page = cx.new(|_| {
        ProjectPage::new(
            metadata("a"),
            Arc::new(CountingLoader {
                calls: calls.clone(),
                fail: true,
            }),
        )
    });
    page.update(cx, |page, cx| page.activate(cx));
    cx.run_until_parked();
    assert!(matches!(
        page.read_with(cx, |page, _| page.load_state().clone()),
        ProjectLoadState::Failed { .. }
    ));
    page.update(cx, |page, cx| {
        page.retry(cx);
        page.retry(cx);
    });
    cx.run_until_parked();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[gpui::test]
fn close_invalidation_rejects_pending_generation(cx: &mut TestAppContext) {
    let calls = Arc::new(AtomicUsize::new(0));
    let page = cx.new(|_| {
        ProjectPage::new(
            metadata("a"),
            Arc::new(CountingLoader {
                calls: calls.clone(),
                fail: false,
            }),
        )
    });
    page.update(cx, |page, cx| {
        page.activate(cx);
        assert!(matches!(
            page.load_state(),
            ProjectLoadState::Loading { .. }
        ));
        page.cancel_pending_load(cx);
    });
    cx.run_until_parked();
    assert!(matches!(
        page.read_with(cx, |page, _| page.load_state().clone()),
        ProjectLoadState::NotLoaded
    ));
    page.update(cx, |page, cx| page.activate(cx));
    cx.run_until_parked();
    assert!(matches!(
        page.read_with(cx, |page, _| page.load_state().clone()),
        ProjectLoadState::Ready(_)
    ));
}

#[gpui::test]
fn project_pages_keep_independent_sidebar_selection(cx: &mut TestAppContext) {
    let calls = Arc::new(AtomicUsize::new(0));
    let first = cx.new(|_| {
        ProjectPage::new(
            metadata("first"),
            Arc::new(CountingLoader {
                calls: calls.clone(),
                fail: false,
            }),
        )
    });
    let second = cx.new(|_| {
        ProjectPage::new(
            metadata("second"),
            Arc::new(CountingLoader { calls, fail: false }),
        )
    });
    assert_eq!(
        first.read_with(cx, |page, _| page.selected_section()),
        ProjectSection::FileStatus
    );
    first.update(cx, |page, cx| {
        page.select_section(ProjectSection::History, cx)
    });
    assert_eq!(
        first.read_with(cx, |page, _| page.selected_section()),
        ProjectSection::History
    );
    assert_eq!(
        second.read_with(cx, |page, _| page.selected_section()),
        ProjectSection::FileStatus
    );
}
