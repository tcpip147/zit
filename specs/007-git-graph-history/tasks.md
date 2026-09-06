---

description: "Dependency-ordered implementation tasks for Git graph history"
---

# Tasks: Git Graph History

**Input**: Design documents from `/specs/007-git-graph-history/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/project-history.md, quickstart.md

**Tests**: Automated topology, repository integration, page contract, structural, and performance tests are required by the measurable outcomes and validation plan.

**Organization**: Tasks are grouped by user story so repository-wide history, graph topology, commit context, and History presentation can be implemented and validated as distinct increments.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches a different file and has no dependency on another incomplete task in the phase.
- **[Story]**: Maps the task to a user story in `spec.md`.
- Every task includes an exact target file path.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish feature-specific test and validation surfaces without changing runtime behavior.

- [X] T001 Create the compilable topology contract test target and shared commit-descriptor fixture shell in `crates/zit/tests/project_graph_contract.rs`
- [X] T002 [P] Add graph-history fixture helpers for branches, merges, commit messages, and timestamps to `crates/zit/tests/project_repository_integration.rs`
- [X] T003 [P] Add the 007 automated, structural, interactive, and performance command checklist placeholders to `specs/007-git-graph-history/quickstart.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Introduce graph-domain records and keep all existing consumers compiling before story behavior changes.

**CRITICAL**: No user-story implementation begins until the enriched model and existing fixtures are updated.

- [X] T004 Define `CommitGraphMarker` and `CommitGraphEdge` with equality/debug/default behavior and documented invariants in `crates/zit/src/project/model.rs`
- [X] T005 Extend `CommitSummary` with message body, commit marker, input/output edges, HEAD state, and branch labels in `crates/zit/src/project/model.rs`
- [X] T006 Export the new graph-domain records through the existing project module boundary in `crates/zit/src/project/mod.rs`
- [X] T007 [P] Update repository integration `CommitSummary` expectations and constructors for the enriched model in `crates/zit/tests/project_repository_integration.rs`
- [X] T008 [P] Update page and store test loaders/fixtures for enriched commit summaries without changing existing assertions in `crates/zit/tests/project_page_contract.rs`
- [X] T009 [P] Update any remaining project test fixtures that construct `CommitSummary` in `crates/zit/tests/project_performance_contract.rs` and `crates/zit/tests/project_store_contract.rs`

**Checkpoint**: The enriched graph-track model compiles throughout the workspace and remains free of GPUI/DataGrid types.

---

## Phase 3: User Story 1 - Read Repository-Wide Graph History (Priority: P1) — MVP

**Goal**: Return a unique, newest-first, 100-track history reachable from all readable local and remote branch tips.

**Independent Test**: Load temporary repositories with diverged and overlapping local/remote branches, no branches, invalid individual refs, and more than 100 commits; verify roots, order, uniqueness, error isolation, and the limit without relying on graph drawing.

### Tests for User Story 1

> Write these tests first and confirm they fail for the intended missing behavior.

- [X] T010 [US1] Add multi-root, shared-ancestry de-duplication, branch-only commit, and newest-first ordering tests in `crates/zit/tests/project_repository_integration.rs`
- [X] T011 [US1] Add no-valid-root, mixed-valid/invalid-reference, and 100-track limit tests in `crates/zit/tests/project_repository_integration.rs`

### Implementation for User Story 1

- [X] T012 [US1] Add repository-private `BranchTip` collection for readable local and remote branches with independent peel failure handling in `crates/zit/src/project/repository.rs`
- [X] T013 [US1] Replace HEAD-only history traversal with one multi-tip gix walk using newest-first commit-time sorting and a 100-entry bound in `crates/zit/src/project/repository.rs`
- [X] T014 [US1] Preserve empty success for repositories with no valid branch roots and translate fatal enumeration/walk errors into `RepositoryError` in `crates/zit/src/project/repository.rs`

**Checkpoint**: User Story 1 independently provides bounded repository-wide commit history, even before lane geometry is populated.

---

## Phase 4: User Story 2 - Understand Branch and Merge Topology (Priority: P1)

**Goal**: Produce deterministic markers and edges for linear, carried, diverging, converging, and multi-parent histories using the supplied algorithm.

**Independent Test**: Feed ordered in-memory commit descriptors for linear, branch, merge, criss-cross convergence, and octopus histories into the pure builder and compare every marker and edge field exactly.

### Tests for User Story 2

> Write these tests first and confirm they fail for the intended missing behavior.

- [X] T015 [US2] Add exact linear-lane, first-track color, and uninterrupted carried-edge assertions in `crates/zit/tests/project_graph_contract.rs`
- [X] T016 [US2] Add divergence, two-parent merge, existing-parent-lane reuse, and convergence assertions in `crates/zit/tests/project_graph_contract.rs`
- [X] T017 [US2] Add multi-parent, duplicate-destination compaction, output ordering, missing-parent safety, and deterministic-repeat assertions in `crates/zit/tests/project_graph_contract.rs`

