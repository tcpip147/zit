# Public Contract: DataGrid Widget

## Module and exports

- Implementation file: `crates/widget/src/data-grid.rs`
- Rust module identifier: `data_grid`, declared with an explicit path attribute in `crates/widget/src/lib.rs`
- Public exports: `DataGrid`, `DataGridColumn`, `DataGridColumnWidth`, `DataGridRow`, `DataGridCell`, `DataGridError`, and `DataGridTheme`

Exact Rust signatures may be refined during implementation, but the observable contract below is stable.

## Construction contract

Construction requires:

1. An ordered column collection.
2. A non-negative row count.
3. A retained row-renderer callback capable of producing a row for a requested valid index.
4. A positive uniform row height, defaulting to the shared widget value.

Construction rejects duplicate column IDs, empty column IDs, non-positive fixed widths, and non-finite or non-positive fractional widths. Failure returns a typed configuration error without a partially initialized grid.

## Column contract

Each column exposes stable identity, visible and accessible label, fixed or proportional width, and optional trailing divider presentation. Header and body cells use the same ordered width definition. Hosts may replace all columns atomically; invalid replacement leaves the prior configuration unchanged.

## Row-renderer contract

- The grid invokes the renderer only with `index < row_count`.
- The renderer is called lazily for the range requested by the visible uniform list.
- Returning fewer cells than columns pads the row with empty cells.
- Returning more cells than columns ignores the excess cells.
- A cell may hold plain text or an arbitrary one-use visual element.
- The callback owns domain formatting; the grid does not inspect application data.

## Mutation and inspection contract

The public API supports replacing columns after validation; replacing row count and renderer together; updating row count with a compatible renderer; setting and reading `DataGridTheme`; reading row count, columns, scroll offset, and scrollbar visibility; and requesting that a valid row index be scrolled into view.

Any data update clamps the existing scroll offset to the new valid range. An invalid scroll-to-row request returns `false` and leaves state unchanged.

## Rendering contract

1. The header remains outside the vertically scrolling body.
2. The body uses uniform-height range rendering.
3. Header and row cells share column alignment.
4. Cell overflow is clipped and does not change neighboring cell bounds.
5. Zero columns and zero rows render safely.
6. The widget supplies no empty-state message.

## Scroll-control contract

1. The thumb is functional only when the body overflows vertically.
2. Thumb size is proportional to the viewport with a 24-logical-pixel minimum, capped by track height.
3. Thumb position represents the clamped body offset.
4. Wheel activity, track hover, and dragging reveal the control.
5. One second after activity ends, the control hides unless hovered or dragged.
6. Window-level move and primary-button-up observation keeps dragging coherent outside widget bounds.
7. Each grid uses independent state and a unique internal element identity.

## Accessibility contract

- The outer component exposes table/grid semantics and an optional host label.
- The header exposes one row and one column-header node per visible column.
- Each rendered body row exposes a row node and one cell node per declared column.
- Available row/column position and count metadata matches host indexes and declarations.
- Custom visual cells may provide an explicit accessible label when visual content alone is insufficient.

## Project-history adapter contract

The migrated History section configures Graph, Commit, Author, Date, and Hash columns in that order. The project adapter supplies graph visualization, commit summary, author, authored time, and an abbreviated object ID. `DataGrid` imports no types from `zit`, does not load repositories, and does not own loading, error, retry, or empty-history states.

