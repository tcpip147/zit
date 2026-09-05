# Data Model: Component Tab Bar

| Entity | Fields | Rules |
|---|---|---|
| TabId | stable opaque string | Unique per bar; independent of name and index |
| TabEntry | id, name, SVG path, page entity, focus handle | Exactly one page; complete name retained |
| TabBarState | ordered entries, active id, focused id, scroll handle | Ids refer to entries or are none |
| TabViewport | offset, maximum offset, child bounds | Offset clamped; zero when content fits |
| TabBarEvent | selection changed, close requested, closed | Stable id; request precedes removal |

## Validation

- Reject duplicate IDs without mutation. Normalize invalid active ID to first tab or none.
- Empty, long, and duplicate names are valid; identity never derives from labels.
- Missing SVG resolves to fallback. Rejected closure preserves all state.

## State Transitions

- First insertion selects, focuses, and reveals; later insertion preserves selection unless activated.
- Selecting an inactive ID updates active/focused ID, reveals, and emits; active selection is idempotent.
- Inactive closure preserves active ID. Active closure chooses right else left; final closure clears state.
- Wheel over overflow maps to x, clamps, and notifies. Resize remeasures, clamps, and reveals active.

## Rendering Invariants

- Exactly one active page, or none when empty.
- Every tab orders project SVG, `text_xs` name, then X SVG.
- Tab and close hit regions are separate; close consumes input.
- Active tab is fully visible and addressed by stable identity.