### Implementation for User Story 2

- [X] T018 [US2] Define repository-private ordered commit descriptors and per-load graph color allocation in `crates/zit/src/project/repository.rs`
- [X] T019 [US2] Implement previous-output to current-input derivation with distinct destination lanes, reaching-edge marker selection, and carried-lane compaction in `crates/zit/src/project/repository.rs`
- [X] T020 [US2] Implement non-tip continuations, first-parent continuation, existing additional-parent lane reuse, and new-parent lane/color allocation in `crates/zit/src/project/repository.rs`
- [X] T021 [US2] Sort outgoing edges, remove unchecked edge/parent lookups, and expose a deterministic testable graph-building boundary in `crates/zit/src/project/repository.rs`
- [X] T022 [US2] Feed the bounded gix traversal descriptors through the graph builder and publish enriched commit tracks in `crates/zit/src/project/repository.rs`

**Checkpoint**: User Story 2 independently proves topology correctness for supplied-algorithm fixtures with no GPUI dependency.

---

## Phase 5: User Story 3 - Identify Commit Context (Priority: P2)

**Goal**: Attach complete message metadata, resolved HEAD status, and all shortened local/remote branch labels to the correct graph tracks.

**Independent Test**: Load a repository where multiple references and HEAD point at known commits and verify title, body, author, time, identity, HEAD, and labels exactly.

### Tests for User Story 3

> Write these tests first and confirm they fail for the intended missing behavior.

- [X] T023 [US3] Add HEAD, detached/unborn HEAD, multiple labels at one commit, and labels outside the returned window tests in `crates/zit/tests/project_repository_integration.rs`
- [X] T024 [US3] Add title/body/empty-body, author identity, authored time/offset, full object ID, and metadata failure assertions in `crates/zit/tests/project_repository_integration.rs`

### Implementation for User Story 3

- [X] T025 [US3] Build the branch-label map from peeled branch tips while preserving every shortened local and remote name in `crates/zit/src/project/repository.rs`
- [X] T026 [US3] Resolve optional HEAD independently of branch traversal and mark exactly the matching returned track in `crates/zit/src/project/repository.rs`
- [X] T027 [US3] Read message title/body, author name/email, commit time/offset, parents, and full identity with user-safe failure mapping in `crates/zit/src/project/repository.rs`
- [X] T028 [US3] Attach branch labels and HEAD state during graph-track construction without changing topology or result ordering in `crates/zit/src/project/repository.rs`

**Checkpoint**: User Story 3 provides complete repository context on every returned graph track.

---

## Phase 6: User Story 4 - View the Graph in Project History (Priority: P2)

**Goal**: Render arbitrary graph tracks in the existing History DataGrid while preserving all established page behavior.

**Independent Test**: Open linear, diverged, merged, empty, and failing repositories in History; verify visible graph geometry, labels/context, fixed columns, lazy loading, retry, scrolling, and project isolation.

### Tests for User Story 4

> Write these tests first and confirm they fail for the intended missing behavior.

- [X] T029 [P] [US4] Extend page contracts for enriched graph rows, Graph/Commit/Author/Date/Hash order, empty/error/retry behavior, and two-project isolation in `crates/zit/tests/project_page_contract.rs`
- [X] T030 [P] [US4] Add structural assertions preventing GPUI/DataGrid types in graph models and Git graph types in the reusable widget in `crates/zit/tests/project_graph_contract.rs`

### Implementation for User Story 4

- [X] T031 [US4] Add deterministic cycling graph palette selection and lane-to-pixel geometry helpers in `crates/zit/src/project/page.rs`
- [X] T032 [US4] Replace the placeholder commit/merge glyph with GPUI canvas rendering for every incoming edge, outgoing edge, diagonal transition, and commit marker in `crates/zit/src/project/page.rs`
- [X] T033 [US4] Render lane-changing input/output edges as smooth cubic curves while preserving straight same-lane continuations in `crates/zit/src/project/page.rs`
- [X] T034 [US4] Add HEAD and branch context to the graph/commit presentation without obscuring topology or changing existing commit values in `crates/zit/src/project/page.rs`
- [X] T035 [US4] Preserve clipping, DataGrid virtualization/resizing, lazy loading, empty/failure/retry behavior, and immutable per-page graph snapshots in `crates/zit/src/project/page.rs`

**Checkpoint**: Project History displays complete graph topology with no regression and DataGrid remains Git-independent.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Complete performance, robustness, documentation, and repository-wide quality validation.

