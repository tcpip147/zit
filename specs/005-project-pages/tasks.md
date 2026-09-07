---
description: "Completed implementation record for project pages"
---

# Tasks: Project Pages

**Status**: As-built task record; every listed task is present in the current source.

## Phase 1: Setup and Foundation

- [X] T001 Add serialization and temporary-repository dependencies in `crates/zit/Cargo.toml`
- [X] T002 Create and export project modules in `crates/zit/src/project/mod.rs` and `crates/zit/src/lib.rs`
- [X] T003 [P] Define project identity, metadata, availability, commit summary, project data, and load-state types in `crates/zit/src/project/model.rs`
- [X] T004 [P] Define the injectable repository loader and user-safe error boundary in `crates/zit/src/project/repository.rs`
- [X] T005 Install one global `ProjectStore` during startup in `crates/zit/src/main.rs`

## Phase 2: User Story 1 - Restore Open Project Tabs

- [X] T006 [US1] Implement versioned session loading and atomic replacement persistence in `crates/zit/src/project/session.rs`
- [X] T007 [US1] Implement `gix` discovery, canonical identity, display names, and unavailable metadata in `crates/zit/src/project/repository.rs`
- [X] T008 [US1] Implement ordered restore, identity deduplication, active fallback, retained pages, metadata updates, close removal, and persistence in `crates/zit/src/project/store.rs`
- [X] T009 [US1] Build tabs from global metadata and reconcile selection, close, and metadata events in `crates/zit/src/shell.rs`
- [X] T010 [US1] Verify store, session, page identity, and tab metadata behavior in `crates/zit/tests/project_store_contract.rs`, `project_session_integration.rs`, and `crates/widget/tests/tab_bar_contract.rs`

## Phase 3: User Story 2 - Activate and Load Project Data

- [X] T011 [US2] Walk HEAD history through `gix` and produce commit summaries, including empty and detached repositories, in `crates/zit/src/project/repository.rs`
- [X] T012 [US2] Implement first-activation loading, one in-flight generation, ready reuse, and project-result validation in `crates/zit/src/project/page.rs`
- [X] T013 [US2] Route active-project changes only to the matching retained page in `crates/zit/src/shell.rs` and `crates/zit/src/project/store.rs`
- [X] T014 [US2] Verify activation-only loading, ready reuse, canonical aliases, and repository history in `crates/zit/tests/project_page_contract.rs` and `project_repository_integration.rs`

## Phase 4: User Story 3 - Recover From Repository Failures

- [X] T015 [US3] Preserve unavailable projects and convert discovery and history failures into project-scoped diagnostics in `crates/zit/src/project/repository.rs`
- [X] T016 [US3] Implement retry, repeated-load suppression, generation invalidation, and close cleanup in `crates/zit/src/project/page.rs` and `crates/zit/src/project/store.rs`
- [X] T017 [US3] Render accessible loading, empty, failed, and retry states in `crates/zit/src/project/page.rs`
- [X] T018 [US3] Verify unavailable-neighbor isolation, retry, and stale closed-page results in `crates/zit/tests/project_store_contract.rs` and `project_page_contract.rs`

## Phase 5: User Story 4 - Navigate the Project Workspace

- [X] T019 [US4] Implement independent File Status, History, and Search selection per retained page in `crates/zit/src/project/page.rs`
- [X] T020 [US4] Render the project toolbar and project-scoped action events using SVG assets in `crates/zit/src/project/page.rs` and `crates/zit/assets/project-toolbar/`
- [X] T021 [US4] Render virtualized commit rows with graph, summary, author, date, short hash, wheel feedback, and draggable scrollbar in `crates/zit/src/project/page.rs`
- [X] T022 [US4] Verify independent section selection across project pages in `crates/zit/tests/project_page_contract.rs`

## Phase 6: Validation and Development Bootstrap

- [X] T023 [P] Add 20-project restoration and inactive-load validation in `crates/zit/tests/project_performance_contract.rs`
- [X] T024 [P] Add empty-store development repository seeding helpers and their contract test in `crates/zit/src/project/store.rs` and `crates/zit/tests/project_store_contract.rs`
- [X] T025 Connect current development bootstrap and optional sample repository replacement in `crates/zit/src/main.rs`
- [X] T026 Run formatting, lint, workspace tests, and constitutional dependency checks and record readiness in `specs/005-project-pages/checklists/requirements.md`
- [ ] T100 Add History lower-pane, changed-file loading, DiffEditor reset, and per-project isolation acceptance evidence in `specs/005-project-pages/checklists/requirements.md`
- [X] T101 [US4] Retain the File Status navigation slot and per-page ownership boundary in `crates/zit/src/project/page.rs`

## Dependencies and Delivery

Foundation precedes all stories. US1 establishes retained pages, US2 adds lazy data, US3 adds recovery, and US4 adds the complete workspace presentation. Repository, session, and page tests can be developed independently after shared types exist. The MVP is US1 plus US2; the synchronized implementation includes all stories and the explicitly documented development bootstrap.
