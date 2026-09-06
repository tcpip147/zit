# Data Model: Reusable Data Grid Widget

## DataGrid

Retained state for one grid instance.

| Field | Meaning | Invariant |
|---|---|---|
| columns | Ordered `DataGridColumn` definitions | Stable order shared by header and every row |
| row_count | Number of host-addressable rows | May be zero; renderer is never asked for an index at or above this value |
| row_renderer | Host callback producing one `DataGridRow` for an index | Contains no project-specific assumptions |
| row_height | Uniform body-row height | Strictly positive |
| header_height | Fixed header height | Strictly positive |
| scroll | Uniform-list scroll state | Offset remains inside measured bounds |
| drag | Optional active thumb drag | Cleared on primary-button release, including outside bounds |
| scrollbar_visible | Current visual visibility | True during active drag or hover |
| hide_generation/task | Delayed-hide ownership | Only the newest scheduled hide may change visibility |
| theme | Grid-specific appearance values | Defaults are usable without host customization |

## DataGridColumn

| Field | Meaning | Validation |
|---|---|---|
| id | Stable host-facing identity | Non-empty and unique within one grid |
| label | Visible and accessible header label | May be empty only when the host intentionally supplies an icon-only accessible equivalent |
| width | Fixed or proportional width policy | Fixed width is positive; fraction weight is finite and positive |
| divider | Whether a trailing divider is displayed | Last-column divider may be omitted by default |

Invalid duplicate IDs or invalid widths cause configuration rejection before grid state is mutated.

## DataGridColumnWidth

- **Fixed**: A positive logical-pixel width.
- **Fraction**: A positive proportional share of the remaining grid width.

The initial project-history migration uses five proportional columns to preserve its current layout.

## DataGridRow

| Field | Meaning | Invariant |
|---|---|---|
| index | Position in the host row set | `0 <= index < row_count` |
| cells | Ordered host-provided cells | Normalized to the declared column count at render time |
| accessible_label | Optional row-level summary | Does not replace cell semantics |

## DataGridCell

A one-use render payload containing either text or a custom GPUI element plus an optional accessible label. Content is clipped to its column and rendered without wrapping by default.

## ScrollbarDrag

| Field | Meaning |
|---|---|
| pointer_start_y | Pointer y-coordinate where the drag began |
| scroll_start_y | Body scroll offset where the drag began |

During movement, the pointer delta is scaled by `max_scroll / thumb_travel`, clamped to the range from the end offset through zero.

## State Transitions

### Scrollbar visibility

```text
Hidden --wheel/track hover/thumb down--> Visible
Visible --thumb down--> Dragging
Dragging --pointer move--> Dragging (bounded offset update)
Dragging --pointer up--> Visible (schedule hide)
Visible --hover exit or wheel idle--> Visible (replace hide task)
Visible --one second idle and not dragging/hovered--> Hidden
```

### Data updates

```text
Configured --set columns--> Revalidated and relaid out
Configured --set row count/renderer--> Scroll bounds clamped and body refreshed
Overflowing --row count shrinks--> Offset clamped to the new maximum
Any --set theme--> Same data and scroll state with refreshed appearance
```

## Relationships

- One `DataGrid` owns many ordered `DataGridColumn` values.
- One `DataGrid` requests zero or more `DataGridRow` values from one host renderer.
- Each row is normalized against the grid's column count.
- Each grid owns exactly one independent viewport and scrollbar state.
- `ProjectPage` owns or retains one history `DataGrid` but continues to own project loading and commit formatting.

