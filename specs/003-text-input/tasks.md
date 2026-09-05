# Tasks: Reusable TextInput Widget

**Migration status (2026-09-05)**: Imported design; no component implementation exists in Zit.
All tasks are pending verification and implementation in this repository. Previously checked task IDs
from the source project (historical only): T001, T002, T003, T004, T005, T006, T007, T008, T009, T010, T011, T012, T013, T014, T015, T016, T017, T018, T019, T020, T021, T022, T023, T024, T025, T026, T027, T028, T029, T030, T031, T032, T033, T034, T035, T036, T037, T038, T039, T040, T041, T042, T043, T044, T045, T046, T047, T048, T049, T050, T051, T052, T053, T054, T055, T056, T057, T058, T059, T060.

**Input**: Design documents from `/specs/component/001-text-input/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Tests are required by this component implementation plan. Write each
story's tests first and confirm they fail for the intended missing behavior before implementation.

**Organization**: Tasks are grouped by user story. P1 stories are ordered US1, US2, then US6;
P2 stories follow as US3 and US4, with P3 host integration in US5. Tasks T001-T045 define the original implementation; T046-T052 are the cursor-blinking amendment generated from
FR-037 and SC-008. T053-T060 cover immediate pointer-focus caret visibility and minimum-height
behavior from FR-038-FR-039 and SC-009-SC-010.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it changes a different file and has no incomplete-task
  dependency.
- **[Story]**: Maps the task to its user story in `spec.md`.
- Every task identifies the exact file it creates or modifies.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the Zit workspace and reusable widget library, preserving any existing application setup.

- [ ] T001 Create or extend root Cargo.toml with workspace members `crates/zit` and `crates/widget`, the official GPUI Git dependency, and gix; create or preserve the `zit` application manifest at crates/zit/Cargo.toml and executable entry point at crates/zit/src/main.rs
- [ ] T002 [P] Verify the imported Rust 1.95.0 candidate against the selected official GPUI revision and pin the compatible toolchain with rustfmt and clippy in rust-toolchain.toml
- [ ] T003 Create the `widget` library manifest with GPUI, Unicode segmentation, and GPUI test-support configuration in crates/widget/Cargo.toml
- [ ] T004 [P] Create the widget crate export surface and TextInput module declaration in crates/widget/src/lib.rs
- [ ] T005 Generate and retain the official GPUI dependency resolution in Cargo.lock and verify its source with `cargo metadata`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared state, invariants, actions, and rendering/input integration required
by every user story.

**?좑툘 CRITICAL**: No user story implementation begins until this phase is complete.

- [ ] T006 Define TextInput construction, committed/working values, directed selection, enabled state, focus handle, and state invariants in crates/widget/src/text_input.rs
- [ ] T007 Implement newline normalization plus checked UTF-8, UTF-16, and grapheme-boundary conversion helpers in crates/widget/src/text_input.rs
- [ ] T008 Define the TextInput key context and semantic editing, navigation, selection, clipboard, undo, and redo actions in crates/widget/src/text_input.rs
- [ ] T009 Implement the base GPUI Render/Element lifecycle, focus tracking, input-handler registration, one-line shaping, and cached geometry in crates/widget/src/text_input.rs
- [ ] T010 Implement placeholder, enabled, disabled, focused, and unfocused base visual states in crates/widget/src/text_input.rs
- [ ] T011 [P] Add foundational invariant and Unicode conversion tests covering invalid/out-of-range platform offsets in crates/widget/tests/text_input_foundation.rs

**Checkpoint**: The widget can be created, focused, rendered, and queried with valid single-line
state, but story-specific editing behavior is not yet complete.

---

## Phase 3: User Story 1 - Enter and Edit Text (Priority: P1) ?렞 MVP

**Goal**: Users can type, insert, and delete visible single-line text at a clear cursor position.

**Independent Test**: Focus an enabled input, type at the start, middle, and end, use Backspace and
Delete at ordinary and boundary positions, and verify the visible/readable value and caret.

### Tests for User Story 1

- [ ] T012 [P] [US1] Add failing unit tests for insertion, selection replacement, Backspace, Delete, boundary no-ops, and line-break normalization in crates/widget/tests/text_input_editing.rs
- [ ] T013 [P] [US1] Add failing GPUI interaction tests for focus, text input dispatch, caret painting, and caret visibility during horizontal overflow in crates/widget/tests/text_input_rendering.rs

### Implementation for User Story 1

- [ ] T014 [US1] Implement insertion, selection replacement, Backspace, Delete, and committed-value synchronization in crates/widget/src/text_input.rs
- [ ] T015 [US1] Implement caret painting and horizontal viewport adjustment that keeps the active caret visible in crates/widget/src/text_input.rs
- [ ] T016 [US1] Run the US1 tests and record the passing MVP validation commands in specs/component/001-text-input/quickstart.md

**Checkpoint**: US1 is a demonstrable MVP for basic single-line entry and correction.

---

## Phase 4: User Story 2 - Navigate and Select Text (Priority: P1)

**Goal**: Users can position the cursor and create clearly visible selections with standard
keyboard and mouse interactions.

**Independent Test**: Exercise character, word, Home, End, shift-selection, click placement, and
drag selection on ASCII and multi-code-point text and verify every resulting boundary and visual.

### Tests for User Story 2

- [ ] T017 [P] [US2] Add failing tests for character/word/start/end navigation and directed keyboard selection across grapheme clusters in crates/widget/tests/text_input_navigation.rs
- [ ] T018 [P] [US2] Add failing GPUI pointer tests for nearest-boundary click placement, drag direction, and dragging beyond widget bounds in crates/widget/tests/text_input_pointer.rs

### Implementation for User Story 2

- [ ] T019 [US2] Implement character, word, start, and end cursor movement plus selection-extending variants in crates/widget/src/text_input.rs
- [ ] T020 [US2] Implement shaped-line hit testing, click focus/placement, pointer capture state, and drag selection in crates/widget/src/text_input.rs
- [ ] T021 [US2] Paint normalized selection geometry distinctly while preserving directed selection state in crates/widget/src/text_input.rs
- [ ] T022 [US2] Run US2 navigation and pointer tests and confirm the story's acceptance scenarios in crates/widget/tests/text_input_navigation.rs

**Checkpoint**: US2 independently supports precise keyboard and pointer targeting and selection.

---

## Phase 5: User Story 6 - Enter International Text (Priority: P1)

**Goal**: Users can compose, update, commit, and cancel international text through platform IMEs
without corrupting committed text.

**Independent Test**: Drive marked-text callbacks through start, multiple updates, commit, and
cancel in the middle of existing text, checking UTF-16 ranges, selection, display, and notifications.

### Tests for User Story 6

- [ ] T023 [P] [US6] Add failing input-handler contract tests for UTF-16 selected/marked ranges, requested text, range bounds, and point-to-character conversion in crates/widget/tests/text_input_ime.rs
- [ ] T024 [US6] Add failing composition lifecycle tests for start, update, selection replacement, commit, cancel, multiline normalization, and no duplicate text in crates/widget/tests/text_input_ime.rs

### Implementation for User Story 6

- [ ] T025 [US6] Implement the complete GPUI EntityInputHandler range-query, geometry, committed replacement, and marked replacement contract in crates/widget/src/text_input.rs
- [ ] T026 [US6] Implement composition snapshots, provisional marked-text display, commit/cancel transitions, marked underline painting, and commit-only notification in crates/widget/src/text_input.rs
- [ ] T027 [US6] Run automated IME tests and document manual Korean, Japanese, and Chinese composition checks in specs/component/001-text-input/quickstart.md

**Checkpoint**: All P1 stories work, including platform text services and IME composition.

---

## Phase 6: User Story 3 - Replace and Transfer Text (Priority: P2)

**Goal**: Users can replace selections and transfer plain text through standard copy, cut, and
paste commands.

**Independent Test**: Copy and cut forward/reversed selections, then paste over a selection and at
a cursor, verifying clipboard content, normalized value, cursor placement, and disabled no-ops.

### Tests for User Story 3

- [ ] T028 [P] [US3] Add failing GPUI clipboard tests for copy, cut, paste, selection replacement, empty clipboard, line-break normalization, and disabled behavior in crates/widget/tests/text_input_clipboard.rs

### Implementation for User Story 3

- [ ] T029 [US3] Implement plain-text clipboard copy and enabled-only cut behavior through GPUI services in crates/widget/src/text_input.rs
- [ ] T030 [US3] Implement enabled-only paste at the cursor or over selection with single-line normalization in crates/widget/src/text_input.rs
- [ ] T031 [US3] Bind platform-conventional copy, cut, paste, and select-all commands within the TextInput key context in crates/widget/src/text_input.rs

**Checkpoint**: US3 clipboard and replacement workflows pass independently.

---

## Phase 7: User Story 4 - Undo and Redo Changes (Priority: P2)

**Goal**: Users can reverse and restore committed editing transactions with expected desktop
grouping and redo invalidation.

**Independent Test**: Perform typing, deletion, replacement, cut, paste, and IME commit; undo and
redo each transaction while checking value, directed selection, events, coalescing, and redo reset.

### Tests for User Story 4

- [ ] T032 [P] [US4] Add failing transaction tests for snapshots, typing/deletion coalescing, selection restoration, redo invalidation, and programmatic reset in crates/widget/tests/text_input_history.rs
- [ ] T033 [US4] Add failing integration tests covering undo/redo of cut, paste, replacement, and IME commit with exact change-event counts in crates/widget/tests/text_input_history.rs

### Implementation for User Story 4

- [ ] T034 [US4] Implement edit transactions, coalescing boundaries, undo/redo stacks, selection restoration, and new-edit redo invalidation in crates/widget/src/text_input.rs
- [ ] T035 [US4] Integrate every committed typing, deletion, replacement, cut, paste, and IME commit path with history and bind undo/redo actions in crates/widget/src/text_input.rs

**Checkpoint**: US4 provides reversible editing without exposing provisional composition as edits.

---

## Phase 8: User Story 5 - Configure and Observe the Widget (Priority: P3)

**Goal**: Host applications can configure, focus, reuse, read, and observe independent TextInput
instances without embedding business logic in the widget.

**Independent Test**: Render two differently configured inputs, edit each independently, toggle
enabled state, set values programmatically, and verify public reads and exact event behavior.

### Tests for User Story 5

- [ ] T036 [P] [US5] Add failing public-contract tests for construction, placeholder, focus handle, enabled state, value reads/sets, history baseline reset, and change events in crates/widget/tests/text_input_contract.rs
- [ ] T037 [P] [US5] Create a two-instance example host with independent configuration and event display in crates/widget/examples/text_input.rs

### Implementation for User Story 5

- [ ] T038 [US5] Finalize the public TextInput constructors, configuration methods, value accessors, focus access, and programmatic value semantics in crates/widget/src/text_input.rs
- [ ] T039 [US5] Define and emit the public committed-value change event only for value-changing user edits, undo, and redo in crates/widget/src/text_input.rs
- [ ] T040 [US5] Export TextInput, its change event, semantic actions, and one-time action/key-binding registration entry point from crates/widget/src/lib.rs

**Checkpoint**: All six user stories and the public contract are independently verifiable.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Validate the complete widget across quality, platform, compatibility, and reuse gates.

- [ ] T041 [P] Add rustdoc examples and document public behavior, event semantics, disabled behavior, and platform conventions in crates/widget/src/text_input.rs
- [ ] T042 [P] Add regression cases for long values, mixed scripts, combining marks, emoji sequences, bidirectional text, empty operations, and focus loss in crates/widget/tests/text_input_regression.rs
- [ ] T043 Verify formatting, linting, all targets/features, tests, and official GPUI dependency provenance using the commands in specs/component/001-text-input/quickstart.md
- [ ] T044 Run and record manual Windows, macOS, X11, Wayland, display-scale, theme, clipboard, pointer, and IME coverage or explicit environment gaps in specs/component/001-text-input/quickstart.md
- [ ] T045 Trace FR-001 through FR-035 and SC-001 through SC-006 to automated or manual evidence and record final compliance in specs/component/001-text-input/implementation-validation.md

---

## Phase 10: User Story 2 Amendment - Blink the Caret (Priority: P1)

**Goal**: A focused, enabled TextInput with an empty selection displays a steadily blinking caret;
user interaction resets it to immediately visible, while ineligible states stop it safely.

**Independent Test**: Use GPUI's controllable test executor to focus an input, advance across
successive 500 ms boundaries, and verify visible/hidden phases; then exercise typing, navigation,
selection, pointer placement, undo, redo, paste, IME updates, blur, and disablement while checking
cycle reset, stop conditions, stale-generation rejection, and the absence of blink-only value
events.

### Tests for the User Story 2 Amendment

- [ ] T046 [P] [US2] Add failing deterministic tests for initial visibility, 500 ms toggles, interaction reset, focus/selection/enabled eligibility, independent instances, stale timer invalidation, and no blink-only change events in crates/widget/tests/text_input_blink.rs

### Implementation for the User Story 2 Amendment

- [ ] T047 [US2] Add entity-owned caret visibility, blink generation, 500 ms interval, cancellable task, and focus/blur subscriptions to TextInput in crates/widget/src/text_input.rs
- [ ] T048 [US2] Reset the caret to visible and restart or stop its generation-safe cycle from every editing, navigation, selection, pointer, clipboard, history, IME, focus, and enabled-state transition in crates/widget/src/text_input.rs
- [ ] T049 [US2] Gate caret painting on blink visibility while preserving inherited text color, one-pixel width, and display-pixel alignment in crates/widget/src/text_input.rs
- [ ] T050 [US2] Extend the interactive two-instance host with observable focus, selection, enabled, and blink-state scenarios in crates/widget/examples/text_input.rs

**Checkpoint**: FR-037 and SC-008 pass deterministically, and the pre-existing TextInput behavior
remains green.

---

## Phase 11: Cursor Blink Polish & Cross-Cutting Validation

**Purpose**: Complete documentation, regression, and release evidence for the amendment.

- [ ] T051 [P] Document the 500 ms lifecycle, reset triggers, stop conditions, and non-event guarantee in public rustdoc in crates/widget/src/text_input.rs
- [ ] T052 Run formatting, Clippy, all-target compilation, full tests, and manual blink checks, then trace FR-037 and SC-008 evidence in specs/component/001-text-input/implementation-validation.md

---

## Phase 12: User Story 2 Amendment - Show the Caret Immediately on Pointer Focus (Priority: P1)

**Goal**: Ensure clicking an unfocused enabled TextInput shows the caret at the clicked boundary in
the first focused frame, before any blink interval or later interaction.

**Independent Test**: Render an unfocused input, dispatch a primary-button press inside its text
area without advancing the test clock, and verify focus, cursor placement, and visible caret state
in the first resulting render; then advance 500 ms and verify normal blinking continues.

### Tests for the User Story 2 Pointer-Focus Amendment

- [ ] T053 [P] [US2] Add failing GPUI pointer tests for first-frame focus, clicked-boundary placement, immediate caret visibility without clock advancement, and disabled click rejection in crates/widget/tests/text_input_pointer.rs
- [ ] T054 [P] [US2] Add failing timer-regression tests proving pointer focus starts one fresh blink generation and deferred focus notification or stale timers cannot hide the new caret in crates/widget/tests/text_input_blink.rs

### Implementation for the User Story 2 Pointer-Focus Amendment

- [ ] T055 [US2] Make primary-pointer focus, caret placement, visible-phase reset, and blink restart one synchronous generation-safe interaction while preserving keyboard and host-driven focus behavior in crates/widget/src/text_input.rs

**Checkpoint**: FR-039 and SC-010 pass without advancing the clock, and the next 500 ms boundary
continues the ordinary FR-037 blink cycle.

---

## Phase 13: User Story 5 Amendment - Preserve a One-Line Minimum Height (Priority: P3)

**Goal**: Prevent host layout constraints from shrinking TextInput below one complete text line and
its normal vertical spacing and boundary treatment.

**Independent Test**: Render TextInputs with multiple inherited text sizes inside containers that
request smaller heights and verify the resulting control and inner text bounds retain one complete,
unclipped line together with selection and caret geometry.

### Tests for the User Story 5 Minimum-Height Amendment

- [ ] T056 [P] [US5] Add failing constrained-layout tests across multiple text sizes for outer minimum height, inner line height, and unclipped caret and selection geometry in crates/widget/tests/text_input_rendering.rs

### Implementation for the User Story 5 Minimum-Height Amendment

- [ ] T057 [US5] Derive and enforce the outer control minimum height from the active line metrics plus vertical padding and borders, and prevent the inner text element from flex-shrinking below one line in crates/widget/src/text_input.rs
- [ ] T058 [US5] Extend the two-instance example with a constrained-height and enlarged-text scenario for interactive FR-038 validation in crates/widget/examples/text_input.rs

**Checkpoint**: FR-038 and SC-009 pass across the tested text sizes without changing value,
selection, focus, or event behavior.

---

## Phase 14: Pointer-Focus and Layout Polish & Cross-Cutting Validation

**Purpose**: Complete public documentation, full regression gates, and traceability for the two
latest amendments.

- [ ] T059 [P] Document immediate pointer-focus caret visibility and the text-metric-derived minimum-height guarantee in public rustdoc in crates/widget/src/text_input.rs
- [ ] T060 Run formatting, Clippy, all-target compilation, the full automated suite, and manual first-frame/layout checks, then trace FR-038-FR-039 and SC-009-SC-010 in specs/component/001-text-input/implementation-validation.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately. T002 and T004 may run alongside T001; T003 follows the
  workspace decision, and T005 follows manifests.
- **Foundational (Phase 2)**: Depends on Setup and blocks all user stories.
- **P1 stories (Phases 3-5)**: US1 establishes editing, US2 extends its cursor/selection behavior,
  and US6 integrates composition with the same replacement path. Implement in that order for a
  single developer.
- **P2 stories (Phases 6-7)**: US3 depends on selection/replacement; US4 depends on all committed
  edit paths so its history coverage is complete.
- **P3 story (Phase 8)**: US5 finalizes and verifies the accumulated public contract.
- **Polish (Phase 9)**: Depends on all original stories selected for release.
- **Caret Blink Amendment (Phase 10)**: Depends on completed US1 and US2 caret, focus, selection,
  rendering, and interaction behavior. T046 precedes T047-T050.
- **Blink Polish (Phase 11)**: Depends on the Phase 10 checkpoint.
- **Pointer-Focus Amendment (Phase 12)**: Depends on the completed caret blink and pointer
  foundations. T053-T054 precede T055.
- **Minimum-Height Amendment (Phase 13)**: Depends on the base rendering and host integration
  foundations. T056 precedes T057-T058 and may be prepared alongside Phase 12 tests.
- **Latest Amendment Polish (Phase 14)**: Depends on both Phase 12 and Phase 13 checkpoints.

### User Story Dependency Graph

```text
Setup ??Foundation ??US1 ??р넂 US2 ??US6
                          ?붴넂 US3 ??р넂 US4
                                  ?붴넂 US5
