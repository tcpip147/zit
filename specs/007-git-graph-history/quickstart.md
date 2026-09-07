# Quickstart: Validate Git Graph History

## Prerequisites

- Repository Rust toolchain and resolved workspace dependencies.
- Temporary repositories may be created by tests.
- A graphical desktop session only for interactive inspection.

## Automated Validation

```powershell
cargo test -p zit --test project_graph_contract
cargo test -p zit --test project_repository_integration
cargo test -p zit --test project_page_contract
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: exact topology; de-duplicated multi-root history; at most 100 tracks per page; bottom-triggered pages append older unique commits; repeated bottom events make one request; exhaustion stops requests; failed pages are retryable; existing states remain unchanged.

## Structural Validation

```powershell
rg -n "gpui|DataGrid" crates/zit/src/project/model.rs crates/zit/src/project/repository.rs
rg -n "CommitGraph|CommitSummary|Repository|gix" crates/widget/src/data-grid.rs
rg -n "Command::new|git.exe|process::Command" crates/zit/src/project
```

Expected: no GPUI in domain/repository models, no Git graph types in DataGrid, and no Git CLI invocation. Legitimate gix imports in `repository.rs` are reviewed separately.

## Implementation Validation Record

Validated on Windows on 2026-09-06:

- Pure linear, merge, multi-parent, deterministic, and 10,000-descriptor graph contracts pass.
- Temporary-repository tests confirm local and remote branch roots, newest-first ordering, de-duplication, HEAD, branch labels, and full message body.
- Existing ProjectPage loading, empty, retry, and isolation contracts pass.
- Lane-changing inputs and outputs use smooth cubic curves; same-lane continuations remain straight.
- Interactive rendering remains available through `cargo run -p zit`; the automated environment did not open a desktop window.
- `cargo fmt --all -- --check`, strict workspace Clippy, and `cargo test --workspace` pass.
- A real 105-commit temporary repository returns exactly the newest 100 tracks.

## Interactive Validation

```powershell
cargo run -p zit
```

1. Open a repository with branches and merges and select History.
2. Verify continuous lanes, divergence, convergence, HEAD, and all branch labels.
3. Resize Graph and the window; drawing stays clipped and row-aligned.
4. Scroll all returned tracks and switch projects; state remains independent.
5. Verify existing empty and unavailable-repository presentations.
6. Verify Author uses `name <email>`, Date uses the commit's timezone in `YYYY-MM-DD HH:MI`, and Hash shows the first eight object-ID characters.
7. Scroll to the grid bottom repeatedly and verify automatic append, no duplicates, preserved position, retry on failure, and stable exhaustion.

## Performance Validation

Load an automated fixture or local repository with at least 10,000 reachable commits. Each page returns no more than 100 tracks within two seconds, and repeated bottom events during one in-flight load create only one request.
