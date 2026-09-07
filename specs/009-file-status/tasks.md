---
description: "Dependency-ordered tasks for File Status"
---

# Tasks: File Status

## Phase 1: Foundation

- [X] T001 Define repository status entry and File Status state types in `crates/zit/src/project/model.rs`
- [X] T002 Add injectable status/stage/unstage/commit operations to `crates/zit/src/project/repository.rs`
- [X] T003 Add File Status contract fixture scaffolding in `crates/zit/tests/file_status_contract.rs`

## Phase 2: User Story 1 - Review Staged and Unstaged Changes

- [X] T004 [US1] Load and normalize staged/unstaged paths with user-safe repository errors in `crates/zit/src/project/repository.rs`
- [X] T005 [US1] Render the horizontal workflow/DiffEditor split and left-side staged/unstaged panels with empty states in `crates/zit/src/project/page.rs`
- [X] T005a [US2] Render the full-width commit panel below both columns with author metadata, multi-line message editor, options, and right-aligned Commit action in `crates/zit/src/project/page.rs`
- [X] T006 [US1] Add status and file-selection integration coverage in `crates/zit/tests/file_status_contract.rs`

## Phase 3: User Story 2 - Prepare a Commit

- [X] T007 [US2] Connect commit-message TextEditor state and commit options to the retained project page in `crates/zit/src/project/page.rs` and `crates/widget/src/text_editor.rs`
- [X] T008 [US2] Implement non-empty message validation, asynchronous Commit action, and recoverable errors in `crates/zit/src/project/page.rs` and `crates/zit/src/project/repository.rs`
- [X] T009 [US2] Add commit footer and validation interaction coverage in `crates/zit/tests/file_status_contract.rs`

## Phase 4: User Story 3 - Manage Working-Tree State

- [X] T010 [US3] Implement stage and unstage operations and refresh status after mutation in `crates/zit/src/project/repository.rs`
- [X] T011 [US3] Add stage/unstage actions to file rows and preserve selected-file state in `crates/zit/src/project/page.rs`
- [X] T012 [US3] Add stage, unstage, commit, refresh, and project-isolation integration coverage in `crates/zit/tests/file_status_integration.rs`

## Phase 5: Polish

- [X] T013 [P] Add runnable File Status scenarios to `specs/009-file-status/quickstart.md`
- [ ] T014 Run formatting and package checks in the workspace; `cargo check -p zit` passes, while workspace format/test cleanup remains.
- [X] T015 Add 8px draggable resize hit areas for horizontal and vertical File Status splits in `crates/zit/src/project/page.rs`
- [X] T016 Use measured File Status container bounds and pointer-start deltas for exact resize conversion in `crates/zit/src/project/page.rs`

## Dependencies

Foundation → US1 → US2 → US3 → Polish.
