use tempfile::tempdir;
use zit::project::{GixRepositoryLoader, RepositoryLoader};

#[test]
fn reports_unstaged_and_staged_files_separately() {
    let directory = tempdir().unwrap();
    gix::init(directory.path()).unwrap();
    std::fs::write(directory.path().join("file.txt"), "one\n").unwrap();
    let loader = GixRepositoryLoader;
    let metadata = loader.inspect(directory.path(), None);
    let files = loader.load_working_tree_files(&metadata).unwrap();
    assert_eq!(files.len(), 1);
    assert!(!files[0].staged);
    loader.stage_file(&metadata, "file.txt").unwrap();
    let files = loader.load_working_tree_files(&metadata).unwrap();
    assert_eq!(files[0].path, "file.txt");
    assert!(files[0].staged);
    assert!(loader.load_working_tree_diff(&metadata, "file.txt", true).is_ok());
}

#[test]
fn empty_commit_message_is_not_sent_to_repository() {
    assert!("  \n".trim().is_empty());
}
