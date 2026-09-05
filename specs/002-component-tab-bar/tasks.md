---
description: "Dependency-ordered implementation tasks for the reusable component tab bar"
---

# Tasks: Component Tab Bar

**Input**: Design documents from `specs/002-component-tab-bar/`.

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md), [API contract](contracts/tab-bar-api.md), and
[quickstart.md](quickstart.md).

**Tests**: The specification defines independent selection, close, overflow, keyboard, and
assistive-technology scenarios. Focused contract and GPUI interaction tests are included where
they verify observable behavior rather than mirroring implementation.

**Organization**: Tasks are grouped by user story in priority order. Each story ends with an
independent checkpoint and can be demonstrated using only the foundation and earlier public API.

## Format: `[ID] [P?] [Story] Description`

- `[P]` means the task uses different files and has no dependency on another incomplete task in its group.
- Story tasks include `[US1]` through `[US4]`; setup, foundation, and polish tasks have no story label.
- Every task identifies the exact repository-relative file it changes.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add the module and bundled assets without changing existing widget behavior.

- [x] T001 Create the tab-bar module boundary and public re-export placeholders in crates/widget/src/tab_bar.rs and crates/widget/src/lib.rs.
- [x] T002 [P] Add project-authored currentColor close and fallback project SVGs with viewBox and license/provenance metadata in crates/widget/assets/tab-bar/close.svg, crates/widget/assets/tab-bar/fallback-project.svg, and crates/widget/assets/tab-bar/manifest.json.
- [x] T003 [P] Register the tab_bar example target in crates/widget/Cargo.toml and add a compilable GPUI application skeleton in crates/widget/examples/tab_bar.rs.

**Checkpoint**: The workspace compiles with an empty exported tab-bar module and resolvable example/assets.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the shared state and host contract required by every story.

- [x] T004 Define TabId, TabSpec, TabEntry, TabBarEvent, duplicate-ID error, and read-only inspection types from data-model.md and contracts/tab-bar-api.md in crates/widget/src/tab_bar.rs.
- [x] T005 Implement validated TabBar construction, first-tab fallback selection, insertion, stable-ID lookup, empty state, and page-entity preservation in crates/widget/src/tab_bar.rs.
- [x] T006 Add TabBar focus ownership, per-tab focus handles, ScrollHandle ownership, and GPUI EventEmitter/Focusable boundaries in crates/widget/src/tab_bar.rs.
- [ ] T007 Create shared test fixtures with distinct retained page entities, project SVG paths, and tab builders in crates/widget/tests/support/tab_bar.rs.

**Checkpoint**: A host can construct and inspect a stable, repository-independent TabBar entity; all stories may build on this boundary.

---

## Phase 3: User Story 1 - Switch Between Project Pages (Priority: P1) ??MVP

**Goal**: Select one project tab and show exactly its retained page.

**Independent Test**: Construct three tabs with distinct page identities, select each by pointer
and host API, and observe exactly one active tab and matching page without page recreation.

- [x] T008 [P] [US1] Write selection contract coverage for initial fallback, unknown ID rejection, idempotent active selection, event count, and retained page identity in crates/widget/tests/tab_bar_contract.rs.
- [x] T009 [US1] Implement stable-ID select transitions, SelectionChanged emission, focused-ID synchronization, and active-item reveal request in crates/widget/src/tab_bar.rs.
- [x] T010 [US1] Render the TabList row, selectable tab bodies, active/inactive state, and only the active page in crates/widget/src/tab_bar.rs.
- [x] T011 [US1] Extend the three-page demonstration with pointer selection and visible page identity in crates/widget/examples/tab_bar.rs.

**Checkpoint**: US1 is a complete MVP: selecting any existing tab displays only its associated page.

---

## Phase 4: User Story 2 - Identify and Close Tabs (Priority: P1)

**Goal**: Render SVG/name/X tabs and remove tabs using deterministic neighbor selection.

**Independent Test**: Inspect SVG/name/X order, then close inactive, first active, middle active,
last active, and sole tabs while checking selection, page removal, focus, and event order.

- [ ] T012 [P] [US2] Write closure contract tests for inactive/active/final removal, right-then-left selection, close request acceptance/veto, and event order in crates/widget/tests/tab_bar_navigation.rs.
- [ ] T013 [P] [US2] Write GPUI pointer tests proving close-click propagation does not select the tab and invalid SVG falls back in crates/widget/tests/tab_bar_close_pointer.rs.
- [x] T014 [US2] Implement CloseRequested and explicit accepted close transitions, veto-safe state, deterministic neighbor selection, page removal, and focus recovery in crates/widget/src/tab_bar.rs.
- [x] T015 [US2] Render fixed-space project SVG, truncated text_xs project name, and currentColor X SVG with separate hoverable pointer hit regions and fallback lookup in crates/widget/src/tab_bar.rs.
- [x] T016 [US2] Add inactive/active/final close controls and a missing-icon case to crates/widget/examples/tab_bar.rs.

**Checkpoint**: US1 and US2 work together; tabs are identifiable and every close position produces the specified page state.

---

## Phase 5: User Story 3 - Reach Overflowing Tabs (Priority: P2)

**Goal**: Reach any tab through bounded horizontal wheel/trackpad scrolling without unreadable shrinkage.

