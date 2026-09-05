---
description: "Dependency-ordered implementation tasks for the reusable TextEditor widget"
---

# Tasks: Reusable TextEditor Widget

**Migration status (2026-09-05)**: Imported design; no component implementation exists in Zit.
All tasks are pending verification and implementation in this repository. Previously checked task IDs
from the source project (historical only): T001, T002, T003, T004, T008, T009, T010, T011.

**Input**: Design documents from `/specs/component/002-text-editor/`

**Prerequisites**: Complete the workspace and TextInput baseline in `specs/component/001-text-input/tasks.md` before these tasks. Required documents: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/public-api.md`

**Tests**: Included because this implementation plan requires focused verification for reusable
widgets. Within each story, write the listed tests first and confirm they fail before implementation.

**Organization**: Tasks are grouped by user story in priority order. Story labels preserve the
numbers in `spec.md`, so P1 User Story 5 precedes P2 User Story 4.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel with adjacent marked tasks because it targets a different file and
  has no dependency on incomplete work in that group.
- **[Story]**: Maps the task to its specification user story.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the module, public entry points, example target, and test scaffolding.

- [ ] T001 Add the private `text_editor` module and initial public re-export placeholders in `crates/widget/src/lib.rs`
- [ ] T002 Create the compilable `TextEditor` entity shell, idempotent key-binding initializer, focus handle, and `Render` shell in `crates/widget/src/text_editor.rs`
- [ ] T003 [P] Register the `text_editor` example target in `crates/widget/Cargo.toml`
- [ ] T004 [P] Create a minimal two-instance TextEditor demonstration shell in `crates/widget/examples/text_editor.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Implement shared invariants and geometry primitives required by every story.

**Critical**: No user-story implementation starts until this phase passes its focused tests.

- [ ] T005 [P] Add public construction/default-state and per-instance independence tests in `crates/widget/tests/text_editor_contract.rs`
- [ ] T006 [P] Add Unicode offset, logical-line index, trailing-newline, and blank-line invariant tests in `crates/widget/tests/text_editor_regression.rs`
- [ ] T007 Implement committed UTF-8 document storage, logical-line indexing, normalized byte positions, directed selection, and external `set_value` semantics in `crates/widget/src/text_editor.rs`
- [ ] T008 Implement grapheme-boundary traversal plus centralized UTF-8/UTF-16 offset and range conversion helpers in `crates/widget/src/text_editor.rs`
- [ ] T009 Implement shared snapshots, mutation permission checks, focus subscriptions, caret blink state, and exactly-once `TextEditorChange` emission primitives in `crates/widget/src/text_editor.rs`
- [ ] T010 Implement the custom element layout/prepaint/paint skeleton, clipped viewport bounds, base text styling, and `ElementInputHandler` registration in `crates/widget/src/text_editor.rs`
- [ ] T011 Export the concrete `TextEditor`, `TextEditorChange`, initializer, and foundational action types from `crates/widget/src/lib.rs`
- [ ] T012 Run the foundational contract/regression tests and resolve failures in `crates/widget/tests/text_editor_contract.rs` and `crates/widget/tests/text_editor_regression.rs`

**Checkpoint**: Empty/populated editors construct, expose valid line/selection state, render a
focusable shell, and preserve existing TextInput tests.

---

## Phase 3: User Story 1 - Enter and Edit Multi-line Text (Priority: P1) — MVP

**Goal**: Enter, insert, replace, and delete plain text across logical lines.

**Independent Test**: Focus an enabled editor, create multiple and blank lines, insert at arbitrary
positions, and delete characters, line boundaries, and cross-line selections.

### Tests for User Story 1

- [ ] T013 [US1] Add failing multi-line insertion, newline, selection replacement, backward-delete, and forward-delete GPUI tests in `crates/widget/tests/text_editor_editing.rs`
- [ ] T014 [P] [US1] Add failing mixed-script grapheme editing and document-boundary no-op cases in `crates/widget/tests/text_editor_regression.rs`

### Implementation for User Story 1

