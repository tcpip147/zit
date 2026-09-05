---

description: "Dependency-ordered implementation tasks for project pages"
---

# Tasks: Project Pages

**Input**: Design documents from `/specs/005-project-pages/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/project-page.md, quickstart.md

**Tests**: Contract and integration tests are included because the specification defines independently testable scenarios and measurable automated acceptance outcomes.

**Organization**: Tasks are grouped by user story so restoration, lazy loading, and recovery can each be implemented and verified as a coherent increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches different files and has no dependency on another incomplete task in the same phase
- **[Story]**: Maps the task to a user story in spec.md
- Every task includes an exact target file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare the application crate and module boundaries described by the implementation plan.

- [X] T001 Add `serde`, `serde_json`, and test-only temporary-directory dependencies to `crates/zit/Cargo.toml`
- [X] T002 Create and export the project module skeleton in `crates/zit/src/project/mod.rs` and register the module in `crates/zit/src/main.rs`
- [X] T003 [P] Add isolated session-path configuration for production and tests in `crates/zit/src/project/session.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared types, repository boundaries, and global ownership required by every user story.

**⚠️ CRITICAL**: No user story work begins until this phase is complete.

- [X] T004 Define `ProjectId`, `ProjectMetadata`, `ProjectAvailability`, `CommitSummary`, `ProjectData`, and `ProjectLoadState` with their validation invariants in `crates/zit/src/project/model.rs`
- [X] T005 [P] Define injectable lightweight-inspection and history-loading boundaries plus production error types in `crates/zit/src/project/repository.rs`
- [X] T006 Implement the application-wide `ProjectStore` entity, GPUI `Global` wrapper, ordered metadata access, and project-store events in `crates/zit/src/project/store.rs`
- [X] T007 Install one global `ProjectStore` during application startup and make reopen paths reuse it in `crates/zit/src/main.rs`

**Checkpoint**: Project types, loader boundaries, and a single global store are available to all stories.

---

## Phase 3: User Story 1 - Restore Open Project Tabs (Priority: P1) 🎯 MVP

**Goal**: Restore all unique open Git repositories in saved order and present exactly one retained tab/page per project without loading history.

**Independent Test**: Start from a session containing multiple projects, a duplicate repository reference, and a selected project; verify ordered unique tabs, stable one-to-one page identities, active fallback, and the empty-project state while all history states remain `NotLoaded` except any separately activated page.

### Tests for User Story 1

> Write these tests first and confirm that they fail before implementing the story.

- [X] T008 [P] [US1] Add store contract tests for ordered uniqueness, metadata authority, active-project fallback, stable page records, and removal invariants in `crates/zit/tests/project_store_contract.rs`
- [X] T009 [P] [US1] Add session tests for versioned round-trip, duplicate entries, missing files, malformed content, and unsupported versions in `crates/zit/tests/project_session_integration.rs`
- [X] T010 [P] [US1] Extend retained tab tests to verify metadata-driven labels and unchanged page entity identity in `crates/widget/tests/tab_bar_contract.rs`

### Implementation for User Story 1

- [X] T011 [P] [US1] Implement versioned `OpenProjectSession`/`SessionProject` parsing and atomic persistence without heavyweight data in `crates/zit/src/project/session.rs`
- [X] T012 [P] [US1] Implement `gix` repository discovery, canonical Git-directory identity normalization, display-name derivation, and lightweight availability inspection in `crates/zit/src/project/repository.rs`
- [X] T013 [US1] Implement restore deduplication, open order, page-record creation, active fallback, metadata updates, close removal, and persistence triggers in `crates/zit/src/project/store.rs`
- [X] T014 [US1] Create the retained lightweight `ProjectPage` shell with project identity and initial `NotLoaded` presentation in `crates/zit/src/project/page.rs`
- [X] T015 [US1] Replace hard-coded projects with store-derived `TabSpec` values, reconcile `TabBarEvent` selection/close events, and preserve one-to-one page entities in `crates/zit/src/shell.rs`
- [X] T016 [US1] Render the no-project empty state and ensure startup/reopen restores tabs without history traversal in `crates/zit/src/shell.rs` and `crates/zit/src/main.rs`

**Checkpoint**: Session restoration and one-project-to-one-tab/page navigation work independently; history loading is still absent.

