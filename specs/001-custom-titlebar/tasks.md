---
description: "Dependency-ordered tasks for the Zit custom title bar and menu bar"
---

# Tasks: Platform-Specific Custom Title Bar and Menu Bar

**Input**: Design documents from `specs/001-custom-titlebar/`.

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md),
[data-model.md](data-model.md), [UI contract](contracts/application-shell.md), and [quickstart.md](quickstart.md).

**Validation**: The specification requires acceptance, accessibility, state, failure-injection,
and visual checks. Each story includes execution and evidence tasks; no blanket TDD requirement
or redundant implementation-mirroring test suite is introduced.

**Organization**: Preserve story IDs and execute priorities US1 (P1), US2 (P1), US4 (P1), US3 (P2).
All tasks start pending. This document does not certify native feasibility or reference capture.

## Format: `[ID] [P?] [Story] Description`

- `[P]` identifies disjoint work that can run together after the stated prerequisites.
- Story-phase tasks carry their specification story ID; infrastructure and final tasks do not.
- Paths are repository-relative. Source resides under crates/; feature evidence resides here.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the constitutional workspace and resolve actual dependency compatibility.

- [X] T001 Create or extend root Cargo.toml and crates/zit/Cargo.toml with Rust edition 2024, preserve crates/widget as a workspace member, declare the exact official GPUI Git dependency and same-source gpui_platform desktop features, and include gix in zit.
- [X] T002 Add the initial compilable zit executable at crates/zit/src/main.rs, start it directly with gpui_platform::application(), and open a GPUI window without introducing a shell library or alternate UI framework.
- [X] T003 Resolve the official GPUI revision and compatible toolchain starting from plan candidates, write rust-toolchain.toml and Cargo.lock, run cargo metadata and dependency-source checks, and record actual versions/build prerequisites in specs/001-custom-titlebar/research.md.

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Close native compatibility and reference-image gaps before production story work.
A missing native runner leaves the corresponding gate pending, not passed. Setup and bounded
prototypes may proceed; final production story work waits for T012.

- [X] T004 Define shared semantic commands, capability flags, WindowSnapshot, and platform profile selection from data-model.md in crates/zit/src/platform.rs and crates/zit/src/shell.rs.
- [ ] T005 Build a temporary GPUI-only native capability harness at crates/zit/examples/chrome_probe.rs using the T004 boundary to exercise custom decorations, window actions, accessibility, and window-independent startup error presentation.
- [ ] T006 Prototype Windows hit testing, empty-region drag, resize borders, state queries, shortcuts, and visible startup failure through GPUI facilities in crates/zit/src/platform.rs; record native results in specs/001-custom-titlebar/validation/windows-probe.md.
- [ ] T007 Prototype Linux client decorations for X11 and Wayland, drag/resize, minimize/maximize, external state observation, accessibility and startup error behavior in crates/zit/src/platform.rs; record compositor-specific results in specs/001-custom-titlebar/validation/linux-probe.md.
- [ ] T008 Prototype macOS transparent chrome with preserved resize/minimize/fullscreen capabilities and hidden native traffic lights in crates/zit/src/platform.rs; if needed add compatible target-only AppKit dependencies in crates/zit/Cargo.toml, isolate main-thread raw-handle use, and record fullscreen/reopen/accessibility/error results in specs/001-custom-titlebar/validation/macos-probe.md.
- [ ] T009 [P] Capture Windows native reference states for the research.md profile into specs/001-custom-titlebar/references/windows/ and write its manifest.json with exact build, theme, scale, geometry, provenance and SHA-256 hashes.
- [ ] T010 [P] Capture macOS native reference states for the research.md profile into specs/001-custom-titlebar/references/macos/ and write its manifest.json with exact build, theme, backing scale, geometry, provenance and SHA-256 hashes; explicitly record unsupported scales.
- [ ] T011 [P] Capture GNOME/Adwaita native reference states for X11 and Wayland into specs/001-custom-titlebar/references/linux/ and write its manifest.json with exact package versions, session, theme, scales, geometry, provenance and SHA-256 hashes.
- [ ] T012 Review T006–T011 evidence against research.md, record resolved capabilities and reference identities in specs/001-custom-titlebar/plan.md and specs/001-custom-titlebar/research.md, and close their planning evidence gap only when supported by actual captures and successful native probes; retain unresolved failures as blockers.

**Checkpoint**: No GPUI fork, native-control substitution, guessed reference pixels, or fabricated
native PASS result is allowed. All three reference profiles and both Linux sessions must be accounted for.

## Phase 3: User Story 1 - Open and Move the Application Window (Priority: P1)

**Goal**: Launch the application shell, display File/Help heading positions instead of title text,
and move/resize the main window without a repository.

**Independent Test**: Launch the shell without repository state, inspect one custom title bar,
drag empty space, resize edges/corners, and exercise the injected startup failure path.

