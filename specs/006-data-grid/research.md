# Research: Reusable Data Grid Widget

## Decision 1: Retained entity with host-owned indexed rows

**Decision**: Implement `DataGrid` as a retained GPUI entity that stores a row count and an `Rc`-backed renderer from row index to `DataGridRow`.

**Rationale**: The entity naturally owns independent scroll, timer, and drag state. Indexed rendering fits GPUI's uniform-list range requests and avoids coupling the widget to a concrete collection or domain type. Capturing an immutable host snapshot avoids borrowing a parent entity while the child grid renders.

**Alternatives considered**:

- Store every row in the grid: simple, but duplicates large host data and weakens virtualization.
- Make the grid generic over a row type: preserves type safety but makes public entities and heterogeneous custom cells substantially harder to expose.
- Read the parent project page from the row callback: risks nested application borrows during rendering and couples lifecycle behavior.

## Decision 2: GPUI uniform-list virtualization

**Decision**: Use GPUI's uniform list and `UniformListScrollHandle` with one configurable row height.

**Rationale**: The existing project history already uses this mechanism successfully. It provides range-based rendering, measured scroll bounds, tracked wheel behavior, and a direct migration path for 10,000-row scenarios.

**Alternatives considered**:

- Render all rows in a scroll container: fails the large-row performance requirement.
- Build a custom layout and viewport algorithm: duplicates framework behavior and increases correctness risk.
- Support variable-height rows: incompatible with the chosen uniform virtualization model and outside the requested extraction.

## Decision 3: Declarative columns with fixed or proportional widths

**Decision**: Represent each column with stable identity, label, and either fixed-pixel or proportional width. Header and rows share the same computed grid template.

**Rationale**: Stable identity supports diagnostics and future evolution, while fixed and proportional widths cover the current equal-width history table and common reusable layouts without introducing resizing.

**Alternatives considered**:

- Equal-width columns only: insufficient for reusable grids with narrow icon or hash columns.
- Fully resizable columns: adds pointer state, persistence, and accessibility outside this feature.
- Let each row choose widths: permits header/body misalignment.

## Decision 4: Normalize row cell counts at the widget boundary

**Decision**: Pad missing cells with empty content and ignore cells beyond the declared column count.

**Rationale**: This matches FR-005, keeps the layout stable, and prevents one malformed host row from altering the grid structure or crashing presentation.

**Alternatives considered**:

- Reject the whole row: unnecessarily hides otherwise valid content.
- Panic in debug or production: violates reusable-widget robustness.
- Add undeclared columns dynamically: breaks header alignment and predictable accessibility.

## Decision 5: Extract and generalize the existing scrollbar behavior

**Decision**: Preserve the current proportional thumb, 24-pixel minimum, hover/wheel/drag reveal, one-second delayed hide, and window-level drag completion.

**Rationale**: It maintains behavioral compatibility for project history and satisfies long-list navigation without adding a new interaction pattern.

**Alternatives considered**:

- Always-visible scrollbar: simpler but changes existing history behavior.
- Framework-native scrollbar only: does not preserve the requested current control presentation.
- Pointer capture limited to the grid element: can leave a drag stuck when release occurs outside bounds.

## Decision 6: Shared theme extension

**Decision**: Add `DataGridTheme` to the existing `Theme`, with defaults matching the current history header, divider, row text, track, thumb, and thumb-hover colors.

**Rationale**: The widget crate already centralizes tab, input, and editor appearance in `Theme`. Moving hard-coded history colors into it makes the extracted grid reusable and configurable.

**Alternatives considered**:

- Keep colors in the generic module: prevents host theming.
- Require every constructor to provide all colors: makes the common case verbose and diverges from existing widgets.
- Reuse `TextEditorTheme`: its semantics do not match headers, cells, and dividers.

## Decision 7: Hyphenated source filename via path attribute

**Decision**: Store the implementation at `crates/widget/src/data-grid.rs` and declare it in `lib.rs` with `#[path = "data-grid.rs"] mod data_grid;`.

**Rationale**: Rust module identifiers cannot contain hyphens, but the user explicitly fixed the filesystem path. The path attribute preserves the exact filename while exposing conventional `DataGrid` APIs.

**Alternatives considered**:

- Rename the file to `data_grid.rs`: conventional, but does not satisfy the explicit path requirement.
- Create a separate crate named `data-grid`: unnecessary architectural expansion.