- [X] T036 [P] Add a 10,000-commit fixture proving the two-second load target and 100-track bound in `crates/zit/tests/project_graph_contract.rs`
- [X] T037 [P] Add Rust documentation for graph records, lane semantics, error policy, and the pure builder boundary in `crates/zit/src/project/model.rs` and `crates/zit/src/project/repository.rs`
- [X] T038 Run all automated and interactive scenarios and record platform-specific results in `specs/007-git-graph-history/quickstart.md`
- [X] T039 Run `cargo fmt --all -- --check`, strict workspace clippy, and `cargo test --workspace`, fix affected findings, and record results in `specs/007-git-graph-history/quickstart.md`
- [X] T040 Verify gix-only Git access, GPUI-only UI rendering, domain/widget separation, workspace layout, English artifacts, and unchanged `crates/zit/src/main.rs` entry point against `.specify/memory/constitution.md`
- [X] T041 Format History Author as `name <email>` and Date as offset-aware `YYYY-MM-DD HH:MI` with date regression tests in `crates/zit/src/project/page.rs`
- [X] T042 Add explicit committer-time fields from `commit.time()`, use them for Date, and preserve the complete commit object ID in the domain model in `crates/zit/src/project/model.rs`, `crates/zit/src/project/repository.rs`, and `crates/zit/src/project/page.rs`
- [X] T043 Display an eight-character Hash and configure initial History widths for Graph, Author, Date, and Hash while Commit fills remaining space in `crates/zit/src/project/page.rs`
- [X] T044 Rename the visible Commit header to Description and set Graph and Author initial widths to 200 pixels in `crates/zit/src/project/page.rs`
- [X] T045 Fill non-HEAD graph markers, keep the HEAD graph marker hollow, and replace the Description HEAD label with a leading circular indicator in `crates/zit/src/project/page.rs`
- [X] T044 Adjust the initial History widths to Graph 300 pixels, Author 250 pixels, and Date 120 pixels in `crates/zit/src/project/page.rs`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundation (Phase 2)**: Depends on Setup and blocks all stories.
- **US1 Repository-Wide History (Phase 3)**: Depends on Foundation.
- **US2 Graph Topology (Phase 4)**: Depends on US1's ordered commit stream and Foundation's graph records.
- **US3 Commit Context (Phase 5)**: Depends on US1 roots/traversal and US2 track construction.
- **US4 History Presentation (Phase 6)**: Depends on enriched tracks from US2 and US3.
- **Polish (Phase 7)**: Depends on every selected story.

### User Story Dependency Graph

```text
Setup → Foundation → US1 Repository-Wide History → US2 Graph Topology
                                                     ↓
                                              US3 Commit Context
                                                     ↓
                                             US4 History UI → Polish
```

- **US1 (P1)**: Independently testable as bounded multi-branch commit collection.
- **US2 (P1)**: Uses US1 ordering but is independently testable through pure topology fixtures.
- **US3 (P2)**: Adds labels and metadata without altering topology.
- **US4 (P2)**: Consumes completed graph tracks and preserves page contracts.

### Within Each User Story

- Write the story tests first and confirm the intended failure.
- Complete model/descriptor state before algorithms consuming it.
- Complete repository behavior before page integration.
- Stop at each checkpoint and run the story-specific tests.

### Parallel Opportunities

- T002 and T003 can run in parallel after T001.
- T007, T008, and T009 update separate test files after the shared model is defined.
- Test-design tasks within US1, US2, US3, and US4 can be authored in parallel when they touch separate files; tasks sharing one file remain logically coordinated.
- T029 and T030 can run in parallel because they target separate test files.
- T035 and T036 can run in parallel after all stories.

## Parallel Example: User Story 2

```text
Task T015: Add linear and carried-edge fixtures in crates/zit/tests/project_graph_contract.rs
Task T016: Add divergence and merge fixtures in crates/zit/tests/project_graph_contract.rs
Task T017: Add multi-parent and invariant fixtures in crates/zit/tests/project_graph_contract.rs
```

These tests cover independent fixture groups but should be merged carefully because they share one target file.

## Parallel Example: User Story 4

```text
Task T029: Extend ProjectPage graph integration contracts in crates/zit/tests/project_page_contract.rs
Task T030: Add domain/widget structural contracts in crates/zit/tests/project_graph_contract.rs
```

## Implementation Strategy

### MVP First

1. Complete Setup and Foundation.
2. Complete US1 to deliver bounded repository-wide history.
3. Validate US1 independently before adding topology.
4. Complete US2 for the functional graph-data MVP.

### Incremental Delivery

1. **US1**: Multi-branch, unique, ordered, bounded commits.
2. **US2**: Supplied lane and edge algorithm.
3. **US3**: HEAD, labels, and complete commit context.
4. **US4**: GPUI graph presentation inside DataGrid.
5. **Polish**: Performance, docs, quickstart, and full quality gates.

### Team Strategy

Repository implementation in `project/repository.rs` stays serialized. Test fixture groups and UI/model documentation may proceed independently where file ownership does not overlap.

## Notes

- `[P]` marks only work that can proceed without an incomplete dependency in the same phase.
- `[US1]` through `[US4]` provide direct traceability to `spec.md`.
- Every task preserves the exact requested gix and supplied-algorithm constraints.
- Commit after each coherent task group and validate at every checkpoint.
