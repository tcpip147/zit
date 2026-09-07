---
description: "Dependency-ordered tasks for Diff Editor"
---

# Tasks: Diff Editor

## Phase 1: Foundation

- [ ] T001 Extract DiffEditor ownership and public export into `crates/widget/src/diff_editor.rs` and `crates/widget/src/lib.rs`
- [ ] T002 [P] Define owned parsed-row types and fixed-row geometry constants in `crates/widget/src/diff_editor.rs`
- [ ] T003 [P] Add DiffEditor contract fixture scaffolding in `crates/widget/tests/diff_editor_contract.rs`

## Phase 2: User Story 1 - Read Parsed File Changes

- [ ] T004 [US1] Parse hunk ranges, additions, deletions, context rows, and hidden diff headers in `crates/widget/src/diff_editor.rs`
- [ ] T005 [US1] Calculate before/after line numbers and dynamic gutter widths in `crates/widget/src/diff_editor.rs`
- [ ] T006 [US1] Render row colors, hunk separators, fixed line-number columns, and bottom clearance in `crates/widget/src/diff_editor.rs`
- [ ] T007 [US1] Add parser and rendering contract cases in `crates/widget/tests/diff_editor_contract.rs`

## Phase 3: User Story 2 - Navigate Large and Wide Diffs

- [ ] T008 [US2] Split gutter and code panes while preserving synchronized vertical movement and code-only horizontal movement in `crates/widget/src/diff_editor.rs`
- [ ] T009 [US2] Implement visible-range virtualization, overscan, total content sizing, and stale-index clamping in `crates/widget/src/diff_editor.rs`
- [ ] T010 [US2] Reset vertical and horizontal offsets on diff replacement and connect project-page updates in `crates/zit/src/project/page.rs`
- [ ] T011 [US2] Add large-diff, fixed-gutter, long-line, and file-switch regression cases in `crates/widget/tests/diff_editor_contract.rs` and `crates/zit/tests/project_page_contract.rs`

## Phase 4: User Story 3 - Control Scrollbars Directly

- [ ] T012 [US3] Implement axis-correct scrollbar track/thumb geometry and boundary clamping in `crates/widget/src/diff_editor.rs`
- [ ] T013 [US3] Implement thumb drag capture, outside release, and track-click thumb centering in `crates/widget/src/diff_editor.rs`
- [ ] T014 [US3] Implement hover/drag thumb colors and independent axis state in `crates/widget/src/diff_editor.rs`
- [ ] T015 [US3] Add matching vertical drag, track-click, hover, and outside-release behavior to `crates/widget/src/scroll_container.rs`
- [ ] T016 [US3] Add scrollbar interaction regression coverage in `crates/widget/tests/diff_editor_contract.rs`

## Phase 5: Polish

- [ ] T017 [P] Update `specs/008-diff-editor/quickstart.md` with runnable parser and interaction validation steps
- [ ] T018 Run formatting, workspace checks, and widget/project regression tests

## Dependencies

Foundation → US1 → US2 → US3 → Polish. Parser and fixture scaffolding can proceed in parallel; scrollbar work depends on the final pane geometry.

## MVP

Deliver US1 and US2 first: readable parsed diffs with fixed gutters, synchronized vertical scrolling, code-only horizontal scrolling, and virtualization.