---

## Phase 4: User Story 2 - Activate a Project and Load Its Data (Priority: P1)

**Goal**: Load commit history only on first activation, retain successful results across tab switches, and prevent duplicate or cross-project updates.

**Independent Test**: Restore at least two projects with a deterministic counting loader, activate only one, and verify immediate `Loading`, exactly one request for that project, zero requests for the inactive project, `Ready` retention across switches, and a valid empty result for a repository without commits.

### Tests for User Story 2

> Write these tests first and confirm that they fail before implementing the story.

- [X] T017 [P] [US2] Add deterministic loader contract tests for activation-only loading, request deduplication, ready-state reuse, and cross-project isolation in `crates/zit/tests/project_page_contract.rs`
- [X] T018 [P] [US2] Add `gix` integration tests for HEAD history summaries, detached HEAD, and valid empty/unborn repositories in `crates/zit/tests/project_repository_integration.rs`

### Implementation for User Story 2

- [X] T019 [P] [US2] Implement HEAD resolution and newest-first `gix` revision walking into owned `ProjectData`/`CommitSummary` values in `crates/zit/src/project/repository.rs`
- [X] T020 [US2] Implement `NotLoaded → Loading → Ready` transitions, one in-flight background task, and generation-checked result application in `crates/zit/src/project/page.rs`
- [X] T021 [US2] Route initial and subsequent active-project changes to the matching retained page without activating inactive pages in `crates/zit/src/shell.rs`
- [X] T022 [US2] Render accessible loading, ready history, and ready-empty presentations while keeping tab switching responsive in `crates/zit/src/project/page.rs`

**Checkpoint**: Lazy loading, session-scoped reuse, and commit-history display work independently for available projects.

---

## Phase 5: User Story 3 - Handle Unavailable and Failed Repositories (Priority: P2)

**Goal**: Isolate repository failures, make them understandable and retryable, and reject obsolete results after retry or close.

**Independent Test**: Restore one valid and one unavailable project, force a load failure, then repair and retry it; verify the valid page stays usable, the same failed page reaches ready without duplication, and a completion arriving after close cannot mutate or recreate any project.

### Tests for User Story 3

> Write these tests first and confirm that they fail before implementing the story.

- [X] T023 [P] [US3] Extend page contract tests with failed-load retry, repeated-retry suppression, stale-generation rejection, and close-during-load cases in `crates/zit/tests/project_page_contract.rs`
- [X] T024 [P] [US3] Extend session integration tests with mixed available/unavailable restoration and recovery without blocking valid projects in `crates/zit/tests/project_session_integration.rs`

### Implementation for User Story 3

- [X] T025 [P] [US3] Convert discovery and history failures into scoped, user-safe diagnostics while retaining unavailable project metadata in `crates/zit/src/project/repository.rs`
- [X] T026 [US3] Implement `Failed → Loading` retry, generation invalidation, dropped-page result rejection, and project-removal cleanup in `crates/zit/src/project/page.rs` and `crates/zit/src/project/store.rs`
- [X] T027 [US3] Render accessible unavailable/failed states and a keyboard/a11y-activated project-specific Retry action in `crates/zit/src/project/page.rs`
- [X] T028 [US3] Ensure tab close during loading removes the global entry, selects the existing neighbor, persists the new session, and never recreates a closed page in `crates/zit/src/shell.rs`

**Checkpoint**: All three stories function independently and together, including recovery and race handling.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Validate performance, accessibility, platform behavior, and constitutional compliance across the complete feature.

- [X] T029 [P] Add a 20-project startup/lazy-load request-count acceptance harness in `crates/zit/tests/project_performance_contract.rs`
- [X] T030 [P] Audit platform path normalization and atomic session replacement behavior with focused tests in `crates/zit/tests/project_session_integration.rs`
- [X] T031 Run every scenario and record any environment-specific observations in `specs/005-project-pages/quickstart.md`
- [X] T032 Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`, fixing findings in the affected files
- [X] T033 Verify GPUI-only UI flow, `gix`-only Git access, workspace layout, and `crates/zit/src/main.rs` entry-point compliance against `.specify/memory/constitution.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies; starts immediately.
- **Foundational (Phase 2)**: Depends on Setup and blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational and establishes restored retained pages used by later stories.
- **User Story 2 (Phase 4)**: Depends on Foundational and the retained page/tab association from US1.
- **User Story 3 (Phase 5)**: Depends on the page load lifecycle from US2 and the close/session behavior from US1.
- **Polish (Phase 6)**: Depends on all selected user stories.

