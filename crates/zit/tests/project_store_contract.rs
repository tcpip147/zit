use std::{path::Path, sync::Arc};

use gpui::{AppContext, TestAppContext};
use tempfile::tempdir;
use zit::project::{
    OpenProjectSession, ProjectAvailability, ProjectData, ProjectId, ProjectMetadata, ProjectStore,
    RepositoryError, RepositoryLoader, SessionProject,
};

struct MetadataLoader;

impl RepositoryLoader for MetadataLoader {
    fn inspect(&self, path: &Path, fallback: Option<&str>) -> ProjectMetadata {
        let key = path.to_string_lossy().replace('\\', "/");
        ProjectMetadata {
            id: ProjectId::new(key).unwrap(),
            repository_path: path.into(),
            display_name: fallback.unwrap_or("project").into(),
            availability: ProjectAvailability::Available,
        }
    }

    fn load_history(&self, metadata: &ProjectMetadata) -> Result<ProjectData, RepositoryError> {
        Ok(ProjectData {
            project_id: metadata.id.clone(),
            head: None,
            commits: Vec::new(),
        })
    }
}

#[gpui::test]
fn restore_is_ordered_unique_and_pages_are_stable(cx: &mut TestAppContext) {
    let directory = tempdir().unwrap();
    let path = directory.path().join("session.json");
    let a = SessionProject {
        identity_key: "a".into(),
        repository_path: "a".into(),
        display_name: "A".into(),
    };
    let b = SessionProject {
        identity_key: "b".into(),
        repository_path: "b".into(),
        display_name: "B".into(),
    };
    OpenProjectSession {
        version: 1,
        projects: vec![a.clone(), a, b],
        active_project: Some(ProjectId::new("b").unwrap()),
    }
    .save(&path)
    .unwrap();
    let store = cx.new(|_| ProjectStore::with_loader(path, Arc::new(MetadataLoader)));
    store.update(cx, |store, cx| store.restore(cx));
    let (ids, active, page) = store.read_with(cx, |store, _| {
        let ids = store
            .projects()
            .iter()
            .map(|project| project.id.as_str().to_owned())
            .collect::<Vec<_>>();
        let active = store.active_project().cloned();
        let page = store
            .page(&ProjectId::new("a").unwrap())
            .unwrap()
            .entity_id();
        (ids, active, page)
    });
    assert_eq!(ids, ["a", "b"]);
    assert_eq!(active, Some(ProjectId::new("b").unwrap()));
    assert_eq!(
        store.read_with(cx, |store, _| store
            .page(&ProjectId::new("a").unwrap())
            .unwrap()
            .entity_id()),
        page
    );
    store.update(cx, |store, cx| {
        assert!(store.remove(&ProjectId::new("b").unwrap(), cx));
    });
    assert_eq!(
        store.read_with(cx, |store, _| store.active_project().cloned()),
        Some(ProjectId::new("a").unwrap())
    );
}

#[gpui::test]
fn active_falls_back_and_empty_session_stays_empty(cx: &mut TestAppContext) {
    let directory = tempdir().unwrap();
    let path = directory.path().join("session.json");
    let store = cx.new(|_| ProjectStore::with_loader(path.clone(), Arc::new(MetadataLoader)));
    store.update(cx, |store, cx| store.restore(cx));
    assert!(store.read_with(cx, |store, _| store.projects().is_empty()));
    OpenProjectSession {
        version: 1,
        projects: vec![SessionProject {
            identity_key: "a".into(),
            repository_path: "a".into(),
            display_name: "A".into(),
        }],
        active_project: Some(ProjectId::new("missing").unwrap()),
    }
    .save(&path)
    .unwrap();
    store.update(cx, |store, cx| store.restore(cx));
    assert_eq!(
        store.read_with(cx, |store, _| store.active_project().cloned()),
        Some(ProjectId::new("a").unwrap())
    );
}

#[gpui::test]
fn unavailable_project_does_not_block_available_neighbor(cx: &mut TestAppContext) {
    let directory = tempdir().unwrap();
    let repository = directory.path().join("valid");
    gix::init(&repository).unwrap();
    let loader = zit::project::GixRepositoryLoader;
    let available = loader.inspect(&repository, None);
    let missing = directory.path().join("missing");
    let path = directory.path().join("session.json");
    OpenProjectSession {
        version: 1,
        projects: vec![
            SessionProject {
                identity_key: available.id.as_str().into(),
                repository_path: repository,
                display_name: "Valid".into(),
            },
            SessionProject {
                identity_key: "missing-id".into(),
                repository_path: missing.clone(),
                display_name: "Missing".into(),
            },
        ],
        active_project: None,
    }
    .save(&path)
    .unwrap();
    let store = cx.new(|_| ProjectStore::new(path));
    store.update(cx, |store, cx| store.restore(cx));
    let availability = store.read_with(cx, |store, _| {
        store
            .projects()
            .iter()
            .map(|project| project.availability.clone())
            .collect::<Vec<_>>()
    });
    assert_eq!(availability.len(), 2);
    assert!(matches!(availability[0], ProjectAvailability::Available));
    assert!(matches!(
        availability[1],
        ProjectAvailability::Unavailable { .. }
    ));
    assert_eq!(
        store.read_with(cx, |store, _| store.active_project().cloned()),
        Some(available.id)
    );
    gix::init(&missing).unwrap();
    store.update(cx, |store, cx| {
        assert!(store.refresh_metadata(&ProjectId::new("missing-id").unwrap(), cx));
    });
    assert!(
        store.read_with(cx, |store, _| store.projects().iter().any(|project| {
            project.id == ProjectId::new("missing-id").unwrap()
                && matches!(project.availability, ProjectAvailability::Available)
        }))
    );
}

#[gpui::test]
fn seeds_three_distinct_test_repositories_only_for_empty_store(cx: &mut TestAppContext) {
    let directory = tempdir().unwrap();
    let path = directory.path().join("session.json");
    let store = cx.new(|_| ProjectStore::new(path.clone()));
    store.update(cx, |store, cx| {
        store.restore(cx);
        store.seed_test_projects(3, cx);
        store.seed_test_projects(3, cx);
    });
    assert_eq!(store.read_with(cx, |store, _| store.projects().len()), 3);
    assert_eq!(
        store.read_with(cx, |store, _| store
            .projects()
            .iter()
            .map(|project| project.display_name.clone())
            .collect::<Vec<_>>()),
        ["Test Project 1", "Test Project 2", "Test Project 3"]
    );
    assert!(path.exists());
    for number in 1..=3 {
        assert!(
            directory
                .path()
                .join("test-projects")
                .join(format!("project-{number}"))
                .join(".git")
                .is_dir()
        );
    }
}
