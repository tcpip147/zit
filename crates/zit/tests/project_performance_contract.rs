use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Instant,
};

use gpui::{AppContext, TestAppContext};
use tempfile::tempdir;
use zit::project::{
    OpenProjectSession, ProjectAvailability, ProjectData, ProjectId, ProjectMetadata, ProjectStore,
    RepositoryError, RepositoryLoader, SessionProject,
};

struct CountingLoader(Arc<AtomicUsize>);
impl RepositoryLoader for CountingLoader {
    fn inspect(&self, path: &Path, _: Option<&str>) -> ProjectMetadata {
        let key = path.to_string_lossy().into_owned();
        ProjectMetadata {
            id: ProjectId::new(key.clone()).unwrap(),
            repository_path: path.into(),
            display_name: key,
            availability: ProjectAvailability::Available,
        }
    }
    fn load_history(&self, metadata: &ProjectMetadata) -> Result<ProjectData, RepositoryError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(ProjectData {
            project_id: metadata.id.clone(),
            head: None,
            commits: vec![],
        })
    }
}

#[gpui::test]
fn restores_twenty_without_loading_inactive_history(cx: &mut TestAppContext) {
    let directory = tempdir().unwrap();
    let path = directory.path().join("session.json");
    let projects = (0..20)
        .map(|index| SessionProject {
            identity_key: format!("repo-{index}"),
            repository_path: format!("repo-{index}").into(),
            display_name: format!("Repo {index}"),
        })
        .collect();
    OpenProjectSession {
        version: 1,
        projects,
        active_project: None,
    }
    .save(&path)
    .unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let started = Instant::now();
    let store =
        cx.new(|_| ProjectStore::with_loader(path, Arc::new(CountingLoader(calls.clone()))));
    store.update(cx, |store, cx| store.restore(cx));
    assert_eq!(store.read_with(cx, |store, _| store.projects().len()), 20);
    assert!(started.elapsed().as_secs_f32() < 2.0);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}
