use std::process::Command;

use tempfile::tempdir;
use zit::project::{GixRepositoryLoader, RepositoryLoader};

fn run_git(path: &std::path::Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn stage_unstage_and_commit_refresh_working_tree_status() {
    let directory = tempdir().unwrap();
    gix::init(directory.path()).unwrap();
    run_git(
        directory.path(),
        &["config", "user.name", "File Status Test"],
    );
    run_git(
        directory.path(),
        &["config", "user.email", "file-status@example.com"],
    );
    std::fs::write(directory.path().join("file.txt"), "one\n").unwrap();
    run_git(directory.path(), &["add", "file.txt"]);
    run_git(directory.path(), &["commit", "-m", "initial"]);
    std::fs::write(directory.path().join("file.txt"), "two\n").unwrap();

    let loader = GixRepositoryLoader;
    let metadata = loader.inspect(directory.path(), None);
    assert!(!loader.load_working_tree_files(&metadata).unwrap()[0].staged);
    loader.stage_file(&metadata, "file.txt").unwrap();
    assert!(loader.load_working_tree_files(&metadata).unwrap()[0].staged);
    loader.unstage_file(&metadata, "file.txt").unwrap();
    assert!(!loader.load_working_tree_files(&metadata).unwrap()[0].staged);
    loader.commit(&metadata, "updated file").unwrap_err();
    loader.stage_file(&metadata, "file.txt").unwrap();
    loader.commit(&metadata, "updated file").unwrap();
    assert!(
        loader
            .load_working_tree_files(&metadata)
            .unwrap()
            .is_empty()
    );
}
