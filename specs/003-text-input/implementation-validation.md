# TextInput Implementation Validation

**Zit status (2026-09-05)**: NOT RUN. No implementation is present in this repository.
All evidence below is an imported historical record from the source project, not a Zit result.
Retain it for reference and record new Zit evidence separately after implementation.

## Historical Source-Project Record

**Date**: 2026-09-04
**GPUI source**: Official `https://github.com/zed-industries/zed`, locked at `81da416e`

## Functional requirement trace

| Requirements | Evidence |
|---|---|
| FR-001, FR-006–FR-009, FR-023, FR-025 | `text_input_editing`, `text_input_foundation`, and source Unicode unit tests |
| FR-002–FR-005, FR-014–FR-015, FR-020, FR-024, FR-036 | `text_input_rendering`, `text_input_editing`, pixel-snapped one-pixel caret geometry, inherited text color, and interactive example review path |
| FR-010–FR-013, FR-026 | `text_input_navigation`, `text_input_pointer`, semantic actions, and shaped-line hit testing |
| FR-016–FR-017 | `text_input_clipboard` using GPUI's clipboard service |
| FR-018–FR-019 | `text_input_history` undo, redo, and redo-invalidation coverage |
| FR-021–FR-022, FR-027–FR-028 | `text_input_contract`, public event contract, two-instance example, and Cargo metadata/tree |
| FR-029–FR-035 | `text_input_ime` marked-text lifecycle and UTF-16 platform range coverage |
| FR-038 | `text_input_rendering` constrains default and enlarged text to a one-pixel host height and verifies the TextInput retains the text size plus vertical chrome |
| FR-039 | `text_input_pointer` verifies pointer-acquired focus, clicked caret placement, first-update visibility without clock advancement, and disabled rejection; `text_input_blink` verifies the following 500 ms cycle |

## Success criteria trace

| Criterion | Evidence/status |
|---|---|
| SC-001 | Automated suites cover every behavior family; all 23 tests pass |
| SC-002 | Requires representative-user usability study; release validation pending |
| SC-003 | Editing paths are synchronous; formal 100 ms interaction measurement remains a manual release check |
| SC-004 | Distinct focus, disabled, caret, selection, placeholder, and marked styles implemented; multi-reviewer check pending |
| SC-005 | Two independently configured instances run in `examples/text_input.rs` without widget changes |
| SC-006 | User-changing paths emit `TextInputChange`; no-op/navigation/provisional paths do not emit |
| SC-007 | Caret uses the shaped text style color and pixel-snapped one-pixel bounds; interactive visual confirmation remains a release check |
| SC-008 | `text_input_blink` advances GPUI's deterministic clock across 500 ms boundaries and verifies immediate reset plus ineligible-state stopping |
| SC-009 | Constrained-layout tests cover 14 px and 32 px inherited text while the interactive example requests 8 px for 28 px text |
| SC-010 | Pointer tests observe a visible caret immediately after the focus-producing mouse-down and before advancing the deterministic clock |

## Quality gates

- Workspace compilation: PASS
- Formatting: PASS
- Clippy with warnings denied: PASS
- Full automated test suite: PASS
- Official GPUI dependency provenance: PASS
- Cross-platform visual, physical clipboard, and system IME matrix: PENDING outside this headless
  environment and explicitly retained as a pre-release manual gate
- Cursor blink lifecycle: PASS in deterministic GPUI tests; physical display cadence remains part
  of the interactive example's release check
- Pointer-focus first frame: PASS without deterministic clock advancement; physical display
  confirmation remains part of the interactive example's release check
- Minimum-height layout: PASS at 14 px and 32 px inherited text sizes under a one-pixel requested
  host height; broader platform/font visual confirmation remains a release check