**Independent Test**: Render 30 fixed-minimum-width tabs in a narrow viewport, scroll to both
ends using vertical and horizontal deltas, resize, and activate initially hidden tabs.

- [ ] T017 [P] [US3] Write GPUI scroll tests for vertical-to-horizontal mapping, native horizontal preference, both bounds, no-overflow stability, resize clamping, and active reveal in crates/widget/tests/tab_bar_scroll.rs.
- [x] T018 [US3] Implement minimum/maximum tab sizing, overflow_x_scroll, tracked ScrollHandle measurement, wheel delta conversion, directional consumption, and offset clamping in crates/widget/src/tab_bar.rs.
- [x] T019 [US3] Integrate scroll_to_item with selection/insertion/closure and clamp the tracked offset after layout and resize changes in crates/widget/src/tab_bar.rs.
- [x] T020 [US3] Expand crates/widget/examples/tab_bar.rs to 30 tabs with a resizable viewport and visible first/last/active diagnostics for native wheel validation.

**Checkpoint**: Any tab in the overflow example can be revealed and selected without blank overscroll.

---

## Phase 6: User Story 4 - Operate Tabs Accessibly (Priority: P2)

**Goal**: Provide keyboard and assistive-technology equivalents for tab selection and closure.

**Independent Test**: Navigate first/middle/last tabs without a pointer, select and close them,
then inspect TabList/Tab/Button names, order, selected state, and focus recovery.

- [x] T021 [P] [US4] Write keyboard tests for Left/Right wrap policy, Home/End, Enter/Space selection, close action, and post-close focus in crates/widget/tests/tab_bar_accessibility.rs.
- [ ] T022 [P] [US4] Add GPUI accessibility-tree assertions for TabList, Tab, Button, full names, position/count, selected state, and accessible Click in crates/widget/tests/tab_bar_a11y_tree.rs.
- [x] T023 [US4] Define and initialize tab navigation/select/close GPUI actions and key bindings in crates/widget/src/tab_bar.rs and export the initialization function from crates/widget/src/lib.rs.
- [x] T024 [US4] Add roving focus, keyboard handlers, aria active/selected/position/count data, action-specific close names, focus-visible styling, and accessible select/close activation in crates/widget/src/tab_bar.rs.
- [x] T025 [US4] Initialize tab-bar bindings and add keyboard/accessibility instructions to crates/widget/examples/tab_bar.rs.

**Checkpoint**: All tab navigation and closure is available through pointer, keyboard, and accessibility APIs.

---

## Phase 7: Polish and Cross-Cutting Validation

**Purpose**: Verify appearance, performance, public documentation, and workspace compatibility.

- [x] T026 [P] Add rustdoc examples and behavioral guarantees for TabBar, TabId, TabSpec, events, close veto, and asset-source expectations in crates/widget/src/tab_bar.rs and crates/widget/src/lib.rs.
- [x] T027 Validate text_xs, active/inactive/hover/focus styles, SVG/name/X non-overlap, light/dark appearance, supported scales, and sub-100-ms feedback; record results and native gaps in specs/002-component-tab-bar/validation/acceptance.md.
- [x] T028 Run every command in specs/002-component-tab-bar/quickstart.md, fix implementation defects, and record toolchain, GPUI revision, test counts, Clippy, formatting, and platform results in specs/002-component-tab-bar/validation/build.md.
- [x] T029 Reconcile FR-001?밊R-020 and SC-001?밪C-008 against tests and native evidence, retaining unavailable native checks as NOT RUN in specs/002-component-tab-bar/validation/acceptance.md.

## Dependencies and Execution Order

```text
Setup T001?밫003
  -> Foundation T004?밫007
     -> US1 T008?밫011 (MVP)
        -> US2 T012?밫016
           -> US3 T017?밫020
           -> US4 T021?밫025
              -> Polish T026?밫029
```

- US1 must precede US2 because closure selects another page through the US1 transition.
- US2 must precede US3 because removal changes measured child order and scroll bounds.
- After US2, US3 and US4 may proceed independently; both edit `tab_bar.rs`, so sequential execution
  is the safe default for one working tree.
- Test tasks marked `[P]` may run together before their corresponding implementation tasks.

## Parallel Execution Examples

- Setup: T002 assets and T003 example skeleton can proceed after T001 without touching the same files.
- US2: T012 state contract and T013 pointer contract use separate test targets.
- After US2: T017 scroll tests and T021/T022 accessibility tests can be authored independently.
- Polish: T026 documentation may proceed while native acceptance evidence is collected for T027.

## Implementation Strategy

1. Complete setup and foundation once, keeping existing TextInput/TextEditor APIs unchanged.
2. Deliver US1 as the smallest useful tab-to-page navigation component.
3. Add US2 to make the P1 component operational for project-page management.
4. Add overflow and accessibility independently after the P1 behavior is stable.
5. Run deterministic tests before native visual/input checks; never label unavailable platform evidence PASS.

## Coverage and Counts

| Scope | Tasks | Count |
|---|---|---|
| Setup and foundation | T001?밫007 | 7 |
| US1 | T008?밫011 | 4 |
| US2 | T012?밫016 | 5 |
| US3 | T017?밫020 | 4 |
| US4 | T021?밫025 | 5 |
| Polish | T026?밫029 | 4 |
| **Total** | T001?밫029 | **29** |