- [X] T013 [P] [US1] Implement the GPUI root entity and empty content area in crates/zit/src/shell.rs with initial 1024 x 720 and minimum 480 x 320 logical dimensions and entity-owned window subscriptions.
- [ ] T014 [P] [US1] Implement the platform-ordered title-bar layout in crates/zit/src/titlebar.rs with File/Help heading slots, window-control slots, and at least 64 logical pixels of noninteractive draggable space; exclude interactive regions from caption hit testing.
- [ ] T015 [US1] Connect shell, title bar, and validated platform policy directly through crates/zit/src/main.rs; open exactly one window through gpui_platform::application(), retain Zit as OS metadata, handle activation, and route startup failure to the proven visible error path with failure exit.
- [ ] T016 [US1] Implement normal-window edge/corner resizing and configured empty-titlebar double-click behavior in crates/zit/src/platform.rs, ensuring title-bar controls and heading slots cannot trigger move or double-click actions; retain GPUI zoom for Windows maximize and the bounded raw-handle `SW_RESTORE` adapter for restore.
- [ ] T017 [US1] Validate launch, drag/resize, minimum dimensions and injected failure scenarios, using crates/zit/tests/window_contract.rs for observable lifecycle coverage and recording native evidence in specs/001-custom-titlebar/validation/us1.md.

**Checkpoint**: US1 is the initial shell increment. Heading slots become functional menus in US4;
final SVG controls arrive in US2. This increment is not acceptance of the complete feature.

## Phase 4: User Story 2 - Use Familiar Platform Window Controls (Priority: P1)

**Goal**: Provide actual window operations with native-matching SVG controls and authoritative state.

**Independent Test**: Compare each platform's controls to its frozen references and exercise
minimize/reactivate, maximize/restore or fullscreen, external state changes, and Close.

- [ ] T018 [P] [US2] Author Windows minimize/maximize/restore/close SVG assets and state styles against T009 references in crates/zit/assets/window-controls/windows/; record geometry and licensing in its manifest.json.
- [ ] T019 [P] [US2] Author GNOME/Adwaita minimize/maximize/restore/close SVG assets and state styles against T011 references in crates/zit/assets/window-controls/linux/; record geometry and licensing in its manifest.json.
- [ ] T020 [P] [US2] Author macOS traffic-light shapes and state-dependent close/minimize/fullscreen symbols against T010 references in crates/zit/assets/window-controls/macos/; record geometry and licensing in its manifest.json.
- [ ] T021 [US2] Implement bundled asset lookup, platform/state selection, missing-asset errors and GPUI SVG rendering inputs in crates/zit/src/assets.rs after T018–T020.
- [X] T022 [US2] Render and wire window controls in crates/zit/src/titlebar.rs using actual WindowSnapshot and capability guards; handle press cancellation and propagation, and refresh maximize/restore/fullscreen presentation after OS-initiated changes through crates/zit/src/platform.rs.
- [ ] T023 [US2] Implement last-window lifecycle in crates/zit/src/main.rs: Windows/Linux close quits, macOS close may retain the process and activation reopens a usable window; preserve operating-system window shortcuts.
- [ ] T024 [US2] Extend crates/zit/tests/window_contract.rs for command/state/cancellation boundaries and execute native US2 actions plus attribute-by-attribute reference comparisons; record actual results and remaining gaps in specs/001-custom-titlebar/validation/us2.md.

## Phase 5: User Story 4 - Access File and Help Menus (Priority: P1)

**Goal**: Replace heading slots with accessible in-window menus, functional Exit and About, and
disabled repository commands.

**Independent Test**: Check exact labels/order, pointer and keyboard navigation, one-open-menu
behavior, dismissal, rejected disabled commands, About focus restoration, and process-level Exit.

- [X] T025 [P] [US4] Implement immutable File/Help definitions, MenuState and enabled-command dispatch in crates/zit/src/menu.rs with exact File labels New Repository, Open ..., Clone ..., Exit and Help item About; reject the first three commands before host dispatch.
- [ ] T026 [P] [US4] Implement a single modal About overlay in crates/zit/src/about.rs showing Zit and the host-supplied package version, Escape/Close dismissal, focus containment, return-focus fallback and suppression of underlying input.
- [X] T027 [US4] Implement pointer opening, atomic heading switching, bounded popups, outside-click consumption and blur dismissal in crates/zit/src/menu.rs; integrate real headings into crates/zit/src/titlebar.rs without exposing drag regions beneath popups.
- [ ] T028 [US4] Add Tab entry, Left/Right heading switching, Down/Enter opening, Up/Down enabled-item navigation, Enter activation, Escape restoration and menu/menuitem accessible roles, names and states in crates/zit/src/menu.rs.
- [X] T029 [US4] Wire ShowAbout and ExitApplication into crates/zit/src/shell.rs and crates/zit/src/main.rs, close menus before command execution, supply package version, and ensure Exit quits the application on every platform.
- [ ] T030 [US4] Validate the menu state machine and integration in crates/zit/tests/menu_contract.rs, run native Story 4 scenarios including process-level Exit and assistive activation, and record evidence in specs/001-custom-titlebar/validation/us4.md.