All completed stories ??Polish
```

- **US1 (P1)**: Starts after Foundation; basic editing MVP.
- **US2 (P1)**: Uses US1's cursor and replacement state but is independently validated for
  navigation and selection.
- **US6 (P1)**: Uses US1/US2 range mechanics; independently validated through the platform input
  contract.
- **US3 (P2)**: Uses US1 replacement and US2 selection; independently validated with GPUI clipboard.
- **US4 (P2)**: Follows all value-changing paths so one history model covers them consistently.
- **US5 (P3)**: Finalizes the host-facing contract after behavioral slices stabilize.
- **US2 amendment (P1)**: Extends the completed US2 caret presentation without changing its text
  or selection contract; deterministic time-based tests isolate the new behavior.
- **US2 pointer-focus amendment (P1)**: Extends the same focus and blink lifecycle so the first
  click-produced frame is independently testable before timer advancement.
- **US5 minimum-height amendment (P3)**: Extends reusable host layout behavior without depending on
  value editing, clipboard, history, or IME behavior.

### Within Each User Story

- Add tests and confirm they fail for missing behavior before implementation.
- Implement state transitions before GPUI event/render integration that consumes them.
- Run the story-specific suite before declaring the checkpoint complete.
- Do not begin a dependent story while its prerequisite checkpoint is failing.

### Parallel Opportunities

- T002 and T004 can run in parallel after the intended workspace paths are known.
- T011 can be prepared while foundational rendering work proceeds after T006 and T007 stabilize.
- Within US1, T012 and T013 target different test files and can run in parallel.
- Within US2, T017 and T018 target different test files and can run in parallel.
- Contract/lifecycle test design can be split within US6, although T023 and T024 share a file and
  must be coordinated rather than edited concurrently.
- T036 and T037 can run in parallel because contract tests and the example are separate files.
- T041 and T042 can run in parallel after all public behavior is complete.
- T046 can be prepared independently in its own test file before blink implementation begins.
- T051 can proceed alongside example refinement after the Phase 10 public behavior stabilizes.
- T053 and T054 target separate pointer and blink test files and can run in parallel.
- T056 targets rendering tests and can be prepared alongside T053-T054.
- After both amendment implementations stabilize, T059 documentation can proceed while T060's
  validation evidence is prepared in a separate file.

---

## Parallel Examples

### User Story 1

```text
Task T012: Add editing-transition tests in crates/widget/tests/text_input_editing.rs
Task T013: Add GPUI caret/render tests in crates/widget/tests/text_input_rendering.rs
```

### User Story 2

```text
Task T017: Add keyboard navigation tests in crates/widget/tests/text_input_navigation.rs
Task T018: Add pointer interaction tests in crates/widget/tests/text_input_pointer.rs
```

### User Story 5

```text
Task T036: Add public-contract tests in crates/widget/tests/text_input_contract.rs
Task T037: Create the reusable two-instance host in crates/widget/examples/text_input.rs
```

### User Story 2 Cursor Blink Amendment

```text
Task T046: Add deterministic blink lifecycle tests in crates/widget/tests/text_input_blink.rs
Task T051: Document the blink contract in crates/widget/src/text_input.rs after implementation stabilizes
```

### User Story 2 Pointer-Focus Amendment

```text
Task T053: Add first-frame pointer-focus tests in crates/widget/tests/text_input_pointer.rs
Task T054: Add pointer-focus blink generation tests in crates/widget/tests/text_input_blink.rs
```

### User Story 5 Minimum-Height Amendment

```text
Task T056: Add constrained-height rendering tests in crates/widget/tests/text_input_rendering.rs
Task T058: Add interactive constrained-height scenarios in crates/widget/examples/text_input.rs after T057
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Setup and Foundational phases.
2. Write and fail T012-T013.
3. Complete T014-T015 and pass US1 tests.
4. Complete T016 and stop for independent MVP validation.