- [ ] T015 [US1] Implement editable insertion, newline insertion, range replacement, backward deletion, and forward deletion with logical-line index maintenance in `crates/widget/src/text_editor.rs`
- [ ] T016 [US1] Implement edit action bindings and platform `replace_text_in_range` behavior while preserving line breaks in `crates/widget/src/text_editor.rs`
- [ ] T017 [US1] Paint logical lines, selection spans, and caret geometry for unwrapped visible content in `crates/widget/src/text_editor.rs`
- [ ] T018 [US1] Complete the editable multi-line demonstration and live value display in `crates/widget/examples/text_editor.rs`

**Checkpoint**: User Story 1 works independently as a multi-line editing MVP.

---

## Phase 4: User Story 2 - Navigate and Select Precisely (Priority: P1)

**Goal**: Provide complete keyboard and pointer cursor/selection behavior across visual lines.

**Independent Test**: Exercise all specified movement and selection commands on unequal, blank,
long, and wrapped lines and verify exact ranges and pointer hit results.

### Tests for User Story 2

- [ ] T019 [P] [US2] Add failing horizontal, word, line, document, vertical, and page navigation tests in `crates/widget/tests/text_editor_navigation.rs`
- [ ] T020 [P] [US2] Add failing click, drag, out-of-viewport drag, word selection, and Select All tests in `crates/widget/tests/text_editor_pointer.rs`

### Implementation for User Story 2

- [ ] T021 [US2] Implement canonical offset/logical-row/visual-line/viewport-point mappings and preferred horizontal position tracking in `crates/widget/src/text_editor.rs`
- [ ] T022 [US2] Implement Left/Right, word, Up/Down, visual Home/End, document Home/End, Page Up/Page Down, and every Shift-selection variant in `crates/widget/src/text_editor.rs`
- [ ] T023 [US2] Implement pointer hit testing, focus-and-place, directed drag selection, word selection, Select All, and drag autoscroll in `crates/widget/src/text_editor.rs`
- [ ] T024 [US2] Register platform-conventional TextEditor navigation and selection key bindings without altering TextInput bindings in `crates/widget/src/text_editor.rs`

**Checkpoint**: User Story 2 independently supports precise navigation and selection.

---

## Phase 5: User Story 3 - Transfer and Recover Content (Priority: P1)

**Goal**: Copy/cut/paste multi-line text and reverse/restore completed edits safely.

**Independent Test**: Transfer a cross-line selection, execute all mutation kinds, walk undo/redo
stacks, and branch history while checking value, selection, and event count.

### Tests for User Story 3

- [ ] T025 [P] [US3] Add failing multi-line copy, cut, paste, empty-selection, and unavailable-clipboard tests in `crates/widget/tests/text_editor_clipboard.rs`
- [ ] T026 [P] [US3] Add failing snapshot restoration, redo invalidation, no-op, and exactly-once event tests in `crates/widget/tests/text_editor_history.rs`

### Implementation for User Story 3

- [ ] T027 [US3] Implement plain-text Copy, editable Cut/Paste, multi-line preservation, and clipboard action bindings in `crates/widget/src/text_editor.rs`
- [ ] T028 [US3] Implement undo/redo snapshot transfer for all completed mutation types with selection restoration and redo invalidation in `crates/widget/src/text_editor.rs`
- [ ] T029 [US3] Audit every mutation path for one history entry and exactly one `TextEditorChange` event in `crates/widget/src/text_editor.rs`

**Checkpoint**: User Story 3 independently supports transfer and recovery without false events.

---

## Phase 6: User Story 5 - Enter International Text (Priority: P1)

**Goal**: Support provisional, committed, and cancelled host-platform IME composition.

**Independent Test**: Compose, revise, cancel, and commit international text in empty/middle/cross-
line-selection positions and verify committed value, selection, bounds, history, and event count.

### Tests for User Story 5

- [ ] T030 [US5] Add failing IME start/update/commit/cancel, selection replacement, UTF-16 conversion, and exactly-once tests in `crates/widget/tests/text_editor_ime.rs`
- [ ] T031 [P] [US5] Add failing combining-character, emoji, Korean, and mixed-direction composition regressions in `crates/widget/tests/text_editor_regression.rs`

### Implementation for User Story 5

