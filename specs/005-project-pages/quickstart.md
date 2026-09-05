# Quickstart Validation: Project Pages

## Prerequisites

- Rust toolchain 1.97.1 with `rustfmt` and `clippy`
- A supported desktop environment for manual GPUI validation
- Two local test repositories: one with commits and one valid repository with no commits
- A temporary location where one saved repository path can intentionally be made unavailable

The expected state model is documented in [data-model.md](data-model.md), and observable UI behavior is defined by [contracts/project-page.md](contracts/project-page.md).

## 1. Static and automated validation

From the repository root:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

Expected outcome:

- Formatting and lint checks finish without errors or warnings.
- Store contract tests prove uniqueness, ordering, active fallback, and removal invariants.
- Page contract tests prove first-activation loading, request deduplication, retry, and stale-result rejection.
- Session integration tests prove versioned round-trip, malformed-file recovery, and mixed available/unavailable restoration.
- Repository integration tests prove committed and empty repositories are valid without invoking the Git CLI.

## 2. Multiple-project startup

1. Prepare session state containing both test repositories in a known order and select the second as active.
2. Start the application with that isolated session location.
3. Observe the initial project tabs and active page.

Expected outcome:

- Both tabs appear in persisted order.
- The second tab is active.
- Each project has exactly one tab/page.
- Only the active project's page enters loading; the inactive project remains not loaded.
- Tabs become interactive without waiting for commit history.

## 3. Lazy-load and reuse

1. Wait for the initially active project to reach ready.
2. Select the previously inactive project.
3. Switch repeatedly between the two projects.

Expected outcome:

- Selecting the inactive project immediately shows loading or ready feedback.
- Its history begins only after selection.
- Returning to either ready page reuses its loaded result with no loading flicker or duplicate request.
- The empty repository reaches ready with an empty-history message, not failed.

## 4. Failure isolation and retry

1. Add an unavailable repository path to the isolated session alongside a valid repository.
2. Start the application and activate both tabs in turn.
3. Restore the unavailable path as a valid repository and invoke Retry.

Expected outcome:

- The valid project remains usable throughout.
- The unavailable page clearly identifies its failure and exposes Retry.
- Retry uses the same tab/page and can transition to ready.
- No duplicate project appears.

## 5. Close during loading

1. Use a repository or deterministic test loader whose history request remains pending.
2. Activate its tab, confirm loading, and close the tab before completion.
3. Allow the pending request to finish.

Expected outcome:

- The tab and retained page disappear immediately after accepted close.
- The result does not recreate the project or alter the newly active page.
- The next launch restores only projects that remained open.

## 6. Performance acceptance

1. Prepare an isolated session with 20 distinct valid local repositories.
2. Measure from application launch until all tabs accept selection.
3. Record history-load requests before visiting each tab.

Expected outcome:

- All 20 tabs are interactive within 2 seconds on the validation machine.
- No unvisited inactive project has issued a history request.
- Each activation produces visible feedback within 100 milliseconds.

## Validation record

**Date**: 2026-09-06

- Formatting, strict linting, and the complete workspace test suite passed on Windows.
- Multiple-project restoration, ordering, duplicate normalization, active fallback, stable page identity, unavailable-neighbor isolation, and the empty session were exercised by store contracts.
- Activation-only loading, single-request behavior, ready reuse, failed retry, and retry suppression were exercised with deterministic loaders.
- Real `gix` integration covered repository aliases, an unborn/empty repository, and an authored HEAD commit without invoking the Git CLI.
- Session validation covered missing, malformed, unsupported, duplicate, replacement, temporary-file cleanup, and Windows path round-trip cases.
- The 20-project harness restored all entries in under 2 seconds and observed zero inactive history requests.
- Manual visual inspection remains appropriate when changing layout styling, but no environment-specific functional deviation was found by the automated end-to-end contracts.
