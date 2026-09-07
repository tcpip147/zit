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
    assert_eq!(data.commits[0].message_body, "body");
    assert_eq!(data.commits[0].author_name, "Zit Test");
    assert_eq!(data.commits[0].commit_time_seconds, 1);
    assert_eq!(data.commits[0].commit_time_offset_minutes, 0);
    assert_eq!(data.commits[0].object_id, commit_id.to_string());
    assert!(data.commits[0].is_head);
    assert!(!data.commits[0].branches.is_empty());

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

#[test]
fn reads_unique_newest_first_history_from_local_and_remote_branches() {
    let directory = tempdir().unwrap();
    let repository = gix::init(directory.path()).unwrap();
    let tree = repository.write_object(gix::objs::Tree::empty()).unwrap();
    let signature = |time| gix::actor::SignatureRef {
        name: gix::bstr::BStr::new(b"Graph Test"),
        email: gix::bstr::BStr::new(b"graph@example.com"),
        time,
    };
    let base = repository
        .commit_as(
            signature("1 +0000"),
            signature("1 +0000"),
            "HEAD",
            "base",
            tree,
            std::iter::empty::<gix::ObjectId>(),
        )
        .unwrap();
    let feature = repository
        .commit_as(
            signature("2 +0000"),
            signature("2 +0000"),
            "refs/heads/feature",
            "feature",
            tree,
            [base],
        )
        .unwrap();
    let remote = repository
        .commit_as(
            signature("3 +0000"),
            signature("3 +0000"),
            "refs/remotes/origin/topic",
            "remote",
            tree,
            [base],
        )
        .unwrap();

    let loader = GixRepositoryLoader;
    let data = loader
        .load_history(&loader.inspect(directory.path(), None))
        .unwrap();
    let ids = data
        .commits
        .iter()
        .map(|commit| commit.object_id.as_str())
        .collect::<Vec<_>>();
    let expected = [remote.to_string(), feature.to_string(), base.to_string()];
    assert_eq!(ids, expected.iter().map(String::as_str).collect::<Vec<_>>());
    assert_eq!(
        ids.iter().collect::<std::collections::HashSet<_>>().len(),
        3
    );
    assert!(
        data.commits
            .iter()
            .any(|commit| commit.branches.iter().any(|name| name == "feature"))
    );
    assert!(
        data.commits
            .iter()
            .any(|commit| commit.branches.iter().any(|name| name.contains("topic")))
    );
    let remote_commit = data
        .commits
        .iter()
        .find(|commit| commit.object_id == remote.to_string())
        .unwrap();
    assert!(remote_commit.reachable_from_remote);
    assert!(!remote_commit.reachable_from_local);
    assert!(!remote_commit.reachable_from_current);
    assert!(
        remote_commit
            .remote_branches
            .iter()
            .any(|name| name.contains("topic"))
    );
    let feature_commit = data
        .commits
        .iter()
        .find(|commit| commit.object_id == feature.to_string())
        .unwrap();
    assert!(feature_commit.reachable_from_local);
    assert!(!feature_commit.reachable_from_remote);
    assert!(!feature_commit.reachable_from_current);
    assert!(feature_commit.remote_branches.is_empty());
    let base_commit = data
        .commits
        .iter()
        .find(|commit| commit.object_id == base.to_string())
        .unwrap();
    assert!(base_commit.reachable_from_current);
    assert!(base_commit.reachable_from_local);
    assert!(base_commit.reachable_from_remote);
    assert_eq!(base_commit.committer_name, "Graph Test");
    assert_eq!(base_commit.committer_email, "graph@example.com");
    assert!(
        loader
            .load_commit_files(&loader.inspect(directory.path(), None), &base.to_string(),)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn limits_repository_history_to_one_hundred_tracks() {
    let directory = tempdir().unwrap();
    let repository = gix::init(directory.path()).unwrap();
    let tree = repository.write_object(gix::objs::Tree::empty()).unwrap();
    let mut parent = None;
    for index in 0..105 {
        let time = format!("{} +0000", index + 1);
        let signature = gix::actor::SignatureRef {
            name: gix::bstr::BStr::new(b"Limit Test"),
            email: gix::bstr::BStr::new(b"limit@example.com"),
            time: &time,
        };
        parent = Some(
            repository
                .commit_as(
                    signature,
                    signature,
                    "HEAD",
                    format!("commit-{index}"),
                    tree,
                    parent,
                )
                .unwrap(),
        );
    }

    let loader = GixRepositoryLoader;
    let data = loader
        .load_history(&loader.inspect(directory.path(), None))
        .unwrap();
    assert_eq!(data.commits.len(), 100);
    assert_eq!(data.commits[0].summary, "commit-104");
}

#[test]
fn loads_the_next_history_page_from_an_offset() {
    let directory = tempdir().unwrap();
    let repository = gix::init(directory.path()).unwrap();
    let tree = repository.write_object(gix::objs::Tree::empty()).unwrap();
    let mut parent = None;
    for index in 0..105 {
        let time = format!("{} +0000", index + 1);
        let signature = gix::actor::SignatureRef {
            name: gix::bstr::BStr::new(b"Page Test"),
            email: gix::bstr::BStr::new(b"page@example.com"),
            time: &time,
        };
        parent = Some(repository.commit_as(
            signature,
            signature,
            "HEAD",
            format!("commit-{index}"),
            tree,
            parent,
        ).unwrap());
    }
    let loader = GixRepositoryLoader;
    let page = loader
        .load_history_page(&loader.inspect(directory.path(), None), 100)
        .unwrap();
    assert_eq!(page.commits.len(), 5);
    assert_eq!(page.commits[0].summary, "commit-4");
    assert_eq!(page.commits[4].summary, "commit-0");
}