- [ ] T032 [US5] Implement provisional composition state, marked-range replacement, cancellation restoration, and one-step commit history in `crates/widget/src/text_editor.rs`
- [ ] T033 [US5] Complete `EntityInputHandler` selected/marked range reporting, UTF-16 replacement conversion, unmarking, and visible `bounds_for_range` in `crates/widget/src/text_editor.rs`
- [ ] T034 [US5] Paint composition text and underline distinctly and keep its active position visible in `crates/widget/src/text_editor.rs`

**Checkpoint**: All P1 stories work together; this is the recommended full core editor release.

---

## Phase 7: User Story 4 - Read and Edit Long Documents (Priority: P2)

**Goal**: Provide wrapping, two-axis scrolling, page movement, reflow, and viewport-scale rendering.

**Independent Test**: Toggle wrapping on oversized content, resize, scroll to every edge, navigate
off-screen, and verify clamped offsets, cursor reveal, unchanged value, and bounded layout work.

### Tests for User Story 4

- [ ] T035 [P] [US4] Add failing vertical/horizontal wheel, clamp, cursor reveal, page movement, and resize tests in `crates/widget/tests/text_editor_scrolling.rs`
- [ ] T036 [P] [US4] Add failing wrap-fragment, selection/caret geometry, clipping, and reflow tests in `crates/widget/tests/text_editor_rendering.rs`
- [ ] T037 [P] [US4] Add a 10,000-line/1,000,000-character visible-working-set and 100 ms interaction acceptance test in `crates/widget/tests/text_editor_regression.rs`

### Implementation for User Story 4

- [ ] T038 [US4] Implement visible-plus-overscan visual-line shaping/cache and invalidation for text, style, width, and wrap changes in `crates/widget/src/text_editor.rs`
- [ ] T039 [US4] Implement wrapped fragment ranges and shared painting/hit-testing/navigation geometry in `crates/widget/src/text_editor.rs`
- [ ] T040 [US4] Implement clamped vertical scrolling, no-wrap horizontal scrolling, wrap-mode horizontal reset, viewport reflow, and minimal cursor reveal in `crates/widget/src/text_editor.rs`
- [ ] T041 [US4] Update Page Up/Page Down and out-of-bounds drag selection to use measured viewport height and visual-line progress in `crates/widget/src/text_editor.rs`
- [ ] T042 [US4] Add wrap toggles and oversized-document scroll demonstrations in `crates/widget/examples/text_editor.rs`

**Checkpoint**: User Story 4 independently validates long-document viewport behavior.

---

## Phase 8: User Story 6 - Configure Interaction States (Priority: P2)

**Goal**: Expose placeholder, wrap, read-only, disabled, host value, and change-event contracts.

**Independent Test**: Configure multiple instances, attempt every input category in each state, and
verify presentation, allowed operations, composition cancellation, and notification cardinality.

### Tests for User Story 6

- [ ] T043 [P] [US6] Add failing public placeholder/wrap/read-only/disabled/value/event contract tests in `crates/widget/tests/text_editor_contract.rs`
- [ ] T044 [P] [US6] Add failing focused/unfocused/read-only/disabled/placeholder visual tests in `crates/widget/tests/text_editor_rendering.rs`
- [ ] T045 [P] [US6] Add failing pointer/focus rejection tests for disabled state in `crates/widget/tests/text_editor_pointer.rs`

### Implementation for User Story 6

- [ ] T046 [US6] Complete public placeholder, wrap, read-only, enabled, value, selection, and state query methods per `specs/component/002-text-editor/contracts/public-api.md` in `crates/widget/src/text_editor.rs`
- [ ] T047 [US6] Centralize read-only versus disabled command permissions and composition/state transitions in `crates/widget/src/text_editor.rs`
- [ ] T048 [US6] Implement placeholder and distinct focus/read-only/disabled presentation without selectable placeholder content in `crates/widget/src/text_editor.rs`
- [ ] T049 [US6] Add interactive read-only/disabled/placeholder controls for independent instances in `crates/widget/examples/text_editor.rs`

**Checkpoint**: All six stories and the complete public contract are independently testable.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Verify compatibility, quality, documentation, and measurable outcomes across stories.