## Phase 6: User Story 3 - Use Controls Across Display and Input Conditions (Priority: P2)

**Goal**: Preserve clarity, hit areas and accessible operation across appearance, scale and focus changes.

**Independent Test**: Change appearance and focus, move between different-scale monitors, shrink
to minimum size, cancel presses and operate controls using native assistive technology.

- [ ] T031 [P] [US3] Implement active/inactive and light/dark style selection plus scale-sensitive asset metrics in crates/zit/src/assets.rs using the frozen platform profiles, preserving SVG geometry and pixel-aligned interactive areas; use GPUI `text_xs`, appearance-aware lightened title-bar/popup surfaces, and pointer plus background hover feedback for all menu rows.
- [ ] T032 [P] [US3] Implement entity-lifetime appearance, focus and scale subscriptions in crates/zit/src/shell.rs; clear stale pressed/popup state on blur and refresh layout without polling or stealing external focus.
- [ ] T033 [US3] Add button roles, current action labels, disabled states and assistive activation to crates/zit/src/titlebar.rs and dialog semantics to crates/zit/src/about.rs; preserve minimum-size menu/control space and reject unsupported actions.
- [ ] T034 [US3] Validate semantic roles/actions and focus behavior in crates/zit/tests/accessibility.rs, then execute Narrator, VoiceOver and Orca scenarios with minimum-size and multi-monitor scale checks; record results in specs/001-custom-titlebar/validation/us3.md.

## Phase 7: Polish and Cross-Cutting Concerns

- [X] T035 Run cargo metadata, GPUI/gix source checks, formatting, Clippy and workspace tests from specs/001-custom-titlebar/quickstart.md on applicable targets, fix identified implementation defects, and record commands/toolchain/lock revision and results in specs/001-custom-titlebar/validation/build.md.
- [ ] T036 Execute the full native acceptance matrix from specs/001-custom-titlebar/quickstart.md including ten launches per target, reference comparison, input cancellation, startup failure, all supported scales and menu/hover latency; record SC-001–SC-008 evidence and NOT RUN gaps in specs/001-custom-titlebar/validation/acceptance.md.
- [ ] T037 Reconcile FR-001–FR-020 and SC-001–SC-009 against evidence in specs/001-custom-titlebar/validation/acceptance.md, update specs/001-custom-titlebar/checklists/requirements.md and specs/001-custom-titlebar/quickstart.md with actual outcomes, and confirm Constitution v1.2.0 compliance without treating missing native evidence as passed.

## Dependencies and Execution Order

```text
T001 -> T002 -> T003 -> T004 -> T005 -> T006 -> T007 -> T008
                                                then T009 / T010 / T011
all foundation evidence -> T012
T012 -> US1 (T013–T017) -> US2 (T018–T024) -> US4 (T025–T030)
     -> US3 (T031–T034, after US2 and US4) -> Polish (T035–T037)
```

US2 and US4 each use the completed US1 shell; they can be verified independently of one another,
but their integration edits share titlebar.rs and main.rs, so the default schedule is sequential.
US3 validates the integrated controls and menus and therefore follows both. No TextInput or
TextEditor task is a dependency. Within a phase, execute unmarked tasks in order; a marked group
may run together only after all preceding prerequisites are complete.

Native evidence is required before T012 can close. Lack of a runner does not authorize marking
T006–T012 complete or silently reducing platform scope. Already authorized setup, evidence
preparation and prototype work may continue while native access remains unavailable.

## Parallel Execution Examples

- Foundation: after T008, T009/T010/T011 use separate reference directories and native runners.
- US1: T013 shell implementation and T014 title-bar layout can proceed together against T004;
  T015 integrates them only after both finish.
- US2: T018/T019/T020 author separate platform asset sets; T021 consumes all three.
- US4: T025 menu model and T026 About overlay use independent files and the shared host contract;
  T027–T029 perform sequential integration.
- US3: T031 asset styles and T032 shell observations use independent files; T033 follows both.

## Implementation Strategy

1. Complete setup and the evidence-gated foundation first.
2. Deliver US1 as the smallest shell demonstration, explicitly identifying unfinished control/menu behavior.
3. Add US2 and US4 for the useful P1 application shell; retain disabled repository entries.
4. Complete US3 and all native acceptance evidence before declaring the feature complete.
5. Record evidence as work is performed. Do not copy PASS claims from imported component documents.

## Coverage and Counts

| Scope | Tasks | Count | Requirements |
|---|---|---|---|
| Setup and foundation | T001–T012 | 12 | Constitutional setup and platform/reference gates |
| US1 | T013–T017 | 5 | FR-001/002/007/011 |
| US2 | T018–T024 | 7 | FR-003/004/005/006/008 |
| US4 | T025–T030 | 6 | FR-012/013/014/015/016/017 |
| US3 | T031–T034 | 4 | FR-008/009/010/015 |
| Cross-cutting | T035–T037 | 3 | All FRs and SC-001–SC-008 |

Total: 37 tasks. Native checks and behavioral validation remain pending until implementation.