### Incremental Delivery

1. **US1**: Basic entry, insertion, deletion, caret, and overflow behavior.
2. **US2**: Precise keyboard/pointer navigation and selection.
3. **US6**: International input and composition, completing the P1 baseline.
4. **US3**: Clipboard transfer and selection replacement.
5. **US4**: Undo and redo across all committed editing paths.
6. **US5**: Stable reusable host-facing configuration and observation contract.
7. **Polish**: Cross-platform, Unicode, performance, documentation, and traceability validation.
8. **US2 amendment**: Deterministic caret blinking, reset, stop, and stale-timer behavior.
9. **Blink polish**: Full regression gates and FR-037/SC-008 traceability.
10. **US2 pointer-focus amendment**: First-frame caret visibility and generation-safe blink restart.
11. **US5 minimum-height amendment**: Text-metric-derived layout floor and interactive example.
12. **Latest amendment polish**: Full regression gates and FR-038-FR-039/SC-009-SC-010 traceability.

### Parallel Team Strategy

After Foundation, keep edits to `crates/widget/src/text_input.rs` owned by one integration stream
while other contributors prepare story-specific tests and the example in separate files. Merge
tests before each corresponding implementation task, preserve story order for shared state, and
use the checkpoints as integration boundaries.

---

## Notes

- `[P]` marks only tasks safe to execute concurrently without editing the same file.
- Story labels map exactly to `spec.md`, including the P1 IME story `[US6]`.
- The official GPUI Git dependency is a release gate, not a replaceable implementation detail.
- Commit after each task or cohesive red/green task group.
- Stop at checkpoints to keep failures localized to the current story.