### User Story Dependency Graph

```text
Setup → Foundation → US1 Restore Tabs → US2 Lazy Load → US3 Recover Failures → Polish
```

- **US1 (P1)**: First independently demonstrable increment; provides global restoration and stable navigation without history.
- **US2 (P1)**: Builds on US1 page identity to provide the primary lazy-loading behavior.
- **US3 (P2)**: Builds on US1/US2 transitions to cover failures, retry, close races, and unavailable repositories.

### Within Each User Story

- Write the story's contract/integration tests first and confirm they fail for the intended missing behavior.
- Complete data/persistence or repository adapters before store/page orchestration that consumes them.
- Complete page/store behavior before shell integration.
- Run the story-specific tests at its checkpoint before moving to the next story.

### Parallel Opportunities

- T003 can proceed alongside T001–T002 once the target module path is agreed.
- T005 can proceed alongside T004 because it is isolated in `repository.rs`.
- US1 tests T008–T010 can be authored in parallel; implementations T011 and T012 can then proceed in parallel.
- US2 tests T017–T018 can be authored in parallel; T019 can proceed independently of page rendering until integration at T020.
- US3 tests T023–T024 and diagnostic adapter task T025 touch separate files and can proceed in parallel.
- Polish tasks T029–T030 can proceed in parallel before the final quickstart and full quality gates.

---

## Parallel Example: User Story 1

```text
Task T008: Add store contract tests in crates/zit/tests/project_store_contract.rs
Task T009: Add session tests in crates/zit/tests/project_session_integration.rs
Task T010: Extend retained tab tests in crates/widget/tests/tab_bar_contract.rs

After those tests are established:
Task T011: Implement session persistence in crates/zit/src/project/session.rs
Task T012: Implement lightweight gix inspection in crates/zit/src/project/repository.rs
```

## Parallel Example: User Story 2

```text
Task T017: Add lazy page contract tests in crates/zit/tests/project_page_contract.rs
Task T018: Add gix history integration tests in crates/zit/tests/project_repository_integration.rs
Task T019: Implement history traversal in crates/zit/src/project/repository.rs
```

## Parallel Example: User Story 3

```text
Task T023: Add retry/race page tests in crates/zit/tests/project_page_contract.rs
Task T024: Add unavailable restoration tests in crates/zit/tests/project_session_integration.rs
Task T025: Implement scoped repository diagnostics in crates/zit/src/project/repository.rs
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Setup and Foundational phases.
2. Complete US1 tests and implementation.
3. Stop at the US1 checkpoint and demonstrate session restoration, deduplication, active fallback, and stable one-to-one tabs/pages.
4. Treat this as the navigation/restoration MVP; continue immediately to US2 for the requested lazy history capability.

### Incremental Delivery

1. **US1**: Replace placeholder tabs with globally restored real Git projects.
2. **US2**: Add background, activation-only history and ready-state reuse.
3. **US3**: Add unavailable states, retry, and stale-result safety.
4. **Polish**: Prove performance, platform behavior, accessibility, and all quality gates.

### Parallel Team Strategy

After the shared foundation:

1. One contributor can prepare store/session tests and persistence while another prepares repository discovery and integration tests.
2. During US2, history-adapter work can proceed alongside deterministic page contract tests.
3. During US3, unavailable-session coverage can proceed alongside retry/race page coverage.
4. Tasks that modify `shell.rs`, `page.rs`, or `store.rs` should remain serialized to avoid overlapping state-machine edits.

---

## Notes

- `[P]` marks only tasks that can proceed without overlapping incomplete work.
- `[US1]`, `[US2]`, and `[US3]` provide traceability to spec.md.
- Tests precede story implementation and must fail for the intended behavioral gap before production changes.
- No task invokes the Git CLI; all repository work remains within the approved `gix` backend.
- Commit after each task or coherent task group, and stop at each checkpoint for independent validation.
