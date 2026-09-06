---

description: "Dependency-ordered implementation tasks for the reusable DataGrid widget"
---

# Tasks: Reusable Data Grid Widget

**Input**: Design documents from `/specs/006-data-grid/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/public-api.md, quickstart.md

**Tests**: Automated contract, scroll, accessibility, performance, and project-integration tests are required by the specification's measurable outcomes.

**Organization**: Tasks are grouped by user story so basic structured display, large-row navigation, direct scrollbar interaction, and project-history migration can be implemented and verified as separate increments.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches different files and has no dependency on another incomplete task in the same phase.
- **[Story]**: Maps the task to a user story in `spec.md`.
- Every task includes an exact target file path.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the requested module, exports, theme boundary, and example target before behavior is implemented.

- [X] T001 Create `crates/widget/src/data-grid.rs`, declare it as `data_grid` with `#[path = "data-grid.rs"]`, and add placeholder public exports in `crates/widget/src/lib.rs`
- [X] T002 [P] Add `DataGridTheme` defaults and integrate them into the shared `Theme` in `crates/widget/src/theme.rs`
- [X] T003 [P] Register the `data_grid` example target in `crates/widget/Cargo.toml` and create its compilable shell in `crates/widget/examples/data_grid.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Define validated public configuration and independent retained state required by every story.

**⚠️ CRITICAL**: No user-story rendering work begins until this phase is complete.

- [X] T004 Define `DataGridColumn`, `DataGridColumnWidth`, `DataGridCell`, `DataGridRow`, and `DataGridError` with validation rules from `contracts/public-api.md` in `crates/widget/src/data-grid.rs`
- [X] T005 Implement `DataGrid` construction, row-renderer ownership, unique per-entity element identity, row/header height configuration, theme access, and independent scroll/drag/visibility state in `crates/widget/src/data-grid.rs`
- [X] T006 Implement atomic column replacement, row-count/renderer replacement, read-only inspection, invalid-index rejection, and scroll-bound clamping entry points in `crates/widget/src/data-grid.rs`

**Checkpoint**: The public model compiles, validates configuration without partial mutation, and can represent independent empty grid instances.

---

## Phase 3: User Story 1 - Display Structured Rows (Priority: P1) 🎯 MVP

**Goal**: Render a reusable fixed-header grid with ordered columns, normalized rows, custom cells, clipping, theming, and accessible table structure.

**Independent Test**: Configure five columns with exact, missing, and excess cells plus an empty grid; verify header order, cell normalization, custom content, clipping, and semantic relationships.

### Tests for User Story 1

> Write these tests first and confirm they fail for the intended missing behavior.

- [X] T007 [P] [US1] Add construction, validation, empty-grid, ordered-column, missing-cell padding, excess-cell truncation, and atomic-update contract tests in `crates/widget/tests/data_grid_contract.rs`
- [X] T008 [P] [US1] Add visible table, header, row, cell, label, position, and count accessibility assertions in `crates/widget/tests/data_grid_accessibility.rs`

### Implementation for User Story 1

- [X] T009 [US1] Implement shared fixed/proportional column layout calculation used by header and rows in `crates/widget/src/data-grid.rs`
- [X] T010 [US1] Render the fixed header, normalized rows, clipped no-wrap cells, custom elements, dividers, and zero-column/zero-row states in `crates/widget/src/data-grid.rs`
- [X] T011 [US1] Apply `DataGridTheme` header, row, text, divider, track, and thumb values without hard-coded consumer colors in `crates/widget/src/data-grid.rs`
- [X] T012 [US1] Add table/grid, header-row, column-header, body-row, and cell semantics with available position/count metadata in `crates/widget/src/data-grid.rs`

**Checkpoint**: US1 works with a small arbitrary data set and has no dependency on project or Git types.

---

## Phase 4: User Story 2 - Navigate a Large Row Set (Priority: P1)

**Goal**: Virtualize uniform rows, keep the header fixed, bound vertical scrolling, and remain responsive with 10,000 rows.

**Independent Test**: Display 0, 1, 100, and 10,000 rows in fixed viewports; verify only valid visible indexes are requested and both scroll boundaries are reachable without blank overscroll.

### Tests for User Story 2

> Write these tests first and confirm they fail for the intended missing behavior.

- [X] T013 [P] [US2] Add visible-range requests, fixed-header behavior, wheel scrolling, boundary clamping, no-overflow stability, and row-count shrink tests in `crates/widget/tests/data_grid_scroll.rs`
- [X] T014 [P] [US2] Add 0/1/100/10,000-row request-count and sub-100-ms interaction acceptance coverage in `crates/widget/tests/data_grid_performance.rs`

### Implementation for User Story 2

- [X] T015 [US2] Integrate GPUI uniform-list range rendering and `UniformListScrollHandle` tracking with the indexed row renderer in `crates/widget/src/data-grid.rs`
- [X] T016 [US2] Implement body-only vertical wheel/trackpad scrolling, boundary clamping, row-count-change clamping, and valid `scroll_to_row` behavior in `crates/widget/src/data-grid.rs`
- [X] T017 [US2] Ensure header layout remains fixed and aligned while the virtual body scrolls and requests only visible buffered indexes in `crates/widget/src/data-grid.rs`

**Checkpoint**: US2 independently supports responsive bounded navigation through 10,000 rows.

---

## Phase 5: User Story 3 - Use the Grid Scroll Control (Priority: P2)

**Goal**: Provide a proportional, directly draggable, transient vertical scrollbar whose lifecycle remains correct across pointer exit and multiple grid instances.

**Independent Test**: Overflow two grids, reveal and drag each thumb to both boundaries, release outside, advance the deterministic clock, and verify independent visibility and offsets.

### Tests for User Story 3

> Write these tests first and confirm they fail for the intended missing behavior.

- [X] T018 [P] [US3] Add thumb geometry, 24-pixel minimum, proportional drag, boundary clamping, outside release, and no-overflow tests in `crates/widget/tests/data_grid_scroll.rs`
- [X] T019 [P] [US3] Add deterministic hover/wheel/drag reveal, one-second hide, stale-hide rejection, and two-grid independence tests in `crates/widget/tests/data_grid_contract.rs`

### Implementation for User Story 3

- [X] T020 [US3] Implement overflow detection plus proportional scroll-track and minimum-size thumb geometry in `crates/widget/src/data-grid.rs`
- [X] T021 [US3] Implement thumb mouse-down, scaled window-level move handling, bounded offset updates, and primary-button release outside the grid in `crates/widget/src/data-grid.rs`
- [X] T022 [US3] Implement hover/wheel/drag visibility transitions and generation-safe one-second delayed hiding per grid instance in `crates/widget/src/data-grid.rs`

**Checkpoint**: US3 supports direct long-list navigation with no stuck drag or shared timer state.

---

## Phase 6: User Story 4 - Reuse the Grid in Project History (Priority: P2)

**Goal**: Replace private project-history grid mechanics with `DataGrid` while preserving history columns, lazy loading, retry, empty state, and page isolation.

**Independent Test**: Load non-empty and empty repository histories, switch between projects, scroll the non-empty history, and verify unchanged columns, values, load states, and independent retained pages.

### Tests for User Story 4

> Extend the existing project contracts before migrating the consumer.

- [X] T023 [P] [US4] Extend history integration coverage for five-column order, formatted commit cells, empty-history ownership, lazy loading, retry, and per-project grid isolation in `crates/zit/tests/project_page_contract.rs`
- [X] T024 [P] [US4] Add a structural dependency test preventing project, repository, or commit domain references in `crates/widget/tests/data_grid_contract.rs`

### Implementation for User Story 4

- [X] T025 [US4] Add one retained history `DataGrid` to each `ProjectPage` and configure Graph, Commit, Author, Date, and Hash columns in `crates/zit/src/project/page.rs`
- [X] T026 [US4] Move generic header, cell, virtualization, scrollbar, drag, and hide-timer behavior out of `ProjectPage` and supply commit-specific `DataGridRow` values from `crates/zit/src/project/page.rs`
- [X] T027 [US4] Preserve project-owned commit graph rendering, summary/author/date/hash formatting, loading/failure/retry states, and empty-history presentation in `crates/zit/src/project/page.rs`
- [X] T028 [US4] Remove obsolete project-page grid fields, imports, event capture, and private table helpers after migration in `crates/zit/src/project/page.rs`

**Checkpoint**: Project History uses the reusable widget with no user-visible regression and the widget crate remains domain-independent.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Complete reusable documentation, interactive validation, and repository quality gates.

- [X] T029 [P] Expand the two-grid and 10,000-row interactive host with clipping, empty, overflow, and theme demonstrations in `crates/widget/examples/data_grid.rs`
- [X] T030 [P] Add rustdoc examples and document validation errors, renderer lifetime, row normalization, scrolling, and asset-free usage in `crates/widget/src/data-grid.rs` and `crates/widget/src/lib.rs`
- [X] T031 Run every automated and interactive scenario and record environment-specific observations in `specs/006-data-grid/quickstart.md`
- [X] T032 Run the full formatting, lint, and workspace test commands, fix findings in affected source and test files, and record results in `specs/006-data-grid/quickstart.md`
- [X] T033 Verify GPUI-only UI flow, absence of Git-domain coupling in `crates/widget/src/data-grid.rs`, workspace layout, and `crates/zit/src/main.rs` entry-point compliance against `.specify/memory/constitution.md`
- [X] T034 Add an 8-pixel centered header-boundary resize target, horizontal resize cursor, and window-level drag lifecycle in `crates/widget/src/data-grid.rs`
- [X] T035 Keep adjacent widths stable as a pair, enforce a 40-pixel minimum, and add resize regression coverage in `crates/widget/src/data-grid.rs`
- [X] T036 Store resized columns as proportional weights so their ratios survive grid and window size changes in `crates/widget/src/data-grid.rs`
- [X] T037 Align body-cell flex sizing with header cells by moving padding and clipping into an inner content container in `crates/widget/src/data-grid.rs`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies; starts immediately.
- **Foundational (Phase 2)**: Depends on Setup and blocks every user story.
- **US1 Structured Display (Phase 3)**: Depends on Foundation and provides the minimum reusable grid.
- **US2 Large Row Set (Phase 4)**: Depends on US1 row rendering and adds virtualization and scrolling.
- **US3 Scroll Control (Phase 5)**: Depends on US2 measured scroll state.
- **US4 Project History (Phase 6)**: Depends on US1 through US3 so the extracted widget can replace all private table mechanics.
- **Polish (Phase 7)**: Depends on every selected story.

### User Story Dependency Graph

```text
Setup → Foundation → US1 Structured Display → US2 Virtualized Navigation
                                             ↓
                                      US3 Scroll Control → US4 Project Migration → Polish
