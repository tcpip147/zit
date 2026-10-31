# Tasks: New Repository

## Phase 1: Setup

- [x] T001 Confirm existing menu, shell, store, and repository boundaries in `crates/zit/src`

## Phase 2: User Story 1 - Create a Repository

- [x] T002 [US1] Enable `NewRepository` in `crates/zit/src/platform.rs`
- [x] T003 [US1] Add child-folder gix initialization and project activation in `crates/zit/src/project/store.rs`
- [x] T004 [US1] Add creation-mode dialog submission in `crates/zit/src/shell.rs`
- [x] T004a [US1] Activate the created project page so History loading begins immediately in `crates/zit/src/project/store.rs`
- [x] T004b [US1] Refresh the active History after a successful commit in `crates/zit/src/project/page.rs`
- [x] T005 [US1] Add repository creation and duplicate-path coverage in `crates/zit/tests/project_store_contract.rs`

## Phase 3: User Story 2 - Choose a Destination

- [x] T006 [US2] Reuse folder chooser and preserve form values in `crates/zit/src/shell.rs`

## Phase 4: User Story 3 - Recover from Invalid Input

- [x] T007 [US3] Validate names, missing folders, existing destinations, and initialization failures in `crates/zit/src/project/store.rs`
- [ ] T008 [US3] Add shell-level invalid-input and cancellation contract coverage in `crates/zit/tests`

## Phase 5: Polish

- [ ] T009 Run quickstart, format, check, and full affected test suites; record results in `specs/011-new-repository/quickstart.md`
