use tempfile::tempdir;
use zit::project::{GixRepositoryLoader, ProjectAvailability, RepositoryLoader};

#[test]
fn discovers_repository_and_empty_history_is_ready() {
    let directory = tempdir().unwrap();
    gix::init(directory.path()).unwrap();
    let loader = GixRepositoryLoader;
    let metadata = loader.inspect(directory.path(), None);
    assert!(matches!(
        metadata.availability,
        ProjectAvailability::Available
    ));
    let data = loader.load_history(&metadata).unwrap();
    assert!(data.head.is_none());
    assert!(data.commits.is_empty());
}

#[test]
fn aliases_inside_a_worktree_share_identity() {
    let directory = tempdir().unwrap();
    gix::init(directory.path()).unwrap();
    let nested = directory.path().join("nested");
    std::fs::create_dir(&nested).unwrap();
    let loader = GixRepositoryLoader;
    assert_eq!(
        loader.inspect(directory.path(), None).id,
        loader.inspect(&nested, None).id
    );
}

#[test]
fn reads_head_commit_summary_without_git_cli() {
    let directory = tempdir().unwrap();
    let repository = gix::init(directory.path()).unwrap();
    let tree = repository.write_object(gix::objs::Tree::empty()).unwrap();
    let signature = gix::actor::SignatureRef {
        name: gix::bstr::BStr::new(b"Zit Test"),
        email: gix::bstr::BStr::new(b"test@example.com"),
        time: "1 +0000",
    };
    let commit_id = repository
        .commit_as(
            signature,
            signature,
            "HEAD",
            "initial commit\n\nbody",
            tree,
            std::iter::empty::<gix::ObjectId>(),
        )
        .unwrap();
    let loader = GixRepositoryLoader;
    let metadata = loader.inspect(directory.path(), None);
    let data = loader.load_history(&metadata).unwrap();
    assert!(data.head.is_some());
    assert_eq!(data.commits.len(), 1);
    assert_eq!(data.commits[0].summary, "initial commit");
    assert_eq!(data.commits[0].author_name, "Zit Test");

    std::fs::write(repository.git_dir().join("HEAD"), format!("{commit_id}\n")).unwrap();
    let detached = loader
        .load_history(&loader.inspect(directory.path(), None))
        .unwrap();
    assert_eq!(
        detached.head.as_deref(),
        Some(commit_id.to_string().as_str())
    );
    assert_eq!(detached.commits.len(), 1);
}
