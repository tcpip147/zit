# Tasks: Open Project

**Input**: Design documents from `specs/010-open-project/`

## Phase 1: Setup (Shared Infrastructure)

- [x] T001 Add the native folder dialog dependency to `Cargo.toml` and `crates/zit/Cargo.toml`
- [x] T002 Run `cargo check -p zit` and resolve dependency/platform compilation issues

## Phase 2: Foundational (Blocking Prerequisites)

- [x] T003 [P] Add `add_repository` to `crates/zit/src/project/store.rs` with canonicalization and Git-directory validation
- [x] T004 [P] Define recoverable validation errors and duplicate behavior in `crates/zit/src/project/store.rs`
- [x] T005 Update `crates/zit/src/platform.rs` so `OpenRepository` is enabled

**Checkpoint**: The store validates, deduplicates, persists, and activates repositories.

## Phase 3: User Story 1 - Open a Git Project (Priority: P1) 🎯 MVP

**Goal**: Select File > Open, enter a name and repository, and create one active tab.

**Independent Test**: Confirm a valid name and Git repository and verify one active tab.

- [x] T006 [US1] Add open-project form state and two `TextInput` entities in `crates/zit/src/shell.rs`
- [x] T007 [US1] Render the modal with name, folder, Open, Cancel, and accessible dialog controls in `crates/zit/src/shell.rs`
- [x] T008 [US1] Initialize the form when File > Open is selected in `crates/zit/src/shell.rs`
- [x] T009 [US1] Submit the form through the project store and close the dialog after success in `crates/zit/src/shell.rs`
- [x] T010 [US1] Verify new project store events update and activate the tab bar in `crates/zit/src/shell.rs`

## Phase 4: User Story 2 - Choose a Folder (Priority: P1)

**Goal**: Choose the repository folder through the OS folder chooser.

**Independent Test**: Browse to a folder and verify the path populates; cancel without losing form values.

- [x] T011 [US2] Implement asynchronous native folder selection and populate the folder input in `crates/zit/src/shell.rs`
- [x] T012 [US2] Preserve form values on chooser cancellation and show chooser errors in `crates/zit/src/shell.rs`
- [x] T013 [US2] Add Browse button hit-target and accessibility behavior in `crates/zit/src/shell.rs`

## Phase 5: User Story 3 - Recover from Invalid Input (Priority: P2)

**Goal**: Keep the dialog open and explain invalid input safely.

**Independent Test**: Confirm missing name, missing path, non-Git folder, and duplicate repository.

- [x] T014 [US3] Validate trimmed name and folder fields before submission in `crates/zit/src/shell.rs`
- [x] T015 [US3] Render actionable errors while retaining entered values in `crates/zit/src/shell.rs`
- [x] T016 [US3] Handle Cancel and Escape without project-store changes in `crates/zit/src/shell.rs`
- [x] T017 [US3] Add store contract coverage for invalid paths, non-Git folders, duplicates, and activation in `crates/zit/tests/project_store_contract.rs`

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T018 [P] Update menu enabled-state assertions in `crates/zit/src/menu.rs` and `crates/zit/tests/menu_contract.rs`
- [x] T019 [P] Keep UI contract labels and behavior aligned in `specs/010-open-project/contracts/open-project-ui.md`
- [x] T020 Run `specs/010-open-project/quickstart.md` scenarios and record platform limitations
- [x] T021 Run `cargo fmt --all`, `cargo check -p zit`, and affected `cargo test -p zit` suites

## Dependencies & Execution Order

- Setup precedes Foundational; Foundational precedes all stories.
- US1 and US2 can proceed in parallel after Foundational, with shared form ownership coordinated.
- US3 depends on US1 confirmation and US2 form population.
- Polish depends on completed stories.

## Implementation Strategy

Complete Setup, Foundational, and US1 first as the MVP. Then add native browsing (US2), validation and cancellation hardening (US3), and final regression checks.
