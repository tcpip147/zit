use std::path::PathBuf;

use tempfile::tempdir;
use zit::project::{OpenProjectSession, ProjectId, SessionError, SessionProject};

fn sample() -> OpenProjectSession {
    let id = ProjectId::new("repo-a").unwrap();
    OpenProjectSession {
        version: 1,
        projects: vec![SessionProject {
            identity_key: id.as_str().into(),
            repository_path: PathBuf::from("repo-a"),
            display_name: "A".into(),
        }],
        active_project: Some(id),
    }
}

#[test]
fn session_round_trips_and_replaces_existing_content() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("session.json");
    let session = sample();
    session.save(&path).unwrap();
    assert_eq!(OpenProjectSession::load(&path).unwrap(), session);
    let mut updated = session.clone();
    updated.projects[0].display_name = "Renamed".into();
    updated.save(&path).unwrap();
    assert_eq!(OpenProjectSession::load(&path).unwrap(), updated);
    assert!(!path.with_extension("json.tmp").exists());
}

#[test]
fn missing_is_empty_and_bad_content_isolated() {
    let directory = tempdir().unwrap();
    let missing = directory.path().join("missing.json");
    assert_eq!(
        OpenProjectSession::load(&missing).unwrap(),
        OpenProjectSession::default()
    );
    let malformed = directory.path().join("bad.json");
    std::fs::write(&malformed, "{").unwrap();
    assert!(matches!(
        OpenProjectSession::load(&malformed),
        Err(SessionError::Json(_))
    ));
    let unsupported = directory.path().join("future.json");
    std::fs::write(&unsupported, r#"{"version":99,"projects":[]}"#).unwrap();
    assert!(matches!(
        OpenProjectSession::load(&unsupported),
        Err(SessionError::UnsupportedVersion(99))
    ));
}

#[test]
fn duplicate_entries_round_trip_in_order_for_store_normalization() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("session.json");
    let mut session = sample();
    session.projects.push(session.projects[0].clone());
    session.save(&path).unwrap();
    let loaded = OpenProjectSession::load(&path).unwrap();
    assert_eq!(loaded.projects.len(), 2);
    assert_eq!(loaded.projects[0], loaded.projects[1]);
}

#[cfg(windows)]
#[test]
fn windows_session_paths_round_trip_without_case_loss() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("session.json");
    let mut session = sample();
    session.projects[0].repository_path = PathBuf::from(r"C:\Users\Example\Repo");
    session.save(&path).unwrap();
    assert_eq!(
        OpenProjectSession::load(&path).unwrap().projects[0].repository_path,
        PathBuf::from(r"C:\Users\Example\Repo")
    );
}