```

- **US1 (P1)**: Independently demonstrates a small reusable structured table.
- **US2 (P1)**: Extends US1 with large-list performance and bounded vertical navigation.
- **US3 (P2)**: Extends measured US2 scrolling with direct thumb interaction.
- **US4 (P2)**: Proves the complete extracted component with the existing project-history consumer.

### Within Each User Story

- Write the story's tests first and confirm they fail for the intended missing behavior.
- Complete state and validation before rendering that state.
- Complete core widget behavior before consumer integration.
- Run story-specific tests at each checkpoint before proceeding.

### Parallel Opportunities

- T002 and T003 can proceed in parallel after T001 establishes export names.
- T007 and T008 can be authored in parallel; T009 can begin while accessibility assertions are refined.
- T013 and T014 can be authored in parallel because they target separate scroll and performance suites.
- T018 and T019 can be authored in parallel within separate interaction concerns.
- T023 and T024 can be authored in parallel because they target different crates.
- T029 and T030 can proceed in parallel after all stories are complete.

---

## Parallel Example: User Story 1

```text
Task T007: Add configuration and row-normalization contracts in crates/widget/tests/data_grid_contract.rs
Task T008: Add semantic-tree assertions in crates/widget/tests/data_grid_accessibility.rs
```

## Parallel Example: User Story 2

```text
Task T013: Add visible-range and scroll-bound tests in crates/widget/tests/data_grid_scroll.rs
Task T014: Add 10,000-row performance coverage in crates/widget/tests/data_grid_performance.rs
```

## Parallel Example: User Story 3

```text
Task T018: Add thumb geometry and drag tests in crates/widget/tests/data_grid_scroll.rs
Task T019: Add visibility timer and independence tests in crates/widget/tests/data_grid_contract.rs
```

## Parallel Example: User Story 4

```text
Task T023: Extend project history regression contracts in crates/zit/tests/project_page_contract.rs
Task T024: Add generic-module dependency checks in crates/widget/tests/data_grid_contract.rs
```

---

## Implementation Strategy

### MVP First

1. Complete Setup and Foundation.
2. Complete US1 and validate a small arbitrary grid independently.
3. Complete US2 before treating the component as suitable for project history.
4. Demonstrate US1 plus US2 as the reusable large-list MVP.

### Incremental Delivery

1. **US1**: Deliver validated columns, normalized rows, custom cells, theming, and semantics.
2. **US2**: Add virtualized bounded navigation for large data sets.
3. **US3**: Add proportional transient scrollbar interaction.
4. **US4**: Migrate project History and remove duplicate private mechanics.
5. **Polish**: Complete examples, documentation, and full quality gates.

### Parallel Team Strategy

After the shared foundation, test authorship for behavior, accessibility, and performance can proceed in separate files. Implementation in `crates/widget/src/data-grid.rs` remains serialized to avoid overlapping state-machine edits. Project migration work begins only after the generic widget contract is stable.

## Notes

- `[P]` marks only work that can proceed without overlapping an incomplete dependency.
- `[US1]` through `[US4]` provide direct traceability to `spec.md`.
- No task adds Git behavior to the widget crate.
- The exact requested filename remains `crates/widget/src/data-grid.rs`; Rust exports use conventional `DataGrid` identifiers.
- Commit after each task or coherent task group and stop at each checkpoint for independent validation.