- [ ] T050 [P] Add rustdoc examples and document platform conventions, ranges, event semantics, and state behavior in `crates/widget/src/text_editor.rs`
- [ ] T051 [P] Audit and finalize crate-root exports and initialization documentation in `crates/widget/src/lib.rs`
- [ ] T052 Run `cargo fmt --all -- --check` and resolve formatting issues in `crates/widget/src/text_editor.rs`, `crates/widget/src/lib.rs`, `crates/widget/examples/text_editor.rs`, and `crates/widget/tests/text_editor_*.rs`
- [ ] T053 Run `cargo test --workspace` and resolve TextEditor failures and TextInput regressions in `crates/widget/src/text_editor.rs` and `crates/widget/tests/`
- [ ] T054 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and resolve diagnostics in `crates/widget/`
- [ ] T055 Execute every scenario in `specs/component/002-text-editor/quickstart.md` and record any platform-specific limitations in `specs/component/002-text-editor/quickstart.md`
- [ ] T056 Review implementation against FR-001 through FR-029 and SC-001 through SC-008, documenting final evidence in `specs/component/002-text-editor/checklists/requirements.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Setup has no dependency.
- Foundation depends on Setup and blocks every user story.
- P1 sequence: US1 supplies mutation primitives; US2 supplies geometry/navigation; US3 builds on
  mutations/selection; US5 builds on replacement, history, geometry, and viewport reveal.
- P2 sequence: US4 completes wrapping/scrolling after core geometry; US6 finalizes public state
  controls over all interaction paths.
- Polish depends on all stories selected for release.

### User Story Dependency Graph

```text
Setup -> Foundation -> US1 -> US2 -> US3 -> US5 -> US4 -> US6 -> Polish
```

This is the safest single-developer order. Test-file preparation marked `[P]` can overlap, and the
stories retain their own independent acceptance checkpoints even where they extend shared editor
internals.

### Parallel Opportunities

- T003 and T004 can proceed in parallel after the entity shell contract is understood.
- T005 and T006 can be authored in parallel before T007-T011.
- Within each story, `[P]` test tasks target separate files and can be authored concurrently before
  implementation.
- T050 and T051 can proceed in parallel after the public API stabilizes.
- Format, test, and clippy commands remain sequential because they inspect the same working tree and
  fixes can overlap.

## Parallel Examples

### US2

```text
Task T019: Write keyboard navigation tests in crates/widget/tests/text_editor_navigation.rs
Task T020: Write pointer selection tests in crates/widget/tests/text_editor_pointer.rs
```

### US3

```text
Task T025: Write clipboard tests in crates/widget/tests/text_editor_clipboard.rs
Task T026: Write history/event tests in crates/widget/tests/text_editor_history.rs
```

### US4

```text
Task T035: Write scrolling tests in crates/widget/tests/text_editor_scrolling.rs
Task T036: Write wrap/render tests in crates/widget/tests/text_editor_rendering.rs
Task T037: Write large-document acceptance tests in crates/widget/tests/text_editor_regression.rs
```

### US6

```text
Task T043: Write public state contract tests in crates/widget/tests/text_editor_contract.rs
Task T044: Write state rendering tests in crates/widget/tests/text_editor_rendering.rs
Task T045: Write disabled pointer/focus tests in crates/widget/tests/text_editor_pointer.rs
```

## Implementation Strategy

### MVP First

1. Complete Setup and Foundation.
2. Complete US1 through T018.
3. Stop and validate multi-line insertion/deletion independently.

### Recommended Core Release

1. Deliver the MVP.
2. Add US2, US3, and US5 in dependency order.
3. Validate keyboard/pointer navigation, clipboard/history, and IME as one P1 core.

### Full Increment

1. Add US4 for long-document layout, wrapping, and scrolling.
2. Add US6 for complete host configuration and capability states.
3. Complete all cross-cutting gates and quickstart scenarios.

## Notes

- Each task contains an exact file path and follows the required checklist syntax.
- Tests precede implementation within each story and must initially fail for the intended reason.
- Changes to `crates/widget/src/text_editor.rs` are intentionally sequential to avoid overlapping
  state-machine and geometry edits.
- Preserve the official GPUI Git dependency and existing `TextInput` behavior throughout.

